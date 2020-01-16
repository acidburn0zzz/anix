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
use alloc::prelude::v1::{Box, Vec};
use lazy_static::lazy_static;
use spin::Mutex;

use super::partitions::Partition;
use crate::disk::sata::Disk;

lazy_static! {
    pub static ref PARTITION_MANAGER: Mutex<PartitionManager> = Mutex::new(PartitionManager::new());
}

/// A partition manager which is an interface to use disks and partitions easier
pub struct PartitionManager {
    disks: Vec<Box<dyn Disk>>,
    next_disk_id: usize,
    partitions: Vec<Partition>,
    next_partition_id: usize,
}

unsafe impl Send for PartitionManager {}

impl PartitionManager {
    fn new() -> Self {
        Self {
            disks: Vec::new(),
            next_disk_id: 0,
            partitions: Vec::new(),
            next_partition_id: 0,
        }
    }

    // Disks

    /// Add a disk in the disks list
    pub fn add_disk(&mut self, disk: Box<dyn Disk>) {
        self.disks.insert(self.next_disk_id, disk);
        self.next_disk_id += 1;
    }

    /// Remove a disk in the disks list
    pub fn remove_disk(&mut self, id: usize) {
        self.disks.remove(id);
    }

    /// Get a specific disk among all disks
    pub fn get_disk(&self, id: usize) -> &Box<dyn Disk> {
        self.disks.get(id).unwrap()
    }

    /// Get all disks
    pub fn get_all_disks(&self) -> Vec<&Box<dyn Disk>> {
        let mut disks = Vec::new();
        for disk in 0..self.disks.len() {
            disks.push(self.disks.get(disk).unwrap());
        }
        disks
    }

    /// Get the disk which is used
    pub fn get_current_disk(&mut self) -> &Box<dyn Disk> {
        // For now, we just use the first partition, another are empty
        self.disks.get(0).unwrap()
    }

    /// Get a mutable reference of the disk which is used
    pub fn get_current_disk_mut(&mut self) -> &mut Box<dyn Disk> {
        // For now, we just use the first partition, another are empty
        &mut self.disks[0]
    }

    // Partitions

    /// Add a partition in the partitions list
    pub fn add_partition(&mut self, part: Partition) {
        self.partitions.insert(self.next_partition_id, part);
        self.next_partition_id += 1;
    }

    /// Remove a partition in the partitions list
    pub fn remove_partition(&mut self, id: usize) {
        self.partitions.remove(id);
    }

    /// Get a specific partition among all partitions
    pub fn get_partition(&self, id: usize) -> &Partition {
        self.partitions.get(id).unwrap()
    }

    /// Get the partition which is used
    pub fn get_current_partition(&self) -> &Partition {
        // For now, we just use the first partition, another are empty
        self.partitions.get(0).unwrap()
    }

    /// Get all partitions which are in a disk
    pub fn get_partitions_by_disk(&self, disk: usize) -> Vec<(usize, &Partition)> {
        let mut parts = Vec::new();
        for part in self.partitions.iter().enumerate() {
            if part.1.disk == disk {
                parts.push(part);
            }
        }
        parts
    }
}
