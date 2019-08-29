use core::mem::size_of;
use crate::fs::PARTITIONS;
use crate::disk::sata::read_disk;
use self::gd::*;
use self::inode::*;
pub mod superblock;
pub mod gd;
pub mod inode;

pub const INODE_ROOT: u32 = 2;

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

pub fn init() {
    let part = &PARTITIONS.lock()[0];
    let superblock = part.superblock.unwrap();
    let block_size = 1024 << superblock.data.s_log_block_size;

    // TODO: Include this variables in the Partition struct
    let gdt = GDTable::new(part.lba_start * 512 + block_size as u64, block_size);

    // TODO: Just pass a Partition struct
    let inode = Inode::new(part.lba_start * 512, 14, block_size, superblock, &gdt);
    println!("Inode in mode {:#x}, with size {}", inode.i_mode, inode.i_size);

    println!("Content of inode: {}", inode.read(part.lba_start * 512).expect("cannot read the inode"));

    // TODO: Read directory entries + get inode from path + move the `boot` directory in the `files`
    // directory ànd create `bin`, `usr`, ... + open and close system + resolve random Page Faults
}
