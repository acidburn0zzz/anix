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

use core::ptr::copy_nonoverlapping;

use super::ext2::superblock::Superblock;

use ::read_num_bytes;

pub struct Partition {
    pub bootable: bool,
    pub system_id: u8,
    pub lba_start: u64,
    pub lba_count: u64,
    pub superblock: Option<Superblock>
}

impl Default for Partition {
    fn default() -> Partition {
        Partition {
            bootable: false,
            system_id: 0,
            lba_start: 0,
            lba_count: 0,
            superblock: None,
        }
    }
}

#[derive(Debug)]
pub enum PartType {
    EMPTY         = 0x00,
    FAT12         = 0x01,
    FAT16         = 0x04,
    NTFS          = 0x07,
    FAT32CHS      = 0x0b,
    FAT32LBA      = 0x0c,
    MINIX         = 0x80,
    LINUX         = 0x83,
    LINUXEXTENDED = 0x85,
    LINUXLVM      = 0x8e,
    ISO9660       = 0x96,
    UNKNOWN       = 0xff,
}

impl From<u8> for PartType {
    fn from(num: u8) -> PartType{
        match num {
            0x00 => PartType::EMPTY,
            0x01 => PartType::FAT12,
            0x04 => PartType::FAT16,
            0x07 => PartType::NTFS,
            0x0b => PartType::FAT32CHS,
            0x0c => PartType::FAT32LBA,
            0x80 => PartType::MINIX,
            0x83 => PartType::LINUX,
            0x85 => PartType::LINUXEXTENDED,
            0x8e => PartType::LINUXLVM,
            0x96 => PartType::ISO9660,
            _    => PartType::UNKNOWN,
        }
    }
}


impl Partition {
    pub fn new(data: &[u8]) -> Option<Partition> {
        assert!(data.len() >= 16);

        if data[4] == 0 {
            return None;
        }

        if data[0] & 0x7E != 0 {
            println!("Partition entry has reserved bits set in byte 0 {:#x}",
                data[0]);
            return None;
        }

        let (base, len) = if data[0] & 1 != 0 {
            println!("non-standard 48-bit LBA");
            (0, 0)
        } else {
            let base = read_num_bytes!(u32, &data[8..]) as u64;
            let len = read_num_bytes!(u32, &data[12..]) as u64;
            (base, len)
        };

        Some(Partition {
            bootable: (data[0] & 0x80) != 0,
            system_id: data[4],
            lba_start: base,
            lba_count: len,
            superblock: None,
        })
    }
    pub fn part_type(&self) -> PartType{
        PartType::from(self.system_id)
    }
}
