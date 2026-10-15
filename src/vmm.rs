use core::arch::asm;

use crate::{GLOBAL_PMM, HHDM::{self, phys_to_virt}, println, vmm::{TypesOfPTE::{pde, pml4e, pte}, UnmapError::DoesntExist}};

const PRESENT: u64  = 1 << 0;
const WRITABLE: u64 = 1 << 1;
const USER: u64     = 1 << 2;
const HPF: u64 		= 1 << 7;
const NX: u64       = 1 << 63;

#[derive(Debug)]
pub enum UnmapError {
    DoesntExist(TypesOfPTE),
	FatalError
}

#[derive(Debug)]
pub enum TypesOfPTE {
	pml4e,
	pdpte,
	pde,
	pte,
}

pub struct VMM {
    pml4_phys: usize,
}

impl VMM {
	pub fn init() -> Self {
        let cr3 = read_cr3();

        let pml4_phys = cr3 & 0x000F_FFFF_FFFF_F000;

        Self {
            pml4_phys,
        }
    }


	// pub fn map_page(&mut self, virt: usize, phys: usize, huge: bool) {
	// 	let pml4_i = (virt >> 39) & 0x1FF;
	// 	let pdpt_i = (virt >> 30) & 0x1FF;
	// 	let pd_i = (virt >> 21) & 0x1FF;
	// 	let pt_i = (virt >> 12) & 0x1FF;

	// 	let pml4 = Self::phys_to_pt(self.pml4_phys);

	// 	let pml4_entry = pml4.entries[pml4_i];

	// 	if pml4_entry.0 & PRESENT == 0 {
	// 		pml4.entries[pml4_i] =
	// 			PageTableEntry(PageTable::alloc_table());

	// 		let pdpt_entry = pml4.entries[pml4_i];
	// 		let pdpt = Self::phys_to_pt(pdpt_entry.0 as usize);

	// 		pdpt.fill_zeros();

	// 		pdpt.entries[pdpt_i] =
	// 			PageTableEntry(PageTable::alloc_table() | PRESENT | WRITABLE);

	// 		let pd_entry = pdpt.entries[pdpt_i];
	// 		let pd = Self::phys_to_pt(pd_entry.0 as usize);

	// 		pd.fill_zeros();

	// 		if huge {
	// 			pd.entries[pd_i] =
	// 				PageTableEntry((phys as u64) | PRESENT | WRITABLE | HPF);
	// 			return;
	// 		}

	// 		pd.entries[pd_i] =
	// 			PageTableEntry(PageTable::alloc_table() | PRESENT | WRITABLE);

	// 		let pt_entry = pd.entries[pd_i];
	// 		let pt = Self::phys_to_pt(pt_entry.0 as usize);

	// 		pt.fill_zeros();

	// 		pt.entries[pt_i] =
	// 			PageTableEntry((phys as u64) | PRESENT | WRITABLE);
	// 	} else {
	// 		let pdpt = Self::phys_to_pt(pml4_entry.0 as usize);
	// 		let pdpt_entry = pdpt.entries[pdpt_i];

	// 		if pdpt_entry.0 & PRESENT == 0 {
	// 			pdpt.entries[pdpt_i] =
	// 				PageTableEntry(PageTable::alloc_table() | PRESENT | WRITABLE);

	// 			let pd_entry = pdpt.entries[pdpt_i];
	// 			let pd = Self::phys_to_pt(pd_entry.0 as usize);

	// 			pd.fill_zeros();

	// 			if huge {
	// 				pd.entries[pd_i] =
	// 					PageTableEntry((phys as u64) | PRESENT | WRITABLE | HPF);
	// 				return;
	// 			}

	// 			pd.entries[pd_i] =
	// 				PageTableEntry(PageTable::alloc_table() | PRESENT | WRITABLE);

	// 			let pt_entry = pd.entries[pd_i];
	// 			let pt = Self::phys_to_pt(pt_entry.0 as usize);

	// 			pt.fill_zeros();

	// 			pt.entries[pt_i] =
	// 				PageTableEntry((phys as u64) | PRESENT | WRITABLE);
	// 		} else {
	// 			let pd = Self::phys_to_pt(pdpt_entry.0 as usize);
	// 			let pd_entry = pd.entries[pd_i];

	// 			if huge {
	// 				pd.entries[pd_i] =
	// 					PageTableEntry((phys as u64) | PRESENT | WRITABLE | HPF);
	// 				return;
	// 			}

