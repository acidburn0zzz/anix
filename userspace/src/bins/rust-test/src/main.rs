/*
 * Copyright (C) 2018-2019 Nicolas Fouquet
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 2 of the License, or
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

#![feature(asm)]
/*use std::{
    prelude::v1::*,
    io::Read,
    fs::{
        File,
    }
};*/

pub fn main() {
    println!("Hello world!");
    // use std::str::from_utf8;

    /*let string2 = "world from rust with std binary!";
    let string = "Hello world with an ELF program written in Rust.";
    let reference: &str = string.as_ref();
    let (mut number, ptr, len) = (339, string.as_ptr(), reference.len());
    unsafe {
        asm!("syscall"
            : "={rax}"(number)
            : "{rax}"(number), "{rdi}"(ptr), "{rsi}"(len)
            : "rcx", "r11", "memory"
            : "intel", "volatile");
    }*/

    // let string2 = "weird world!";
    // print!("Hello {}", string2);
    // println!(" Second print on the same line");
    // println!("Third print on a new line");
    /*
    let mut vec = vec![1, 2, 3];
    vec.push(4);
    println!("Vec result: {:?}", vec);

    // let mut buffer: &mut [u8] = &mut [0; 80];
    let mut buffer: &mut String = &mut String::new();
    let mut fd = File::open("/home/user/hello2.txt").expect("cannot open file");
    fd.read_to_string(&mut buffer).expect("cannot read file");
    println!("Content of /home/user/hello2.txt:\n{:?}", buffer);

    // Exit
    // exit(); TODO: look at the exit syscall with one parameter*/
    /*unsafe {
        let mut a = 60;
        asm!("syscall"
            : "={rax}"(a)
            : "{rax}"(a)
            : "rcx", "r11", "memory"
            : "volatile");
    }*/
}

