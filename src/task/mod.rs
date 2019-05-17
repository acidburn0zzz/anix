/*Copyright (C) 2018-2019 Nicolas Fouquet 

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see https://www.gnu.org/licenses.
*/
pub mod scheduler;
use core::ffi::c_void;

pub static mut CURRENT_PID: usize = 0; //The pid for create a new task
//TODO: Use a Vec
pub static mut CURRENT_TASKS: [Option<Task>; 4096] = [None; 4096]; //The array which contains all tasks
pub static mut TASK_RUNNING: Option<Task> = None; //The pid of the running task

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState{
	Alive = 0,
	Dead = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Task {
	pub name: &'static str,
	pub pid: usize,
	pub esp: u32, //Stack pointer
	pub eip: u32, //Instruction pointer (next instruction which will be executed)
	pub ebp: u32,
	pub state: TaskState,
	//TODO: Files used by the task
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
	///Create a new alive task and add it in the tasks array
	pub unsafe fn new(name: &'static str, addr: u32) -> Self{
		let new_task = Self{
			name: name,
			pid: CURRENT_PID,
			esp: 0,
			eip: addr,
			ebp: 0,
			state: TaskState::Alive,
		};
		
		CURRENT_TASKS[CURRENT_PID] = Some(new_task);
		CURRENT_PID += 1;
		new_task
	}
	
	pub unsafe fn kill(&self){
		//TODO: Kill a task with this function
		//		-Dont use the pid such as tasks array index
		//		-Create the task on a None or a Dead task
		//		-Run the task if it is not a Dead task
		
		//Dont work
		for selected_task in self.pid..4096 {
			if CURRENT_TASKS[selected_task as usize] == None {
				//No more tasks
				break;
			}
			if CURRENT_TASKS[selected_task as usize].unwrap().pid == 0 {
				//Nothing to do it is the system task!
				print!("\nIt is not possible to kill the system task!");
			}
			else{
				CURRENT_TASKS[selected_task as usize] = CURRENT_TASKS[(selected_task + 1) as usize];
				CURRENT_TASKS[selected_task as usize].unwrap().pid = selected_task;
			}
		}
	}
	
	///Give informations about the task
	pub fn information(&self){
		print!("\nTask {} with pid {}\nEip: {} and Ebp: {}\nState: {:?}", self.name, self.pid, self.eip, self.ebp, self.state);
	}

	pub fn getpid(&self) -> usize{
		self.pid
	}
}

pub unsafe fn kill(){
	TASK_RUNNING.unwrap().kill();
}
