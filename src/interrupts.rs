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

pub struct Input{
	pub actived: bool,
	pub content: [char; 30],
	pub number: usize,
}

pub static mut input: Input = Input{
	actived: true,
	number: 0,
	content: [' '; 30],
};

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
	/*unsafe{
	    if time.seconds >= 18 || time.minutes >= 1{
			//Print the time in minutes:seconds format
			WRITER.lock().row = 0;
			WRITER.lock().col = 0;
			WRITER.lock().color_code = ColorCode::new(Color::Red, Color::White);
			print!("{}:{}", time.minutes, time.seconds);
	    }
    }*/
    unsafe{PICS.lock().notify_end_of_interrupt(TIMER_INTERRUPT_ID)}
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
		    unsafe{
			if input.actived == true{
			    //If enter is pressed exec the function
			    if character == '\n'{
				input.actived = false;
				detectcmd(input.content);
				WRITER.lock().new_line();
				print!(">");
				input.content = [' '; 30];
				input.number = 0;
				input.actived = true;
			    }
			    else{
				print!("{}", character);
				input.content[input.number] = character;
				input.number += 1;
			    }
			}
		    }
		},
                DecodedKey::RawKey(key) => {
		    unsafe{
			if input.actived == true{
			    print!("{:?}", key);
			}
		    }
		},
            }
        }
    }

    unsafe { PICS.lock().notify_end_of_interrupt(KEYBOARD_INTERRUPT_ID) }
}

pub fn unknown_command(cmd: [char; 30]){
    println!("Unknow command");
}

pub fn hello_world(cmd: [char; 30]){
    print!("Hello world !");
}

static commands: [Command; 2] = [
    Command{cmd: [' '; 30], function: unknown_command},
    Command{cmd: ['h', 'e', 'l', 'l', 'o', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], function: hello_world},
];

pub struct Command{
    pub cmd: [char; 30],
    pub function: fn(cmd: [char; 30]),
}

///Function for detect command and exec it
pub fn detectcmd(cmd: [char; 30]){
    //TODO: be able to pass parameters (split the array with spaces)
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
	println!("Unknow command: {:?}", cmd);
    }
}
