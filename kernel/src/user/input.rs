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
use alloc::prelude::v1::{String, ToString, ToOwned};
use lazy_static::lazy_static;
use spin::Mutex;

use crate::commands::{hello_world, date, lspci, help, test_mem, startflame};

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

pub fn terminal() {
    use crate::syscall::call::exit;
    use crate::debug;
    debug!("The terminal doesn't work yet. \
            But, I can get the time: {} and I can convert it to a timestamp: {} ;)\n",
            date(), date().to_timestamp());
    /*use ::debug;
    for e in &mut *EVENTS.lock() {
        if e.is_used_by("terminal") == false {
            if let EventType::Keyboard(c) = e.r#type {
                if INPUT.lock().actived == true {
                    if c == '\n' {
                        // Block the input
                        INPUT.lock().actived = false;

                        // Do the command
                        detectcmd(INPUT.lock().content.to_owned());

                        // Show the prompt
                        WRITER.lock().new_line();
                        debug!("xsh>");

                        // Clear the buffer
                        INPUT.lock().content = "".to_string();

                        // Reactivate input
                        INPUT.lock().actived = true;
                    }
                    else{
                        debug!("{}", c.to_string());
                        INPUT.lock().content.push_str(c.to_string().as_str());
                    }
                }
            }
            e.mark_as_used(TASK_RUNNING.unwrap());
        }
    }*/
    exit();
}

/// Function for detect command and exec it
pub unsafe fn detectcmd(cmd: String){
    // TODO: Use Vec instead of Array
    let commands = [
        Command{cmd: "hello".to_string(), function: hello_world},
        Command{cmd: "date".to_string(), function: date},
        Command{cmd: "lspci".to_string(), function: lspci},
        Command{cmd: "help".to_string(), function: help},
        Command{cmd: "mem".to_string(), function: test_mem},
        Command{cmd: "startflame".to_string(), function: startflame},
    ];

    let mut is_command_exec = false;

    // Find the function in the array of functions
    for c in commands.iter(){
        // If the command selected is the same as the input
        if cmd.to_owned().as_str().starts_with(c.cmd.to_owned().as_str()){
            // Call the function

            // TODO: Decomment this when we can kill tasks
            // let cmd_task = Task::new(c.cmd.to_owned(), (c.function) as *const () as u32);
            (c.function)(cmd.to_owned());
            // cmd_task.kill();
            is_command_exec = true;
        }
    }

    // Detects if the function has been executed
    if is_command_exec == false{
        print!("\nxsh: command not found: {:?}", cmd);
    }
}
