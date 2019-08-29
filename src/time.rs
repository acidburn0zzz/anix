/*
Copyright (C) 2018-2019 Nicolas Fouquet

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see https://www.gnu.org/licenses.
*/
use x86::time::rdtsc;

pub struct DateTime {
    pub date: Date,
    pub time: Time,
}

pub struct Date {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

pub struct Time {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

impl DateTime {
    pub fn new() -> Self {
        unsafe {
			println!("Timestamp: {}", rdtsc());
            Self::from_timestamp(rdtsc())
        }
    }
    pub fn from_timestamp(timestamp: u64) -> Self{
        Self {
            date: Date {
                year: 2019,
                month: 8,
                day: (timestamp / 86400) as u32
            },
            time: Time {
                hour: (timestamp % 86400 / 3600) as u32,
                minute: (timestamp % 86400 % 3600 / 60) as u32,
                second: (timestamp % 86400 % 3600 % 60) as u32,
            }
        }
    }
}

impl core::fmt::Display for DateTime {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result{
        write!(f, "{}-{}-{} {}:{}:{}", self.date.year, self.date.month, self.date.day, self.time.hour, self.time.minute, self.time.second)
    }
}

//TODO: Use RTC, PIT, CMOS, ...
