/*
 * Copyright (C) 2018-2020 Nicolas Fouquet
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see https://www.gnu.org/licenses.
 */

#![no_std]
#![feature(global_asm)]
#![feature(box_syntax)]
#![feature(thread_local)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(abi_x86_interrupt)]
#![feature(ptr_internals)]
#![feature(allocator_api)]
#![feature(alloc_error_handler)]
#![feature(associated_type_bounds)]
#![feature(naked_functions)]

#![allow(exceeding_bitshifts)]
#![allow(non_snake_case)]

// Imports

// Crates
#[macro_use]
extern crate alloc;
extern crate spin;
extern crate x86_64; // WARNING: You must add #![feature(try_from)] to the crate x86_64 0.6.0 (in ~/.cargo/registry/src/.../x86_64-0.6.0/src/lib.rs)
extern crate lazy_static;
extern crate volatile;
extern crate pic8259_simple;
extern crate pc_keyboard;
extern crate uart_16550;
extern crate x86;
extern crate bitflags;
extern crate rlibc;
extern crate multiboot2;
extern crate byteorder;
extern crate genio;
extern crate plain;
extern crate goblin;
#[macro_use]
extern crate once;

// Modules
#[macro_use]
pub mod screen; // Utilities for screen (print, ...)
pub mod gdt; // GDT
pub mod idt; // IDT (Interrupts Descriptor Table)
pub mod time; // Time management (TODO)
pub mod fs; // Filesystem (TODO)
pub mod commands; // Commands for input (for add a command, see the header of commands.rs)
pub mod user; // User functionnalities (TODO)
pub mod common; // Common functions
pub mod irq; // Interrupts management
pub mod memory; // Memory management
pub mod processes; // Processes management
pub mod errors; // Errors
pub mod pci; // Pci management (TODO)
pub mod disk; // Disk read and write (support ide (not tested) and sata)
pub mod drivers; // Drivers management
pub mod graphics; // Display things on screen
pub mod usb; // USB management
pub mod io; // IO (memory) management
pub mod syscall; // Syscalls management
pub mod device; // Devices management
pub mod elf; // Elf files loader
pub mod sse; // Enable SSE

#[cfg(feature="x86_64-qemu-Anix")] // Use this function only in Qemu
pub mod serial; // Qemu serial logging

#[cfg(not(test))]
use core::panic::PanicInfo;
#[cfg(not(test))]
use alloc::alloc::Layout;
use alloc::prelude::v1::String;
use spin::Mutex;
use x86::bits64::registers::*;
use linked_list_allocator::LockedHeap;

use idt::PICS;
use memory::{
    *,
    table::ActivePageTable,
    paging::{
        EntryFlags,
    }
};
use common::hlt_loop;
use processes::Process;

#[global_allocator]
pub static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();
pub static mut AREA_FRAME_ALLOCATOR: Mutex<Option<AreaFrameAllocator>> =
                Mutex::new(None);
pub static mut ACTIVE_TABLE: Mutex<Option<ActivePageTable>> = Mutex::new(None);
pub static mut VESA_BUFFER: Mutex<u32> =
                Mutex::new(0);

/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.
#[no_mangle] // don't mangle the name of this function
#[cfg(not(test))]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    screen::logo_screen();

    println!("Welcome!\nAnix v0.0.3 is starting...");

    println!("DEBUG: enable SSE");
    sse::enable_sse();

    unsafe {
        println!("DEBUG: init GDT");
        gdt::init();
        gdt::init_paging(rsp());
    }

    unsafe {
        irq::syscalls::init();
    }

    println!("DEBUG: init IDT");
    idt::init_idt();

    println!("DEBUG: init pics");
    unsafe { PICS.lock().initialize() };

    println!("DEBUG: Init memory");
    enable_nxe_bit();
    enable_write_protect_bit();

    unsafe {
        let boot_info = multiboot2::load(multiboot_information_address);

        memory::init(
            boot_info.start_address(),
            boot_info.end_address(),
            boot_info.elf_sections_tag().expect("cannot get elf sections tag"),
            boot_info.memory_map_tag().expect("Memory map tag required").memory_areas()
        );
        *VESA_BUFFER.lock() = boot_info.vbe_info_tag().unwrap().mode_info.framebuffer_base_ptr;
    }

    println!("DEBUG: Init the heap");
    memory::heap::init();

    println!("DEBUG: Start SATA driver");
    disk::sata::init();

    println!("DEBUG: Start VBE driver");
    graphics::vesa::init();

    println!("DEBUG: Start PCI driver");
    pci::init();

    // TODO: Use the multiboot crate to determinate the disk which will be read and write
    println!("DEBUG: Test Ext2 filesystem");
    fs::init();
    fs::ext2::init();

    println!("DEBUG: Start tasking system");

    // Map usermode tasks
    unsafe {
        map(system as *const () as usize, system as *const () as usize + 15,
        EntryFlags::PRESENT | EntryFlags::WRITABLE | EntryFlags::USER_ACCESSIBLE);
        Process::new(String::from("system"), system as *const () as u64);
        Process::new(String::from("terminal"), user::input::terminal as *const () as u64);

        // last task + 1 = first task
        // TASK_RUNNING = CURRENT_TASKS.last().unwrap().to_owned();
    }

    println!("DEBUG: Start elf loader");
    elf::init();

    println!("DEBUG: enable interrupts");
    x86_64::instructions::interrupts::enable();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("System->Panic! Debug informations: {:#?}", info);
    hlt_loop();
}

#[cfg(not(test))]
#[alloc_error_handler]
fn handle_alloc_error(layout: Layout) -> ! {
    println!("Heap->Out of memory! The size {:#x} bytes is too big", layout.size());
    hlt_loop();
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

fn system() {
    use processes::scheduler::switch;
    switch();
}
