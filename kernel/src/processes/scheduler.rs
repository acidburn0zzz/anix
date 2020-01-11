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

use alloc::prelude::v1::Vec;
use lazy_static::lazy_static;
use spin::RwLock;
/*use x86_64::{
    structures::paging::frame::PhysFrame,
    registers::control::{
        Cr3,
        Cr3Flags
    }
};*/

use super::Process;

lazy_static! {
    pub static ref SCHEDULER: RwLock<Scheduler> = RwLock::new(Scheduler::new());
}

pub struct Scheduler {
    processes: Vec<Process>,
    current_process: Option<usize>,
}

impl Scheduler {
    fn new() -> Self {
        Self {
            processes: Vec::new(),
            current_process: None,
        }
    }

    pub fn add_process(&mut self, process: Process) {
        self.processes.push(process);
    }

    pub unsafe fn schedule(&mut self) {
        // TODO: Go into Kernel Mode here
        if !self.current_process.is_none() && self.current_process.unwrap() < self.processes.len() {
            let current_registers = &mut self.processes[self.current_process.unwrap()].registers;
            // Save all registers
            // current_registers.save_cr3();

            asm!("pushfq ; pop $0" : "=r"(&current_registers.rflags)
                                   :
                                   : "memory"
                                   : "intel", "volatile");

            asm!("mov $0, rbx"     : "=r"(&current_registers.rbx)
                                   :
                                   : "memory"
                                   : "intel", "volatile");

            asm!("mov $0, r12"     : "=r"(&current_registers.r12)
                                   :
                                   : "memory"
                                   : "intel", "volatile");

            asm!("mov $0, r13"     : "=r"(&current_registers.r13)
                                   :
                                   : "memory"
                                   : "intel", "volatile");

            asm!("mov $0, r14"     : "=r"(&current_registers.r14)
                                   :
                                   : "memory"
                                   : "intel", "volatile");

            asm!("mov $0, r15"     : "=r"(&current_registers.r15)
                                   :
                                   : "memory"
                                   : "intel", "volatile");

            asm!("mov $0, rsp"     : "=r"(&current_registers.rsp)
                                   :
                                   : "memory"
                                   : "intel", "volatile");

            asm!("mov $0, rbp"     : "=r"(&current_registers.rbp)
                                   :
                                   : "memory"
                                   : "intel", "volatile");

            if (self.current_process.unwrap() + 1) >= self.processes.len() {
                // Return to the first process to prevent overflow
                self.current_process = Some(0);
            }
            else {
                self.current_process = Some(self.current_process.unwrap() + 1);
            }
        }
        else {
            self.current_process = Some(0);
        }

        let current_registers = &self.processes[self.current_process.unwrap()].registers;

        // current_registers.restore_cr3();

        asm!("push $0 ; popfq"     :
                                   : "r"(current_registers.rflags)
                                   : "memory"
                                   : "intel", "volatile");

        asm!("mov rbx, $0"         :
                                   : "r"(current_registers.rbx)
                                   : "memory"
                                   : "intel", "volatile");

        asm!("mov r12, $0"         :
                                   : "r"(current_registers.r12)
                                   : "memory"
                                   : "intel", "volatile");

        asm!("mov r13, $0"         :
                                   : "r"(current_registers.r13)
                                   : "memory"
                                   : "intel", "volatile");

        asm!("mov r14, $0"         :
                                   : "r"(current_registers.r14)
                                   : "memory"
                                   : "intel", "volatile");

        asm!("mov r15, $0"         :
                                   : "r"(current_registers.r15)
                                   : "memory"
                                   : "intel", "volatile");

        asm!("mov rbp, $0"         :
                                   : "r"(current_registers.rbp)
                                   : "memory"
                                   : "intel", "volatile");

        self.processes[self.current_process.unwrap()].jmp();
    }
    pub fn get_all_processes(&self) -> &Vec<Process> {
        &self.processes
    }
    pub fn request_pid(&mut self) -> usize {
        self.processes.len()
    }
    fn kill_process(&mut self, id: usize) {
        if id != 0 { // Preserve the system process
            self.processes.remove(id);
        }
    }
    fn kill_current_process(&mut self) {
        self.kill_process(self.current_process.unwrap());
    }
}

// Helper functions

/// Kill the current process
pub fn kill() {
    unsafe {
        SCHEDULER.force_write_unlock();
    }
    SCHEDULER.try_write().unwrap().kill_current_process();
}

/// Switch to the next process
pub fn switch() {
    unsafe {
        SCHEDULER.force_write_unlock();
        SCHEDULER.try_write().unwrap().schedule();
    }
}
