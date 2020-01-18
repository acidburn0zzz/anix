/*
Copyright (C) 2015 Philipp Oppermann
Copyright (C) 2018-2020 Nicolas Fouquet

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see https://www.gnu.org/licenses.
*/

#![allow(deprecated)]

pub mod paging;
pub mod table;
pub mod heap;
pub mod consts;

use multiboot2::{MemoryAreaIter, MemoryArea, ElfSectionsTag};
use x86_64::PhysAddr;
use x86_64::structures::paging::{
    UnusedPhysFrame,
    FrameAllocator,
    page::Size4KiB,
    frame::PhysFrame,
};

use self::paging::{EntryFlags, Page, InactivePageTable};
use self::paging::temporary_page::TemporaryPage;
use self::table::ActivePageTable;
use crate::{AREA_FRAME_ALLOCATOR, ACTIVE_TABLE};
use crate::errors::Result;

pub const PAGE_SIZE: usize = 4096;

#[derive(Copy, Clone)]
pub struct AreaFrameAllocator {
    next_free_frame: PhysFrame,
    current_area: Option<&'static MemoryArea>,
    areas: MemoryAreaIter,
    kernel_start: PhysFrame,
    kernel_end: PhysFrame,
    multiboot_start: PhysFrame,
    multiboot_end: PhysFrame,
}

impl AreaFrameAllocator {
    fn choose_next_area(&mut self) {
        self.current_area = self.areas.clone().filter(|area| {
            let address = area.start_address() + area.size() - 1;
            PhysFrame::containing_address(PhysAddr::new(address)) >= self.next_free_frame
        }).min_by_key(|area| area.start_address());

        if let Some(area) = self.current_area {
            let start_frame = PhysFrame::containing_address(PhysAddr::new(area.start_address()));
            if self.next_free_frame < start_frame {
                self.next_free_frame = start_frame;
            }
        }
    }

    pub fn new<'a>(kernel_start: u64, kernel_end: u64,
        multiboot_start: u64, multiboot_end: u64,
        memory_areas: MemoryAreaIter) -> AreaFrameAllocator
    {
        let mut allocator = AreaFrameAllocator {
            next_free_frame: PhysFrame::containing_address(PhysAddr::new(0)),
            current_area: None,
            areas: memory_areas,
            kernel_start: PhysFrame::containing_address(PhysAddr::new(kernel_start)),
            kernel_end: PhysFrame::containing_address(PhysAddr::new(kernel_end)),
            multiboot_start: PhysFrame::containing_address(PhysAddr::new(multiboot_start)),
            multiboot_end: PhysFrame::containing_address(PhysAddr::new(multiboot_end)),
        };
        allocator.choose_next_area();
        allocator
    }
}

unsafe impl FrameAllocator<Size4KiB> for AreaFrameAllocator {
    fn allocate_frame(&mut self) -> Option<UnusedPhysFrame> {
        if let Some(area) = self.current_area {
            // "Clone" the frame to return it if it's free. PhysFrame doesn't
            // implement Clone, but we can construct an identical frame.
            let frame = self.next_free_frame;

            // the last frame of the current area
            let current_area_last_frame = {
                let address = area.start_address() + area.size() - 1;
                PhysFrame::containing_address(PhysAddr::new(address))
            };

            if frame > current_area_last_frame {
                // all frames of current area are used, switch to next area
                self.choose_next_area();
            } else if frame >= self.kernel_start && frame <= self.kernel_end {
                //  `frame` is used by the kernel
                self.next_free_frame = PhysFrame::from_start_address(
                    PhysAddr::new(self.kernel_end.start_address().as_u64() + 4096)
                ).unwrap();
            } else if frame >= self.multiboot_start && frame <= self.multiboot_end {
                // `frame` is used by the multiboot information structure
                self.next_free_frame = PhysFrame::from_start_address(
                    PhysAddr::new(self.multiboot_end.start_address().as_u64() + 4096)
                ).unwrap();
            } else {
                // frame is unused, increment `next_free_frame` and return it
                self.next_free_frame = PhysFrame::from_start_address(
                    PhysAddr::new(self.next_free_frame.start_address().as_u64() + 4096)
                ).unwrap();
                return unsafe { Some(UnusedPhysFrame::new(frame)) };
            }
            // `frame` was not valid, try it again with the updated `next_free_frame`
            self.allocate_frame()
        } else {
            None // no free frames left
        }
    }
}

