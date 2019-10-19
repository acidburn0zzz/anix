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
use fs::ext2::file::*;
use memory::{map, paging::EntryFlags};
use task::Task;
use goblin::elf::*;
use core::ptr::copy_nonoverlapping;

// TODO: Move these consts in a consts.rs file
pub const USER_OFFSET: u64 = 0x40000000;
pub const USER_STACK: u64 = 0xE0000000;

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

                    if start < USER_OFFSET {
                        ()
                    }

                    if end > USER_STACK {
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
