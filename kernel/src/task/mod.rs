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
use alloc::prelude::v1::Vec;

pub mod scheduler;

// TODO: Use lazy_static!
pub static mut CURRENT_TASKS: Vec<Option<Task>> = Vec::new();
pub static mut TASK_RUNNING: Option<Task> = None;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState{
    Alive,
    Dead,
    Unknown,
}


#[derive(Debug, Copy, Clone)]
pub struct Task {
    pub name: &'static str,
    pid: usize,
    pub rsp: u64, // Stack pointer
    pub rip: u64, // Instruction pointer (next instruction which will be executed)
    pub rbp: u64,
    pub state: TaskState,
    // TODO: Files used by the task
}

impl Default for Task {
    fn default() -> Self{
        Self {
            name: "NULL",
            pid: 65536,
            rsp: 0,
            rip: 0,
            rbp: 0,
            state: TaskState::Unknown,
        }
    }
}

impl Task {
    /// Create a new alive task and add it in the tasks Vec
    pub unsafe fn new(name: &'static str, addr: u64) -> Self {
        // FIXME: Problem with terminal Task and Vecs
        // TODO: Map each stack pointer
        let mut stack = vec![0; 30000].into_boxed_slice();

        let new_task = Self {
            name: name,
            pid: CURRENT_TASKS.len(),
            rsp: stack.as_mut_ptr() as u64,
            rip: addr,
            rbp: 0,
            state: TaskState::Alive,
        };

        let new_task_option = Some(new_task);
        CURRENT_TASKS.push(new_task_option);
        new_task
    }

    pub unsafe fn kill(&self){
        if self.pid == 0 {
            println!("It is not possible to kill the system task!");
        }
        else {
            CURRENT_TASKS.remove(self.pid);
        }
    }

    /// Give informations about the task
    pub fn information(&self){
        println!("Task {} with pid {}\nRip: {}, Rbp: {} and Rsp: {}\nState: {:?}", self.name,
                                                                          self.pid,
                                                                          self.rip,
                                                                          self.rbp,
                                                                          self.rsp,
                                                                          self.state);
    }

    pub fn getpid(&self) -> usize{
        self.pid
    }

    // TODO: Create a next_to function to use self to save registers
}

pub unsafe fn kill(){
    TASK_RUNNING.unwrap().kill();
}
