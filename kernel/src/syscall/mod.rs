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
pub mod number;
pub mod call;
mod file;
mod misc;
mod process;
mod time;
mod memory;

use crate::errors::*;
use self::number::*;

pub struct Syscall {}

impl Syscall {
    pub fn new() -> Self { Self {} }
    pub fn syscall(self, num: usize, args: [usize; 6]) -> usize {
        match num {
            // File
            SYS_OPEN => self.sys_open(args[0], args[1], args[2]),
            SYS_READ => self.sys_read(args[0], args[1], args[2]),
            SYS_WRITE => self.sys_write(args[0], args[1], args[2]),

            // Misc
            SYS_DEBUG => self.sys_debug(args[0], args[1]),
            SYS_ARCHPRCTL => self.sys_archprctl(args[0], args[1]),

            // Process
            SYS_EXIT => self.sys_exit(args[0]),
            SYS_TKILL => self.sys_tkill(args[0], args[1]),
            SYS_SET_TID_ADDR => self.sys_set_tid_addr(args[0]),
            SYS_SIGACTION => self.sys_sigaction(args[0], args[1], args[2]),
            SYS_SIGPROCMASK => self.sys_sigprocmask(args[0], args[1], args[2]),
            SYS_SIGALSTACK => self.sys_sigalstack(args[0], args[1]),

            // Time
            SYS_TIME => self.sys_time(args[0], args[1]),

            // Memory
            SYS_MMAP => unsafe { self.sys_mmap(args[0], args[1], args[2], args[3], args[4], args[5]) },
            SYS_BRK => unsafe { self.sys_brk(args[0]) },
            _ => Error::mux(Err(Error::new(ENOSYS))),
        }
    }
}
