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
pub mod scheduler;
pub mod context;

use alloc::prelude::v1::{String, Box};

use self::scheduler::*;

#[derive(Clone, Debug)]
struct Arguments {
    argc: isize,
    argv: u64,
}

#[derive(Clone, Debug)]
pub struct Process {
    name: String,
    pid: usize,
    registers: context::Registers,
    args: Arguments,
}

impl Process {
    pub fn new(name: String,
               entry: u64,
               argv: Box<&[&[u8]]>) -> Self {
        let stack = vec![0; 65_536].into_boxed_slice().as_mut_ptr() as u64;

        let mut new_process = Self {
          name,
          pid: SCHEDULER.try_write().unwrap().request_pid(),
          registers: context::Registers::default(),
          args: Arguments {
            argc: argv.len() as isize,
            argv: argv.as_ptr() as u64,
          },
        };

        // Set entry
        new_process.set_entry(entry);

        // Set the stack
        new_process.registers.rsp = stack as u64;

        SCHEDULER.try_write().unwrap().add_process(new_process.clone());

        new_process
    }

    unsafe fn jmp(&self) {
        usermode(self.registers.rip,
                 self.registers.rsp,
                 self.args.argc as u64,
                 self.args.argv);
    }

    fn set_entry(&mut self, entry: u64) {
        self.registers.rip = entry;
    }
    pub fn getpid(&self) -> usize {
        self.pid
    }
}

#[naked]
unsafe fn usermode(ip: u64, sp: u64, arg1: u64, arg2: u64) {
    use crate::gdt;
    // Create the stack frame
    asm!("push r9
          push r10
          push r11
          push r12
          push r13
          push r14
          push r15"
          : // No output
          :   "{r9}"(gdt::GDT_USER_DATA << 3 | 3), // Data segment
              "{r10}"(sp), // Stack pointer
              "{r11}"(1 << 9), // Flags - Set interrupt enable flag
              "{r12}"(gdt::GDT_USER_CODE << 3 | 3), // Code segment
              "{r13}"(ip), // IP
              "{r14}"(arg2), // Argument 2 (argv)
              "{r15}"(arg1) // Argument 1 (argc)
          : // No clobbers
          : "intel", "volatile");

    // Go to usermode
    asm!("mov ds, r14d
         mov es, r14d
         mov fs, r15d
         mov gs, r14d
         xor rax, rax
         xor rbx, rbx
         xor rcx, rcx
         xor rdx, rdx
         xor rsi, rsi
         xor rdi, rdi
         xor rbp, rbp
         xor r8, r8
         xor r9, r9
         xor r10, r10
         xor r11, r11
         xor r12, r12
         xor r13, r13
         xor r14, r14
         xor r15, r15
         finit
         pop rdi
         pop rsi
         iretq"
         : // No output because it never returns
         :   "{r14}"(gdt::GDT_USER_DATA << 3 | 3), // Data segment
             "{r15}"(gdt::GDT_USER_TLS << 3 | 3) // TLS segment
         : // No clobbers because it never returns
         : "intel", "volatile");
    unreachable!();

}
