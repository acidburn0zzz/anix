use multiboot2::{MemoryAreaIter, MemoryArea};
pub mod paging;
pub mod table;
pub mod heap;
use multiboot2::BootInformation;
use self::paging::{EntryFlags, Page, InactivePageTable, PhysicalAddress};
use self::paging::temporary_page::TemporaryPage;
use self::table::ActivePageTable;

pub const PAGE_SIZE: usize = 4096;

struct FrameIter {
	start: Frame,
	end: Frame,
}

impl Iterator for FrameIter {
	type Item = Frame;

	fn next(&mut self) -> Option<Frame> {
		if self.start <= self.end {
			let frame = self.start.clone();
			self.start.number += 1;
			Some(frame)
		} else {
			None
			}
	}
}
 
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {
    pub fn containing_address(address: usize) -> Frame {
        Frame{ number: address / PAGE_SIZE }
    }
    fn start_address(&self) -> PhysicalAddress {
		self.number * PAGE_SIZE
	}
	fn clone(&self) -> Frame {
        Frame { number: self.number }
    }
    fn range_inclusive(start: Frame, end: Frame) -> FrameIter {
        FrameIter {
            start: start,
            end: end,
        }
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

pub struct AreaFrameAllocator {
    next_free_frame: Frame,
    current_area: Option<&'static MemoryArea>,
    areas: MemoryAreaIter,
    kernel_start: Frame,
    kernel_end: Frame,
    multiboot_start: Frame,
    multiboot_end: Frame,
}

impl AreaFrameAllocator{
	fn choose_next_area(&mut self) {
		self.current_area = self.areas.clone().filter(|area| {
			let address = area.base_addr + area.length - 1;
			Frame::containing_address(address as usize) >= self.next_free_frame
		}).min_by_key(|area| area.base_addr);

		if let Some(area) = self.current_area {
			let start_frame = Frame::containing_address(area.base_addr as usize);
			if self.next_free_frame < start_frame {
				self.next_free_frame = start_frame;
			}
		}
	}
	
	pub fn new(kernel_start: usize, kernel_end: usize,
      multiboot_start: usize, multiboot_end: usize,
      memory_areas: MemoryAreaIter) -> AreaFrameAllocator
	{
		let mut allocator = AreaFrameAllocator {
			next_free_frame: Frame::containing_address(0),
			current_area: None,
			areas: memory_areas,
			kernel_start: Frame::containing_address(kernel_start),
			kernel_end: Frame::containing_address(kernel_end),
			multiboot_start: Frame::containing_address(multiboot_start),
			multiboot_end: Frame::containing_address(multiboot_end),
		};
		allocator.choose_next_area();
		allocator
	}

}

impl FrameAllocator for AreaFrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame> {
        if let Some(area) = self.current_area {
			// "Clone" the frame to return it if it's free. Frame doesn't
			// implement Clone, but we can construct an identical frame.
			let frame = Frame{ number: self.next_free_frame.number };

			// the last frame of the current area
			let current_area_last_frame = {
				let address = area.base_addr + area.length - 1;
				Frame::containing_address(address as usize)
			};

			if frame > current_area_last_frame {
				// all frames of current area are used, switch to next area
				self.choose_next_area();
			} else if frame >= self.kernel_start && frame <= self.kernel_end {
				// `frame` is used by the kernel
				self.next_free_frame = Frame {
					number: self.kernel_end.number + 1
				};
			} else if frame >= self.multiboot_start && frame <= self.multiboot_end {
				// `frame` is used by the multiboot information structure
				self.next_free_frame = Frame {
					number: self.multiboot_end.number + 1
				};
			} else {
				// frame is unused, increment `next_free_frame` and return it
				self.next_free_frame.number += 1;
				return Some(frame);
			}
			// `frame` was not valid, try it again with the updated `next_free_frame`
			self.allocate_frame()
		} else {
			None // no free frames left
		}
    }

    fn deallocate_frame(&mut self, frame: Frame) {
        unimplemented!()
    }
}

pub fn init(boot_info: &BootInformation) {
	assert_has_not_been_called!("memory::init must be called only once");
	
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");
    let elf_sections_tag = boot_info.elf_sections_tag().expect("Elf sections tag required");

    let kernel_start = elf_sections_tag.sections().filter(|s| s.is_allocated()).map(|s| s.addr).min().unwrap();
    let kernel_end = elf_sections_tag.sections().filter(|s| s.is_allocated()).map(|s| s.addr + s.size).max().unwrap();

    println!("\nKernel start: {:#x}, Kernel end: {:#x}", kernel_start, kernel_end);
    println!("\nMultiboot start: {:#x}, Multiboot end: {:#x}", boot_info.start_address(), boot_info.end_address());

    let mut frame_allocator = AreaFrameAllocator::new(
        kernel_start as usize, kernel_end as usize,
        boot_info.start_address(), boot_info.end_address(),
        memory_map_tag.memory_areas());

    let mut active_table = remap_the_kernel(&mut frame_allocator, boot_info);
    
    use self::paging::Page;
    use {HEAP_START, HEAP_SIZE};

    let heap_start_page = Page::containing_address(HEAP_START);
    let heap_end_page = Page::containing_address(HEAP_START + HEAP_SIZE-1);

    for page in Page::range_inclusive(heap_start_page, heap_end_page) {
        active_table.map(page, EntryFlags::WRITABLE, &mut frame_allocator);
    }
}

pub fn remap_the_kernel<A>(allocator: &mut A, boot_info: &BootInformation) -> ActivePageTable
    where A: FrameAllocator
{
   let mut temporary_page = TemporaryPage::new(Page { number: 0xcafebabe },
        allocator);

    let mut active_table = unsafe { ActivePageTable::new() };
    let mut new_table = {
        let frame = allocator.allocate_frame().expect("no more frames");
        InactivePageTable::new(frame, &mut active_table, &mut temporary_page)
    };

    active_table.with(&mut new_table, &mut temporary_page, |mapper| {
        let elf_sections_tag = boot_info.elf_sections_tag()
            .expect("Memory map tag required");
		
		//Map allocated kernel sections
		for section in elf_sections_tag.sections() {

			if !section.is_allocated() {
				// section is not loaded to memory
				continue;
			}
			
			println!("\nMapping section at addr: {:#x}, size: {:#x}, flags: {:#?}", section.addr, section.size, section.flags());
			
			if section.start_address() % PAGE_SIZE != 0{
				print!("\nSections need to be page aligned!!");
			}

			let flags = EntryFlags::from_elf_section_flags(section);

			let start_frame = Frame::containing_address(section.start_address());
			let end_frame = Frame::containing_address(section.end_address() - 1);
			for frame in Frame::range_inclusive(start_frame, end_frame) {
				mapper.identity_map(frame, flags, allocator);
			}
		}
		
		//Map VGA Text Buffer
		let vga_buffer_frame = Frame::containing_address(0xb8000); // new
		mapper.identity_map(vga_buffer_frame, EntryFlags::WRITABLE, allocator); // new
		
		//Map multiboot structure
		let multiboot_start = Frame::containing_address(boot_info.start_address());
		let multiboot_end = Frame::containing_address(boot_info.end_address() - 1);
		for frame in Frame::range_inclusive(multiboot_start, multiboot_end) {
			mapper.identity_map(frame, EntryFlags::PRESENT, allocator);
		}
    });
    
    let old_table = active_table.switch(new_table);
    
    // turn the old p4 page into a guard page
    let old_p4_page = Page::containing_address(
      old_table.p4_frame.start_address()
    );
    active_table.unmap(old_p4_page, allocator);
    println!("\nGuard page at {:#x}", old_p4_page.start_address());
    
    active_table
}
