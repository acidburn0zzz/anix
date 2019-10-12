/*
Copyright (C) 2015 Philipp Oppermann
Copyright (C) 2018-2019 Nicolas Fouquet

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

use super::Page;
use memory::FrameAllocator;
use crate::memory::table::{Table, Level1};

#[derive(Copy, Clone)]
struct TinyAllocator([Option<Frame>; 3]);

#[derive(Copy, Clone)]
pub struct TemporaryPage {
    page: Page,
    allocator: TinyAllocator,
}

use crate::memory::table::{ActivePageTable};
use super::VirtualAddress;
use memory::Frame;

impl TemporaryPage {
	pub fn new<A>(page: Page, allocator: &mut A) -> TemporaryPage where A: FrameAllocator{
		TemporaryPage {
			page: page,
			allocator: TinyAllocator::new(allocator),
		}
	}

    /// Maps the temporary page to the given frame in the active table.
    /// Returns the start address of the temporary page.
    pub fn map(&mut self, frame: Frame, active_table: &mut ActivePageTable)
        -> VirtualAddress
    {
        use memory::paging::EntryFlags;

        assert!(active_table.translate_page(self.page).is_none(), "temporary page is already mapped");
        active_table.map_to(self.page, frame, EntryFlags::WRITABLE, &mut self.allocator);
        self.page.start_address()
    }

    /// Unmaps the temporary page in the active table.
    pub fn unmap(&mut self, active_table: &mut ActivePageTable) {
        active_table.unmap(self.page, &mut self.allocator)
    }
    
    pub fn map_table_frame(&mut self, frame: Frame, active_table: &mut ActivePageTable) -> &mut Table<Level1> {
		unsafe { &mut *(self.map(frame, active_table) as *mut Table<Level1>) }
	}
}

impl FrameAllocator for TinyAllocator {
    fn allocate_frame(&mut self) -> Option<Frame> {
        for frame_option in &mut self.0 {
            if frame_option.is_some() {
                return frame_option.take();
            }
        }
        None
    }

    fn deallocate_frame(&mut self, frame: Frame) {
        for frame_option in &mut self.0 {
            if frame_option.is_none() {
                *frame_option = Some(frame);
                return;
            }
        }
        panic!("Tiny allocator can hold only 3 frames.");
    }
}

impl TinyAllocator {
    fn new<A>(allocator: &mut A) -> TinyAllocator
        where A: FrameAllocator
    {
        let mut f = || allocator.allocate_frame();
        let frames = [f(), f(), f()];
        TinyAllocator(frames)
    }
}
