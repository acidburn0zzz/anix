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
use alloc::prelude::v1::{Vec, Box};

use crate::disk::sata::read_disk;
use super::partitions::{Partition, PartType};
use super::ext2::{Ext2Filesystem, superblock::Superblock};

pub struct Mbr {}

impl Mbr {
    pub fn new() -> Self {
        Self {}
    }
    pub fn get_partitions(&self, disk: usize) -> Vec<Partition> {
        let mbr_partitions_addr: [u64; 4] = [0x1be, 0x1ce, 0x1de, 0x1ee];
        let mut partitions = Vec::new();
        for part in 0..4 {
            let partition = Partition::new(
                disk,
                &read_disk(mbr_partitions_addr[part], mbr_partitions_addr[part] + 0x10 as u64)
                .expect("failed to read disk")
            );
            if !partition.is_none() {
                let mut partition = partition.unwrap();

                match partition.part_type() {
                    PartType::EMPTY => {},
                    PartType::LINUX => {
                        // TODO: test magic number
                        partition.set_fs(Box::new(
                            Ext2Filesystem::new(
                                Superblock::new(
                                    partition.lba_start * 512
                                )
                            )
                        ));
                    },
                    _ => (),
                }
                partitions.push(partition);
            }
        }
        partitions
    }
}
