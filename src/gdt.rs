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

use core::mem;
use x86::*;
use x86::segmentation::SegmentSelector;
use x86::Ring::Ring0;
use x86::current::task::TaskStateSegment;
use x86::bits64::segmentation::load_cs;
use x86::segmentation::Descriptor;
use x86::dtables::DescriptorTablePointer;

pub const USER_TCB_OFFSET: usize = 0xB000_0000;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
pub const GDT_NULL: usize = 0;
pub const GDT_KERNEL_CODE: usize = 1;
pub const GDT_KERNEL_DATA: usize = 2;
pub const GDT_KERNEL_TLS: usize = 3;
pub const GDT_USER_CODE: usize = 4;
pub const GDT_USER_DATA: usize = 5;
pub const GDT_USER_TLS: usize = 6;
pub const GDT_TSS: usize = 7;
pub const GDT_TSS_HIGH: usize = 8;

pub const GDT_A_PRESENT: u8 = 1 << 7;
pub const GDT_A_RING_0: u8 = 0 << 5;
pub const GDT_A_RING_1: u8 = 1 << 5;
pub const GDT_A_RING_2: u8 = 2 << 5;
pub const GDT_A_RING_3: u8 = 3 << 5;
pub const GDT_A_SYSTEM: u8 = 1 << 4;
pub const GDT_A_EXECUTABLE: u8 = 1 << 3;
pub const GDT_A_CONFORMING: u8 = 1 << 2;
pub const GDT_A_PRIVILEGE: u8 = 1 << 1;
pub const GDT_A_DIRTY: u8 = 1;

pub const GDT_A_TSS_AVAIL: u8 = 0x9;
pub const GDT_A_TSS_BUSY: u8 = 0xB;

pub const GDT_F_PAGE_SIZE: u8 = 1 << 7;
pub const GDT_F_PROTECTED_MODE: u8 = 1 << 6;
pub const GDT_F_LONG_MODE: u8 = 1 << 5;


static mut INIT_GDTR: DescriptorTablePointer<Descriptor> = DescriptorTablePointer {
    limit: 0,
    base: 0 as *const Descriptor
};

static mut INIT_GDT: [GdtEntry; 4] = [
    // Null
    GdtEntry::new(0, 0, 0, 0),
    // Kernel code
    GdtEntry::new(0, 0, GDT_A_PRESENT | GDT_A_RING_0 | GDT_A_SYSTEM | GDT_A_EXECUTABLE | GDT_A_PRIVILEGE, GDT_F_LONG_MODE),
    // Kernel data
    GdtEntry::new(0, 0, GDT_A_PRESENT | GDT_A_RING_0 | GDT_A_SYSTEM | GDT_A_PRIVILEGE, GDT_F_LONG_MODE),
    // Kernel TLS
    GdtEntry::new(0, 0, GDT_A_PRESENT | GDT_A_RING_3 | GDT_A_SYSTEM | GDT_A_PRIVILEGE, GDT_F_LONG_MODE)
];

#[thread_local]
pub static mut GDTR: DescriptorTablePointer<Descriptor> = DescriptorTablePointer {
    limit: 0,
    base: 0 as *const Descriptor
};

#[thread_local]
pub static mut GDT: [GdtEntry; 9] = [
    // Null
    GdtEntry::new(0, 0, 0, 0),
    // Kernel code
    GdtEntry::new(0, 0, GDT_A_PRESENT | GDT_A_RING_0 | GDT_A_SYSTEM | GDT_A_EXECUTABLE | GDT_A_PRIVILEGE, GDT_F_LONG_MODE),
    // Kernel data
    GdtEntry::new(0, 0, GDT_A_PRESENT | GDT_A_RING_0 | GDT_A_SYSTEM | GDT_A_PRIVILEGE, GDT_F_LONG_MODE),
    // Kernel TLS
    GdtEntry::new(0, 0, GDT_A_PRESENT | GDT_A_RING_0 | GDT_A_SYSTEM | GDT_A_PRIVILEGE, GDT_F_LONG_MODE),
    // User code
    GdtEntry::new(0, 0, GDT_A_PRESENT | GDT_A_RING_3 | GDT_A_SYSTEM | GDT_A_EXECUTABLE | GDT_A_PRIVILEGE, GDT_F_LONG_MODE),
    // User data
    GdtEntry::new(0, 0, GDT_A_PRESENT | GDT_A_RING_3 | GDT_A_SYSTEM | GDT_A_PRIVILEGE, GDT_F_LONG_MODE),
    // User TLS
    GdtEntry::new(0, 0, GDT_A_PRESENT | GDT_A_RING_3 | GDT_A_SYSTEM | GDT_A_PRIVILEGE, GDT_F_LONG_MODE),
    // TSS
    GdtEntry::new(0, 0, GDT_A_PRESENT | GDT_A_RING_3 | GDT_A_TSS_AVAIL, 0),
    // TSS must be 16 bytes long, twice the normal size
    GdtEntry::new(0, 0, 0, 0),
];

#[thread_local]
pub static mut TSS: TaskStateSegment = TaskStateSegment {
    reserved: 0,
    rsp: [0; 3],
    reserved2: 0,
    ist: [0; 7],
    reserved3: 0,
    reserved4: 0,
    iomap_base: 0xFFFF
};

