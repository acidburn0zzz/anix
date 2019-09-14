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

use crate::errors::*;
use crate::io::dma::Dma;
use crate::io::mmio::Mmio;
use crate::io::io::Io;

use super::ring::Ring;
use super::trb::Trb;

#[repr(packed)]
pub struct EventRingSte {
    pub address: Mmio<u64>,
    pub size: Mmio<u16>,
    _rsvd: Mmio<u16>,
    _rsvd2: Mmio<u32>,
}

pub struct EventRing {
    pub ste: Dma<EventRingSte>,
    pub ring: Ring,
}

impl EventRing {
    pub fn new() -> Result<EventRing> {
        let mut ring = EventRing {
            ste: Dma::zeroed()?,
            ring: Ring::new(false)?,
        };

        ring.ste.address.write(ring.ring.trbs.physical() as u64);
        ring.ste.size.write(ring.ring.trbs.len() as u16);

        Ok(ring)
    }

    pub fn next(&mut self) -> &mut Trb {
        self.ring.next().0
    }
}
