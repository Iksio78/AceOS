use core::arch::global_asm;

global_asm!(r#"
.global invalid_opcode_stub
.type invalid_opcode_stub, @function

invalid_opcode_stub:
    call invalid_opcode_handler

2:
    hlt
    jmp 2b
"#);

global_asm!("
.global page_fault_stub
.type page_fault_stub, @function

page_fault_stub:
    mov rdi, [rsp]      # error code
    lea rsi, [rsp + 8]  # InterruptStackFrame
    call page_fault_handler

    add rsp, 8          # zdejmij error code
    iretq
");

global_asm!(
".global general_protection_stub
.type general_protection_stub, @function

general_protection_stub:
    mov rdi, [rsp]      # error code
    lea rsi, [rsp + 8]  # InterruptStackFrame
    call general_protection_handler

    add rsp, 8
    iretq
");

global_asm!("
.global double_fault_stub
.type double_fault_stub, @function

double_fault_stub:
    mov rdi, rsp
    call double_fault_handler

2:
    hlt
    jmp 2b
");