#![allow(non_snake_case)] 
#![no_std] 
#![no_main] 
#![feature(abi_x86_interrupt)] 
 
mod LFB; 
mod COM; 
mod gdt; 
mod idt; 
mod stubs; 
mod handlers; 
mod tss; 
mod bitmap; 
mod HHDM; 
mod PMM; 
mod vmm; 
 
use core::{arch::asm, fmt, panic::PanicInfo}; 
use limine::request::{FramebufferRequest, MemmapRequest}; 
use spin::Mutex; 
use crate::{ 
	LFB::{FramebufferWriter, lfb}, 
	gdt::GdtEntry, 
	vmm::VMM 
}; 
 
// Globalne schowki 
pub static GLOBAL_LFB: Mutex<Option<lfb>> = Mutex::new(None); 
pub static GLOBAL_FBW: Mutex<Option<FramebufferWriter>> = Mutex::new(None); 
pub static GLOBAL_PMM: Mutex<Option<PMM::PMM>> = Mutex::new(None); 
pub static GLOBAL_VMM: Mutex<Option<vmm::VMM>> = Mutex::new(None); 
 
const PAGE_SIZE: usize = 4096; 
const PAGES_PER_GIB: usize = (1024 * 1024 * 1024) / PAGE_SIZE; 
 
#[used] 
#[link_section = ".requests"] 
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new(); 
 
#[used] 
#[link_section = ".requests"] 
static MEMMAP_REQUEST: MemmapRequest = MemmapRequest::new(); 
 
#[no_mangle] 
pub extern "C" fn _start() -> ! { 
 
	// Inicjalizujemy LFB z requestu Limine 
	let framebuffer_instance = 
		lfb::init(&FRAMEBUFFER_REQUEST).unwrap(); 
 
	let writer_instance = 
		FramebufferWriter::new(0x0000A6A6, 0x00000000); 
 
	unsafe { 
		tss::init(); 
		gdt::init(); 
		idt::init(); 
		HHDM::init(); 
	} 
 
	// Globalne instancje 
	*GLOBAL_LFB.lock() = Some(framebuffer_instance); 
	*GLOBAL_FBW.lock() = Some(writer_instance); 
 
	// PMM 
	let pmm_instance = match PMM::PMM::init(&MEMMAP_REQUEST) { 
		Some(pmm) => pmm, 
		None => panic!("PMM initialization failed"), 
	}; 
 
	*GLOBAL_PMM.lock() = Some(pmm_instance); 
 
	// VMM 
	let vmm_instance = VMM::init(); 
	*GLOBAL_VMM.lock() = Some(vmm_instance); 
 
	println!("[ AceOS Kernel Starting... ]"); 
 
	println!( 
"######A######\n
  ####A A####\n
   ##AAAAA##\n
   #A     A#" 
	); 
 
	println!( 
		"Free RAM: {:.2} GiB", 
		GLOBAL_PMM 
			.lock() 
			.as_ref() 
			.unwrap() 
			.free_pages as f32 
			/ PAGES_PER_GIB as f32 
	); 
 
	GLOBAL_FBW 
		.lock() 
		.as_mut() 
		.unwrap() 
		.change_colors(0x00FF0000, 0x00000000); 
 
	// Adresy testowe 
	let coffe: usize = 0xC00FEE; 
 
	let deadbeef: u64 = 0xDEADBEEF; 
 
	println!("przed ADDR1");
 
	// Alokujemy dwie fizyczne strony 
	let addr1 = GLOBAL_PMM 
		.lock() 
		.as_mut() 
		.unwrap() 
		.alloc_page() 
		.unwrap(); 

	println!("po ADDR"); 
	// ------------------------------------------------- 
	// VMM 
	// ------------------------------------------------- 
 
	// Zachowujemy MutexGuard osobno. 
	// Dzięki temu referencja `v` będzie żyła tak długo, 
	// jak guard. 
	println!("przed wycioganiem VMM z Mutexa"); 
	let mut vmm_guard = GLOBAL_VMM.lock(); 
 
	let v = vmm_guard 
		.as_mut() 
		.unwrap(); 
 
	// Mapowanie
	println!("przed mapowaniem COFFE");
	v.map_page(coffe, addr1, false); 

	println!("po mapowaniach"); 
	// ------------------------------------------------- 
	// Test mapowania 
	// ------------------------------------------------- 
 
	unsafe { 
		println!("przed zapisem do COFFE"); 
		(coffe as *mut u64).write_volatile(deadbeef); 

		println!("przed odczytem z COFFE"); 
		println!("{}", (coffe as *mut u64).read_volatile()); 
	} 

	println!("po unsafe"); 
 
	// ------------------------------------------------- 
	// Unmap 
	// ------------------------------------------------- 
 
	
	println!("przed odmapowaniem COFFE"); 
	println!("COFEE jest mapowane na {}", addr1);
	v.unmap_page(coffe); 
	v.debug_pte(coffe);

	println!("po unmap");
	// ------------------------------------------------- 
	// Test page fault 
	// ------------------------------------------------- 
 
	println!("addr1: {:#x}", addr1);
	unsafe {  
		// Tutaj powinien nastąpić #PF, 
		// ponieważ `coffe` zostało odmapowane. 
		println!("{}", (coffe as *mut u64).read_volatile()); 
	} 

	println!("przed remapowaniem COFFE");
	v.map_page(coffe, addr1, false); 

	println!("po mapowaniach"); 
	// ------------------------------------------------- 
	// Test mapowania 
	// ------------------------------------------------- 
 
	unsafe { 
		println!("przed zapisem do COFFE"); 
		(coffe as *mut u64).write_volatile(deadbeef); 

		println!("przed odczytem z COFFE"); 
		println!("{}", (coffe as *mut u64).read_volatile()); 
	} 

	println!("po unsafe"); 
	println!("po testach"); 
 
	GLOBAL_FBW 
		.lock() 
		.as_mut() 
		.unwrap() 
		.change_colors(0x0000FF00, 0x00000000); 
 
	println!("keyboard doesn't work yet"); 
	println!("Scheduler doesn't exist yet"); 
	println!("Nvme Disk doesn't work yet"); 
 
	loop {} 
} 
 
#[panic_handler] 
fn panic(info: &PanicInfo) -> ! { 
	println!("\n!!! KERNEL PANIC !!!"); 
	println!("{}", info); 
 
	loop {} 
}

