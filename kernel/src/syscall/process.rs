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
    // INPUT: arg1 -> error code
    pub fn sys_exit(&self, code: usize) -> usize {
        // TODO: Kill or stop the processes
        use crate::processes::scheduler::{switch, kill};
        println!("exit({}) = 0", code);
        kill();
        switch();
        0
    }

    // INPUT: arg1 -> Thread ID
    //        arg2 -> Signal
    pub fn sys_tkill(&self, tid: usize, sig: usize) -> usize {
        use crate::processes::scheduler::{kill, switch};
        println!("tkill({}, {}) = 0", tid, sig);
        kill();
        switch();
        0
    }
    // INPUT: arg1 -> tidptr
    pub fn sys_set_tid_addr(&self, tidptr: usize) -> usize {
        use crate::processes::scheduler::SCHEDULER;
        unsafe {
            SCHEDULER.force_write_unlock();
        }
        let value = SCHEDULER.try_read().unwrap().get_current_process().getpid();
        println!("set_tid_addr({:#x}) = {}", tidptr, value);
        value as usize
    }
    // INPUT: arg1 -> signum
    //        arg2 -> action
    //        arg3 -> old action
    pub fn sys_sigaction(&self, signum: usize, action: usize, oldaction: usize) -> usize {
        println!("rt_sigaction({:#x}, {:#x}, {:#x}) = 0", signum, action, oldaction);
        0
    }

    // INPUT: arg1 -> how
    //        arg2 -> set
    //        arg3 -> old set
    pub fn sys_sigprocmask(&self, how: usize, set: usize, oldset: usize) -> usize {
        println!("rt_sigprocmask({:#x}, {:#x}, {:#x}) = 0", how, set, oldset);
        0
    }
    // INPUT: arg1 -> new ss (used to change ss)
    //        arg2 -> old ss (used to get ss)
    pub fn sys_sigalstack(&self, newss: usize, oldss: usize) -> usize {
        use crate::memory::consts::USER_OFFSET;

        #[derive(Debug)]
        struct SigaltStack {
            ss_sp: usize,
            ss_flags: u32,
            ss_size: usize,
        }
        println!("sigalstack({:#x}, {:#x}) = 0", newss, oldss);

        if newss != 0 {
            // Set SS
        }
        if oldss != 0 {
            // Get SS
            unsafe {
                *(oldss as *mut SigaltStack) = SigaltStack {
                    ss_sp: USER_OFFSET.start,
                    ss_flags: 2,
                    ss_size: 1024,
                };
            }
        }
        0
    }
}
