#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![allow(unused_imports)]
#![feature(abi_x86_interrupt)]
#![feature(uniform_paths)]

use core::panic::PanicInfo;

#[macro_use]
pub mod screen;
pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod scheduler;

/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn rust_main() -> ! {
	use interrupts::PICS;
    println!("Anix is starting...");
	
	println!("DEBUG: init GDT");
    gdt::init();
    
    println!("DEBUG: init idt");
    interrupts::init_idt();
    
    println!("DEBUG: init pics");
    unsafe { PICS.lock().initialize() };
    
    println!("DEBUG: interrupts are enabled!");
    x86_64::instructions::interrupts::enable();
    
    screen::create_screen();
    
	hlt_loop();
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
