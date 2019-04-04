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

///For add a command:
///Add the function in src/commands.rs
///And in src/user/input.rs in static commands, change length of array and add a struct command
/// # Example
///```
///In src/commands.rs
///pub fn test(){
///		println!("TEST");
///}
///```
///```
///In src/user/input.rs
///use crate::commands::test;
///static commands: [Command; 1] = [
///		Command{cmd: ['t', 'e', 's', 't', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], function: test},
///];
///```
use crate::screen::{ColorCode, Color, WRITER};

pub fn blank(cmd: [char; 30]){
    println!("Input is blank!");
}

pub fn hello_world(cmd: [char; 30]){
    println!("\nHello world!");
}

pub fn startfs(cmd: [char; 30]){
    use crate::fs;
    fs::fs::fsmain();
}

pub fn clock(cmd: [char; 30]){
    use time::*;
	//let day = number_to_day(n.day);
	unsafe{print!("\n{}:{}:{}", time.hours, time.minutes, time.seconds);}
}

pub fn user(cmd: [char; 30]){
    use crate::user::switch::init_user;
    init_user();
}

pub fn help(cmd: [char; 30]){
    print!("\nCommands:\n   hello: Print \"Hello world!\"\n   fs: Test fs\n   clock: Print the time since you have started Anix\n   reset: Restart Anix\n   user: (not work)\n   lspci: Print all pci devices\n   mem: Test memory");
}

pub fn lspci(cmd: [char; 30]){
	extern "C" {
		fn lspci(row: usize, col: usize, color: u8);
	}
	unsafe{
		let r = WRITER.lock().row;
		let c = WRITER.lock().col;
		let color = WRITER.lock().color_code.0;
		lspci(r, c, color);
	}
}

pub fn test_mem(cmd: [char; 30]){
	use crate::memory::table;
	use crate::common::ok;
	
	print!("\nTEST: Tables");
	table::test();
	ok();
	
	print!("\nTEST: Translate VirtAddr to PhysAddr");
	unsafe{
		let page = table::ActivePageTable::new();
		let addr_translated = page.translate(0xb8000).unwrap();
		ok();
		print!("\n0xb8000 -> {:#x}", addr_translated);
	}
}
