use crate::println;

#[repr(align(16))]
pub struct Stack {
    pub data: [u8; 4096],
}

pub static mut DF_STACK: Stack = Stack {
    data: [0; 4096],
};

#[repr(C, packed)]
pub struct Tss {
    pub reserved1: u32,
    pub rsp: [u64; 3],
    pub reserved2: u64,
    pub ist: [u64; 7],
    pub reserved3: u64,
    pub reserved4: u16,
    pub iomap_base: u16,
}

pub static mut TSS: Tss = Tss {
    reserved1: 0,
    rsp: [0; 3],
    reserved2: 0,
    ist: [0; 7],
    reserved3: 0,
    reserved4: 0,
    iomap_base: 0,
};

pub unsafe fn init() {
    let stack_top =
        core::ptr::addr_of!(DF_STACK) as u64
        + core::mem::size_of::<Stack>() as u64;

    TSS.ist[0] = stack_top;

    println!("DF stack: {:#x}", stack_top);
}