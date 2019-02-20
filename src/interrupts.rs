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

// The x86-interrupt calling convention leads to the following LLVM error
// when compiled for a Windows target: "offset is not a multiple of 16". This
// happens for example when running `cargo test` on Windows. To avoid this
// problem we skip compilation of this module on Windows.
#![cfg(not(windows))]

use crate::{gdt, print, println, irq::irq::*, irq::exceptions::*, irq::irqid::*};
use pic8259_simple::ChainedPics;
use spin;
use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable};
use lazy_static::lazy_static;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        
        ///Exceptions
        idt.divide_by_zero.set_handler_fn(divide_by_zero_handler);
        idt.debug.set_handler_fn(debug_handler);
        idt.non_maskable_interrupt.set_handler_fn(non_maskable_interrupt_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.overflow.set_handler_fn(overflow_handler);
        idt.bound_range_exceeded.set_handler_fn(bound_range_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt.device_not_available.set_handler_fn(device_not_available_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt.invalid_tss.set_handler_fn(invalid_tss_handler);
        idt.segment_not_present.set_handler_fn(segment_not_present_handler);
        idt.stack_segment_fault.set_handler_fn(stack_segment_fault_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.x87_floating_point.set_handler_fn(x87_floating_point_handler);
        idt.alignment_check.set_handler_fn(alignment_check_handler);
        idt.machine_check.set_handler_fn(machine_check_handler);
        idt.simd_floating_point.set_handler_fn(simd_floating_point_handler);
        idt.virtualization.set_handler_fn(virtualization_handler);
        idt.security_exception.set_handler_fn(security_exception_handler);
        
        //Irq
        idt[usize::from(TIMER_ID)].set_handler_fn(timer_interrupt_handler);
        idt[usize::from(KEYBOARD_ID)].set_handler_fn(keyboard_interrupt_handler);
        
        idt[usize::from(CASCADE_ID)].set_handler_fn(cascade_interrupt_handler);
		idt[usize::from(COM1_ID)].set_handler_fn(com1_interrupt_handler);
		idt[usize::from(COM2_ID)].set_handler_fn(com2_interrupt_handler);
		idt[usize::from(LPT2_ID)].set_handler_fn(lpt2_interrupt_handler);
		idt[usize::from(FLOPPY_ID)].set_handler_fn(floppy_interrupt_handler);
		idt[usize::from(LPT1_ID)].set_handler_fn(lpt1_interrupt_handler);
		idt[usize::from(RTC_ID)].set_handler_fn(rtc_interrupt_handler);
		idt[usize::from(PCI1_ID)].set_handler_fn(pci1_interrupt_handler);
		idt[usize::from(PCI2_ID)].set_handler_fn(pci2_interrupt_handler);
		idt[usize::from(PCI3_ID)].set_handler_fn(pci3_interrupt_handler);
		idt[usize::from(MOUSE_ID)].set_handler_fn(mouse_interrupt_handler);
		idt[usize::from(FPU_ID)].set_handler_fn(fpu_interrupt_handler);
		idt[usize::from(ATA1_ID)].set_handler_fn(ata1_interrupt_handler);
		idt[usize::from(ATA2_ID)].set_handler_fn(ata2_interrupt_handler);
		idt[usize::from(SYSCALL_ID)].set_handler_fn(syscall_interrupt_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}
