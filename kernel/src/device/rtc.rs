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
use io::{io::Io, pio::Pio};
use time::*;

fn cvt_bcd(value: usize) -> usize {
    (value & 0xF) + ((value / 16) * 10)
}

/// RTC
pub struct Rtc {
    addr: Pio<u8>,
    data: Pio<u8>,
}

impl Rtc {
    /// Create new empty RTC
    pub fn new() -> Self {
        Rtc {
            addr: Pio::<u8>::new(0x70),
            data: Pio::<u8>::new(0x71),
        }
    }

    /// Read
    unsafe fn read(&mut self, reg: u8) -> u8 {
        self.addr.write(reg);
        self.data.read()
    }

    /// Wait
    unsafe fn wait(&mut self) {
        while self.read(0xA) & 0x80 != 0x80 {}
        while self.read(0xA) & 0x80 == 0x80 {}
    }

    pub fn date(&mut self) -> DateTime {
        let second;
        let minute;
        let hour;
        let day;
        let month;
        let year;
        let century;

        unsafe {
            self.wait();
            second = self.read(0) as usize;
            minute = self.read(2) as usize;
            hour = self.read(4) as usize;
            day = self.read(7) as usize;
            month = self.read(8) as usize;
            year = self.read(9) as usize;
            century = self.read(0x32) as usize;
        }

        DateTime {
            date: Date {
                century: cvt_bcd(century) as u32,
                year: cvt_bcd(year) as u32,
                month: cvt_bcd(month) as u32,
                day: cvt_bcd(day) as u32,
            },
            time: Time {
                hour: cvt_bcd(hour) as u32,
                minute: cvt_bcd(minute) as u32,
                second: cvt_bcd(second) as u32,
            },
        }
    }
}
