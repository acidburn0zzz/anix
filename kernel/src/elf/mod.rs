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
use core::ptr::copy_nonoverlapping;
use goblin::elf::*;

use crate::fs::ext2::file::*;
use crate::memory::{map, paging::EntryFlags};
use crate::processes::Process;
use crate::memory::consts::USER_OFFSET;

pub fn init() {
    // This Loader can load any static linked binary
    load_elf("/bin/rust-test");
}

// TODO: Return a Result<T, E> value
pub fn load_elf(path: &'static str) {
    let f = File::open(path, O_RDONLY);
    let content = f.read();
    match Elf::parse(&content) {
        Ok(binary) => {
            let entry = binary.entry;

            for ph in binary.program_headers {
                if ph.p_type == program_header::PT_LOAD {
                    let start = ph.p_vaddr as u64;
                    let end = (ph.p_vaddr + ph.p_memsz) as u64;

                    // TODO: To prevent warnings, use (this)[https://docs.rs/goblin/0.0.24/goblin/elf/section_header/constant.SHF_ALLOC.html]
                    // and (this)[https://wiki.osdev.org/ELF_Tutorial#The_BSS_and_SHT_NOBITS]
                    if start < USER_OFFSET.start as u64 {
                        println!("WARNING: The start of the program is under the user offset ({:#x} < {:#x})",
                            start,
                            USER_OFFSET.start);
                        ()
                    }

                    if end > USER_OFFSET.end as u64 {
                        println!("WARNING: The end of the program is superior as the user offset ({:#x} < {:#x})",
                            start,
                            USER_OFFSET.start);
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
            use alloc::prelude::v1::{String, Box};
            println!("Program {} loaded. Entrypoint at {:#x}", path, entry);
            Process::new(
                String::from("test"),
                entry,
                Box::new(&[path.as_bytes()])
            ); // TODO: Get the name of the program
        },
        Err(e) => {
            println!("[ELF ERROR]: {:?}", e);
            ()
        }
    }
    drop(f);
    drop(content);
}
