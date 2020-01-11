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
        Cr3,
        Cr3Flags
    }
};

#[derive(Clone, Debug)]
pub struct Registers {
    pub fs: u64,
    pub ss: u64,
    pub cs: u64,
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rsi: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub rdi: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rbx: u64,
    pub rax: u64,
    pub rip: u64,
    pub fx: usize,
    pub cr3: (PhysFrame, Cr3Flags),
    pub rflags: u64,
}

impl core::default::Default for Registers {
    fn default() -> Self {
        Self {
            fs: 0x2b,
            ss: 0x33,
            cs: 0x23,
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
            cr3: Cr3::read(),
            rflags: 0x0,
        }
    }
}

impl Registers {
    fn get_cr3(&self) -> (PhysFrame, Cr3Flags) {
        self.cr3
    }
    pub fn restore_cr3(&self) {
        unsafe {
            Cr3::write(self.cr3.0, self.cr3.1);
        }
    }
    pub fn save_cr3(&mut self) {
        self.cr3 = self.get_cr3();
    }
}
// TODO: impl Registers with get_{register name} with the assembly corresponding code
