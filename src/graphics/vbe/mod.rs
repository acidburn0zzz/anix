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

pub mod colors;
pub mod geom;
use self::colors::*;
use self::geom::Shapes;

pub static FB_HEIGHT: u32 = 768;
pub static FB_WIDTH: u32 = 1024;

pub fn init() {
    println!("Vbe driver is starting...");
    Shapes::Rect {
        x: 10,
        y: 10,
        w: 100,
        h: 100,
        color: Rgb::new(0xff, 0xff, 0xff)
    }.draw();
}
