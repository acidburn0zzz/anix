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

use disk::sata::read_disk;
use core::mem::size_of;
use alloc::prelude::v1::Vec;

pub struct GDTable(pub Vec<Gd>);

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Gd {
    pub bg_block_bitmap: u32,
    pub bg_inode_bitmap: u32,
    pub bg_inode_table: u32,
    pub bg_free_blocks_count: u16,
    pub bg_free_inodes_count: u16,
    pub bg_used_dirs_count: u16,
    pub bg_pad: u16,
    _bg_reserved: [u32; 3],
}

impl Gd {
    /// Get the group descriptor table by reading the first block after the start of the partition
    pub fn new(offset: u64) -> Self {
        // TODO: For safe, use the plain crate
        unsafe {
            let gd_size = size_of::<GDTable>();
            *(read_disk(offset, offset + gd_size as u64)
            .expect("failed to read disk")
            .as_slice()
            as *const _ as *const Self)
        }
    }
}

impl GDTable {
    pub fn new(gdtable_start: u64, block_size: u32) -> GDTable{
        let mut table = Vec::new();
        for gd in 0..block_size / size_of::<Gd>() as u32 {
            table.push(
                Gd::new(
                    (gdtable_start as u32 + size_of::<Gd>() as u32 * gd as u32) as u64
                )
            );
        }
        GDTable(table)
    }
}
