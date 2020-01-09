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
use lazy_static::lazy_static;
use spin::Mutex;

use crate::VESA_BUFFER;
use super::{FB_WIDTH, FB_HEIGHT};
use super::colors::Rgb;

lazy_static! {
    pub static ref VESA_WRITER: Mutex<VesaTextPrinter> = Mutex::new(VesaTextPrinter::new(
        Rgb::new(255, 255, 255)
    ));
}

// must not be 0 so that we don't have a .bss section
pub static X_POS: AtomicUsize = AtomicUsize::new(1);
pub static Y_POS: AtomicUsize = AtomicUsize::new(1);

pub struct VesaTextPrinter {
    buffer: u32,
    color: Rgb,
}

impl VesaTextPrinter {
    fn new(color: Rgb) -> Self {
        unsafe {
            let buf = *VESA_BUFFER.lock();
            Self {
                buffer: buf,
                color,
            }
        }
    }
    pub fn clear_screen(&mut self) {
        for i in 0..(FB_WIDTH * FB_HEIGHT) {
            unsafe {
                (self.buffer as *mut u32)
                    .offset(i as isize)
                    .write_volatile(0);
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
                            (self.buffer as *mut u32)
                                .offset(idx as isize)
                                .write_volatile(self.color.to_u32());
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
    pub fn change_color(&mut self, new_color: Rgb) {
        self.color = new_color;
    }
    pub fn get_color(&mut self) -> Rgb {
        self.color
    }
    pub fn move_cursor(&self, x: usize, y: usize) {
        X_POS.store(x, Ordering::SeqCst);
        Y_POS.store(y, Ordering::SeqCst);
    }
}

impl Write for VesaTextPrinter {
    fn write_str(&mut self, s: &str) -> Result {
        for c in s.chars() {
            self.write_char(c);
        }

        Ok(())
    }
}
