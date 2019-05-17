/*Copyright (C) 2018-2019 Nicolas Fouquet 

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see https://www.gnu.org/licenses.
*/
//TODO: Bus_pci
use x86::io::*;
use spin::Mutex;

pub unsafe fn pci_probe(){
	
}

static S_PCI_LOCK: Mutex<PCICfgSpace> = Mutex::new(PCICfgSpace);

struct PCICfgSpace;
impl PCICfgSpace
{
	fn read(&mut self, addr: u32) -> u32 {
		// SAFE: (from accessing the wrong place)
		unsafe {
			outl(0xCF8, 0x80000000 | addr);
			inl(0xCFC)
		}
	}
	fn write(&mut self, addr: u32, val: u32) {
		// SAFE: (from accessing the wrong place)
		unsafe {
			outl(0xCF8, 0x80000000 | addr);
			outl(0xCFC, val)
		}
	}
}

/// Read a word from a pre-calculated PCI address
pub fn read(addr: u32) -> u32
{
	S_PCI_LOCK.lock().read(addr)
}

/// Write a word to a pre-calculated PCI address
pub fn write(addr: u32, val: u32)
{
	S_PCI_LOCK.lock().write(addr, val);
}
