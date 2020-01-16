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
            self.processes[self.current_process.unwrap()].ctx.save();
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

        self.processes[self.current_process.unwrap()].ctx.load();
        self.processes[self.current_process.unwrap()].jmp();
    }
    pub fn get_all_processes(&self) -> &Vec<Process> {
        &self.processes
    }
    pub fn get_current_process(&self) -> Result<&Process, core::option::NoneError> {
        let current = self.current_process?;
        Ok(&self.processes[current])
    }
    pub fn get_current_process_mut(&mut self) -> &mut Process {
        &mut self.processes[self.current_process.unwrap()]
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
