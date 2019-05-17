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
use crate::screen::*;

///Stop the computer
pub fn hlt_loop() -> ! {
	loop{
		x86_64::instructions::hlt();
	}
}

pub fn error(info: &'static str){
	print!("ERROR: {}", info);
	hlt_loop();
}

///Print [ OK ] on the screen
pub fn ok(){
	WRITER.lock().color_code = ColorCode::new(Color::LightGreen, Color::Black);
	print!(" [ OK ]");
	WRITER.lock().color_code = ColorCode::new(Color::Green, Color::Black);
}

///Rust have a left and right shift strange so we use the C left and right shift
//<< Left shift
//>> Right shift
extern "C" {
	pub fn left_shift(number1: u32, number2: u32) -> u32;
	pub fn right_shift(number1: u32, number2: u32) -> u32;
}
