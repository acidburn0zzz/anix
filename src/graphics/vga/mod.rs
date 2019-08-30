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

use crate::drivers::*;
use pci::BusDevice;
use alloc::boxed::Box;

struct VgaDevice {}

/// Driver object for the PCI
struct VgaPciDriver;

static S_VGA_PCI_DRIVER: VgaPciDriver = VgaPciDriver;

pub fn init() {
    // 1. Register Driver
    register_driver(&S_VGA_PCI_DRIVER);
}

impl Driver for VgaPciDriver {
    fn name(&self) -> &str {
        "vga"
    }
    fn bus_type(&self) -> &str {
        "pci"
    }
    fn handles(&self, bus_dev: &dyn BusDevice) -> u32
    {
        let classcode = bus_dev.get_attr("class").unwrap_u32();
        // [class] [subclass] [IF] [ver]
        if classcode & 0xFFFF_FF00 == 0x0300_0000 {
            1    // Handle as weakly as possible (vendor-provided drivers bind higher)
        }
        else {
            0
        }
    }
    fn bind(&self, _bus_dev: &mut dyn BusDevice) -> Box<dyn DriverInstance+'static> {
        box VgaDevice::new()
    }

}

impl VgaDevice {
    pub fn new() -> VgaDevice {
        unsafe {
			Self::test();
		}
        Self{}
    }
    pub unsafe fn test() {
		
    }
}

impl DriverInstance for VgaDevice {
}
