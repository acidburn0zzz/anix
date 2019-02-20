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
#![feature(alloc)]

//Imports

//Crates
#[macro_use]
extern crate core;
extern crate spin;
extern crate x86_64;
extern crate lazy_static;
extern crate volatile;
extern crate pic8259_simple;
extern crate pc_keyboard;
extern crate uart_16550;
extern crate x86;
extern crate bitflags;
extern crate ascii;
extern crate libc;
use core::panic::PanicInfo;

//Modules
#[macro_use]
pub mod screen;
pub mod gdt;
pub mod interrupts;
pub mod time;
pub mod fs;
pub mod commands;
pub mod numbers;
pub mod user;
pub mod shutdown;
pub mod mm;
pub mod common;
pub mod pti;
pub mod irq;
pub mod scheduler;
pub mod malloc;

/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn rust_main() -> ! {
    use interrupts::PICS;
    use x86::time::*;

    screen::logo_screen();
    
    println!("Anix is starting...");
    
    //GDT
    println!("DEBUG: init GDT");
    
    unsafe{
		gdt::init();
		print!("CS:\n{}\n", x86::segmentation::cs());
    }
    
    //IDT
	println!("DEBUG: init idt");
	interrupts::init_idt();
    
    println!("DEBUG: init pics");
    unsafe { PICS.lock().initialize() };
    
    println!("DEBUG: enable interrupts are enabled!");
    x86_64::instructions::interrupts::enable();

    println!("DEBUG: init pic");
    unsafe{fs::pic::init();}
	//Part advancing paging of phil-opp tutorial
	//memory::init(0, kernel_base + ((kernel_size + 4095)/4096) * 4096);

	// Initialize paging
	//let (mut active_table, tcb_offset) = paging::init(0, kernel_base, kernel_base + kernel_size, stack_base, stack_base + stack_size);

	//Part kernel heap of phil-opp tutorail
	//allocator::init(&mut active_table);

	// Read ACPI tables, starts APs
	//#[cfg(feature = "acpi")]
	//acpi::init(&mut active_table);

	// Initialize memory functions after core has loaded
	//memory::init_noncore();

	print!("Anix>");
    
    //screen::starter_screen();
    //println!("DEBUG: Fs is launched");
    //fs::fsmain();
    
    use common::hlt_loop;
    hlt_loop();
}
