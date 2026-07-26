@echo off
setlocal

echo ===========================
echo Building AceOS
echo ===========================

REM ===========================
REM Clean
REM ===========================

del /q *.o 2>nul
del /q kernel.elf 2>nul
del /q AceOS.iso 2>nul

REM ===========================
REM Assemble
REM ===========================

nasm -f elf64 kernel\entry.asm -o entry.o

REM ===========================
REM Compile
REM ===========================

clang++ ^
-target x86_64-unknown-none-elf ^
-ffreestanding ^
-fno-exceptions ^
-fno-rtti ^
-m64 ^
-std=c++20 ^
-c kernel\kernel.cpp ^
-o kernel.o

clang++ ^
-target x86_64-unknown-none-elf ^
-ffreestanding ^
-fno-exceptions ^
-fno-rtti ^
-m64 ^
-std=c++20 ^
-c drivers\video.cpp ^
-o video.o

REM ===========================
REM Link
REM ===========================

ld.lld ^
-m elf_x86_64 ^
-T linker.ld ^
entry.o ^
kernel.o ^
video.o ^
-o kernel.elf

REM ===========================
REM Copy kernel
REM ===========================

copy /Y kernel.elf iso\kernel.elf

REM ===========================
REM Build ISO
REM ===========================

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

REM ===========================
REM Install Limine BIOS
REM ===========================

iso\boot\limine\limine.exe bios-install AceOS.iso

REM ===========================
REM Run
REM ===========================

qemu-system-x86_64 ^
-cdrom AceOS.iso ^
-m 512M ^
-serial stdio ^
-full-screen

pause