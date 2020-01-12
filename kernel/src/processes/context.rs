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

use x86_64::{
    structures::paging::frame::PhysFrame,
    registers::control::{
        Cr3Flags
    }
};
use super::registers::*;

#[derive(Clone, Debug)]
pub struct Context {
    regs: Registers,
}

impl Context {
    pub fn new(rsp: u64, rip: u64) -> Self {
        let mut regs = Registers::default();
        regs.rsp = rsp; // Set stack
        regs.rip = rip; // Set entry

        Self {
            regs,
        }
    }
    pub fn save(&mut self) {
        self.regs.rax = Rax::get();
        self.regs.rbx = Rbx::get();
        self.regs.rcx = Rcx::get();
        self.regs.rdx = Rdx::get();
        self.regs.rsi = Rsi::get();
        self.regs.rdi = Rdi::get();
        self.regs.rbp = Rbp::get();

        self.regs.r8 = R8::get();
        self.regs.r9 = R9::get();
        self.regs.r10 = R10::get();
        self.regs.r11 = R11::get();
        self.regs.r12 = R12::get();
        self.regs.r13 = R13::get();
        self.regs.r14 = R14::get();
        self.regs.r15 = R15::get();
        self.regs.rflags = Rflags::get();
        // TODO: Cr3 (switch to Kernel Mode at the start of the schedule function)
    }
    pub fn load(&self) {
        Rax::set(self.regs.rax);
        Rbx::set(self.regs.rbx);
        Rcx::set(self.regs.rcx);
        Rdx::set(self.regs.rdx);
        Rsi::set(self.regs.rsi);
        Rdi::set(self.regs.rdi);
        Rbp::set(self.regs.rbp);

        R8::set(self.regs.r8);
        R9::set(self.regs.r9);
        R10::set(self.regs.r10);
        R11::set(self.regs.r11);
        R12::set(self.regs.r12);
        R13::set(self.regs.r13);
        R14::set(self.regs.r14);
        R15::set(self.regs.r15);
        Rflags::set(self.regs.rflags);
        // TODO: Cr3 (switch to Kernel Mode at the start of the schedule function)
    }
    pub fn get_rip(&self) -> u64 {
        self.regs.rip
    }
    pub fn get_rsp(&self) -> u64 {
        self.regs.rsp
    }
}

#[derive(Clone, Debug)]
pub struct Registers {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub rip: u64,
    pub rflags: u64,
    pub fx: usize,
    pub cr3: (PhysFrame, Cr3Flags),
}

impl core::default::Default for Registers {
    fn default() -> Self {
        Self {
            r15: 0x0,
            r14: 0x0,
            r13: 0x0,
            r12: 0x0,
            r11: 0x0,
            r10: 0x0,
            r9: 0x0,
            r8: 0x0,
            rsi: 0x0,
            rsp: 0x0,
            rbp: 0x0,
            rdi: 0x0,
            rdx: 0x0,
            rcx: 0x0,
            rbx: 0x0,
            rax: 0x0,
            rip: 0x0,
            fx: 0x0,
            cr3: Cr3::get(),
            rflags: 0x0,
        }
    }
}
