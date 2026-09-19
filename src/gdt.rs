use crate::println;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct GdtEntry {
    pub limit_low: u16,
    pub base_low: u16,
    pub base_middle: u8,
    pub access: u8,
    pub granularity: u8,
    pub base_high: u8,
}

#[repr(C, packed)]
pub struct GdtPointer {
    limit: u16,
    base: u64,
}

pub static mut GDT: [u64; 6] = [
    0x0000000000000000, // 0x00 null
    0x00AF9A000000FFFF, // 0x08 kernel code
    0x00AF92000000FFFF, // 0x10 kernel data
    0,                   // 0x18 TSS low
    0,                   // 0x20 TSS high
    0,
];

unsafe fn set_tss_descriptor(base: u64, limit: u32) {
    let low =
        ((limit as u64) & 0xFFFF)
        | ((base & 0xFFFFFF) << 16)
        | (0x89u64 << 40)
        | ((((limit as u64) >> 16) & 0xF) << 48)
        | (((base >> 24) & 0xFF) << 56);

    let high = base >> 32;

    let gdt = core::ptr::addr_of_mut!(GDT) as *mut u64;

    core::ptr::write_unaligned(gdt.add(3), low);
    core::ptr::write_unaligned(gdt.add(4), high);
}

pub unsafe fn init() {
let tss_base = core::ptr::addr_of!(crate::tss::TSS) as u64;
let tss_limit = (core::mem::size_of::<crate::tss::Tss>() - 1) as u32;

set_tss_descriptor(tss_base, tss_limit);

   let gdt_limit = (core::mem::size_of::<[u64; 6]>() - 1) as u16;
let gdt_base = core::ptr::addr_of!(GDT) as u64;

let gdt_ptr = GdtPointer {
    limit: gdt_limit,
    base: gdt_base,
};


core::arch::asm!(
    "lgdt [{0}]",
    in(reg) core::ptr::addr_of!(gdt_ptr),
    options(readonly, nostack, preserves_flags)
);


core::arch::asm!(
    "push 0x08",
    "lea rax, [rip + 2f]",
    "push rax",
    "retfq",
    "2:",
    "mov ax, 0x10",
    "mov ds, ax",
    "mov es, ax",
    "mov ss, ax",
    out("rax") _,
);


}