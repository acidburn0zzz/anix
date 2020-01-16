/*
 * Copyright (C) 2016 Redox OS Developers
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

use self::{disk_ata::DiskATA, disk_atapi::DiskATAPI};
use self::hba::{HbaMem, HbaPortType};
use crate::drivers::{DriverInstance, Driver};
use crate::pci::BusDevice;
use crate::errors::{Result, EIO, Error};
use crate::io::io::Io;
use crate::fs::partition_manager::PARTITION_MANAGER;

pub mod disk_ata;
pub mod disk_atapi;
pub mod fis;
pub mod hba;

pub static S_PCI_DRIVER: PciDriver = PciDriver;

pub trait Disk {
    fn id(&self) -> usize;
    fn size(&mut self) -> u64;
    fn read(&mut self, block: u64, buffer: &mut [u8]) -> Result<Option<usize>>;
    fn write(&mut self, block: u64, buffer: &[u8]) -> Result<Option<usize>>;
    fn block_length(&mut self) -> Result<u32>;
}

pub fn init(){
    crate::drivers::register_driver(&S_PCI_DRIVER);
}

pub struct PciDriver;

impl Driver for PciDriver
{
    fn name(&self) -> &str {
        "ahci-pci"
    }
    fn bus_type(&self) -> &str {
        "pci"
    }
    fn handles(&self, bus_dev: &dyn BusDevice) -> u32
    {
        let classcode = bus_dev.get_attr("class").unwrap_u32();
        // [class] [subclass] [IF] [ver]
        if classcode & 0xFFFFFF00 == 0x01060100{
            1    // Handle as weakly as possible (vendor-provided drivers bind higher)
        }
        else {
            0
        }
    }
    fn bind(&self, bus_dev: &mut dyn BusDevice) -> Box<dyn DriverInstance+'static>
    {
        // let irq = bus_dev.get_irq(0);
        let base = bus_dev.base_slice(5);
        let controller = SATAController::new(base);
        Box::new(controller)
    }
}

struct SATAController{}

impl SATAController{
    pub fn new<'a>(base: u32) -> Self {
        use crate::memory::{map, paging::EntryFlags};
        println!("\nDisks:");
        unsafe {
            map(base as usize,
                base as usize + 0x200,
                EntryFlags::PRESENT | EntryFlags::WRITABLE);
        }

        let all_disks = disks(base as usize, "disk");
        Self::register_disks(all_disks.1);
        Self::register_partitions();

        Self {}
    }

    pub fn register_disks(disks: Vec<Box<dyn Disk>>) {
        println!("Register disks");
        for mut disk in disks {
            if disk.block_length().expect("Could not read block_length") != 0 && disk.size() != 0 {
                println!("   - Register disk-{}", disk.id());
                unsafe {
                    PARTITION_MANAGER.force_unlock();
                }
                PARTITION_MANAGER.try_lock().unwrap().add_disk(disk);
            }
        }
    }

    pub fn register_partitions() {
        use crate::fs::mbr::Mbr;

        println!("Register partitions");
        unsafe {
            PARTITION_MANAGER.force_unlock();
        }
        for (id, _disk) in PARTITION_MANAGER.try_lock().unwrap().get_all_disks().iter().enumerate() {
            let parts = Mbr::new().get_partitions(id);
            for part in parts {
                println!("   - Register part which starts at {} in disk-{}", part.lba_start, id);
                unsafe {
                    PARTITION_MANAGER.force_unlock();
                }
                PARTITION_MANAGER.try_lock().unwrap().add_partition(part);
            }
        }
    }
}

unsafe impl Send for SATAController {}
impl crate::drivers::DriverInstance for SATAController {}

pub fn disks(base: usize, name: &str) -> (&'static mut HbaMem, Vec<Box<dyn Disk>>) {
    let hba_mem = unsafe { &mut *(base as *mut HbaMem) };
    hba_mem.init();
    let pi = hba_mem.pi.read();
    let disks: Vec<Box<dyn Disk>> = (0..hba_mem.ports.len())
          .filter(|&i| pi & 1 << i as i32 == 1 << i as i32)
          .filter_map(|i| {
              let port = unsafe { &mut *hba_mem.ports.as_mut_ptr().add(i) };
              let port_type = port.probe();
              print!("{}", format!("{}-{}: {:?}\n", name, i, port_type));

              let disk: Option<Box<dyn Disk>> = match port_type {
                  HbaPortType::SATA => {
                      match DiskATA::new(i, port) {
                          Ok(disk) => Some(Box::new(disk)),
                          Err(err) => {
                              print!("{}", format!("{}: {}\n", i, err));
                              None
                          }
                      }
                  }
                  HbaPortType::SATAPI => {
                      match DiskATAPI::new(i, port) {
                          Ok(disk) => Some(Box::new(disk)),
                          Err(err) => {
                              print!("{}", format!("{}: {}\n", i, err));
                              None
                          }
                      }
                  }
                  _ => None,
              };

              disk
          })
          .collect();

    (hba_mem, disks)
}

pub fn read_disk(start: u64, end: u64) -> Result<Vec<u8>>{
    let mut size = (end as usize - start as usize) + start as usize % 512;

    if size < 512 {
        size = 512;
    } else {
        size = size + size % 512
    }

    let mut buffer = Vec::with_capacity(size);
    buffer.resize(size + size % 512, 0);

    unsafe {
        PARTITION_MANAGER.force_unlock();
    }
    let mut manager = PARTITION_MANAGER.try_lock().unwrap();
    let result = manager.get_current_disk_mut().read(start / 512,
                                     &mut buffer.as_mut_slice());

    match result {
        Ok(_s) => {
            return Ok(buffer[
                (start as usize % 512)..
                (end as usize - start as usize) + start as usize % 512
            ].to_vec());
        },
        Err(_e) => {
            return Err(Error::new(EIO));
        },
    }
}
