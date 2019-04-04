use super::PAGE_SIZE;
use super::Frame;
use bitflags::bitflags;

pub const ENTRY_COUNT: usize = 512;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

bitflags! {
    pub struct EntryFlags: u64 {
        const PRESENT =         1 << 0;
        const WRITABLE =        1 << 1;
        const USER_ACCESSIBLE = 1 << 2;
        const WRITE_THROUGH =   1 << 3;
        const NO_CACHE =        1 << 4;
        const ACCESSED =        1 << 5;
        const DIRTY =           1 << 6;
        const HUGE_PAGE =       1 << 7;
        const GLOBAL =          1 << 8;
        const NO_EXECUTE =      1 << 63;
    }
}

pub struct Page {
   number: usize,
}

pub struct Entry(u64);

impl Entry {
    pub fn is_unused(&self) -> bool {
        self.0 == 0
    }

    pub fn set_unused(&mut self) {
        self.0 = 0;
    }
    
    pub fn flags(&self) -> EntryFlags {
		EntryFlags::from_bits_truncate(self.0)
	}

	pub fn pointed_frame(&self) -> Option<Frame> {
		if self.flags().contains(EntryFlags::PRESENT) {
			Some(Frame::containing_address(
				self.0 as usize & 0x000fffff_fffff000
			))
		} else {
			None
		}
	}

	pub fn set(&mut self, frame: Frame, flags: EntryFlags) {
		assert!(frame.start_address() & !0x000fffff_fffff000 == 0);
		self.0 = (frame.start_address() as u64) | flags.bits();
	}
}

impl Page{
	pub fn containing_address(address: VirtualAddress) -> Self {
		assert!(address < 0x0000_8000_0000_0000 || address >= 0xffff_8000_0000_0000, "invalid address: 0x{:x}", address);
		Self { number: address / PAGE_SIZE }
	}
	
	pub fn start_address(&self) -> usize {
		self.number * PAGE_SIZE
	}
	
	pub fn p4_index(&self) -> usize {
		(self.number >> 27) & 0o777
	}
	pub fn p3_index(&self) -> usize {
		(self.number >> 18) & 0o777
	}
	pub fn p2_index(&self) -> usize {
		(self.number >> 9) & 0o777
	}
	pub fn p1_index(&self) -> usize {
		(self.number >> 0) & 0o777
	}
}
