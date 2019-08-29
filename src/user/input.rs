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
use crate::commands::{blank, hello_world, date, user, lspci, help, test_mem, startflame};
use crate::screen::WRITER;
use alloc::prelude::v1::{String, ToString};
use core::ptr;
use lazy_static::lazy_static;
use spin::Mutex;

#[derive(Clone)]
pub struct Input{
    pub actived: bool,
    pub content: String,
}

#[derive(Clone)]
pub struct Command{
    pub cmd: String,
    pub function: fn(cmd: String),
}

lazy_static! {
    pub static ref INPUT: Mutex<Input> = Mutex::new(Input {
        actived: true,
        content: "".to_string(),
    });
}

pub unsafe fn cmd_character(c: char) {
    if INPUT.lock().actived == true {
        // If enter is pressed exec the function
        if c == '\n'{
            // Block the input
            INPUT.lock().actived = false;

            // Do the command
            detectcmd(INPUT.lock().content.clone());

            // Show the prompt
            WRITER.lock().new_line();
            print!("xsh>");

            // Clear the buffer
            INPUT.lock().content = "".to_string();

            // Reactivate input
            INPUT.lock().actived = true;
        }
        else{
            print!("{}", c);
            INPUT.lock().content.push(c);
        }
    }
}

pub unsafe fn cmd_number(n: pc_keyboard::KeyCode) {
    if INPUT.lock().actived == true {
        print!("{:?}", n);
        INPUT.lock().content.push(n as u8 as char);
    }
}

/// Function for detect command and exec it
pub unsafe fn detectcmd(cmd: String){
    // TODO: Use Vec instead of Array
    let commands = [
        Command{cmd: "hello".to_string(), function: hello_world},
        Command{cmd: "date".to_string(), function: date},
        Command{cmd: "user".to_string(), function: user},
        Command{cmd: "lspci".to_string(), function: lspci},
        Command{cmd: "help".to_string(), function: help},
        Command{cmd: "mem".to_string(), function: test_mem},
        Command{cmd: "startflame".to_string(), function: startflame},
    ];

    let mut is_command_exec = false;

    // Find the function in the array of functions
    for c in commands.iter(){
        // If the command selected is the same as the input
        if cmd.clone().as_str().starts_with(c.cmd.clone().as_str()){
            // Call the function

            // TODO: Decomment this when we can kill tasks
            // let cmd_task = Task::new(c.cmd.clone(), (c.function) as *const () as u32);
            (c.function)(cmd.clone());
            // cmd_task.kill();
            is_command_exec = true;
        }
    }

    // Detects if the function has been executed
    if is_command_exec == false{
        print!("\nxsh: command not found: {:?}", cmd);
    }
}