	// 			if pd_entry.0 & PRESENT == 0 {
	// 				pd.entries[pd_i] =
	// 					PageTableEntry(PageTable::alloc_table() | PRESENT | WRITABLE);

	// 				let pt_entry = pd.entries[pd_i];
	// 				let pt = Self::phys_to_pt(pt_entry.0 as usize);

	// 				pt.fill_zeros();

	// 				pt.entries[pt_i] =
	// 					PageTableEntry((phys as u64) | PRESENT | WRITABLE);
	// 			} else {
	// 				let pt = Self::phys_to_pt(pd_entry.0 as usize);

	// 				pt.entries[pt_i] =
	// 					PageTableEntry((phys as u64) | PRESENT | WRITABLE);
	// 			}
	// 		}
	// 	}
	// }

	pub fn map_page(&mut self, virt: usize, phys: usize, huge: bool){
		let pml4_i = (virt >> 39) & 0x1FF;
		let pdpt_i = (virt >> 30) & 0x1FF;
		let pd_i = (virt >> 21) & 0x1FF;
		let pt_i = (virt >> 12) & 0x1FF;

		let pml4 = Self::phys_to_pt(self.pml4_phys);

		let pml4_entry = pml4.entries[pml4_i];

		if pml4_entry.0 & PRESENT == 0{
			pml4.entries[pml4_i] = PageTableEntry(PageTable::alloc_table() | PRESENT | WRITABLE);
		}

		let pdpt = Self::phys_to_pt(pml4.entries[pml4_i].0 as usize);

		let pdpt_entry = pdpt.entries[pdpt_i];

		if pdpt_entry.0 & PRESENT == 0{
			pdpt.entries[pdpt_i] = PageTableEntry(PageTable::alloc_table() | PRESENT | WRITABLE);
		}

		let pd = Self::phys_to_pt(pdpt.entries[pdpt_i].0 as usize);

		let pd_entry = pd.entries[pd_i];

		if pd_entry.0 & PRESENT == 0{
			pd.entries[pd_i] = PageTableEntry(PageTable::alloc_table() | PRESENT | WRITABLE);
		}

		let pt = Self::phys_to_pt(pd.entries[pd_i].0 as usize);

		pt.entries[pt_i] = PageTableEntry(phys as u64 | PRESENT | WRITABLE);

		println!("pdpt[{}] = {:#x}", pdpt_i, pdpt.entries[pdpt_i].0);
		println!("pd[{}]   = {:#x}", pd_i, pd.entries[pd_i].0);

	}

	// pub fn unmap_page(&mut self, virt: usize) -> Result<(), UnmapError>{
	// 	let pml4_i = (virt >> 39) & 0x1FF;
	// 	let pdpt_i = (virt >> 30) & 0x1FF;
	// 	let pd_i   = (virt >> 21) & 0x1FF;
	// 	let pt_i   = (virt >> 12) & 0x1FF;

	// 	let pml4 = Self::phys_to_pt(self.pml4_phys);

	// 	let pml4_entry = pml4.entries[pml4_i];

	// 	if pml4_entry.0 & PRESENT == 0{
	// 		return Err(DoesntExist(TypesOfPTE::pml4e));
	// 	}

	// 	let pdpt = Self::phys_to_pt(pml4_entry.0 as usize);

	// 	let pdpt_entry = pdpt.entries[pdpt_i];

	// 	if pdpt_entry.0 & PRESENT == 0{
	// 		return Err(DoesntExist(TypesOfPTE::pdpte));
	// 	}

	// 	let pd = Self::phys_to_pt(pdpt_entry.0 as usize);

	// 	let pd_entry = pd.entries[pd_i];

	// 	if pd_entry.0 & PRESENT == 0{
	// 		return Err(DoesntExist(TypesOfPTE::pde));
	// 	}

	// 	let pt = Self::phys_to_pt(pd_entry.0 as usize);

	// 	let pt_entry = pt.entries[pt_i];

	// 	if pt_entry.0 & PRESENT == 0 {
   	// 		return Err(DoesntExist(TypesOfPTE::pte));
	// 	}
		
	// 	println!("without mask: {:#x}", pt_entry.0);

	// 	let phys = (pt_entry.0 & 0x000F_FFFF_FFFF_F000) as usize;

	// 	println!("before: {}", pt.entries[pt_i].0);

	// 	pt.entries[pt_i] = PageTableEntry(0);

	// 	println!("after: {}", pt.entries[pt_i].0);

	// 	println!("unmap: virt={:#x}, phys={:#x}", virt, phys);

