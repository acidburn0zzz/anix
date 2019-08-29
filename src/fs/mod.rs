use crate::disk::sata::read_disk;
use self::partitions::Partition;
use lazy_static::lazy_static;
use spin::Mutex;
use alloc::prelude::v1::{Vec, String};
use self::partitions::PartType;
use self::ext2::superblock::Superblock;
use core::str::from_utf8;

pub mod ext2;
pub mod partitions;

lazy_static! {
    pub static ref PARTITIONS: Mutex<Vec<Partition>> = Mutex::new(Vec::new());
}

pub fn init() {
    // TODO: Initramfs for choose partition and disk

    let mbr_partitions_addr: [u64; 4] = [0x1be, 0x1ce, 0x1de, 0x1ee];
    println!("\n| Name | Bootable | Type | Start sector | Size");
    for part in 0..4 {
        let partition = Partition::new(
            &read_disk(mbr_partitions_addr[part], mbr_partitions_addr[part] + 0x10 as u64)
            .expect("failed to read disk")
        );
        let mut partition = if !partition.is_none() {
            partition.unwrap()
        } else {
            Default::default()
        };

        match partition.part_type() {
            PartType::EMPTY => {
                println!("| {:04} | Empty", part + 1);
            }
            PartType::LINUX => {
                // Create superblock
                partition.superblock = Some(Superblock::new(partition.lba_start * 512));

                // TODO: Test magic number
                println!("| {} | {} | {:?} | {} | {}M",
                    from_utf8(&partition.superblock.unwrap().ext.s_volume_name)
                    .expect("can't convert to utf-8"),
                    if partition.bootable {"Yes"} else {"No"},
                    partition.part_type(),
                    partition.lba_start,
                    partition.lba_count / 1000
                )
            }
            _ => println!("| {:04} | {} | {:?} | {} | {}M",
                part + 1,
                if partition.bootable {"Yes"} else {"No"},
                partition.part_type(),
                partition.lba_start,
                partition.lba_count / 1000
                ),
        }

        PARTITIONS.lock().push(partition);
    }
}
