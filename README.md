**AceOS**

AceOS is a hobby operating system written in Rust for x86-64 architecture.

## Features
 - [x] Serial Port output
 - [x] Framebuffer output
 - [x] GDT and IDT
 - [x] Physical memory management
 - [x] Virtual memory management
 - [x] Kernel Heap
 - [ ] PS/2 keyboard support
 - [ ] Scheduler

## Building

The recommended way to build and run AceOS on Windows is to use the provided run.bat script.

If you prefer to build the ISO manually, follow these steps.

1. Build the kernel

Run the following command:

cargo build --release

2. Copy the kernel

Copy the compiled kernel to the ISO directory:

copy /Y target\x86_64-unknown-none\release\aceos iso\kernel.elf

3. Create the ISO image

Run the following command to create AceOS.iso:

xorriso ^
-as mkisofs ^
-b boot/limine/limine-bios-cd.bin ^
-no-emul-boot ^
-boot-load-size 4 ^
-boot-info-table ^
--efi-boot boot/limine/limine-uefi-cd.bin ^
-no-emul-boot ^
-o AceOS.iso ^
iso

4. Install the Limine BIOS stages

Run:

iso\boot\limine\limine.exe bios-install AceOS.iso

5. Run AceOS in QEMU

Launch the virtual machine with:

qemu-system-x86_64 ^
-cdrom AceOS.iso ^
-m 2G ^
-vga std ^
-display sdl,gl=on ^
-serial stdio ^
-full-screen

Requirements:

Rust and Cargo

xorriso

Limine BIOS installation utility

QEMU for x86-64
