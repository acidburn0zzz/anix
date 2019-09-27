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

#![allow(unused_assignments)]
#![allow(unused_variables)]

use crate::task::*;
use x86_64::registers::read_rip;
use alloc::borrow::ToOwned;

/// Change the current task
pub unsafe fn schedule(){
    // Save registers
    save_registers();

    // Change task
    if (TASK_RUNNING.to_owned().unwrap().pid + 1) < CURRENT_TASKS.len() {
        // Select the next task
        TASK_RUNNING = CURRENT_TASKS[TASK_RUNNING.to_owned().unwrap().pid + 1].to_owned();
    }
    else {
        // Come back on the start of the tasks array
        TASK_RUNNING = CURRENT_TASKS[0].to_owned();
    }

    // Restore registers
    restore_registers();

    // Run task
    run_task();
}

/// Save the state of the registers in the tasks array
pub unsafe fn save_registers(){
    let mut esp: u32 = 0; // TODO: Use x86::bits64::registers::rsp
    let mut eip: u32 = 0; // TODO: Use x86::bits64::registers::rip
    let mut ebp: u32 = 0; // TODO: Use x86::bits64::registers::rbp

    // Get the state of the registers
    asm!("movq %rsp, %rax"
        : "={rax}" (esp)
        :
        : "memory"
        : "volatile"
        );

    asm!("movq %rax, %rbp"
        : "={rax}"(ebp)
        :
        : "memory"
        : "volatile"
        );


    eip = read_rip() as u32;

    // Copy the registers in the tasks array
    TASK_RUNNING.to_owned().unwrap().esp = esp; // Copy the stack
    TASK_RUNNING.to_owned().unwrap().eip = eip; // Copy the instruction pointer
    TASK_RUNNING.to_owned().unwrap().ebp = ebp; // Copy the control register
}

/// Restore the state of the registers saved in the tasks array
pub unsafe fn restore_registers(){
    let mut esp: u32 = TASK_RUNNING.to_owned().unwrap().esp;
    let mut ebp: u32 = TASK_RUNNING.to_owned().unwrap().ebp;
    // TODO: Volatile!!!

    // Get the state of the registers
    asm!("movq %rsp, %rax"
        : "={rax}"(esp)
        :
        : "memory"
        : "volatile"
        );

    asm!("movq %rbp, %rax"
        : "={rax}"(ebp)
        :
        : "memory"
        : "volatile"
        );
}

/// Jump to the function
pub unsafe fn run_task(){
    asm!("call rax"
        :
        : "{rax}"(TASK_RUNNING.to_owned().unwrap().eip)
        :
        : "intel", "volatile"
        );
}
