//!For add a command:
//!Add the function in src/commands.rs
//!And in src/user/input.rs in static commands, change length of array and add a struct command
//! # Example
//!```
//! //In src/commands.rs
//!pub fn test(){
//!	println!("test");
//!}
//!```
//!```
//! //In src/user/input.rs
//!use crate::commands::test;
//!static commands: [Command; 1] = [
//!	Command{cmd: ['t', 'e', 's', 't', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], function: test},
//!];
//!```

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

use crate::screen::{ColorCode, Color, WRITER};
use alloc::string::String;

pub fn blank(cmd: String){
    println!("Input is blank!");
}

pub fn hello_world(cmd: String){
    println!("\nHello world!");
}

pub fn startfs(cmd: String){
    use crate::fs;
    fs::fs::fsmain();
}

pub fn clock(cmd: String){
    use time::*;
	//let day = number_to_day(n.day);
	unsafe{print!("\n{}:{}:{}", time.hours, time.minutes, time.seconds);}
}

pub fn user(cmd: String){
    use crate::user::switch::init_user;
    init_user();
}

pub fn help(cmd: String){
    print!("\nCommands:\n   hello: Print \"Hello world!\"\n   fs: Test fs\n   clock: Print the time since you have started Anix\n   reset: Restart Anix\n   user: (not work)\n   lspci: Print all pci devices\n   mem: Test memory");
}

pub fn lspci(cmd: String){
	//use crate::pci::pci_probe;
	extern "C" {
		fn lspci(row: usize, col: usize, color: u8) -> u32;
	}
	unsafe{
		let r = WRITER.lock().row;
		let c = WRITER.lock().col;
		let color = WRITER.lock().color_code.0;
		lspci(r, c, color);
	}
	/*unsafe{
		pci_probe();
	}*/
}

pub fn test_mem(cmd: String){
	use crate::memory::table;
	use crate::common::ok;
	
	print!("\nTEST: Tables");
	table::test();
	ok();
	
	print!("\nTEST: Translate VirtAddr to PhysAddr");
	unsafe{
		let page = table::ActivePageTable::new();
		let addr_translated = page.translate(0x20010a).unwrap();
		ok();
		print!("\n0xb8000 -> {:#x}", addr_translated);
	}
}
