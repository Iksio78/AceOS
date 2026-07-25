section .text

global _start
extern KernelMain

_start:
    call KernelMain

.hang:
    hlt
    jmp .hang