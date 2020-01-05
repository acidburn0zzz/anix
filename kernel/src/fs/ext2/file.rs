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
use alloc::prelude::v1::{Vec, ToString, ToOwned};

use super::INODE_ROOT;
use crate::fs::PARTITIONS;
use super::{Ext2Info, gd::GDTable, inode::Inode};
use crate::errors::{Error, ENOENT};

pub const O_RDONLY: usize    = 0; // Read only
pub const O_WRONLY: usize    = 1; // Write only
pub const O_RDWR: usize      = 2; // Read and write
pub const O_ACCMODE: usize   = 3; // See the man page of `open`
pub const O_CREAT: usize     = 100; // Create the file
pub const O_EXCL: usize      = 200;
pub const O_NOCTTY: usize    = 400;
pub const O_TRUNC: usize     = 1000;
pub const O_APPEND: usize    = 2000;
pub const O_NONBLOCK: usize  = 4000;
pub const O_DSYNC: usize     = 10000;
pub const O_FASYNC: usize    = 20000;
pub const O_DIRECT: usize    = 40000;
pub const O_LARGEFILE: usize = 100000;
pub const O_DIRECTORY: usize = 200000;
pub const O_NOFOLLOW: usize  = 400000;
pub const O_NOATIME: usize   = 1000000;
pub const O_CLOEXEC: usize   = 2000000;

#[derive(Debug, Copy, Clone)]
pub struct File {
    pub content_ptr: usize, // Adress of the array which contains data
    pub data_len: usize,
}

impl File {
    pub fn open(path: &'static str, flags: usize) -> Self {
        // TODO: Mode parameters to change the file permissions on open
        let part = &PARTITIONS.lock()[0];
        let superblock = part.superblock.unwrap();
        let block_size = 1024 << superblock.data.s_log_block_size;
        let gdt = GDTable::new(part.lba_start * 512 + block_size as u64, block_size);
        let info = Ext2Info {
            start: part.lba_start * 512,
            gdt: &gdt,
            block_size: block_size,
            sb: superblock,
        };

        // Find the inode
        let mut vec: Vec<&str> = path.split("/").collect();
        vec.remove(0);
        let n = Self::path_to_inode(0, info, vec, INODE_ROOT).expect("cannot find file");

        // Read the inode
        let inode = Inode::new(info, n);
        let result;
        match flags {
            O_CLOEXEC | O_RDONLY => {
                let content = inode.read(info.start).expect("cannot read file");
                result = Self {
                    content_ptr: content.as_ptr() as usize,
                    data_len: content.len(),
                };
                core::mem::forget(content); // Don't remove the content
            },
            _ => panic!("The flag `{}` is not recognized", flags),
        };
        result
    }
    // TODO: Ext2Info struct in &self or in Inode struct

    /// Convert a path to an inode number
    fn path_to_inode(n_recurs: u32, info: Ext2Info, path: Vec<&str>, i_num: u32)
        -> Result<u32, Error> {
        let inode = Inode::new(info, i_num);
        let dirs = inode.get_dir_entries(info.start)
                   .expect("cannot get dir entries");
        for dir in dirs {
            if dir.name == path[n_recurs as usize] {
                if path.last().unwrap().to_string() == dir.name {
                    return Ok(dir.inode);
                }
                else {
                    let f = Self::path_to_inode(n_recurs + 1, info, path.to_owned(), dir.inode);
                    if let Ok(y) = f {
                        return Ok(y);
                    }
                }
            }
        }
        Err(Error::new(ENOENT))
    }
    pub fn read(&self) -> &'static [u8] {
        // TODO: return pointer and copy the line in irq/syscalls.rs
        use core::slice::from_raw_parts;
        unsafe {
            return from_raw_parts(self.content_ptr as *const u8, self.data_len);
        }
    }
    // TODO: Write + close (call sync)
}
