use core::fmt;
use core::arch::asm;

use limine::framebuffer::Framebuffer;

use crate::LFB::FramebufferWriter;

const COM1: u16 = 0x3F8;

/// Wysyła pojedynczy bajt do portu I/O
pub unsafe fn outb(port: u16, value: u8) {
    asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nomem, nostack, preserves_flags)
    );
}

/// Odczytuje bajt z portu I/O
pub unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    asm!(
        "in al, dx",
        out("al") value,
        in("dx") port,
        options(nomem, nostack, preserves_flags)
    );
    value
}

pub fn send_com1(byte: u8) {
    unsafe {
        while (inb(COM1 + 5) & 0x20) == 0 {}
        outb(COM1, byte);
    }
}

pub fn send_com1_str(s: &str) {
    for byte in s.bytes() {
        if byte == b'\n' {
            send_com1(b'\r');
        }
        send_com1(byte);
    }
}

pub struct SerialWriter;

impl fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        send_com1_str(s);
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    use fmt::Write;

    let mut serial = SerialWriter;
    let _ = serial.write_fmt(args);

    let mut guard = crate::GLOBAL_FBW.lock();

    if let Some(fbw) = guard.as_mut() {
        let _ = fbw.write_fmt(args);
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::COM::_print(format_args!($($arg)*), );
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*));
    };
}