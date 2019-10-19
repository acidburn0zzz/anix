#![feature(asm, lang_items, start)]
#![no_std]
use core::panic::PanicInfo;

// TODO: Create an ELF and add linker parameters

#[no_mangle]
pub extern "C" fn main(_: i32, _: *const *const i8) -> i32 {
    unsafe {
        let string = "Hello world with an ELF program written in Rust.";
        let reference: &str = string.as_ref();
        let (mut number, ptr, len) = (339, string.as_ptr(), reference.len());
        asm!("syscall"
            : "={rax}"(number)
            : "{rax}"(number), "{rdi}"(ptr), "{rsi}"(len)
            : "rcx", "r11", "memory"
            : "intel", "volatile");

        // Exit
        let mut a = 1;
        asm!("syscall"
            : "={rax}"(a)
            : "{rax}"(a)
            : "rcx", "r11", "memory"
            : "volatile");
        a
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
