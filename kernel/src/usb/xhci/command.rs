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

use super::ring::Ring;
use super::trb::Trb;
use super::event::EventRing;
use crate::errors::*;

pub struct CommandRing {
    pub ring: Ring,
    pub events: EventRing,
}

impl CommandRing {
    pub fn new() -> Result<CommandRing> {
        Ok(CommandRing {
            ring: Ring::new(true)?,
            events: EventRing::new()?,
        })
    }

    pub fn crcr(&self) -> u64 {
        self.ring.register()
    }

    pub fn erdp(&self) -> u64 {
        self.events.ring.register()
    }

    pub fn erstba(&self) -> u64 {
        self.events.ste.physical() as u64
    }

    pub fn next(&mut self) -> (&mut Trb, bool, &mut Trb) {
        let cmd = self.ring.next();
        let event = self.events.next();
        (cmd.0, cmd.1, event)
    }

    pub fn next_cmd(&mut self) -> (&mut Trb, bool) {
        self.ring.next()
    }

    pub fn next_event(&mut self) -> &mut Trb {
        self.events.next()
    }
}
