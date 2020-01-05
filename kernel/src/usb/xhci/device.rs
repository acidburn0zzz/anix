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
pub struct DeviceDescriptor {
    pub length: u8,
    pub kind: u8,
    pub usb: u16,
    pub class: u8,
    pub sub_class: u8,
    pub protocol: u8,
    pub packet_size: u8,
    pub vendor: u16,
    pub product: u16,
    pub release: u16,
    pub manufacturer_str: u8,
    pub product_str: u8,
    pub serial_str: u8,
    pub configurations: u8,
}
