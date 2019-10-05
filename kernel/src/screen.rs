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
use core::fmt;
use spin::Mutex;
use volatile::Volatile;
use lazy_static::lazy_static;
use x86::io::*;

lazy_static! {
    /// A global `Writer` instance that can be used for printing to the VGA text buffer.
    ///
    /// Used by the `print!` and `println!` macros.
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        row: BUFFER_HEIGHT - 1,
        col: 0,
        color_code: ColorCode::new(Color::Green, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

/// The standard color palette in VGA text mode.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

/// A combination of a foreground and a background color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorCode(pub u8);

impl ColorCode {
    /// Create a new `ColorCode` with the given foreground and background colors.
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// A screen character in the VGA text buffer, consisting of an ASCII character and a `ColorCode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

/// The height of the text buffer (normally 25 lines).
pub const BUFFER_HEIGHT: usize = 25;
/// The width of the text buffer (normally 80 columns).
pub const BUFFER_WIDTH: usize = 80;

/// A structure representing the VGA text buffer.
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// A writer type that allows writing ASCII bytes and strings to an underlying `Buffer`.
///
/// Wraps lines at `BUFFER_WIDTH`. Supports newline characters and implements the
/// `core::fmt::Write` trait.
pub struct Writer {
    pub row: usize,
    pub col: usize,
    pub color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    /// Writes an ASCII byte to the buffer.
    ///
    /// Wraps lines at `BUFFER_WIDTH`. Supports the `\n` newline character.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row;
                let col = self.col;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.col += 1;
            }
        }
    }

    /// Writes the given ASCII string to the buffer.
    ///
    /// Wraps lines at `BUFFER_WIDTH`. Supports the `\n` newline character. Does **not**
    /// support strings with non-ASCII characters, since they can't be printed in the VGA text
    /// mode.
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    /// Shifts all lines one line up and clears the last row.
    pub fn new_line(&mut self) {
        for r in 1..BUFFER_HEIGHT {
            for c in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[r][c].read();
                self.buffer.chars[r - 1][c].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1, ColorCode::new(Color::Black, Color::Black));
        self.col = 0;
    }

    /// Clears a row by overwriting it with blank characters.
    pub fn clear_row(&mut self, r: usize, color: ColorCode) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: color,
        };
        for c in 0..BUFFER_WIDTH {
            self.buffer.chars[r][c].write(blank);
        }
    }

    pub fn clear_char(&mut self, r: usize, c: usize, color: ColorCode) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: color,
        };
		self.buffer.chars[r][c].write(blank);
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // Redirect text buffer to standard output because we are not in text mode
        #[cfg(feature="x86_64-qemu-Anix")]
        use ::serial_print;
        #[cfg(feature="x86_64-qemu-Anix")]
        serial_print!("{}", s);
        // self.write_string(s);
        Ok(())
    }
}

pub fn fill(color: ColorCode){
    for x in 0..BUFFER_HEIGHT{
        WRITER.lock().clear_row(x, color);
    }
}

/// Like the `print!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::screen::_print(format_args!($($arg)*)));
}

/// Like the `println!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Prints the given formatted string to the VGA text buffer through the global `WRITER` instance.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    // interrupts::without_interrupts(|| {
     WRITER.lock().write_fmt(args).unwrap();
    //});
}

pub fn starter_screen(){
    fill(ColorCode::new(Color::Blue, Color::Blue));
    fill(ColorCode::new(Color::Black, Color::Black));
    logo_screen();
    fill(ColorCode::new(Color::Green, Color::Green));
    fill(ColorCode::new(Color::Black, Color::Black));
    println!("Test Video mode");
    fill(ColorCode::new(Color::Black, Color::Black));
    /*let mut graphic = 0xB8000 as *mut u8;
    unsafe{
        for i in 0..100{
            for ii in 0..100{
                *graphic.offset(320*i + ii as isize) = 15;
            }
        }
    }*/

}

pub fn logo_screen(){
    println!("
----------------------------------
|     -      |-    |  ||   -   - |
|    | |     | -   |         -   |
|   |   |    |  -  |  ||    - -  |
|  |-----|   |   - |  ||   -   - |
| |       |  |    -|  ||  -     -|
----------------------------------
    ");
}

pub fn move_cursor(row: usize, col: usize) {
    const VGA_CMD: u16 = 0x3d4;
    const VGA_DATA: u16 = 0x3d5;
    let cursor_offset = (row * BUFFER_WIDTH + col) as u16;
    let lsb = (cursor_offset & 0xFF) as u8;
    let msb = (cursor_offset >> 8) as u8;

    unsafe {
        outb(VGA_CMD, 0x0f);
        outb(VGA_DATA, lsb);
        outb(VGA_CMD, 0x0e);
        outb(VGA_DATA, msb);
    }
}
