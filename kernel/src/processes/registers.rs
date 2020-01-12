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
//! This file contains helper function to use CPU registers easier
use x86_64::{
    structures::paging::frame::PhysFrame,
    registers::control::{
        Cr3 as Cr3ReadWrite,
        Cr3Flags
    }
};

pub struct Rax ();
impl Rax {
    pub fn set(value: u64) {
        unsafe { asm!("mov rax, $0" :
                           : "r"(value)
                           : "memory"
                           : "intel", "volatile"); }
    }
    pub fn get() -> u64 {
        let mut value;
        unsafe { asm!("mov $0, rax" : "=r"(value)
                           :
                           : "memory"
                           : "intel", "volatile"); }
        value

    }
}

pub struct Rbx ();
impl Rbx {
    pub fn set(value: u64) {
        unsafe { asm!("mov rbx, $0" :
                           : "r"(value)
                           : "memory"
                           : "intel", "volatile"); }
    }
    pub fn get() -> u64 {
        let mut value;
        unsafe { asm!("mov $0, rbx" : "=r"(value)
                           :
                           : "memory"
                           : "intel", "volatile"); }
        value
    }
}

pub struct Rcx ();
impl Rcx {
    pub fn set(value: u64) {
        unsafe { asm!("mov rcx, $0" :
                           : "r"(value)
                           : "memory"
                           : "intel", "volatile"); }
    }
    pub fn get() -> u64 {
        let mut value;
        unsafe { asm!("mov $0, rcx" : "=r"(value)
                           :
                           : "memory"
                           : "intel", "volatile"); }
        value
    }
}

pub struct Rdx ();
impl Rdx {
    pub fn set(value: u64) {
        unsafe { asm!("mov rdx, $0" :
                           : "r"(value)
                           : "memory"
                           : "intel", "volatile"); }
    }
    pub fn get() -> u64 {
        let mut value;
        unsafe { asm!("mov $0, rdx" : "=r"(value)
                           :
                           : "memory"
                           : "intel", "volatile"); }
        value
    }
}

pub struct Rsi ();
impl Rsi {
    pub fn set(value: u64) {
        unsafe { asm!("mov rsi, $0" :
                           : "r"(value)
                           : "memory"
                           : "intel", "volatile"); }
    }
    pub fn get() -> u64 {
        let mut value;
        unsafe { asm!("mov $0, rsi" : "=r"(value)
                           :
                           : "memory"
                           : "intel", "volatile"); }
        value
    }
}

pub struct Rdi ();
impl Rdi {
    pub fn set(value: u64) {
        unsafe { asm!("mov rdi, $0" :
                           : "r"(value)
                           : "memory"
                           : "intel", "volatile"); }
    }
    pub fn get() -> u64 {
        let mut value;
        unsafe { asm!("mov $0, rdi" : "=r"(value)
                           :
                           : "memory"
                           : "intel", "volatile"); }
        value
    }
}

// r8 -> r15

pub struct R8 ();
impl R8 {
    pub fn set(value: u64) {
        unsafe { asm!("mov r8, $0" :
                           : "r"(value)
                           : "memory"
                           : "intel", "volatile"); }
    }
    pub fn get() -> u64 {
        let mut value;
        unsafe { asm!("mov $0, r8" : "=r"(value)
                           :
                           : "memory"
                           : "intel", "volatile"); }
        value
    }
}

pub struct R9 ();
impl R9 {
    pub fn set(value: u64) {
        unsafe { asm!("mov r9, $0" :
                           : "r"(value)
                           : "memory"
                           : "intel", "volatile"); }
    }
    pub fn get() -> u64 {
        let mut value;
        unsafe { asm!("mov $0, r9" : "=r"(value)
                           :
                           : "memory"
                           : "intel", "volatile"); }
        value
    }
}

pub struct R10 ();
impl R10 {
    pub fn set(value: u64) {
        unsafe { asm!("mov r10, $0" :
                           : "r"(value)
                           : "memory"
                           : "intel", "volatile"); }
    }
    pub fn get() -> u64 {
        let mut value;
        unsafe { asm!("mov $0, r10" : "=r"(value)
                           :
                           : "memory"
                           : "intel", "volatile"); }
        value
    }
}

pub struct R11 ();
impl R11 {
    pub fn set(value: u64) {
        unsafe { asm!("mov r11, $0" :
                           : "r"(value)
                           : "memory"
                           : "intel", "volatile"); }
    }
    pub fn get() -> u64 {
        let mut value;
        unsafe { asm!("mov $0, r11" : "=r"(value)
                           :
                           : "memory"
                           : "intel", "volatile"); }
        value
    }
}

pub struct R12 ();
impl R12 {
    pub fn set(value: u64) {
        unsafe { asm!("mov r12, $0" :
                           : "r"(value)
                           : "memory"
                           : "intel", "volatile"); }
    }
    pub fn get() -> u64 {
        let mut value;
        unsafe { asm!("mov $0, r12" : "=r"(value)
                           :
                           : "memory"
                           : "intel", "volatile"); }
        value
    }
}

pub struct R13 ();
impl R13 {
    pub fn set(value: u64) {
        unsafe { asm!("mov r13, $0" :
                           : "r"(value)
                           : "memory"
                           : "intel", "volatile"); }
    }
    pub fn get() -> u64 {
        let mut value;
        unsafe { asm!("mov $0, r13" : "=r"(value)
                           :
                           : "memory"
                           : "intel", "volatile"); }
        value
    }
}

pub struct R14 ();
impl R14 {
    pub fn set(value: u64) {
        unsafe { asm!("mov r14, $0" :
                           : "r"(value)
                           : "memory"
                           : "intel", "volatile"); }
    }
    pub fn get() -> u64 {
        let mut value;
        unsafe { asm!("mov $0, r14" : "=r"(value)
                           :
                           : "memory"
                           : "intel", "volatile"); }
        value
    }
}

pub struct R15 ();
impl R15 {
    pub fn set(value: u64) {
        unsafe { asm!("mov r15, $0" :
                           : "r"(value)
                           : "memory"
                           : "intel", "volatile"); }
    }
    pub fn get() -> u64 {
        let mut value;
        unsafe { asm!("mov $0, r15" : "=r"(value)
                           :
                           : "memory"
                           : "intel", "volatile"); }
        value
    }
}

pub struct Rbp ();
impl Rbp {
    pub fn set(value: u64) {
        unsafe { asm!("mov rbp, $0" :
                           : "r"(value)
                           : "memory"
                           : "intel", "volatile"); }
    }
    pub fn get() -> u64 {
        let mut value;
        unsafe { asm!("mov $0, rbp" : "=r"(value)
                           :
                           : "memory"
                           : "intel", "volatile"); }
        value
    }
}

pub struct Rflags ();
impl Rflags {
    pub fn set(value: u64) {
        unsafe { asm!("push $0 ; popfq" :
                           : "r"(value)
                           : "memory"
                           : "intel", "volatile"); }
    }
    pub fn get() -> u64 {
        let mut value;
        unsafe { asm!("pushfq ; pop $0" : "=r"(value)
                           :
                           : "memory"
                           : "intel", "volatile"); }
        value
    }
}

pub struct Cr3 ();
impl Cr3 {
    pub fn set(value: (PhysFrame, Cr3Flags)) {
        unsafe {
            Cr3ReadWrite::write(value.0, value.1);
        }
    }
    pub fn get() -> (PhysFrame, Cr3Flags) {
        Cr3ReadWrite::read()
    }
}