pub unsafe fn init(start: u64, end: u64, elf_sections_tag: ElfSectionsTag,
                    memory_areas: MemoryAreaIter) {
    assert_has_not_been_called!("memory::init must be called only once");

    let kernel_start = elf_sections_tag.sections().filter(|s| s.is_allocated())
                                                .map(|s| s.start_address()).min().unwrap();
    let kernel_end = elf_sections_tag.sections().filter(|s| s.is_allocated())
                                                .map(|s| s.start_address() + s.size()).max().unwrap();

    println!("\nKernel start: {:#x}, Kernel end: {:#x}", kernel_start, kernel_end);
    println!("\nMultiboot start: {:#x}, Multiboot end: {:#x}", start, end);

    let mut area_frame_allocator = AreaFrameAllocator::new(
        kernel_start, kernel_end,
        start, end,
        memory_areas
    );

    let mut active_table = create_mapping(&mut area_frame_allocator, start, end, elf_sections_tag);

    use self::consts::KERNEL_HEAP_OFFSET;
    let kernel_heap_start_page = Page::containing_address(KERNEL_HEAP_OFFSET.start);
    let kernel_heap_end_page = Page::containing_address(KERNEL_HEAP_OFFSET.end);

    for page in Page::range_inclusive(kernel_heap_start_page, kernel_heap_end_page) {
        active_table.map(page, EntryFlags::WRITABLE |
                               EntryFlags::USER_ACCESSIBLE,
                               &mut area_frame_allocator);
    }

    *ACTIVE_TABLE.lock() = Some(active_table);
    *AREA_FRAME_ALLOCATOR.lock() = Some(area_frame_allocator);
}

