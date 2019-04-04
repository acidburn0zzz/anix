use core::ops::{Index, IndexMut};
use core::marker::PhantomData;
use memory::paging::*;
use memory::paging::ENTRY_COUNT;
use memory::paging::EntryFlags;
use memory::FrameAllocator;
use core::ptr::Unique;
use memory::Frame;
use memory::PAGE_SIZE;

pub struct Table<L: TableLevel> {
    entries: [Entry; ENTRY_COUNT],
    level: PhantomData<L>,
}

pub struct ActivePageTable {
    p4: Unique<Table<Level4>>,
}

pub const P4: *mut Table<Level4> = 0xffffffff_fffff000 as *mut Table<Level4>;

impl ActivePageTable {
    pub unsafe fn new() -> ActivePageTable {
        ActivePageTable {
            p4: Unique::new_unchecked(P4),
        }
    }
    
    pub fn p4(&self) -> &Table<Level4> {
		unsafe { self.p4.as_ref() }
	}

	pub fn p4_mut(&mut self) -> &mut Table<Level4> {
		unsafe { self.p4.as_mut() }
	}
	
	pub fn translate(&self, virtual_address: VirtualAddress) -> Option<PhysicalAddress>{
		let offset = virtual_address % PAGE_SIZE;
		self.translate_page(Page::containing_address(virtual_address)).map(|frame| frame.number * PAGE_SIZE + offset)
	}

	fn translate_page(&self, page: Page) -> Option<Frame> {
		use super::table::P4;
		let p3 = unsafe { &*P4 }.next_table(page.p4_index());

		let huge_page = || {
			p3.and_then(|p3| {
				let p3_entry = &p3[page.p3_index()];
				// 1GiB page?
				if let Some(start_frame) = p3_entry.pointed_frame() {
					if p3_entry.flags().contains(EntryFlags::HUGE_PAGE) {
						// address must be 1GiB aligned
						assert!(start_frame.number % (ENTRY_COUNT * ENTRY_COUNT) == 0);
						return Some(Frame {
							number: start_frame.number + page.p2_index() *
									ENTRY_COUNT + page.p1_index(),
						});
					}
				}
				if let Some(p2) = p3.next_table(page.p3_index()) {
					let p2_entry = &p2[page.p2_index()];
					// 2MiB page?
					if let Some(start_frame) = p2_entry.pointed_frame() {
						if p2_entry.flags().contains(EntryFlags::HUGE_PAGE) {
							// address must be 2MiB aligned
							assert!(start_frame.number % ENTRY_COUNT == 0);
							return Some(Frame {
								number: start_frame.number + page.p1_index()
							});
						}
					}
				}
				None
			})
		};

		p3.and_then(|p3| p3.next_table(page.p3_index())).and_then(|p2| p2.next_table(page.p2_index())).and_then(|p1| p1[page.p1_index()].pointed_frame()).or_else(huge_page)
	}
	
	pub fn map_to<A>(&mut self, page: Page, frame: Frame, flags: EntryFlags, allocator: &mut A) where A: FrameAllocator{
		let mut p3 = self.p4_mut().next_table_create(page.p4_index(), allocator);
		let mut p2 = p3.next_table_create(page.p3_index(), allocator);
		let mut p1 = p2.next_table_create(page.p2_index(), allocator);

		assert!(p1[page.p1_index()].is_unused());
		p1[page.p1_index()].set(frame, flags | EntryFlags::PRESENT);
	}
	
	pub fn map<A>(&mut self, page: Page, flags: EntryFlags, allocator: &mut A) where A: FrameAllocator{
		let frame = allocator.allocate_frame().expect("out of memory");
		self.map_to(page, frame, flags, allocator)
	}
	
	pub fn identity_map<A>(&mut self, frame: Frame, flags: EntryFlags, allocator: &mut A) where A: FrameAllocator
	{
		let page = Page::containing_address(frame.start_address());
		self.map_to(page, frame, flags, allocator)
	}
	
	fn unmap<A>(&mut self, page: Page, allocator: &mut A) where A: FrameAllocator
	{
		assert!(self.translate(page.start_address()).is_some());

		let p1 = self.p4_mut()
					.next_table_mut(page.p4_index())
					.and_then(|p3| p3.next_table_mut(page.p3_index()))
					.and_then(|p2| p2.next_table_mut(page.p2_index()))
					.expect("mapping code does not support huge pages");
		let frame = p1[page.p1_index()].pointed_frame().unwrap();
		p1[page.p1_index()].set_unused();
		
		use x86_64::instructions::tlb;
		use x86_64::VirtAddr;
		tlb::flush(VirtAddr::new(page.start_address() as u64));
    
		// TODO free p(1,2,3) table if empty
		//allocator.deallocate_frame(frame);
	}
}


impl<L> Table<L> where L: TableLevel{
	///Set all entries to zero
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
			self.entries[index].set(frame, EntryFlags::PRESENT | EntryFlags::WRITABLE);
			self.next_table_mut(index).unwrap().zero();
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
