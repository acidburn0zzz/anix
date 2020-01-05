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

use bitflags::bitflags;

use crate::io::io::Io;
use crate::io::mmio::Mmio;

pub static HCSPARAMS_N_PORTS_MASK: u32 = (15 << 0);
pub static PORT_ENABLE: u32 = (1 << 2);

/// USB Speeds
#[allow(dead_code)]
enum Speed {
    FULL = 0,
    HIGH = 1,
    LOW = 2,
}

bitflags! {
    pub struct PortFlags: u32 {
        const PORT_CCS = 1 << 0;
        const PORT_PED = 1 << 1;
        const PORT_OCA = 1 << 3;
        const PORT_PR =  1 << 4;
        const PORT_PP =  1 << 9;
        const PORT_PIC_AMB = 1 << 14;
        const PORT_PIC_GRN = 1 << 15;
        const PORT_LWS = 1 << 16;
        const PORT_CSC = 1 << 17;
        const PORT_PEC = 1 << 18;
        const PORT_WRC = 1 << 19;
        const PORT_OCC = 1 << 20;
        const PORT_PRC = 1 << 21;
        const PORT_PLC = 1 << 22;
        const PORT_CEC = 1 << 23;
        const PORT_CAS = 1 << 24;
        const PORT_WCE = 1 << 25;
        const PORT_WDE = 1 << 26;
        const PORT_WOE = 1 << 27;
        const PORT_DR =  1 << 30;
        const PORT_WPR = 1 << 31;
    }
}

#[repr(packed)]
/// An USB port
pub struct Port {
    pub portsc : Mmio<u32>,
    pub portpmsc : Mmio<u32>,
    pub portli : Mmio<u32>,
    pub porthlpmc : Mmio<u32>,
}

impl Port {
    pub fn read(&self) -> u32 {
        self.portsc.read()
    }

    pub fn state(&self) -> u32 {
        (self.read() & (0b1111 << 5)) >> 5
    }

    pub fn speed(&self) -> u32 {
        (self.read() & (0b1111 << 10)) >> 10
    }

    pub fn flags(&self) -> PortFlags {
        PortFlags::from_bits_truncate(self.read())
    }
}