pub fn create_mapping<A>(allocator: &mut A, start: u64,
                        end: u64, elf_sections_tag: ElfSectionsTag)
                        -> ActivePageTable where A: FrameAllocator<Size4KiB> {
    let mut temporary_page = TemporaryPage::new(Page { number: 0xcafebabe },
        allocator);

    let mut active_table = unsafe { ActivePageTable::new() };
    let mut new_table = {
        let frame = allocator.allocate_frame().expect("no more frames");
        InactivePageTable::new(frame, &mut active_table, &mut temporary_page)
    };

    active_table.with(&mut new_table, /*&mut temporary_page,*/ |mapper| {
        // Map allocated kernel sections
        for section in elf_sections_tag.sections() {

            if !section.is_allocated() {
                // section is not loaded to memory
                continue;
            }

            println!("Mapping section at addr: {:#x}, size: {:#x}",
               section.start_address(), section.size());

            if section.start_address() % PAGE_SIZE as u64 != 0{
                println!("Sections need to be page aligned!!");
            }

            let flags = EntryFlags::from_elf_section_flags(&section);

            let start_frame = PhysFrame::containing_address(PhysAddr::new(section.start_address()));
            let end_frame = PhysFrame::containing_address(PhysAddr::new(section.end_address()- 1));
            for frame in PhysFrame::range_inclusive(start_frame, end_frame) {
                mapper.identity_map(frame,
                                    flags | EntryFlags::USER_ACCESSIBLE,
                                    allocator);
            }
        }

        // Map multiboot structure
        let multiboot_start = PhysFrame::containing_address(PhysAddr::new(start));
        let multiboot_end = PhysFrame::containing_address(PhysAddr::new(end - 1));
        for frame in PhysFrame::range_inclusive(multiboot_start, multiboot_end) {
            mapper.identity_map(frame,
                                EntryFlags::PRESENT,
                                allocator);
        }

        let ahci_frames_start = PhysFrame::containing_address(PhysAddr::new(0x550000));
        let ahci_frames_end = PhysFrame::containing_address(PhysAddr::new(0x6000000));

        for frame in PhysFrame::range_inclusive(ahci_frames_start, ahci_frames_end) {
            mapper.identity_map(frame,
                                EntryFlags::PRESENT | EntryFlags::WRITABLE,
                                allocator);
        }

        let ahci_frames_start2 = PhysFrame::containing_address(PhysAddr::new(0x110000));
        let ahci_frames_end2 = PhysFrame::containing_address(PhysAddr::new(0x200000));

        for frame in PhysFrame::range_inclusive(ahci_frames_start2, ahci_frames_end2) {
            mapper.identity_map(frame,
                                EntryFlags::PRESENT | EntryFlags::WRITABLE |
                                EntryFlags::USER_ACCESSIBLE, allocator);
        }

        let program_start = PhysFrame::containing_address(PhysAddr::new(0x2000));
        let program_end = PhysFrame::containing_address(PhysAddr::new(0x3000));

        for frame in PhysFrame::range_inclusive(program_start, program_end) {
            mapper.identity_map(frame,
                                EntryFlags::PRESENT | EntryFlags::WRITABLE |
                                EntryFlags::USER_ACCESSIBLE,
                                allocator);
        }

        let program2_start = PhysFrame::containing_address(PhysAddr::new(0x2000000));
        let program2_end = PhysFrame::containing_address(PhysAddr::new(0xb000010));

        for frame in PhysFrame::range_inclusive(program2_start, program2_end) {
            mapper.identity_map(frame,
                                EntryFlags::PRESENT | EntryFlags::WRITABLE |
                                EntryFlags::USER_ACCESSIBLE, allocator);
        }

        let program3_start = PhysFrame::containing_address(PhysAddr::new(0x3c));
        let program3_end = PhysFrame::containing_address(PhysAddr::new(0x10000));

        for frame in PhysFrame::range_inclusive(program3_start, program3_end) {
            mapper.identity_map(frame,
                                EntryFlags::PRESENT | EntryFlags::WRITABLE |
                                EntryFlags::USER_ACCESSIBLE, allocator);
        }

        let program4_start = PhysFrame::containing_address(PhysAddr::new(0x40022400));
        let program4_end = PhysFrame::containing_address(PhysAddr::new(0x40022800));

        for frame in PhysFrame::range_inclusive(program4_start, program4_end) {
            mapper.identity_map(frame,
                                EntryFlags::PRESENT | EntryFlags::WRITABLE |
                                EntryFlags::USER_ACCESSIBLE, allocator);
        }

        use super::consts::USER_HEAP_OFFSET;
        let user_heap_start = PhysFrame::containing_address(PhysAddr::new(USER_HEAP_OFFSET.start as u64));
        let user_heap_end = PhysFrame::containing_address(PhysAddr::new(USER_HEAP_OFFSET.end as u64));

        for frame in PhysFrame::range_inclusive(user_heap_start, user_heap_end) {
            mapper.identity_map(frame,
                                EntryFlags::PRESENT | EntryFlags::WRITABLE |
                                EntryFlags::USER_ACCESSIBLE, allocator);
        }
     });

    let old_table = active_table.switch(new_table);

    // turn the old p4 page into a guard page
    let old_p4_page = Page::containing_address(
        old_table.p4_frame.start_address().as_u64() as usize
    );
    active_table.unmap(old_p4_page, allocator);

    active_table
}

/// Allocate a frame
pub fn allocate_frames() -> Option<UnusedPhysFrame> {
    unsafe{
        if let Some(ref mut allocator) = *AREA_FRAME_ALLOCATOR.lock() {
            return allocator.allocate_frame();
        } else {
            panic!("frame allocator not initialized");
        }
    }
}

pub fn physalloc() -> Result<usize>{
    Ok(allocate_frames().unwrap().start_address().as_u64() as usize)
}

pub unsafe fn map(start_address: u64, end_address: u64, flags: EntryFlags) {
    use x86_64::instructions::tlb;
    let start = PhysFrame::containing_address(PhysAddr::new(start_address));
    let end = PhysFrame::containing_address(PhysAddr::new(end_address));

    // TODO: Remove EntryFlags use PageTableFlags
    // TODO: Use lazy_static for ACTIVE_TABLE and AREA_FRAME_ALLOCATOR
    tlb::flush_all();
    for frame in PhysFrame::range_inclusive(start, end) {
        ACTIVE_TABLE.lock().unwrap().identity_map(frame,
                            flags,
                            &mut AREA_FRAME_ALLOCATOR
                            .lock().unwrap());
    }
}
