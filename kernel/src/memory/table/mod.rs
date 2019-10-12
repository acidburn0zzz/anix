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

use core::ops::{Index, IndexMut};
use core::marker::PhantomData;
use memory::paging::*;
use memory::FrameAllocator;
use memory::Frame;
use memory::paging::mapper::Mapper;
use core::ops::{Deref, DerefMut};

pub struct Table<L: TableLevel> {
    entries: [Entry; ENTRY_COUNT],
    level: PhantomData<L>,
}


pub const P4: *mut Table<Level4> = 0xffffffff_fffff000 as *mut Table<Level4>;

#[derive(Copy, Clone)]
pub struct ActivePageTable {
    mapper: Mapper,
}

impl Deref for ActivePageTable {
    type Target = Mapper;

    fn deref(&self) -> &Mapper {
        &self.mapper
    }
}

impl DerefMut for ActivePageTable {
    fn deref_mut(&mut self) -> &mut Mapper {
        &mut self.mapper
    }
}

impl ActivePageTable {
    pub unsafe fn new() -> ActivePageTable {
        ActivePageTable {
            mapper: Mapper::new(),
        }
    }

    pub fn with<F>(&mut self,
                   table: &mut InactivePageTable,
                   _temporary_page: &mut temporary_page::TemporaryPage,
                   f: F) where F: FnOnce(&mut Mapper) {
        use x86_64::instructions::tlb;

        // overwrite recursive mapping
        self.p4_mut()[511].set(table.p4_frame.clone(), EntryFlags::PRESENT | EntryFlags::WRITABLE);
        tlb::flush_all();

        // execute f in the new context
        f(self);

        // TODO restore recursive mapping to original p4 table
        // temporary_page.unmap(self);
    }

    pub fn switch(&mut self, new_table: InactivePageTable) -> InactivePageTable {
        use x86::controlregs;

        let old_table = InactivePageTable {
            p4_frame: Frame::containing_address(
                unsafe{controlregs::cr3() as usize}
            ),
        };
        unsafe {
            controlregs::cr3_write(new_table.p4_frame.start_address() as u64);
        }
        old_table
    }
}

impl<L> Table<L> where L: TableLevel{
    /// Set all entries to zero
    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.set_unused();
        }
    }
}
impl<L> Table<L> where L: HierarchicalLevel{
    pub fn next_table_address(&self, index: usize) -> Option<usize>{
        let entry_flags = self[index].flags();
        if entry_flags.contains(EntryFlags::PRESENT) && !entry_flags.contains(EntryFlags::HUGE_PAGE) {
            let table_address = self as *const _ as usize;
            Some((table_address << 9) | (index << 12))
        } else {
            None
        }
    }

    pub fn next_table(&self, index: usize) -> Option<&Table<L::NextLevel>> {
        self.next_table_address(index)
            .map(|address| unsafe { &*(address as *const _) })
    }

    pub fn next_table_mut(&mut self, index: usize) -> Option<&mut Table<L::NextLevel>>{
        self.next_table_address(index)
            .map(|address| unsafe { &mut *(address as *mut _) })
    }

    pub fn next_table_create<A>(&mut self, index: usize, allocator: &mut A) -> &mut Table<L::NextLevel> where A: FrameAllocator{
        if self.next_table(index).is_none() {
            assert!(!self.entries[index].flags().contains(EntryFlags::HUGE_PAGE), "mapping code does not support huge pages");
            let frame = allocator.allocate_frame().expect("no frames available");
            self.entries[index].set(frame, EntryFlags::PRESENT | EntryFlags::WRITABLE | EntryFlags::USER_ACCESSIBLE);
            // println!("{:#?}", index);
            if index != 489 && index != 501 { // Fix a strange error
                self.next_table_mut(index).unwrap().zero();
            }
        }
        self.next_table_mut(index).unwrap()
    }
}

impl<L> Index<usize> for Table<L> where L: TableLevel {
    type Output = Entry;

    fn index(&self, index: usize) -> &Entry {
        &self.entries[index]
    }
}

impl<L> IndexMut<usize> for Table<L> where L: TableLevel {
    fn index_mut(&mut self, index: usize) -> &mut Entry {
        &mut self.entries[index]
    }
}

pub trait TableLevel {}

pub enum Level4 {}
pub enum Level3 {}
pub enum Level2 {}
pub enum Level1 {}

impl TableLevel for Level4 {}
impl TableLevel for Level3 {}
impl TableLevel for Level2 {}
impl TableLevel for Level1 {}

pub trait HierarchicalLevel: TableLevel {
    type NextLevel: TableLevel;
}

impl HierarchicalLevel for Level4 {
    type NextLevel = Level4;
}

impl HierarchicalLevel for Level3 {
    type NextLevel = Level3;
}

impl HierarchicalLevel for Level2 {
    type NextLevel = Level2;
}

impl HierarchicalLevel for Level1 {
    type NextLevel = Level1;
}

pub fn test() {
    let p4 = unsafe { &*P4 };
    p4.next_table(42).and_then(|p3| p3.next_table(1337)).and_then(|p2| p2.next_table(0xdeadbeaf)).and_then(|p1| p1.next_table(0xcafebabe));
}
