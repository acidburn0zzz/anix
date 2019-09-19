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

use idt::*;
use super::irqid::*;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptStackFrame;
use super::syscalls::*;
use alloc::prelude::v1::{Vec};
use spin::Mutex;
use task::Task;

pub struct Event {
    pub r#type: EventType,
    used_by: Task, // TODO: Use just a &'static str with the task name. Use array or Vec<T>?
}

impl Event {
    /// Mark the event as used (useful for don't use the same event two times)
    pub fn mark_as_used(&mut self, task: Task) {
        self.used_by = task;
    }

    /// Search if an event is used by a task
    pub fn is_used_by(&self, task: &'static str) -> bool {
        if self.used_by.name == task {
            true
        }
        else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EventType {
    Keyboard(char),
}

lazy_static!{
    pub static ref EVENTS: Mutex<Vec<Event>> = Mutex::new(Vec::new());
}

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
    use crate::task::scheduler::schedule;

    // Call the schedule function for switching task
    unsafe {
        schedule();
        PICS.lock().notify_end_of_interrupt(TIMER_ID)
    }
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    use pc_keyboard::*;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::MapLettersToUnicode));
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            EVENTS.lock().push(match key {
                DecodedKey::Unicode(character) =>
                    Event {
                        r#type: EventType::Keyboard(character),
                        used_by: Task::default(),
                    },
                DecodedKey::RawKey(key) =>
                    Event {
                        r#type: EventType::Keyboard(key as u8 as char),
                        used_by: Task::default(),
                    },
            });
        }
    }
    unsafe { PICS.lock().notify_end_of_interrupt(KEYBOARD_ID) }
}
pub extern "x86-interrupt" fn cascade_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\n\nCascade\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(CASCADE_ID) }
}

pub extern "x86-interrupt" fn com1_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nCom1\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(COM1_ID) }
}

pub extern "x86-interrupt" fn com2_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nCom2\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(COM2_ID) }
}

pub extern "x86-interrupt" fn lpt1_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nLpt1\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(LPT1_ID) }
}

pub extern "x86-interrupt" fn floppy_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nFloppy\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(FLOPPY_ID) }
}

pub extern "x86-interrupt" fn lpt2_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nLpt2\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(LPT2_ID) }
}

pub extern "x86-interrupt" fn rtc_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nRTC\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(RTC_ID) }
}

pub extern "x86-interrupt" fn pci1_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nPIC1\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(PCI1_ID) }
}

pub extern "x86-interrupt" fn pci2_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nPIC2\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(PCI2_ID) }
}

pub extern "x86-interrupt" fn pci3_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nPIC3\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(PCI3_ID) }
}

pub extern "x86-interrupt" fn mouse_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nMOUSE\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(MOUSE_ID) }
}

pub extern "x86-interrupt" fn fpu_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\nFPU\n{:#?}", stack_frame);
    unsafe { PICS.lock().notify_end_of_interrupt(FPU_ID) }
}

pub extern "x86-interrupt" fn syscall_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("\n\nSYSCALL\n{:#?}", stack_frame);
    unsafe{do_syscall();} // TODO: Manage syscalls
    print!("\nEND OF SYSCALL");
    unsafe { PICS.lock().notify_end_of_interrupt(SYSCALL_ID) }
}
