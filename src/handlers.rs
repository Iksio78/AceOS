use crate::{idt::{ExceptionStackFrame, InterruptStackFrame}, println};

#[unsafe(no_mangle)]
pub extern "C" fn page_fault_handler(
    error_code: u64,
    frame: *const InterruptStackFrame,
) -> ! {
    let frame = unsafe { &*frame };

    println!("EXCEPTION: PAGE FAULT");
    println!("Error code: {:#x}", error_code);
    println!("RIP: {:#x}", frame.instruction_pointer);

    let cr2: u64;

    unsafe {
        core::arch::asm!(
            "mov {}, cr2",
            out(reg) cr2,
        );
    }

    println!("CR2: {:#x}", cr2);

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[unsafe(no_mangle)]
pub extern "x86-interrupt" fn breakpoint_handler(
    _stack_frame: InterruptStackFrame
) {
    println!("[EXCEPTION] Breakpoint");
}

#[unsafe(no_mangle)]
pub extern "C" fn invalid_opcode_handler() -> ! {
    println!("EXCEPTION: INVALID OPCODE");

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn general_protection_handler(
    error_code: u64,
    frame: *const InterruptStackFrame,
) -> ! {
    let frame = unsafe { &*frame };

    println!("EXCEPTION: GENERAL PROTECTION FAULT");
    println!("Error code: {:#x}", error_code);
    println!("RIP: {:#x}", frame.instruction_pointer);

    println!("Triggering INVALID OPCODE...");

    unsafe {
        core::arch::asm!("ud2");
    }

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn double_fault_handler(
    frame: *const u8
) -> ! {
    println!("!!! DOUBLE FAULT !!!");
    println!("DF frame: {:#x}", frame as u64);

    loop {
        core::hint::spin_loop();
    }
}