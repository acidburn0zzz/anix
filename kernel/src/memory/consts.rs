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

//! # The memory mapping of Anix is in this file
//! Below this is a plan of its memory mapping:
//! MAX --------> |-----------------------------------|
//!               |               FREE                |
//! 0xfebf2000 -> |-----------------------------------|
//! |             |               AHCI                |
//! |             |-----------------------------------|
//! |             |             VGA MMIO              |
//! | Hardware 2  |-----------------------------------|
//! |             |           E1000 MMIO (Qemu)       |
//! |             |-----------------------------------|
//! |             |              VGA VRAM             |
//! 0xfd000000 -> |-----------------------------------|
//!               |             User stack            |
//! 0xe0000000 -> |-----------------------------------|
//!               |               User                |
//! 0x40000000 -> |-----------------------------------|
//!               |           Kernel TCB              |
//! 0xb000000 --> |-----------------------------------|
//!               |               FREE                |
//! 0x1200000 --> |-----------------------------------|
//!               |           Kernel heap             |
//! 0x1000000 --> |-----------------------------------|
//!               |               FREE                |
//! 0x500000 ---> |-----------------------------------|
//!               |         Frame allocator           |
//! 0x3fa000 ---> |-----------------------------------|
//!               |               FREE                |
//! 0x3dc000 ---> |-----------------------------------|
//!               |              Kernel               |
//! 0x100000 ---> |-----------------------------------|
//!               |         Hardware 1 (lowmem)       |
//! 0xa0000 ----> |-----------------------------------|
//!               |               FREE                |
//! 0x1000 -----> |-----------------------------------|
//!               |              GDT + IDT            |
//! 0x0 --------> |-----------------------------------|

pub struct MemoryOffset {
    pub start: usize,
    pub end: usize,
    pub size: usize,
}

pub const GDT_IDT_OFFSET: MemoryOffset = MemoryOffset {
    start: 0x0,
    end: 0xfff,
    size: 0xfff,
};
pub const HARDWARE_LOWMEM_OFFSET: MemoryOffset = MemoryOffset{
    start: 0xa0_000,
    end: 0xff_fff,
    size: 0xff_fff - 0xa0_000,
};
pub const KERNEL_OFFSET: MemoryOffset = MemoryOffset{
    start: 0x100_000,
    end: 0x3f_fff_fff,
    size: 0x3f_fff_fff - 0x100_000,
};

pub const KERNEL_HEAP_OFFSET: MemoryOffset = MemoryOffset{
    start: 0x1_000_000,
    end: 0x1_200_000,
    size: 0x1_200_000 - 0x1_000_000,
};

pub const USER_OFFSET: MemoryOffset = MemoryOffset{
    start: 0x40_000_000,
    end: 0xdf_fff_fff,
    size: 0xdf_fff_fff - 0x40_000_000,
};

pub const USER_TCB_OFFSET: MemoryOffset = MemoryOffset {
    start: 0xb_000_000,
    end: 0,  // We don't know the size
    size: 0, // We don't know the size
};

pub const USER_STACK_OFFSET: MemoryOffset = MemoryOffset{
    start: 0xe0_000_000,
    end: 0xfc_fff_fff,
    size: 0xfc_fff_fff - 0xe0_000_000,
};

pub const HARDWARE2_OFFSET: MemoryOffset = MemoryOffset{
    start: 0xfd_000_000,
    end: 0xfe_bf1_fff,
    size: 0xfe_bf1_fff - 0xfd_000_000,
};

pub const ALL_OFFSETS: [MemoryOffset; 8] = [GDT_IDT_OFFSET, HARDWARE_LOWMEM_OFFSET, KERNEL_OFFSET,
                                            KERNEL_HEAP_OFFSET, USER_OFFSET, USER_STACK_OFFSET,
                                            HARDWARE2_OFFSET, USER_TCB_OFFSET];
