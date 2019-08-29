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
use core::ptr::copy_nonoverlapping;
use crate::screen::*;

/// Stop the computer
pub fn hlt_loop() -> ! {
    loop{
        x86_64::instructions::hlt();
    }
}

pub fn error(info: &'static str){
    print!("ERROR: {}", info);
    hlt_loop();
}

/// Print [ OK ] on the screen
pub fn ok(){
    WRITER.lock().color_code = ColorCode::new(Color::LightGreen, Color::Black);
    print!(" [ OK ]");
    WRITER.lock().color_code = ColorCode::new(Color::Green, Color::Black);
}

#[macro_export]
macro_rules! read_num_bytes {
    ($ty:ty, $src:expr) => ({
        let size = core::mem::size_of::<$ty>();
        assert!(size <= $src.len());
        let mut data: $ty = 0;
        unsafe {
            copy_nonoverlapping(
                $src.as_ptr(),
                &mut data as *mut $ty as *mut u8,
                size);
        }
        data
    });
}
