// The x86-interrupt calling convention leads to the following LLVM error
// when compiled for a Windows target: "offset is not a multiple of 16". This
// happens for example when running `cargo test` on Windows. To avoid this
// problem we skip compilation of this module on Windows.
#![cfg(not(windows))]

use crate::{gdt, print, println, scheduler::time};
use pic8259_simple::ChainedPics;
use spin;
use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable};
use lazy_static::lazy_static;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub const TIMER_INTERRUPT_ID: u8 = PIC_1_OFFSET;
pub const KEYBOARD_INTERRUPT_ID: u8 = PIC_1_OFFSET + 1;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt[usize::from(TIMER_INTERRUPT_ID)].set_handler_fn(timer_interrupt_handler);
        idt[usize::from(KEYBOARD_INTERRUPT_ID)].set_handler_fn(keyboard_interrupt_handler);

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut ExceptionStackFrame,
    _error_code: u64,
) {
    use crate::hlt_loop;

    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    hlt_loop();
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: &mut ExceptionStackFrame) {
	use crate::screen::{WRITER, BUFFER_HEIGHT, BUFFER_WIDTH, ColorCode, Color};
	use crate::scheduler::*;
	unsafe{
		time.deciseconds += 1;
	
		if time.deciseconds >= 19{
			time.seconds += 1;
			time.deciseconds = 0;
		}
	
		if time.seconds >= 59{
			time.minutes += 1;
			time.seconds = 0;
			time.deciseconds = 0;
			
		}
	
		if time.minutes >= 59{
			time.hours += 1;
			time.minutes = 0;
			time.seconds = 0;
			time.deciseconds = 0;
		}
		
	}
	unsafe{
		if time.seconds >= 18 || time.minutes >= 1{
			//Replace the number
			for p in 0..5{
				WRITER.lock().clear_char(0, p, ColorCode::new(Color::Black, Color::Black));
			}
			
			//Print the time in minutes:seconds format
			WRITER.lock().row = 0;
			WRITER.lock().col = 0;
			WRITER.lock().color_code = ColorCode::new(Color::Red, Color::White);
			print!("{}:{}", time.minutes, time.seconds);
			//Wait
			for _l in 0..1000000{}
		}
	}
    unsafe { PICS.lock().notify_end_of_interrupt(TIMER_INTERRUPT_ID) }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: &mut ExceptionStackFrame) {
    use x86_64::instructions::port::Port;
    use crate::screen::{WRITER, BUFFER_HEIGHT, BUFFER_WIDTH, ColorCode, Color};

    use pc_keyboard::{Keyboard, ScancodeSet1, DecodedKey, layouts};
    use spin::Mutex;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1));
    }

    let mut keyboard = KEYBOARD.lock();
    let port = Port::new(0x60);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => {
		    //WRITER.lock().row = 5;
		    //WRITER.lock().col = 0;
		    //WRITER.lock().color_code = ColorCode::new(Color::Yellow, Color::Black);
		    print!("{}", character)
		},
                DecodedKey::RawKey(key) => {
		    //WRITER.lock().row = 5;
		    //WRITER.lock().col = 0;
		    //WRITER.lock().color_code = ColorCode::new(Color::Yellow, Color::Black);
		    print!("{:?}", key)
		},
            }
        }
    }

    unsafe { PICS.lock().notify_end_of_interrupt(KEYBOARD_INTERRUPT_ID) }
}
