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

impl Syscall {
    // INPUT: arg1 -> string
    //        arg2 -> len of string
    pub fn sys_debug(&self, string_addr: usize, len: usize) -> usize {
        use core::slice::from_raw_parts;
        use core::str::from_utf8;

        unsafe {
            print!("{}", from_utf8(from_raw_parts(string_addr as *const u8, len))
                .expect("cannot transform to utf8"));
        }
        0
    }
    // INPUT: arg1 -> option
    //        arg2 -> addr
    pub fn sys_archprctl(&self, option: usize, addr: usize) -> usize {
        use x86_64::{
            instructions::segmentation::{
                load_fs,
                load_gs
            },
            PrivilegeLevel::Ring3,
            structures::gdt::SegmentSelector
        };
        use x86::msr;

        use crate::memory::consts::USER_OFFSET;
        pub const ARCH_SET_GS: usize = 0x1001;
        pub const ARCH_SET_FS: usize = 0x1002;
        pub const _ARCH_GET_FS: usize = 0x1003;
        pub const ARCH_GET_GS: usize = 0x1004;

        if addr > USER_OFFSET.end {
            return Error::mux(Err(Error::new(EPERM))) as usize;
        }
        else if addr < ARCH_SET_FS || option > ARCH_GET_GS {
            return Error::mux(Err(Error::new(EINVAL))) as usize;
        }
        // TODO: EFAULT if addr points to an unmapped address or is outside the process address space.
        else {
            return match option {
                ARCH_SET_GS => {
                    /*
                     * ARCH_SET_GS has always overwritten the index
                     * and the base. Zero is the most sensible value
                     * to put in the index, and is the only value that
                     * makes any sense if FSGSBASE is unavailable.
                     */
                    unsafe {
                        load_fs(SegmentSelector::new(0, Ring3));
                        msr::wrmsr(msr::IA32_GS_BASE, addr as u64);
                    }
                    0
                },
                ARCH_SET_FS => {
                    /*
                     * ARCH_SET_FS has always overwritten the index
                     * and the base. Zero is the most sensible value
                     * to put in the index, and is the only value that
                     * makes any sense if FSGSBASE is unavailable.
                     */
                    unsafe {
                        load_gs(SegmentSelector::new(0, Ring3));
                        msr::wrmsr(msr::IA32_FS_BASE, addr as u64);
                     }
                    0
                },
                _ => {
                    println!("sys_arch_prctl unknown option");
                    Error::mux(Err(Error::new(EINVAL)))
                }
            };
        }
    }
}
