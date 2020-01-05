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

use super::trb::Trb;
use crate::io::dma::Dma;
use crate::errors::*;

pub struct Ring {
    pub link: bool,
    pub trbs: Dma<[Trb; 16]>,
    pub i: usize,
    pub cycle: bool,
}

impl Ring {
    pub fn new(link: bool) -> Result<Ring> {
        Ok(Ring {
            link: link,
            trbs: Dma::zeroed()?,
            i: 0,
            cycle: link,
        })
    }

    pub fn register(&self) -> u64 {
        let base = self.trbs.physical() as *const Trb;
        let addr = unsafe { base.offset(self.i as isize) };
        addr as u64 | self.cycle as u64
    }

    pub fn next(&mut self) -> (&mut Trb, bool) {
        let mut i;
        loop {
            i = self.i;
            self.i += 1;
            if self.i >= self.trbs.len() {
                self.i = 0;

                if self.link {
                    let address = self.trbs.physical();
                    self.trbs[i].link(address, true, self.cycle);
                    self.cycle = !self.cycle;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        (&mut self.trbs[i], self.cycle)
    }
}
