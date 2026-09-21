use core::arch::asm;

use crate::{HHDM, println};

const PRESENT: u64  = 1 << 0;
const WRITABLE: u64 = 1 << 1;
const USER: u64     = 1 << 2;
const NX: u64       = 1 << 63;

pub struct VMM {
    pml4_phys: usize,
}

impl VMM {
    pub fn init() -> Self {
        let cr3 = read_cr3();

        let pml4_phys = cr3 & 0x000F_FFFF_FFFF_F000;

        let pml4_virt =
            pml4_phys + HHDM::offset() as usize;

        let _pml4 = unsafe {
            &mut *(pml4_virt as *mut PageTable)
        };

        println!("CR3: {:#x}", cr3);
        println!("PML4 physical: {:#x}", pml4_phys);
        println!("PML4 virtual: {:#x}", pml4_virt);

        Self {
            pml4_phys,
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct PageTableEntry(u64);

#[repr(C, align(4096))]
pub struct PageTable {
    pub entries: [PageTableEntry; 512],
}

pub fn read_cr3() -> usize {
    let value: usize;

    unsafe {
        asm!("mov {}, cr3", out(reg) value);
    }

    value
}