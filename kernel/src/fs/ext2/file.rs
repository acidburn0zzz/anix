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
use alloc::prelude::v1::{String, Vec, ToString, ToOwned};

use super::INODE_ROOT;
use super::{Ext2Info, gd::GDTable, inode::Inode};
use crate::fs::PARTITIONS;
use crate::errors::{Error, ENOENT};

pub struct File {
    content: Option<String>,
    content_binary: Option<Vec<u8>>,
}

impl File {
    pub fn open(path: &'static str, mode: &'static str) -> Self {
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
        return match mode {
            "r" => {
                let content = inode.read_file(info.start).expect("cannot read file");
                Self {
                    content: Some(content),
                    content_binary: None,
                }
            },
            "rb" => {
                let content = inode.read(info.start).expect("cannot read file");
                Self {
                    content: None,
                    content_binary: Some(content),
                }

            }
            _ => panic!("The mode `{}` is not recognized", mode),
        };
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
    pub fn read(&self) -> String {
        if self.content.is_none() == false {
            return self.content.to_owned().unwrap();
        }
        else {
            panic!("You can't read a file open in binary mode");
        }
    }
    pub fn read_binary(&self) -> Vec<u8> {
        // TODO: Choose the lenght of the string read
        if self.content_binary.is_none() == false {
            return self.content_binary.to_owned().unwrap();
        }
        else {
            panic!("You can't read a binary file open in normal read mode");
        }

    }
}
