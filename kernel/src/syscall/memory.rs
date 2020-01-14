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
use crate::errors::*;
use bitflags::bitflags;
use crate::memory::{map, paging::EntryFlags};

bitflags! {
    struct MmapFlags: u32 {
        const SHARED = 1 << 0;
        const PRIVATE = 1 << 1;
        const FIXED = 1 << 4;
        const ANONYMOUS = 1 << 5;
    }
}
bitflags! {
    struct MmapProt: u32 {
        const NONE = 0;
        const READ = 1 << 0;
        const WRITE = 1 << 1;
        const EXEC = 1 << 2;
    }
}

impl From<MmapProt> for EntryFlags {
    fn from(flags: MmapProt) -> Self {
        let mut result = EntryFlags::PRESENT;
        if !flags.contains(MmapProt::NONE) {
            result = EntryFlags::empty();
        }
        if flags.contains(MmapProt::READ) {
            result |= EntryFlags::USER_ACCESSIBLE;
        }
        if flags.contains(MmapProt::WRITE) {
            result |= EntryFlags::WRITABLE;
        }
        if !flags.contains(MmapProt::EXEC) {
            result |= EntryFlags::NO_EXECUTE;
        }
        result
    }
}

impl Syscall {
    // INPUT: arg1 -> addr
    //        arg2 -> length
    //        arg3 -> prot
    //        arg4 -> flags
    //        arg5 -> file
    //        arg6 -> offset
    pub unsafe fn sys_mmap(&self, addr: usize, len: usize, prot: usize, flags: usize, fd: usize, offset: usize)
        -> usize {
        use crate::processes::scheduler::SCHEDULER;

        print!("mmap({:#x}, {}, {}, {}, {:#x}, {}) = ", addr, len, prot, flags, fd, offset);

        let prot = MmapProt::from_bits_truncate(prot as u32);
        let flags = MmapFlags::from_bits_truncate(flags as u32);

        if flags.contains(MmapFlags::ANONYMOUS) {
            if addr == 0 {
                SCHEDULER.force_write_unlock();
                let addr =
                    SCHEDULER.try_write().unwrap().get_current_process_mut().heap.try_lock().unwrap().bottom() - 4096;
                map(
                    addr - 4096,
                    addr,
                    EntryFlags::from(prot)
                );
                // Fill memory area with zeros
                // TODO: Create a helper function in the common.rs file
                //       (use it also in src/elf/mod.rs)
                for i in 0..4096 {
                    SCHEDULER.force_write_unlock();
                    let ptr = SCHEDULER.try_write().unwrap().get_current_process_mut().heap.try_lock().unwrap().bottom() as *mut u8;
                    *ptr.offset(i as isize) = 0;
                }
                SCHEDULER.force_write_unlock();
                println!("{:#x}", SCHEDULER.try_write().unwrap().get_current_process_mut().heap.try_lock().unwrap().bottom());
                SCHEDULER.try_write().unwrap().get_current_process_mut().heap.try_lock().unwrap().bottom()
            }
            else {
                Error::mux(Err(Error::new(EINVAL)))
            }
        }
        else {
            Error::mux(Err(Error::new(EINVAL)))
        }
    }
    pub unsafe fn sys_brk(&self, addr: usize) -> usize {
        use crate::processes::scheduler::SCHEDULER;
        print!("brk({:#x}) = ", addr);
        if addr == 0 {
            // Get brk
            SCHEDULER.force_write_unlock();
            println!("{:#x}", SCHEDULER.try_write().unwrap().get_current_process_mut().heap
                .try_lock().unwrap().bottom());
            SCHEDULER.try_write().unwrap().get_current_process_mut().heap.try_lock().unwrap()
                .bottom()
        }
        else {
            // Set brk
            let current_brk = SCHEDULER.try_write().unwrap().get_current_process_mut().heap
                .try_lock().unwrap().bottom();

            SCHEDULER.force_write_unlock();
            SCHEDULER.try_write().unwrap().get_current_process_mut().heap.force_unlock();
            SCHEDULER.force_write_unlock();

            SCHEDULER.try_write().unwrap().get_current_process_mut().heap.try_lock().unwrap()
                .extend(addr - current_brk);

            SCHEDULER.force_write_unlock();
            SCHEDULER.try_write().unwrap().get_current_process_mut().heap.force_unlock();
            SCHEDULER.force_write_unlock();

            println!("{:#x}", SCHEDULER.try_write().unwrap().get_current_process_mut().heap
                .try_lock().unwrap().bottom());
            SCHEDULER.try_write().unwrap().get_current_process_mut().heap.try_lock().unwrap()
                .bottom()
        }
    }
}
