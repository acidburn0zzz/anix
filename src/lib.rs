/*Copyright (C) 2018-2019 Nicolas Fouquet

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see https://www.gnu.org/licenses.
*/
#![feature(lang_items)]
#![no_std] // don't link the Rust standard library
#![feature(type_ascription)]
#![feature(abi_x86_interrupt)]

//Crates
extern crate rlibc;
extern crate x86_64;
extern crate spin;
extern crate pic8259_simple;
extern crate lazy_static;
extern crate pc_keyboard;
extern crate volatile;
extern crate uart_16550;

use core::panic::PanicInfo;

//Mods
#[macro_use]
pub mod screen;

pub mod gdt;

pub mod fs;
use fs::mainfs;

pub mod scheduler;

pub mod interrupts;

#[no_mangle]
pub extern fn rust_main() {
    //Start msg
    println!("Anix is starting...");
    
    //Interrupts
    use interrupts::PICS;
    use x86_64::instructions::interrupts::*;
    
    println!("DEBUG: init gdt");
    gdt::init();

    println!("DEBUG: init idt");
    interrupts::init_idt();
    
    println!("DEBUG: init pics");
    unsafe { PICS.lock().initialize() };

    println!("DEBUG: enable interrupts");
    x86_64::instructions::interrupts::enable();
    
    //Screen
    screen::create_screen();
    
    //FS(does not work yet)
    fs::mainfs();
    
    //End msg
    println!("Anix is started successfully! It returns: {}", "It works");
    
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
#[no_mangle]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();            // new
}

//Tools
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
