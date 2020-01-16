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
use alloc::collections::BTreeMap;
use alloc::string::FromUtf8Error;

use crate::fs::vfs::{Filesystem, FileDesc};
use self::gd::*;
use self::file::File;
use self::superblock::Superblock;
use crate::errors::{Result/*, Error*/};

pub mod superblock;
pub mod gd;
pub mod inode;
pub mod file;

pub const INODE_ROOT: u32 = 2;

pub struct Ext2Filesystem {
    tmp_files: BTreeMap<usize, File>, // Files used when the process system is not started (use it also instead of the field fds in the Process struct?)
    next_id: usize,
    sb: Superblock,
}

impl Ext2Filesystem {
    pub fn new(sb: Superblock) -> Self {
        Self {
            tmp_files: BTreeMap::new(),
            next_id: 3,
            sb,
        }
    }
}

impl Filesystem for Ext2Filesystem {
    fn get_name(&self) -> String {
        String::from("ext2")
    }
    fn open(&mut self, path: String, flags: usize) -> FileDesc {
        use crate::processes::scheduler::SCHEDULER;

        unsafe {
            SCHEDULER.force_write_unlock();
        }

        if let Err(_e) = SCHEDULER.try_write().unwrap().get_current_process() {
            // The process system is not started yet
            self.tmp_files.insert(self.next_id, File::open(path, flags));
            self.next_id += 1;
            FileDesc {
                num: self.next_id - 1,
            }
        }
        else {
            unsafe {
                SCHEDULER.force_write_unlock();
            }
            let id = SCHEDULER.try_write().unwrap().get_current_process()
                .expect("the process system is not started").next_file_id();
            let file = File::open(path, flags);
            unsafe {
                SCHEDULER.force_write_unlock();
            }
            SCHEDULER.try_write().unwrap().get_current_process()
                .expect("the process system is not started").add_new_file(file);

            FileDesc {
                num: id,
            }
        }
    }
    fn read(&self, fd: usize, count: usize) -> Result<&[u8]> {
        use core::ptr::copy_nonoverlapping;

        use crate::processes::scheduler::SCHEDULER;

        if let Err(_e) = SCHEDULER.try_write().unwrap().get_current_process() {
            // The process system is not started yet
            Ok(self.tmp_files.get(&fd).unwrap().read())
        }
        else {
            let mut buf = &[];
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
                    &mut buf as *mut _ as *mut u8,
                    count);
            }
            Ok(buf)
        }
    }
    fn read_to_string(&self, fd: usize, count: usize) -> core::result::Result<String, FromUtf8Error> {
        let buf = self.read(fd, count);
        let string = String::from_utf8(buf.expect("cannot read file").to_vec())?;
        Ok(string)
    }
    fn get_superblock(&self) -> Superblock {
        self.sb
    }
}

pub enum InodeMode {
    Ext2SIfmt   = 0xF000, /* format mask  */
    Ext2SIfsock = 0xC000, /* socket */
    Ext2SIflnk  = 0xA000, /* symbolic link */
    Ext2SIfreg  = 0x8000, /* regular file */
    Ext2SIfblk  = 0x6000, /* block device */
    Ext2SIfdir  = 0x4000, /* directory */
    Ext2SIfchr  = 0x2000, /* character device */
    Ext2SIfifo  = 0x1000, /* fifo */
}

pub enum InodeRight {
    Ext2SIsuid = 0x0800, /* SUID */
    Ext2SIsgid = 0x0400, /* SGID */
    Ext2SIsvtx = 0x0200, /* sticky bit */
    Ext2SIrwxu = 0x01C0, /* user access rights mask */
    Ext2SIrusr = 0x0100, /* read */
    Ext2SIwusr = 0x0080, /* write */
    Ext2SIxusr = 0x0040, /* execute */
    Ext2SIrwxg = 0x0038, /* group access rights mask */
    Ext2SIrgrp = 0x0020, /* read */
    Ext2SIwgrp = 0x0010, /* write */
    Ext2SIxgrp = 0x0008, /* execute */
    Ext2SIrwxo = 0x0007, /* others access rights mask */
    Ext2SIroth = 0x0004, /* read */
    Ext2SIwoth = 0x0002, /* write */
    Ext2SIxoth = 0x0001, /* execute */
}

#[derive(Copy, Clone)]
pub struct Ext2Info<'a> {
    pub start: u64,
    pub gdt: &'a GDTable,
    pub block_size: u32,
    pub sb: Superblock,
}

pub fn init() {
    use self::file::*;
    use core::str::from_utf8;
    let f = File::open(String::from("/home/user/hello.txt"), O_RDONLY);
    let c = f.read();
    println!("Content of file /home/user/hello.txt:\n{}", from_utf8(c).expect("cannot transform file /home/user/hello.txt to utf-8"));

    // let f = File::open("/usr/share/system/logo.bmp", "rb");
    // let c = f.read_binary();
    // This file is huge, so we print only the 200 first characters
    // The three first characters are `B` -> 66, `M` -> 77
    // // So, in ascii, the first three characters are [66, 77]
    // println!("Content: {:?}", &c[0..200]);
}
