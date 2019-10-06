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
use device::rtc::Rtc;
use core::fmt::*;

#[derive(Default, Copy, Clone)]
pub struct DateTime {
    pub date: Date,
    pub time: Time,
}

#[derive(Default, Copy, Clone)]
pub struct Date {
    pub century: u32,
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

#[derive(Default, Copy, Clone)]
pub struct Time {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

impl DateTime {
    pub fn new() -> Self {
        let mut rtc = Rtc::new();
        rtc.date()
    }
    pub fn to_timestamp(&self) -> u64 {
        let mut year = self.date.year;
        year += self.date.century * 100;

        // Unix time from clock
        let mut secs: u64 = (year as u64 - 1970) * 31_536_000;

        let mut leap_days = (year as u64 - 1972) / 4 + 1;
        if year % 4 == 0 && self.date.month <= 2 {
            leap_days -= 1;
        }
        secs += leap_days * 86_400;

        match self.date.month {
            2 => secs += 2_678_400,
            3 => secs += 5_097_600,
            4 => secs += 7_776_000,
            5 => secs += 10_368_000,
            6 => secs += 13_046_400,
            7 => secs += 15_638_400,
            8 => secs += 18_316_800,
            9 => secs += 20_995_200,
            10 => secs += 23_587_200,
            11 => secs += 26_265_600,
            12 => secs += 28_857_600,
            _ => (),
        }

        secs += (self.date.day as u64 - 1) * 86_400;
        secs += self.time.hour as u64 * 3600;
        secs += self.time.minute as u64 * 60;
        secs += self.time.second as u64;
        secs
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result{
        write!(f, "{:02}{:02}/{:02}/{:02} {:02}:{:02}:{:02} GMT",
            self.date.century,
            self.date.year,
            self.date.month,
            self.date.day,
            self.time.hour,
            self.time.minute,
            self.time.second)
    }
}
