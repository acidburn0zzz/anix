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
use alloc::prelude::v1::{ToOwned, Vec};
use alloc::sync::Arc;
use crate::fs::ext2::file::File;
use spin::Mutex;

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


#[derive(Debug, Clone)]
pub struct Task {
    pub name: &'static str,
    pid: usize,
    pub rsp: u64, // Stack pointer
    pub rip: u64, // Instruction pointer (next instruction which will be executed)
    pub rbp: u64,
    pub state: TaskState,
    pub fds: Arc<Mutex<Vec<File>>>, // Files used by the task
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
            fds: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Task {
    /// Create a new alive task and add it in the tasks Vec
    pub unsafe fn new(name: &'static str, addr: u64) -> Self {
        // TODO: Map each stack pointer
        let mut stack = vec![0; 65_536].into_boxed_slice();

        let new_task = Self {
            name: name,
            pid: CURRENT_TASKS.len(),
            rsp: stack.as_mut_ptr() as u64,
            rip: addr,
            rbp: 0,
            state: TaskState::Alive,
            fds: Arc::new(Mutex::new(Vec::new())),
        };

        let new_task_option = Some(new_task.to_owned());
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
        println!("Task {} with pid {}\nRip: {}, Rbp: {} and Rsp: {}\nState: {:?}\nOpened files: {:?}", self.name,
                                                                          self.pid,
                                                                          self.rip,
                                                                          self.rbp,
                                                                          self.rsp,
                                                                          self.state,
                                                                          self.fds);
    }

    pub fn getpid(&self) -> usize{
        self.pid
    }

    pub fn next_file_id(&self) -> usize {
        self.fds.lock().len()
    }
    pub fn add_new_file(&self, file: File) {
        unsafe {
            CURRENT_TASKS[self.pid].to_owned().unwrap().fds.lock().push(file);
        }
    }

    // TODO: Create a next_to function to use self to save registers
}

pub unsafe fn kill(){
    TASK_RUNNING.to_owned().unwrap().kill();
}
