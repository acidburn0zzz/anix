/*
 * Copyright (C) 2016 Redox OS Developers
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

#[repr(packed)]
#[derive(Clone, Copy, Debug, Default)]
pub struct ConfigDescriptor {
    pub length: u8,
    pub kind: u8,
    pub total_length: u16,
    pub interfaces: u8,
    pub configuration_value: u8,
    pub configuration_str: u8,
    pub attributes: u8,
    pub max_power: u8,
}
