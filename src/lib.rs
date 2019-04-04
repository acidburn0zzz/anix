/*Copyright (C) 2018-2019 Nicolas Fouquet 

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see https://www.gnu.org/licenses.
*/
#![no_std]
#![allow(unused_imports)]
#![allow(exceeding_bitshifts)]
#![feature(abi_x86_interrupt)]
#![feature(uniform_paths)]
#![feature(type_ascription)]
#![feature(asm)]
#![feature(thread_local)]
#![feature(naked_functions)]
#![feature(rustc_private)]
#![feature(c_void_variant)]
#![feature(range_contains)]
#![feature(ptr_internals)]


//Imports

//Crates
#[macro_use]
extern crate spin;
extern crate x86_64;
extern crate lazy_static;
extern crate volatile;
extern crate pic8259_simple;
extern crate pc_keyboard;
extern crate uart_16550;
extern crate x86;
extern crate bitflags;
extern crate multiboot2;

//Modules
#[macro_use]
pub mod screen; //Utilities for screen (print, ...)
pub mod gdt; //GDT
pub mod idt; //IDT (Interrupts Descriptor Table)
pub mod time; //Time management (TODO)
pub mod fs; //Filesystem (TODO)
pub mod commands; //Commands for input (for add a command, see the header of commands.rs)
pub mod user; //User functionnalities (TODO)
pub mod common; //Common functions
pub mod irq; //Interrupts management
pub mod scheduler; //Loop function for tasks management (TODO)
pub mod memory; //Memory management

use core::panic::PanicInfo;
use idt::PICS;
use x86::time::*;
use memory::*;
use common::{hlt_loop, ok};
use x86_64::registers::control::*;
use memory::{Frame, FrameAllocator};
use memory::table::ActivePageTable;
use spin::Mutex;
use lazy_static::lazy_static;

/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn rust_main(multiboot_information_address: usize) -> ! {
    screen::logo_screen();
    
    println!("Welcome!\nAnix is starting...");
    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
	let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");

	println!("Memory areas:");
	for area in memory_map_tag.memory_areas() {
		println!("    start: 0x{:x}, length: 0x{:x}", area.base_addr, area.length);
	}
	
	let elf_sections_tag = boot_info.elf_sections_tag().expect("Elf-sections tag required");

	println!("Kernel sections:");
	for section in elf_sections_tag.sections() {
		println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
        section.addr, section.size, section.flags);
	}
	
	let kernel_start = elf_sections_tag.sections().map(|s| s.addr).min().unwrap();
	let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size).max().unwrap();
	
    let multiboot_start = multiboot_information_address;
	let multiboot_end = multiboot_start + (boot_info.total_size as usize);
	
	let initrd_start = boot_info.module_tags().next().unwrap().start_address();
	let initrd_end = boot_info.module_tags().next().unwrap().end_address();
	
	println!("Kernel\n   start: {}\n   end: {}", kernel_start, kernel_end);
    println!("\nMultiboot\n   start: {}\n   end: {}", multiboot_start, multiboot_end);
    println!("\nInitrd\n   start: {}\n   end: {}", initrd_start, initrd_end);
    
    unsafe{
		print!("\nDEBUG: init GDT");
		gdt::init();
		ok();
		print!("\nCS:\n{}\n", x86::segmentation::cs());
    }
    
	print!("\nDEBUG: init IDT");
	idt::init_idt();
	ok();
    
    print!("\nDEBUG: init pics");
    unsafe { PICS.lock().initialize() };
    ok();
    
    print!("\nDEBUG: enable interrupts");
    x86_64::instructions::interrupts::enable();
    ok();
    
    print!("\nDEBUG: Start allocator system");
    let mut ALLOCATOR = memory::AreaFrameAllocator::new(kernel_start as usize, kernel_end as usize, multiboot_start, multiboot_end, memory_map_tag.memory_areas());
	ok();
	
	for i in 0.. {
		if let None = ALLOCATOR.allocate_frame() {
			println!("\nAllocated {} frames", i);
			break;
		}
	}
	
	unsafe{
		set_initrd_addr_start(initrd_start);
	}
	
	print!("\nAnix>");
    
    hlt_loop();
}

extern "C"{
	fn set_initrd_addr_start(addr: u32);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	print!("\nPANIC\nInfo:\n{:#?}", info);
	hlt_loop();
}
