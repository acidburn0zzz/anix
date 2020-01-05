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

use crate::io::mmio::Mmio;

pub struct CapabilityRegs {
    pub len: Mmio<u8>,
    _rsvd: Mmio<u8>,
    pub hci_ver: Mmio<u16>,
    pub hcs_params1: Mmio<u32>,
    pub hcs_params2: Mmio<u32>,
    pub hcs_params3: Mmio<u32>,
    pub hcc_params1: Mmio<u32>,
    pub db_offset: Mmio<u32>,
    pub rts_offset: Mmio<u32>,
    pub hcc_params2: Mmio<u32>,
}

pub struct OperationalRegs {
    pub usb_cmd: Mmio<u32>,
    pub usb_sts: Mmio<u32>,
    pub page_size: Mmio<u32>,
    _rsvd2: [Mmio<u32>; 2],
    pub dn_ctrl: Mmio<u32>,
    pub crcr: Mmio<u64>,
    _rsvd3: [Mmio<u32>; 4],
    pub dcbaap: Mmio<u64>,
    pub config: Mmio<u32>,
}

#[repr(packed)]
pub struct Interrupter {
    pub iman: Mmio<u32>,
    pub imod: Mmio<u32>,
    pub erstsz: Mmio<u32>,
    _rsvd: Mmio<u32>,
    pub erstba: Mmio<u64>,
    pub erdp: Mmio<u64>,
}

#[repr(packed)]
pub struct RuntimeRegs {
    pub mfindex: Mmio<u32>,
    _rsvd: [Mmio<u32>; 7],
    pub ints: [Interrupter; 1024],
}
