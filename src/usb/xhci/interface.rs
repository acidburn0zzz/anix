/*
 * Copyright (C) 2016 Redox OS Developers
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

use plain::Plain;

#[repr(packed)]
#[derive(Clone, Copy, Debug, Default)]
pub struct InterfaceDescriptor {
    pub length: u8,
    pub kind: u8,
    pub number: u8,
    pub alternate_setting: u8,
    pub endpoints: u8,
    pub class: u8,
    pub sub_class: u8,
    pub protocol: u8,
    pub interface_str: u8,
}

unsafe impl Plain for InterfaceDescriptor {}
