use crate::{COM::inb, println};

fn check_port() -> bool{
	let from_64 = unsafe {inb(0x64)};

	if from_64 & 1 == 0{
		return false;
	} 

	true
}

fn read_port() -> u8{
	unsafe {inb(0x60)}
}

pub fn read_key(){
	if !check_port(){return;}

	println!("{}", read_port());
}