#[cfg(feature = "pti")]
pub unsafe fn set_tss_stack(stack: usize) {
    use arch::x86_64::pti::{PTI_CPU_STACK, PTI_CONTEXT_STACK};
    TSS.rsp[0] = (PTI_CPU_STACK.as_ptr() as usize + PTI_CPU_STACK.len()) as u64;
    PTI_CONTEXT_STACK = stack;
}

#[cfg(not(feature = "pti"))]
pub unsafe fn set_tss_stack(stack: usize) {
    TSS.rsp[0] = stack as u64;
}

// Initialize GDT
pub unsafe fn init() {
    // Setup the initial GDT with TLS, so we can setup the TLS GDT (a little confusing)
    // This means that each CPU will have its own GDT, but we only need to define it once as a thread local
    INIT_GDTR.limit = (INIT_GDT.len() * mem::size_of::<GdtEntry>() - 1) as u16;
    INIT_GDTR.base = INIT_GDT.as_ptr() as *const Descriptor;

    // Load the initial GDT, before we have access to thread locals
    dtables::lgdt(&INIT_GDTR);

    // Load the segment descriptors
    load_cs(SegmentSelector::new(GDT_KERNEL_CODE as u16, Ring0));
    segmentation::load_ds(SegmentSelector::new(GDT_KERNEL_DATA as u16, Ring0));
    segmentation::load_es(SegmentSelector::new(GDT_KERNEL_DATA as u16, Ring0));
    segmentation::load_fs(SegmentSelector::new(GDT_KERNEL_DATA as u16, Ring0));
    segmentation::load_gs(SegmentSelector::new(GDT_KERNEL_DATA as u16, Ring0));
    segmentation::load_ss(SegmentSelector::new(GDT_KERNEL_DATA as u16, Ring0));
}

/// Initialize GDT with TLS
pub unsafe fn init_paging(tcb_offset: usize, stack_offset: usize) {
    // Set the TLS segment to the offset of the Thread Control Block
    INIT_GDT[GDT_KERNEL_TLS].set_offset(tcb_offset as u32);

    // Load the initial GDT, before we have access to thread locals
    dtables::lgdt(&INIT_GDTR);

    // Load the segment descriptors
    segmentation::load_fs(SegmentSelector::new(GDT_KERNEL_TLS as u16, Ring0));

    // Now that we have access to thread locals, setup the AP's individual GDT
    GDTR.limit = (GDT.len() * mem::size_of::<GdtEntry>() - 1) as u16;
    GDTR.base = GDT.as_ptr() as *const Descriptor;

    // Set the TLS segment to the offset of the Thread Control Block
    GDT[GDT_KERNEL_TLS].set_offset(tcb_offset as u32);

    // Set the User TLS segment to the offset of the user TCB
    GDT[GDT_USER_TLS].set_offset(USER_TCB_OFFSET as u32);

    // We can now access our TSS, which is a thread local
    GDT[GDT_TSS].set_offset(&TSS as *const _ as u32);
    GDT[GDT_TSS].set_limit(mem::size_of::<TaskStateSegment>() as u32);

    // Set the stack pointer when coming back from userspace
    set_tss_stack(stack_offset);

    // Load the new GDT, which is correctly located in thread local storage
    dtables::lgdt(&GDTR);

    // Reload the segment descriptors
    load_cs(SegmentSelector::new(GDT_KERNEL_CODE as u16, Ring0));
    segmentation::load_ds(SegmentSelector::new(GDT_KERNEL_DATA as u16, Ring0));
    segmentation::load_es(SegmentSelector::new(GDT_KERNEL_DATA as u16, Ring0));
    segmentation::load_fs(SegmentSelector::new(GDT_KERNEL_TLS as u16, Ring0));
    segmentation::load_gs(SegmentSelector::new(GDT_KERNEL_DATA as u16, Ring0));
    segmentation::load_ss(SegmentSelector::new(GDT_KERNEL_DATA as u16, Ring0));

    // Load the task register
    task::load_tr(SegmentSelector::new(GDT_TSS as u16, Ring0));
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct GdtEntry {
    pub limitl: u16,
    pub offsetl: u16,
    pub offsetm: u8,
    pub access: u8,
    pub flags_limith: u8,
    pub offseth: u8
}

impl GdtEntry {
    pub const fn new(offset: u32, limit: u32, access: u8, flags: u8) -> Self {
        GdtEntry {
            limitl: limit as u16,
            offsetl: offset as u16,
            offsetm: (offset >> 16) as u8,
            access: access,
            flags_limith: flags & 0xF0 | ((limit >> 16) as u8) & 0x0F,
            offseth: (offset >> 24) as u8
        }
    }

    pub fn set_offset(&mut self, offset: u32) {
        self.offsetl = offset as u16;
        self.offsetm = (offset >> 16) as u8;
        self.offseth = (offset >> 24) as u8;
    }

    pub fn set_limit(&mut self, limit: u32) {
        self.limitl = limit as u16;
        self.flags_limith = self.flags_limith & 0xF0 | ((limit >> 16) as u8) & 0x0F;
    }
}
