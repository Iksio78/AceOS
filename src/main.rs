#![allow(non_snake_case)]
#![no_std]
#![no_main]

mod LFB;
mod COM;

use core::{fmt, panic::PanicInfo};
use limine::request::FramebufferRequest;
use spin::Mutex;
use crate::LFB::{FramebufferWriter, lfb};

// Globalny, bezpieczny schowek na lfb dostępny z każdego miejsca w kodzie przez .lock()
pub static GLOBAL_LFB: Mutex<Option<lfb>> = Mutex::new(None);
pub static GLOBAL_FBW: Mutex<Option<FramebufferWriter>> = Mutex::new(None);

#[used]
#[link_section = ".requests"]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Inicjalizujemy LFB z requestu Limine
    let framebuffer_instance = lfb::init(&FRAMEBUFFER_REQUEST).unwrap();
    let writer_instance = FramebufferWriter::new(0x0000A6A6, 0x00000000);

    // Wrzucamy instancję do globalnego schowka, żeby każdy moduł (np. przyszły XECS) miał do niej dostęp
    *GLOBAL_LFB.lock() = Some(framebuffer_instance);
	*GLOBAL_FBW.lock() = Some(writer_instance);

	println!("[ AceOS Kernel Starting... ]");
	println!("######A######\n ####A A####\n  ##AAAAA##\n  #A     A#");


    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n!!! KERNEL PANIC !!!");
    println!("{}", info);
    loop {}
}