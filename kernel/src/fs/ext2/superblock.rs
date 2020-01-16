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

use crate::disk::sata::read_disk;

#[repr(packed,C)]
#[derive(Debug, Clone, Copy)]
pub struct Superblock
{
    pub data: SuperblockData,
    pub ext: SuperblockDataExt,
    pub _s_reserved: Array98<u32>,
    pub s_checksum: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SuperblockData
{
    pub s_inodes_count: u32,        // Inodes count
    pub s_blocks_count: u32,        // Blocks count
    pub s_r_blocks_count: u32,    // Reserved blocks count

    pub s_free_blocks_count: u32,    // Free blocks count
    pub s_free_inodes_count: u32,    // Free inodes count

    pub s_first_data_block: u32,    // First Data Block
    pub s_log_block_size: u32,    // Block size
    pub s_log_cluster_size: i32,    // Cluster size [FEAT_RO_COMPAT_BIGALLOC]

    pub s_blocks_per_group: u32,    // Number Blocks per group
    pub s_clusters_per_group: u32,    // Number of clusters per group
    pub s_inodes_per_group: u32,    // Number Inodes per group

    pub s_mtime: u32,            // Mount time
    pub s_wtime: u32,            // Write time

    pub s_mnt_count: u16,        // Mount count
    pub s_max_mnt_count: i16,    // Maximal mount count

    pub s_magic: u16,            // Magic signature
    pub s_state: u16,            // File system state
    pub s_errors: u16,            // Behaviour when detecting errors
    pub s_minor_rev_level: u16,                // Padding

    pub s_lastcheck: u32,        // time of last check
    pub s_checkinterval: u32,    // max. time between checks

    pub s_creator_os: u32,        // Formatting OS
    pub s_rev_level: u32,        // Revision level

    pub s_def_resuid: u16,        // Default uid for reserved blocks
    pub s_def_resgid: u16,        // Default gid for reserved blocks
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SuperblockDataExt
{
    // The following fields are only valid if s_rev_level > 0
    pub s_first_ino: u32,    // First valid inode
    pub s_inode_size: u16,    // Size of inode structure in bytes
    pub s_block_group_nr: u16,    // Block group number of this superblock (for backups)

    /// Compatible feature set flags (see FEAT_COMPAT_*). Can mount full RW if unknown
    pub s_feature_compat: u32,
    /// Incompatible feature set flags (see FEAT_INCOMPAT_*). Can't mount if unknown
    pub s_feature_incompat: u32,
    /// Read-only compatible feature set flags (see FEAT_RO_COMPAT_*).
    /// Can read but can't write if unknown
    pub s_feature_ro_compat: u32,

    /// 128-bit volume UUID
    pub s_uuid: [u8; 16],
    /// Volume label
    pub s_volume_name: [u8; 16],
    /// Last mounted directory
    pub s_last_mounted: Array64<u8>,

    // FEAT_COMPAT_DIR_PREALLOC
    pub s_prealloc_blocks: u8,
    pub s_prealloc_dir_blocks: u8,
    pub s_reserved_gdt_blocks: u16,

    // FEAT_COMPAT_HAS_JOURNAL
    pub s_journal_uuid: [u8; 16],
    /// Inode number of the journal
    pub s_journal_inum: u32,
    /// Journal device number if an external journal is in use (FEAT_INCOMPAT_JOURNAL_DEV)
    pub s_journal_dev: u32,


    pub s_last_orphan: u32,
    pub s_hash_seed: [u32; 4],
    pub s_def_hash_version: u8,
    pub s_jnl_backup_type: u8,

    /// [FEAT_INCOMPAT_64BIT] Group descriptor size
    pub s_desc_size: u16,
    pub s_default_mount_opts: u32,
    /// [FEAT_INCOMPAT_META_BG] First metadata block group
    pub s_first_meta_bg: u32,
    pub s_mkfs_time: u32,
    pub s_jnl_blocks: [u32; 15+2],
    // FEAT_INCOMPAT_64BIT
    pub s_blocks_count_hi: u32,
    pub s_r_blocks_count_hi: u32,
    pub s_free_blocks_count_hi: u32,
    pub s_min_extra_isize: u16,
    pub s_want_extra_isize: u16,

    pub s_flags: u32,
    pub s_raid_stride: u16,
    pub s_mmp_interval: u16,
    pub s_mmp_block: u64,
    pub s_raid_stripe_width: u32,
    pub s_log_groups_per_flex: u8,
    pub s_checksum_type: u8,
    pub _s_reserved_pad: u16,

    pub s_kbytes_written: u64,
    // Snapshots
    pub s_snapshot_inum: u32,
    pub s_snapshot_id: u32,
    pub s_snapshot_r_blocks_count: u64,
    pub s_snapshot_list: u32,
    // Error tracking
    pub s_error_count: u32,
    pub s_first_error_time: u32,
    pub s_first_error_ino: u32,
    pub s_first_error_block: u64,
    pub s_first_error_func: [u8; 32],
    pub s_first_error_line: u32,
    // - Most recent error
    pub s_last_error_time: u32,
    pub s_last_error_ino: u32,
    pub s_last_error_line: u32,
    pub s_last_error_block: u64,
    pub s_last_error_func: [u8; 32],

    pub s_mount_opts: Array64<u8>,
    pub s_usr_quota_inum: u32,
    pub s_grp_quota_inum: u32,
    pub s_overhead_blocks: u32,
    /// [FEAT_COMPAT_SPARSE_SUPER2]
    pub s_backup_bgs: [u32; 2],
    pub s_encrypt_algos: [u8; 4],
    pub s_encrypt_pw_salt: [u8; 16],
    /// Inode number of `lost+found` folder
    pub s_lpf_ino: u32,
    /// [FEAT_RO_COMPAT_PROJECT] Project quota inode
    pub s_prj_quota_inum: u32,
    pub s_checksum_seed: u32,
}

impl Superblock {
    pub fn new(offset: u64) -> Self {
        // TODO: For safe, use the plain crate
        unsafe {
            *(read_disk(offset + 1024, offset + 2048)
            .expect("failed to read disk")
            .as_slice()
            as *const _ as *const Self)
        }
    }
}

use core::fmt::{Debug, Result, Formatter};
#[derive(Clone, Copy)]
pub struct Array98<T> {
    data: [T; 98]
}

impl<T: Debug> Debug for Array98<T> {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        self.data[..].fmt(formatter)
    }
}

#[derive(Clone, Copy)]
pub struct Array64<T> {
    data: [T; 64]
}

impl<T: Debug> Debug for Array64<T> {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        self.data[..].fmt(formatter)
    }
}
