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
use goblin::elf::*;
use core::ptr::copy_nonoverlapping;

use crate::memory::consts::USER_OFFSET;
use crate::memory::{map, paging::EntryFlags};
use crate::task::Task;
use crate::fs::ext2::file::*;

pub fn init() {
    load_elf("/bin/rust-test");
}

// TODO: Return a Result<T, E> value
pub fn load_elf(path: &'static str) {
    let f = File::open(path, "rb");
    let content = f.read_binary();
    match Elf::parse(&content) {
        Ok(binary) => {
            let entry = binary.entry;

            for ph in binary.program_headers {
                if ph.p_type == program_header::PT_LOAD {
                    let start = ph.p_vaddr as u64;
                    let end = (ph.p_vaddr + ph.p_memsz) as u64;

                    if start < USER_OFFSET.start as u64 {
                        ()
                    }

                    if end > USER_OFFSET.end as u64 {
                        ()
                    }
                    unsafe {
                        map(start as usize,
                            end as usize,
                            EntryFlags::PRESENT | EntryFlags::WRITABLE | EntryFlags::USER_ACCESSIBLE);
                        copy_nonoverlapping((content.as_ptr() as u64 + ph.p_offset) as *const u8,
                                            start as *mut u8,
                                            ph.p_filesz as usize);
                    }

                    if ph.p_memsz > ph.p_filesz {
                        for i in ph.p_filesz..ph.p_memsz {
                            unsafe {
                                let ptr = ph.p_vaddr as *mut u8;
                                *ptr.offset(i as isize) = 0;
                            }
                        }
                    }
                }
            }
            unsafe {
                Task::new("test", entry);
            }
        },
        Err(_) => ()
    }
    drop(f);
    drop(content);
}
