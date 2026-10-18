#![allow(non_snake_case)] 
#![no_std] 
#![no_main] 
#![feature(abi_x86_interrupt)] 
#![feature(ptr_alignment_type)]
 
extern crate alloc;

mod LFB; 
mod COM; 
mod gdt; 
mod idt; 
mod stubs; 
mod handlers; 
mod tss; 
mod bitmap; 
mod HHDM; 
mod pmm; 
mod vmm; 
mod kheap;
mod ps2;
 
use core::{arch::asm, fmt, panic::PanicInfo}; 
use alloc::boxed::Box;
use alloc::string::String;
use limine::request::{FramebufferRequest, MemmapRequest}; 
use spin::Mutex; 
use ps2::read_key;
use crate::{ 
	LFB::{FramebufferWriter, lfb}, gdt::GdtEntry, kheap::KernelHeap, vmm::VMM 
}; 
 
// Globalne schowki 
pub static GLOBAL_LFB: Mutex<Option<lfb>> = Mutex::new(None); 
pub static GLOBAL_FBW: Mutex<Option<FramebufferWriter>> = Mutex::new(None); 
pub static GLOBAL_PMM: Mutex<Option<pmm::PMM>> = Mutex::new(None); 
pub static GLOBAL_VMM: Mutex<Option<vmm::VMM>> = Mutex::new(None);

#[global_allocator]
pub static KERNEL_HEAP: KernelHeap = KernelHeap::new_uninitialized();
 
const PAGE_SIZE: usize = 4096; 
const PAGES_PER_GIB: usize = (1024 * 1024 * 1024) / PAGE_SIZE; 
 
#[used] 
#[link_section = ".requests"] 
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new(); 
 
#[used] 
#[link_section = ".requests"] 
static MEMMAP_REQUEST: MemmapRequest = MemmapRequest::new(); 
 
#[no_mangle] 
pub extern "C" fn _start() -> ! { 
 
	// Inicjalizujemy LFB z requestu Limine 
	let framebuffer_instance = 
		lfb::init(&FRAMEBUFFER_REQUEST).unwrap(); 
 
	let writer_instance = 
		FramebufferWriter::new(0x0000A6A6, 0x00000000); 
 
	unsafe { 
		tss::init(); 
		gdt::init(); 
		idt::init(); 
		HHDM::init(); 
	} 
 
	// Globalne instancje 
	*GLOBAL_LFB.lock() = Some(framebuffer_instance); 
	*GLOBAL_FBW.lock() = Some(writer_instance); 
 
	// PMM 
	let pmm_instance = match pmm::PMM::init(&MEMMAP_REQUEST) { 
		Some(pmm) => pmm, 
		None => panic!("PMM initialization failed"), 
	}; 
 
	*GLOBAL_PMM.lock() = Some(pmm_instance); 
 
	// VMM 
	let vmm_instance = VMM::init(); 
	*GLOBAL_VMM.lock() = Some(vmm_instance); 

	// kheap
	let kheap_address = GLOBAL_PMM.lock().as_mut().unwrap().alloc_page().unwrap();

	unsafe {
		KERNEL_HEAP.heap.lock().init(kheap_address, 0x200000, 32);
	}

 
	println!("[ AceOS Kernel Starting... ]"); 
 
	println!( 
"######A######\n
  ####A A####\n
   ##AAAAA##\n
   #A     A#" 
	); 
 
	println!( 
		"Free RAM: {:.2} GiB", 
		GLOBAL_PMM 
			.lock() 
			.as_ref() 
			.unwrap() 
			.free_pages as f32 
			/ PAGES_PER_GIB as f32 
	); 
 
	GLOBAL_FBW 
		.lock() 
		.as_mut() 
		.unwrap() 
		.change_colors(0x00FF0000, 0x00000000); 

	GLOBAL_FBW 
		.lock() 
		.as_mut() 
		.unwrap() 
		.change_colors(0x0000FF00, 0x00000000); 
 
	println!("keyboard doesn't work yet"); 
	println!("Scheduler doesn't exist yet"); 
	println!("Nvme Disk doesn't work yet"); 

	print!("AceCore> ");
 
	loop {read_key();} 
} 
 
#[panic_handler] 
fn panic(info: &PanicInfo) -> ! { 
	println!("\n!!! KERNEL PANIC !!!"); 
	println!("{}", info); 
 
	loop {} 
}

