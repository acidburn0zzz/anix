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
use crate::commands::{blank, hello_world, startfs, clock, user, lspci, help, test_mem};
use crate::screen::WRITER;

pub struct Input{
	pub actived: bool,
	pub content: [char; 30],
	pub number: usize,
}

pub static mut input: Input = Input{
	actived: true,
	content: [' '; 30],
	number: 0,
};

pub fn cmd_character(c: char){
	unsafe{
		if input.actived == true{
			//If enter is pressed exec the function
			if c == '\n'{
				input.actived = false;
				detectcmd(input.content);
				WRITER.lock().new_line();
				print!("Anix>");
				input.content = [' '; 30];
				input.number = 0;
				input.actived = true;
			}
			else{
				print!("{}", c);
				input.content[input.number] = c;
				input.number += 1;
			}
		}
	}
}

pub fn cmd_number(n: pc_keyboard::KeyCode){
	unsafe{
		if input.actived == true{
			print!("{:?}", n);
			input.content[input.number] = n as u8 as char;
			input.number += 1;
		}
	}
}

static commands: [Command; 8] = [
	Command{cmd: [' ';30], function: blank},
	Command{cmd: ['h', 'e', 'l', 'l', 'o', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], function: hello_world},
	Command{cmd: ['f', 's', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], function: startfs},
	Command{cmd: ['t', 'i', 'm', 'e', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], function: clock},
	Command{cmd: ['u', 's', 'e', 'r', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], function: user},
	Command{cmd: ['l', 's', 'p', 'c', 'i', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], function: lspci},
	Command{cmd: ['h', 'e', 'l', 'p', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], function: help},
	Command{cmd: ['m', 'e', 'm', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], function: test_mem},
];

pub struct Command{
    pub cmd: [char; 30],
    pub function: fn(cmd: [char; 30]),
}

///Function for detect command and exec it
pub fn detectcmd(cmd: [char; 30]){
    //TODO: Pass parameters (split the array with spaces)
    let mut commandIsExec = false;
    
    ///Find the function among the array of functions
    for c in commands.iter(){
		if cmd == c.cmd{
			(c.function)(cmd);
			commandIsExec = true;
		}
    }

    //Detects if the function has been executed
    if commandIsExec == false{
		print!("\nUnknown command: {:?}", cmd);
    }
}
