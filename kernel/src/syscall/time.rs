/*
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
use super::Syscall;

impl Syscall {
    // INPUT: arg1 -> buffer as DateTime
    //        arg2 -> len
    pub fn sys_time(&self, buf: usize, len: usize) -> usize {
        use core::slice::from_raw_parts_mut;

        use crate::device::rtc::Rtc;
        use crate::time::DateTime;

        let pointer = unsafe { from_raw_parts_mut(buf as *mut DateTime, len) };
        pointer[0] = Rtc::new().date();
        println!("time({:#x}) = 0", buf);
        0
    }
}
