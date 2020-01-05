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
