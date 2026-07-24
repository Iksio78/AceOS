@echo off
setlocal

echo == Czyszczenie ==
del *.o 2>nul
del *.elf 2>nul
del AceOS.iso 2>nul

echo == Kompilacja boot.asm ==
nasm -f elf32 boot.asm -o boot.o
if errorlevel 1 goto error

echo == Kompilacja kernel.cpp ==
clang++ -target i386-unknown-none-elf ^
-ffreestanding ^
-fno-exceptions ^
-fno-rtti ^
-std=c++20 ^
-Wall ^
-Wextra ^
-c kernel\kernel.cpp ^
-o kernel.o
if errorlevel 1 goto error

echo == Kompilacja video.cpp ==
clang++ -target i386-unknown-none-elf ^
-ffreestanding ^
-fno-exceptions ^
-fno-rtti ^
-std=c++20 ^
-Wall ^
-Wextra ^
-c drivers\video.cpp ^
-o video.o
if errorlevel 1 goto error

echo == Linkowanie ==
ld.lld ^
-m elf_i386 ^
-T linker.ld ^
-o kernel.elf ^
boot.o kernel.o video.o
if errorlevel 1 goto error

echo == Tworzenie struktury ISO ==

rmdir /S /Q iso 2>nul

mkdir iso
mkdir iso\boot
mkdir iso\boot\grub

copy kernel.elf iso\boot\kernel.elf >nul

(
echo set timeout=0
echo set default=0
echo.
echo menuentry "AceOS" {
echo     multiboot /boot/kernel.elf
echo     boot
echo }
) > iso\boot\grub\grub.cfg

echo == Tworzenie obrazu ISO ==
grub-mkrescue -o AceOS.iso iso
if errorlevel 1 goto error

echo == Uruchamianie ==
D:\QEMU\qemu-system-i386.exe ^
-m 512M ^
-cdrom AceOS.iso ^
-vga std ^
-display gtk ^
-full-screen ^
-no-reboot ^
-no-shutdown

pause
exit /b

:error
echo.
echo ============================
echo WYSTAPIL BLAD KOMPILACJI
echo ============================
pause