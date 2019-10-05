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

//! For add a command:
//! Add the function in src/commands.rs
//! And in src/user/input.rs in static commands, change length of array and add a struct command
//! # Example
//! ```
//! // In src/commands.rs
//! pub fn test() {
//!     println!("test");
//! }
//! ```
//! ```
//! // In src/user/input.rs
//! use crate::commands::test;
//! let commands: [Command; 1] = [
//!     Command{cmd: "test".to_string(), function: test},
//! ];
//! ```

use alloc::string::String;

pub fn blank(_cmd: String) {
    println!("Input is blank!");
}

pub fn hello_world(_cmd: String) {
    println!("\nHello world!");
}

pub fn date(_cmd: String) {
    use time::DateTime;
    println!("{}", DateTime::new());
}

pub fn help(_cmd: String) {
    print!("\nCommands:\n    help: Display this help\n    hello: Print \"Hello world!\"\n    startflame: Start flame (the graphic interface)\n    time: Print the time since you have started Anix\n    user: (not work)\n    lspci: Print all pci devices\n    mem: Test memory");
}

pub fn lspci(_cmd: String) {
    use crate::pci;
    pci::list_devices();
}

pub fn test_mem(_cmd: String) {
    use crate::memory::table;

    print!("\nTEST: Tables");
    table::test();

    print!("\nTEST: Translate VirtAddr to PhysAddr");
    unsafe{
        let page = table::ActivePageTable::new();
        let addr_translated = page.translate(0xb8000).unwrap();
        print!("\n0xb8000 -> {:#x}", addr_translated);
    }
}

pub fn startflame(_cmd: String) {
    use graphics::vbe;
    println!("\nFlame is starting...");

    vbe::init();

    // TODO: Jump in userspace, start filesystem and start graphics

    println!("Flame say to you hello!!!");
}