	// 	if pt_entry.0 & PRESENT != 0{
	// 		GLOBAL_PMM
    // 		.lock()
	//     	.as_mut()
    // 		.unwrap()
    // 		.free_page(phys);
	// 	}

	// 	println!("pml4_phys: {}", self.pml4_phys);
	// 	println!("CR3: {}", read_cr3());
		

	// 	unsafe {
    // 		core::arch::asm!("invlpg [{}]", in(reg) virt, options(nostack, preserves_flags));
	// 	}

	// 	Ok(())
	// }

	pub fn unmap_page(&mut self, virt: usize) -> Result<(), UnmapError>{
		let pml4_i = (virt >> 39) & 0x1FF;
		let pdpt_i = (virt >> 30) & 0x1FF;
		let pd_i   = (virt >> 21) & 0x1FF;
		let pt_i   = (virt >> 12) & 0x1FF;

		let pml4 = Self::phys_to_pt(self.pml4_phys);

		if pml4.entries[pml4_i].0 & PRESENT == 0{
			return Err(UnmapError::DoesntExist(TypesOfPTE::pml4e));
		} 

		let pdpt = Self::phys_to_pt(pml4.entries[pml4_i].0 as usize);

		if pdpt.entries[pdpt_i].0 & PRESENT == 0{
			return Err(UnmapError::DoesntExist(TypesOfPTE::pdpte));
		}

		let pd = Self::phys_to_pt(pdpt.entries[pdpt_i].0 as usize);

		if pd.entries[pd_i].0 & PRESENT == 0{
			return Err(UnmapError::DoesntExist(TypesOfPTE::pde));
		}

		let pt = Self::phys_to_pt(pd.entries[pd_i].0 as usize);

		if pt.entries[pt_i].0 & PRESENT == 0{
			return Err(UnmapError::DoesntExist(TypesOfPTE::pte));
		}

		pt.entries[pt_i] = PageTableEntry(0);

		unsafe{
			asm!("invlpg [{}]", in(reg) virt, options(nostack, preserves_flags));
		}

		if self.does_entry_exist(virt){
			return Err(UnmapError::FatalError);
		}

		println!("pdpt[{}] = {:#x}", pdpt_i, pdpt.entries[pdpt_i].0);
		println!("pd[{}]   = {:#x}", pd_i, pd.entries[pd_i].0);

		Ok(())
	}

	pub fn does_entry_exist(&self, virt: usize) -> bool{
		let pml4_i = (virt >> 39) & 0x1FF;
		let pdpt_i = (virt >> 30) & 0x1FF;
		let pd_i   = (virt >> 21) & 0x1FF;
		let pt_i   = (virt >> 12) & 0x1FF;

		let pml4 = Self::phys_to_pt(self.pml4_phys);

		if pml4.entries[pml4_i].0 & PRESENT == 0{
			println!("error becouse in pml4");
			return false;
		} 

		let pdpt = Self::phys_to_pt(pml4.entries[pml4_i].0 as usize);

		if pdpt.entries[pdpt_i].0 & PRESENT == 0{
			println!("error becouse in pdpt");
			return false;
		}

		let pd = Self::phys_to_pt(pdpt.entries[pdpt_i].0 as usize);

		if pd.entries[pd_i].0 & PRESENT == 0{
			println!("error becouse in pd");
			return false;
		}

		let pt = Self::phys_to_pt(pd.entries[pd_i].0 as usize);

		if pt.entries[pt_i].0 & PRESENT == 0{
			println!("error becouse in pt");
			return false;
		}

		true
	}

	fn phys_to_pt(ptr: usize) -> &'static mut PageTable{
		let page_table_virt = (ptr & 0x000F_FFFF_FFFF_F000)+ HHDM::offset() as usize;

		let page_table = unsafe {
    		&mut *(page_table_virt as *mut PageTable)
		};

		page_table
	}
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PageTableEntry(u64);

#[repr(C, align(4096))]
pub struct PageTable {
    pub entries: [PageTableEntry; 512],
}
impl PageTable {
	pub fn fill_zeros(&mut self) {
		self.entries.fill(PageTableEntry(0));
	}
	pub fn alloc_table() -> u64 {
		let phys = GLOBAL_PMM.lock().as_mut().unwrap().alloc_page().unwrap();
		VMM::phys_to_pt(phys).fill_zeros();
		phys as u64 | PRESENT | WRITABLE
	}
}

pub fn read_cr3() -> usize {
    let value: usize;

    unsafe {
        asm!("mov {}, cr3", out(reg) value);
    }

    value
}