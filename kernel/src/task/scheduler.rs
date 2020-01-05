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

use crate::task::*;

/// Change the current task
pub unsafe fn schedule(){
    switch();
}

pub unsafe fn switch() {
    // Save registers
    save_registers();

    // Change task
    if (TASK_RUNNING.unwrap().pid + 1) < CURRENT_TASKS.len() {
        // Select the next task
        TASK_RUNNING = CURRENT_TASKS[TASK_RUNNING.unwrap().pid + 1];
    }
    else {
        // Come back on the start of the tasks array
        TASK_RUNNING = CURRENT_TASKS[0];
    }

    // Restore registers
    restore_registers();

    // Run task
    run_task();

}

/// Save the state of the registers in the tasks array
pub unsafe fn save_registers() {
    use x86::bits64::registers::*;

    // Copy the registers in the tasks array
    TASK_RUNNING.unwrap().rsp = rsp(); // Copy the stack
    TASK_RUNNING.unwrap().rip = rip(); // Copy the instruction pointer
    TASK_RUNNING.unwrap().rbp = rbp(); // Copy the control register
}

/// Restore the state of the registers saved in the tasks array
pub unsafe fn restore_registers(){
    let esp: u64 = TASK_RUNNING.unwrap().rsp;
    let ebp: u64 = TASK_RUNNING.unwrap().rbp;

    // Get the state of the registers
    asm!("movq %rsp, %rax"
        :
        : "{rax}"(esp)
        : "memory"
        : "volatile"
        );

    asm!("movq %rbp, %rax"
        :
        : "{rax}"(ebp)
        : "memory"
        : "volatile"
        );
}

/// Jump to the function
pub unsafe fn run_task(){
    // TODO: Embed fs, gs, ss, cs and es in the Task struct to choose if the task will be run in
    // userspace or not + load them with x86::segmentation::load_{es, cs, ss, fs, gs}

    usermode(TASK_RUNNING.unwrap().rip as u32, TASK_RUNNING.unwrap().rsp as u32, 0);
}

#[naked]
unsafe fn usermode(ip: u32, sp: u32, arg: u32) {
    use crate::gdt;
    asm!("push r10
          push r11
          push r12
          push r13
          push r14
          push r15"
          : // No output
          :   "{r10}"(gdt::GDT_USER_DATA << 3 | 3), // Data segment
              "{r11}"(sp), // Stack pointer
              "{r12}"(1 << 9), // Flags - Set interrupt enable flag
              "{r13}"(gdt::GDT_USER_CODE << 3 | 3), // Code segment
              "{r14}"(ip), // IP
              "{r15}"(arg) // Argument
          : // No clobbers
          : "intel", "volatile");

    // Go to usermode
    asm!("mov ds, r14d
         mov es, r14d
         mov fs, r15d
         mov gs, r14d
         xor rax, rax
         xor rbx, rbx
         xor rcx, rcx
         xor rdx, rdx
         xor rsi, rsi
         xor rdi, rdi
         xor rbp, rbp
         xor r8, r8
         xor r9, r9
         xor r10, r10
         xor r11, r11
         xor r12, r12
         xor r13, r13
         xor r14, r14
         xor r15, r15
         finit
         pop rdi
         iretq"
         : // No output because it never returns
         :   "{r14}"(gdt::GDT_USER_DATA << 3 | 3), // Data segment
             "{r15}"(gdt::GDT_USER_TLS << 3 | 3) // TLS segment
         : // No clobbers because it never returns
         : "intel", "volatile");
    unreachable!();

}
