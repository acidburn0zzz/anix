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

use super::colors::Rgb;
use crate::VBE_BUFFER;

/// Shapes which can be drawn
pub enum Shapes {
    /// A point
    Point {x: u32, y: u32, color: Rgb},

    /// A rectangle
    Rect {x: u32, y: u32, w: u32, h: u32, color: Rgb},
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
                                Shapes::Point {x: width, y: height, color: *color}.draw();
                            }
                        }
                    },
                }
            }
            else {
                println!("Sorry, your computer doesn't support VBE.");
            }
        }
    }
}
