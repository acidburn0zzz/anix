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
use alloc::string::FromUtf8Error;

use crate::errors::Result;

/// A filesystem (only Ext2 is supported for now)
pub trait Filesystem: Send {
    // Get the name of the filesystem
    fn get_name(&self) -> String;

    // Open a file in this filesystem
    fn open(&mut self, _path: String, _flags: usize) -> FileDesc {
        unimplemented!("It is not possible to open a file with this filesystem.");
    }

    // Read a file in this filesystem
    fn read(&self, _fd: usize, _count: usize) -> Result<&[u8]> {
        unimplemented!("It is not possible to read a file with this filesystem.");
    }

    // Read a file in this filesystem and cast it to a string
    fn read_to_string(&self, _fd: usize, _count: usize) -> core::result::Result<String, FromUtf8Error> {
        unimplemented!("It is not possible to read a file to a string with this filesystem.");
    }

    // Write a file in this filesystem
    fn write(&self) -> Result<usize> {
        unimplemented!("It is not possible to write a file with this filesystem.");
    }

    // Get the superblock if the filesystem support it
    fn get_superblock(&self) -> super::ext2::superblock::Superblock {
        unimplemented!("This filesystem does not have a superblock.");
    }
}


pub struct FileDesc {
    pub num: usize
}
