/*
 * Copyright (C) 2015 Philipp Oppermann
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
use core::ptr::Unique;
use x86_64::PhysAddr;
use x86_64::structures::paging::{
    FrameAllocator,
    PhysFrame,
    page::Size4KiB,
};

use super::{VirtualAddress, PhysicalAddress, Page, ENTRY_COUNT};
use crate::memory::table::{Table, Level4};
use crate::memory::PAGE_SIZE;
use crate::memory::paging::EntryFlags;

#[derive(Copy, Clone)]
pub struct Mapper {
    p4: Unique<Table<Level4>>,
}

impl Mapper {
    pub unsafe fn new() -> Mapper {
        Mapper {
            p4: Unique::new_unchecked(crate::memory::table::P4),
        }
    }

    pub fn p4(&self) -> &Table<Level4> {
        unsafe { self.p4.as_ref() }
    }

    pub fn p4_mut(&mut self) -> &mut Table<Level4> {
        unsafe { self.p4.as_mut() }
    }

    pub fn translate(&self, virtual_address: VirtualAddress) -> Option<PhysicalAddress> {
        let offset = virtual_address % PAGE_SIZE;
        self.translate_page(Page::containing_address(virtual_address))
            .map(|frame| frame.start_address().as_u64() as usize + offset)
    }

    pub fn translate_page(&self, page: Page) -> Option<PhysFrame> {
        let p3 = self.p4().next_table(page.p4_index());

        let huge_page = || {
            p3.and_then(|p3| {
                let p3_entry = &p3[page.p3_index()];
                // 1GiB page?
                if let Some(start_frame) = p3_entry.pointed_frame() {
                    if p3_entry.flags().contains(EntryFlags::HUGE_PAGE) {
                        // address must be 1GiB aligned
                        assert!((start_frame.start_address().as_u64() as usize / 4096)
                                % (ENTRY_COUNT * ENTRY_COUNT) == 0);
                        return Some(PhysFrame::from_start_address(
                                PhysAddr::new(
                                    (
                                        (
                                            (start_frame.start_address().as_u64() as usize / 4096)
                                            + page.p2_index() * ENTRY_COUNT + page.p1_index()) * 4096)
                                    as u64)
                            ).unwrap()
                        );
                    }
                }
                if let Some(p2) = p3.next_table(page.p3_index()) {
                    let p2_entry = &p2[page.p2_index()];
                    // 2MiB page?
                    if let Some(start_frame) = p2_entry.pointed_frame() {
                        if p2_entry.flags().contains(EntryFlags::HUGE_PAGE) {
                            // address must be 2MiB aligned
                            assert!((start_frame.start_address().as_u64() as usize / 4096)
                                    % ENTRY_COUNT == 0);
                            return Some(PhysFrame::from_start_address(
                                    PhysAddr::new((
                                        (start_frame.start_address().as_u64() as usize / 4096)
                                        + page.p1_index() * 4096) as u64)
                                ).unwrap()
                            );
                        }
                    }
                }
                None
            })
        };

        p3.and_then(|p3| p3.next_table(page.p3_index()))
        .and_then(|p2| p2.next_table(page.p2_index()))
        .and_then(|p1| p1[page.p1_index()].pointed_frame())
        .or_else(huge_page)
    }

    pub fn map_to<A>(&mut self, page: Page, frame: PhysFrame, flags: EntryFlags,
                    allocator: &mut A)
        where A: FrameAllocator<Size4KiB>
    {
        let p4 = self.p4_mut();
        let p3 = p4.next_table_create(page.p4_index(), allocator);
        let p2 = p3.next_table_create(page.p3_index(), allocator);
        let p1 = p2.next_table_create(page.p2_index(), allocator);

        // assert!(p1[page.p1_index()].is_unused());
        p1[page.p1_index()].set(frame, flags | EntryFlags::PRESENT);
    }

    pub fn map<A>(&mut self, page: Page, flags: EntryFlags, allocator: &mut A)
        where A: FrameAllocator<Size4KiB>
    {
        let frame = allocator.allocate_frame().expect("out of memory");
        self.map_to(page, frame.frame(), flags, allocator)
    }

    pub fn identity_map<A>(&mut self, frame: PhysFrame, flags: EntryFlags, allocator: &mut A)
        where A: FrameAllocator<Size4KiB>
    {
        let page = Page::containing_address(frame.start_address().as_u64() as usize);
        self.map_to(page, frame, flags, allocator)
    }

    pub fn unmap<A>(&mut self, page: Page, _allocator: &mut A)
        where A: FrameAllocator<Size4KiB>
    {
        use x86_64::instructions::tlb;
        use x86_64::VirtAddr;

        assert!(self.translate(page.start_address()).is_some());

        let p1 = self.p4_mut().next_table_mut(page.p4_index()).and_then(|p3|
            p3.next_table_mut(page.p3_index())).and_then(|p2|
                p2.next_table_mut(page.p2_index()
            )
        ).expect("mapping code does not support huge pages");
        let _frame = p1[page.p1_index()].pointed_frame().unwrap();
        p1[page.p1_index()].set_unused();
        tlb::flush(VirtAddr::new(page.start_address() as u64));
        // TODO free p(1,2,3) table if empty
        // allocator.deallocate_frame(frame);
    }
}
