@echo off
setlocal

echo ===========================
echo Building AceOS (Rust)
echo ===========================

REM ===========================
REM Clean
REM ===========================

cargo clean
if exist AceOS.iso del /q AceOS.iso

REM ===========================
REM Compile & Link via Cargo
REM ===========================

cargo build --release
if errorlevel 1 goto error

REM ===========================
REM Copy kernel
REM ===========================

copy /Y target\x86_64-unknown-none\release\aceos iso\kernel.elf

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

if errorlevel 1 goto error

REM ===========================
REM Install Limine BIOS
REM ===========================

iso\boot\limine\limine.exe bios-install AceOS.iso

REM ===========================
REM Run in QEMU
REM ===========================

qemu-system-x86_64 ^
-cdrom AceOS.iso ^
-m 512M ^
-vga std ^
-display sdl,gl=on ^
-serial stdio ^
-full-screen

goto end

:error
echo ===========================
echo Build failed!
echo ===========================

:end
pause