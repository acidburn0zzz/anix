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
#![feature(unsize,coerce_unsized)]	// For DST smart pointers
#![feature(core_intrinsics)]	// Intrinsics
#![feature(box_syntax)]	// Enables 'box' syntax
#![feature(box_patterns)]	// Used in boxed::unwrap
#![feature(thread_local)]	// Allows use of thread_local
#![feature(lang_items)]	// Allow definition of lang_items
#![feature(asm)]	// Enables the asm! syntax extension
#![feature(optin_builtin_traits)]	// Negative impls
#![feature(slice_patterns)]	// Slice (array) destructuring patterns, used by multiboot code
#![feature(linkage)]	// allows using #[linkage="external"]
#![feature(const_fn)]	// Allows defining `const fn`
#![feature(abi_x86_interrupt)]
#![feature(uniform_paths)]
#![feature(type_ascription)]
#![feature(naked_functions)]
#![feature(rustc_private)]
#![feature(c_void_variant)]
#![feature(range_contains)]
#![feature(ptr_internals)]
#![feature(global_asm)]
#![feature(custom_attribute)]
#![feature(dropck_eyepatch)]
#![feature(panic_info_message)]
#![feature(allocator_api)]
#![feature(alloc_error_handler)]
#![feature(alloc)] // The alloc crate is still unstable

#![allow(unused_imports)]
#![allow(exceeding_bitshifts)]
//Imports

//Crates
#[macro_use]
extern crate alloc;
#[macro_use]
extern crate spin;
extern crate x86_64; //WARNING: You must add #![feature(try_from)] to the crate x86_64 0.6.0 (in ~/.cargo/registry/src/.../x86_64-0.6.0/src/lib.rs)
extern crate lazy_static;
extern crate volatile;
extern crate pic8259_simple;
extern crate pc_keyboard;
extern crate uart_16550;
extern crate x86;
extern crate bitflags;
extern crate rlibc;
extern crate multiboot2;
#[macro_use]
extern crate once;

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
pub mod memory; //Memory management
pub mod task; //Tasks management
pub mod errors; //Errors
pub mod pci; //Pci management (TODO)
pub mod disk; //Disk read and write (support ide (not tested) and sata)

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
use task::*;
use memory::heap::{HEAP_START, HEAP_SIZE, BumpAllocator};
use core::alloc::GlobalAlloc;
use alloc::alloc::Layout;

#[global_allocator]
static HEAP_ALLOCATOR: BumpAllocator = BumpAllocator::new(HEAP_START, HEAP_START + HEAP_SIZE);
    
/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn rust_main(multiboot_information_address: usize) -> ! {
    screen::logo_screen();
    
    println!("Welcome!\nAnix is starting...");
    let boot_info = unsafe {
        multiboot2::load(multiboot_information_address)
    };
	
	let initrd_start = boot_info.module_tags().next().unwrap().start_address();
	let initrd_end = boot_info.module_tags().next().unwrap().end_address();
    
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
    
    print!("\nDEBUG: Init memory");
    enable_nxe_bit();
	enable_write_protect_bit();
    memory::init(boot_info);
    ok();
	
	print!("\nDEBUG: set initrd start");
	unsafe{
		set_initrd_addr_start(initrd_start);
	}
	ok();
    
	print!("\nDEBUG: Start tasking system");
	
	unsafe{
		Task::new("system", system as *const () as u32);
		
		TASK_RUNNING = CURRENT_TASKS[0];
	}
	ok();
	
	print!("\nDEBUG: enable interrupts");
    x86_64::instructions::interrupts::enable();
    ok();

	print!("\nAnix>");
    
    hlt_loop();
}

extern "C"{
	fn set_initrd_addr_start(addr: u32);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	print!("\n{:#?}", info);
	hlt_loop();
}
#[alloc_error_handler]
fn handle_alloc_error(layout: Layout) -> !{
	print!("\n{:#?}", layout);
	hlt_loop();
}

static mut increment_test: u32 = 0;

fn system(){
	
}

fn enable_nxe_bit() {
    use x86::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86::controlregs::{cr0, cr0_write, Cr0};

    unsafe { cr0_write(cr0() | Cr0::CR0_WRITE_PROTECT) };
}
