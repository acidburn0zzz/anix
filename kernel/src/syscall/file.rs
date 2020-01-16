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
use alloc::prelude::v1::String;

use super::Syscall;

impl Syscall {
    // INPUT: arg1 -> path
    //        arg2 -> len of path
    //        arg3 -> flags
    pub fn sys_open(&self, path: usize, len: usize, flags: usize) -> usize {
        use core::slice::from_raw_parts;
        use crate::fs::ext2::file::File;
        use crate::processes::scheduler::SCHEDULER;

        let path = unsafe { String::from_utf8(from_raw_parts(path.clone() as *const u8, len).to_vec())
            .expect("cannot transform to utf8") };
        unsafe {
            SCHEDULER.force_write_unlock();
        }
        let id = SCHEDULER.try_write().unwrap().get_current_process()
            .expect("the process system is not started").next_file_id();
        let file = File::open(path.clone(), flags);
        unsafe {
            SCHEDULER.force_write_unlock();
        }
        SCHEDULER.try_write().unwrap().get_current_process()
            .expect("the process system is not started").add_new_file(file);
        println!("open({}, {:#x}) = {}", path, flags, id);
        id
    }
    // INPUT: arg1 -> file descriptor num
    //        arg2 -> pointer of buffer
    //        arg3 -> len of buffer
    pub fn sys_read(&self, fd: usize, buf: usize, count: usize) -> usize {
        use core::ptr::copy_nonoverlapping;
        use crate::processes::scheduler::SCHEDULER;
        // TODO: from_raw_slice_mut?
        unsafe {
            SCHEDULER.force_write_unlock();
            SCHEDULER.try_write().unwrap().get_current_process()
            .expect("the process system is not started").fds.force_unlock();
            SCHEDULER.force_write_unlock();
        }
        let src = SCHEDULER.try_write().unwrap().get_current_process()
            .expect("the process system is not started").fds.try_lock().unwrap()[fd].content_ptr;
        unsafe {
            copy_nonoverlapping(src as *const u8,
                buf as *mut u8,
                count);
        }
        println!("read({}, {:#x}, {}) = {}", fd, buf, count, 0);
        0
    }
    // INPUT: arg1 -> file descriptor num
    //        arg2 -> pointer of buffer
    //        arg3 -> len of buffer
    pub fn sys_write(&self, fd: usize, buf: usize, count: usize) -> usize {
        const STDIN_FILENO: usize = 0;
        const STDOUT_FILENO: usize = 1;
        const STDERR_FILENO: usize = 2;

         match fd {
            STDIN_FILENO => {
                use core::slice::from_raw_parts;
                use core::str::from_utf8;

                // Write to Stdin
                unsafe {
                    print!("STDIN: {}", from_utf8(from_raw_parts(buf as *const u8, count))
                        .expect("cannot transform to utf8"));
                }
                count
            },
            STDOUT_FILENO => {
                use core::slice::from_raw_parts;
                use core::str::from_utf8;

                // Write to Stdout
                // TODO: Use strlen() instead of arg3
                unsafe {
                    print!("{}", from_utf8(from_raw_parts(buf as *const u8, count))
                        .expect("cannot transform to utf8"));
                }
                count
            },
            STDERR_FILENO => {
                use core::slice::from_raw_parts;
                use core::str::from_utf8;

                // Write to Error Stdout
                unsafe {
                    println!("ERROR: {}", from_utf8(from_raw_parts(buf as *const u8, count))
                        .expect("cannot transform to utf8"));
                }
                count
            },
            _ => {
                println!("Write files not implemented (fd num: {})", fd);
                0
            },
         }
    }
}
