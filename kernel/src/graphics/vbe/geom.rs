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

use core::fmt::{Result, Write};
use core::sync::atomic::{AtomicUsize, Ordering};
use alloc::prelude::v1::String;

use super::colors::Rgb;
use crate::VBE_BUFFER;
use super::{FB_WIDTH, FB_HEIGHT};

/// Shapes which can be drawn
pub enum Shapes {
    /// A point
    Point {x: u32, y: u32, color: Rgb},

    /// A rectangle
    Rect {x: u32, y: u32, w: u32, h: u32, color: Rgb},

    /// A text
    Text {x: u32, y: u32, text: String, color: Rgb},
}

impl Shapes {
    pub fn draw(&self) {
        unsafe {
            if *VBE_BUFFER.lock() != 0 {
                let buf = *VBE_BUFFER.lock() as *mut u32;
                match self {
                    Shapes::Point {x, y, color} => {
                        // TODO: Use a double-buffering with a task to update the screen
                        *buf.offset((y * 1024 + x) as isize) = color.to_u32()
                    },
                    Shapes::Rect {x, y, w, h, color} => {
                        for height in *y..(*h + *y) {
                            for width in *x..(*w + *x) {
                                Shapes::Point {x: width, y: height, color: *color}
                                    .draw();
                            }
                        }
                    },
                    Shapes::Text {x, y, text, color} => {
                        Printer::new(*x as usize, *y as usize, *color)
                            .write_fmt(format_args!("{}", text))
                            .expect("cannot write on the screen");
                    },
                }
            }
            else {
                println!("Sorry, your computer doesn't support VBE.");
            }
        }
    }
}

// must not be 0 so that we don't have a .bss section
pub static X_POS: AtomicUsize = AtomicUsize::new(1);
pub static Y_POS: AtomicUsize = AtomicUsize::new(1);

pub struct Printer {
    buffer: *mut u32,
    color: Rgb,
}

impl Printer {
    fn new(x: usize, y: usize, color: Rgb) -> Self {
        X_POS.store(x, Ordering::SeqCst);
        Y_POS.store(y, Ordering::SeqCst);
        unsafe {
            let buf = *VBE_BUFFER.lock() as *mut u32;
            Self {
                buffer: buf,
                color,
            }
        }
    }
    pub fn clear_screen(&mut self) {
        for i in 0..(FB_WIDTH * FB_HEIGHT) {
            unsafe {
                self.buffer.offset(i as isize).write_volatile(0);
            }
        }

        X_POS.store(0, Ordering::SeqCst);
        Y_POS.store(0, Ordering::SeqCst);
    }

    fn newline(&mut self) {
        let y_pos = Y_POS.fetch_add(8, Ordering::SeqCst);
        X_POS.store(0, Ordering::SeqCst);
        if y_pos >= FB_HEIGHT as usize {
            self.clear_screen();
        }
    }

    fn write_char(&mut self, c: char) {
        use font8x8::UnicodeFonts;

        if c == '\n' {
            self.newline();
            return;
        }

        let x_pos = X_POS.fetch_add(8, Ordering::SeqCst);
        let y_pos = Y_POS.load(Ordering::SeqCst);

        match c {
            ' '..='~' => {
                let rendered = font8x8::BASIC_FONTS
                    .get(c)
                    .expect("character not found in basic font");
                for (y, byte) in rendered.iter().enumerate() {
                    for (x, bit) in (0..8).enumerate() {
                        if *byte & (1 << bit) == 0 {
                            continue;
                        }
                        let idx = (y_pos + y) * FB_WIDTH as usize + x_pos + x;
                        unsafe {
                            self.buffer.offset(idx as isize).write_volatile(self.color.to_u32());
                        }
                    }
                }
            }
            _ => panic!("unprintable character"),
        }

        if x_pos + 8 >= FB_WIDTH as usize {
            self.newline();
        }
    }
}

impl Write for Printer {
    fn write_str(&mut self, s: &str) -> Result {
        for c in s.chars() {
            self.write_char(c);
        }

        Ok(())
    }
}
