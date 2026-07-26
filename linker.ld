OUTPUT_FORMAT(elf64-x86-64)
OUTPUT_ARCH(i386:x86-64)
ENTRY(_start)

KERNEL_BASE = 0xffffffff80000000;

PHDRS
{
    text PT_LOAD FLAGS(5);
    data PT_LOAD FLAGS(6);
}

SECTIONS
{
    . = KERNEL_BASE + 0x100000;

. = ALIGN(0x1000);
.limine_requests :
{
    KEEP(*(.limine_requests_start))
    KEEP(*(.limine_requests))
    KEEP(*(.limine_requests_end))
} :text

. = ALIGN(0x1000);
.text :
{
    *(.text*)
} :text

. = ALIGN(0x1000);
.rodata :
{
    *(.rodata*)
} :text

. = ALIGN(0x1000);
.data :
{
    *(.data*)
} :data

. = ALIGN(0x1000);
.bss :
{
    *(COMMON)
    *(.bss*)
} :data
}