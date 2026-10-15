use core::alloc::{GlobalAlloc, Layout};
use spin::Mutex;

use crate::{
	GLOBAL_PMM,
	GLOBAL_VMM,
	bitmap::Bitmap,
};

const PAGE_SIZE: usize = 4096;

pub struct KernelHeap {
	pub heap: Mutex<HeapState>,
}

pub struct HeapState {
	start: usize,
	size: usize,
	block_size: usize,
	map: Option<Bitmap<'static>>,
}

impl HeapState {
	pub const fn new_uninitialized() -> Self {
		Self {
			start: 0,
			size: 0,
			block_size: 0,
			map: None,
		}
	}

	/// # Safety
	/// `start` musi być wyrównany do strony, a zakres
	/// `start..start + size` musi być zarezerwowany dla heapu.
	pub unsafe fn init(&mut self, start: usize, size: usize, block_size: usize) {
		assert!(block_size > 0);
		assert!(size > 0);
		assert!(start.is_multiple_of(PAGE_SIZE));
		assert!(size.is_multiple_of(PAGE_SIZE));
		assert!(block_size.is_power_of_two());

		for addr in (start..start + size).step_by(PAGE_SIZE) {
			let phys = GLOBAL_PMM
				.lock()
				.as_mut()
				.unwrap()
				.alloc_page()
				.unwrap();

			GLOBAL_VMM
				.lock()
				.as_mut()
				.unwrap()
				.map_page(addr, phys, false);
		}

		let blocks = size / block_size;
		let map_bytes = blocks.div_ceil(8);

		let data = unsafe {
			core::slice::from_raw_parts_mut(start as *mut u8, map_bytes)
		};

		let mut map = Bitmap::new(data);

		// Zarezerwuj bloki zajmowane przez samą bitmapę.
		let map_blocks = map_bytes.div_ceil(block_size);

		for b in map_blocks..blocks {
			map.mark_free(b);
		}

		self.start = start;
		self.size = size;
		self.block_size = block_size;
		self.map = Some(map);
	}
}

impl KernelHeap {
	pub const fn new_uninitialized() -> Self {
		Self {
			heap: Mutex::new(HeapState::new_uninitialized()),
		}
	}

	/// # Safety
	/// Warunki bezpieczeństwa są takie same jak w `HeapState::init`.
	pub unsafe fn init(&self, start: usize, size: usize, block_size: usize) {
		self.heap.lock().init(start, size, block_size);
	}
}

unsafe impl GlobalAlloc for KernelHeap {
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		let mut heap = self.heap.lock();

		let start = heap.start;
		let block_size = heap.block_size;

		if block_size == 0 || layout.size() == 0 {
			return core::ptr::null_mut();
		}

		// Alokator blokowy obsługuje wyrównanie do wielkości bloku.
		if layout.align() > block_size {
			return core::ptr::null_mut();
		}

		let needed_blocks = layout.size().div_ceil(block_size);

		let Some(map) = heap.map.as_mut() else {
			return core::ptr::null_mut();
		};

		let mut block_count = 0;
		let mut first_free = 0;

		for i in 0..map.bits {
			if !map.get(i) {
				if block_count == 0 {
					first_free = i;
				}

				block_count += 1;

				if block_count == needed_blocks {
					for j in first_free..first_free + needed_blocks {
						map.set(j, true);
					}

					return (start + first_free * block_size) as *mut u8;
				}
			} else {
				block_count = 0;
			}
		}

		core::ptr::null_mut()
	}

	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		if ptr.is_null() || layout.size() == 0 {
			return;
		}

		let mut heap = self.heap.lock();

		let start = heap.start;
		let size = heap.size;
		let block_size = heap.block_size;

		if block_size == 0 {
			return;
		}

		let address = ptr as usize;

		// Nie pozwól zwolnić pamięci spoza heapu.
		if address < start || address >= start + size {
			return;
		}

		let offset = address - start;

		// Wskaźnik musi wskazywać początek bloku.
		if !offset.is_multiple_of(block_size) {
			return;
		}

		let index = offset / block_size;
		let block_count = layout.size().div_ceil(block_size);

		let Some(map) = heap.map.as_mut() else {
			return;
		};

		// Nie wychodź poza bitmapę.
		if index + block_count > map.bits {
			return;
		}

		for i in index..index + block_count {
			map.set(i, false);
		}
	}
}