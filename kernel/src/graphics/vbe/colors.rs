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

#[derive(Copy, Clone)]
pub struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl Rgb {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red: red,
            green: green,
            blue: blue,
        }
    }
    pub fn to_u32(&self) -> u32 {
        (
            ((self.blue as u32)  << 0u32) +
            ((self.green as u32) << 8u32) +
            ((self.red as u32)   << 16u32)
        ) as u32
    }
    pub fn red(&self)   -> u8 {self.red}
    pub fn green(&self) -> u8 {self.green}
    pub fn blue(&self)  -> u8 {self.blue}
}
