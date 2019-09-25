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
use alloc::prelude::v1::{Vec};
use alloc::borrow::ToOwned;

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
    pub esp: u32, // Stack pointer
    pub eip: u32, // Instruction pointer (next instruction which will be executed)
    pub ebp: u32,
    pub state: TaskState,
    // TODO: Files used by the task
}

impl Default for Task {
    fn default() -> Self{
        Self {
            name: "NULL",
            pid: 65536,
            esp: 0,
            eip: 0,
            ebp: 0,
            state: TaskState::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stack {
    pub eflags: u32,
    pub cs: u32,
    pub eip: u32,
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub esi: u32,
    pub edi: u32,
    pub ebp: u32,
    pub ds: u32,
    pub fs: u32,
    pub es: u32,
    pub gs: u32,
}

impl Default for Stack{
    fn default() -> Self{
        Self{
            eflags: 0x00000202,
            cs: 0x8,
            eip: 0,
            eax: 0,
            ebx: 0,
            ecx: 0,
            edx: 0,
            esi: 0,
            edi: 0,
            ebp: 0,
            ds: 0x10,
            fs: 0x10,
            es: 0x10,
            gs: 0x10,
        }
    }
}

impl Task{
    /// Create a new alive task and add it in the tasks Vec
    pub unsafe fn new(name: &'static str, addr: u32) -> Self {
        let new_task = Self {
            name: name,
            pid: CURRENT_TASKS.len(),
            esp: 0,
            eip: addr,
            ebp: 0,
            state: TaskState::Alive,
        };

        let t = Some(new_task.to_owned());
        CURRENT_TASKS.push(t);
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
        print!("\nTask {} with pid {}\nEip: {} and Ebp: {}\nState: {:?}", self.name, self.pid, self.eip, self.ebp, self.state);
    }

    pub fn getpid(&self) -> usize{
        self.pid
    }
}

pub unsafe fn kill(){
    TASK_RUNNING.to_owned().unwrap().kill();
}
