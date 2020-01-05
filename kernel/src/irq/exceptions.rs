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

use x86_64::structures::idt::InterruptStackFrame;
use x86_64::structures::idt::PageFaultErrorCode;

use crate::common::hlt_loop;

pub extern "x86-interrupt" fn divide_by_zero_handler(stack_frame: &mut InterruptStackFrame) {
    print!("EXCEPTION: DIVIDE_BY_ZERO\n{:#?}", stack_frame);
    hlt_loop();
}

pub extern "x86-interrupt" fn debug_handler(stack_frame: &mut InterruptStackFrame) {
    print!("EXCEPTION: DEBUG\n{:#?}", stack_frame);
    hlt_loop();
}

pub extern "x86-interrupt" fn non_maskable_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
    print!("EXCEPTION: NON MASKABLE INTERRUPT\n{:#?}", stack_frame);
    hlt_loop();
}

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    print!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    hlt_loop();
}

pub extern "x86-interrupt" fn overflow_handler(stack_frame: &mut InterruptStackFrame) {
    print!("EXCEPTION: OVERFLOW\n{:#?}", stack_frame);
    hlt_loop();
}

pub extern "x86-interrupt" fn bound_range_handler(stack_frame: &mut InterruptStackFrame) {
    print!("EXCEPTION: OVERFLOW\n{:#?}", stack_frame);
    hlt_loop();
}

pub extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: &mut InterruptStackFrame) {
    print!("EXCEPTION: INVALID OPCODE\n{:#?}", stack_frame);
    hlt_loop();
}

pub extern "x86-interrupt" fn device_not_available_handler(stack_frame: &mut InterruptStackFrame) {
    print!("EXCEPTION: DEVICE NOT AVAILABLE\n{:#?}", stack_frame);
    hlt_loop();
}

pub extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut InterruptStackFrame, _error_code: u64) {
    print!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    hlt_loop();
}

pub extern "x86-interrupt" fn invalid_tss_handler(stack_frame: &mut InterruptStackFrame, _error_code: u64) {
    print!("EXCEPTION: INVALID TSS\n{:#?}", stack_frame);
    hlt_loop();
}

pub extern "x86-interrupt" fn segment_not_present_handler(stack_frame: &mut InterruptStackFrame, _error_code: u64) {
    print!("EXCEPTION: SEGMENT NOT PRESENT\n{:#?}", stack_frame);
    hlt_loop();
}

pub extern "x86-interrupt" fn stack_segment_fault_handler(stack_frame: &mut InterruptStackFrame, _error_code: u64) {
    print!("EXCEPTION: STASK SEGMENT FAULT\n{:#?}", stack_frame);
    hlt_loop();
}

pub extern "x86-interrupt" fn general_protection_fault_handler(stack_frame: &mut InterruptStackFrame, _error_code: u64) {
    print!("EXCEPTION: GENERAL PROTECTION FAULT\n{:#?}", stack_frame);
    hlt_loop();
}


pub extern "x86-interrupt" fn page_fault_handler(stack_frame: &mut InterruptStackFrame, error_code: PageFaultErrorCode) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    hlt_loop();
}

pub extern "x86-interrupt" fn x87_floating_point_handler(stack_frame: &mut InterruptStackFrame) {
    print!("EXCEPTION: X86 FLOATING POINT\n{:#?}", stack_frame);
    hlt_loop();
}

pub extern "x86-interrupt" fn alignment_check_handler(stack_frame: &mut InterruptStackFrame, _error_code: u64) {
    print!("EXCEPTION: ALIGNEMENT CHECK\n{:#?}", stack_frame);
    hlt_loop();
}

pub extern "x86-interrupt" fn machine_check_handler(stack_frame: &mut InterruptStackFrame) {
    print!("EXCEPTION: MACHINE CHECK\n{:#?}", stack_frame);
    hlt_loop();
}

pub extern "x86-interrupt" fn simd_floating_point_handler(stack_frame: &mut InterruptStackFrame) {
    print!("EXCEPTION: SIMD FLOATING POINT\n{:#?}", stack_frame);
	hlt_loop();
}

pub extern "x86-interrupt" fn virtualization_handler(stack_frame: &mut InterruptStackFrame) {
    print!("EXCEPTION: VIRTUALIZATION\n{:#?}", stack_frame);
    hlt_loop();
}

pub extern "x86-interrupt" fn security_exception_handler(stack_frame: &mut InterruptStackFrame, _error_code: u64) {
    print!("EXCEPTION: SECURITY EXCEPTION\n{:#?}", stack_frame);
    hlt_loop();
}
