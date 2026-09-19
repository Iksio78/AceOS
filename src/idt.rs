use crate::{handlers::breakpoint_handler, println};

unsafe extern "C"{
	fn invalid_opcode_stub();
    fn page_fault_stub();
	fn general_protection_stub();
	fn double_fault_stub();
}


#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IdtEntry {
	pub offset_low: u16,
	pub selector: u16,
	pub ist: u8,
	pub type_attr: u8,
	pub offset_middle: u16,
	pub offset_high: u32,
	pub zero: u32,
}
impl IdtEntry {
	pub fn new(handler: u64) -> Self {
		Self {
			offset_low: handler as u16,
			selector: 0x08,
			ist: 0,
			type_attr: 0x8E,
			offset_middle: (handler >> 16) as u16,
			offset_high: (handler >> 32) as u32,
			zero: 0,
		}
	}
}
#[repr(C, packed)]
pub struct IdtPointer {
	pub limit: u16,
	pub base: u64,
}
static mut IDT: [IdtEntry; 256] = [IdtEntry {
	offset_low: 0,
	selector: 0,
	ist: 0,
	type_attr: 0,
	offset_middle: 0,
	offset_high: 0,
	zero: 0,
}; 256];

#[repr(C)]
pub struct InterruptStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}

#[repr(C)]
pub struct ExceptionStackFrame {
    pub error_code: u64,
    pub frame: InterruptStackFrame,
}

pub unsafe fn init() {
	let handler = breakpoint_handler as *const () as u64;

	IDT[3] = IdtEntry::new(handler);

	// IDT[6] = IdtEntry::new(
    // invalid_opcode_stub as *const () as u64
	// );

	IDT[13] = IdtEntry::new(
    general_protection_stub as *const () as u64
	);

	IDT[14] = IdtEntry::new(
    page_fault_stub as *const () as u64
	);
	let mut entry = IdtEntry::new(
    double_fault_stub as *const () as u64
	);

	entry.ist = 1;

	IDT[8] = entry;

	let idt_ptr = IdtPointer {
		limit: (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16,
		base: core::ptr::addr_of!(IDT) as u64,
	};

	core::arch::asm!(
    "lidt [{0}]",
    in(reg) core::ptr::addr_of!(idt_ptr),
    options(readonly, nostack, preserves_flags)
);
}