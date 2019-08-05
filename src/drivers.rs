use alloc::prelude::{Box, Vec};
use pci::{BusDevice, BusManager};
use spin::Mutex;
use crate::memory::paging::PhysicalAddress;
use core::ops;
use crate::memory::PAGE_SIZE;
use x86::io::*;

/*fn read_pointer(p: &u32) -> u8{
    let q = p as *const u32;
    let r = q as u64;
    unsafe {
        *((r) as *const u8)
    }
}

fn write_pointer(p: &mut u32, value: u8){
    let q = p as *mut u32;
    let r = q as u64;
    unsafe{
        *(r as *mut u8).offset(0) = value;
    }
}
*/

#[derive(Copy, Clone)]
/// IO range binding
pub enum IOBinding
{
	/// Memory-mapped IO space (Base and offset)
	Memory(u64, u64),
	/// x86 IO bus (Base and offset)
	IO(u16,u16),
}

impl IOBinding
{
	/// Returns the x86 IO space base
	#[inline]
	pub fn io_base(&self) -> u16 {
		match *self{
			IOBinding::IO(base, _size) => base,
			IOBinding::Memory(base, _size) => base as u16,
		}
	}
	
	/// Read a single u8 from the binding
	#[inline]
	pub unsafe fn read_8(&self, ofs: u64) -> u8
	{
		//log_trace!("read_8({:?}, {:#x})", self, ofs);
		match *self{
			IOBinding::IO(base, s) => {
				assert!( ofs+1 <= s.into(), "read_u8(IO addr {:#x}+1 > {:#x})", ofs, s );
				inb(base + ofs as u16)
			},
			IOBinding::Memory(base, _offset) => {
				let q = (base + ofs) as *const u32;
				let r = q as u64;
				unsafe {
					*((r) as *const u8)
				}
			}
		}
	}
	/// Read a single u16 from the binding
	#[inline]
	pub unsafe fn read_16(&self, ofs: u64) -> u16
	{
		//log_trace!("read_16({:?}, {:#x})", self, ofs);
		match *self{
			IOBinding::IO(base, s) => {
				assert!( ofs+2 <= s.into(), "read_u16(IO addr {:#x}+2 > {:#x})", ofs, s );
				inw(base + ofs as u16)
			},
			IOBinding::Memory(base, _offset) => {
				let q = (base + ofs) as *const u32;
				let r = q as u64;
				unsafe {
					*((r) as *const u16)
				}
			}
		}
	}
	/// Read a single u32 from the binding
	#[inline]
	pub unsafe fn read_32(&self, ofs: u64) -> u32
	{
		match *self{
			IOBinding::IO(base, s) => {
				assert!( ofs+4 <= s.into(), "read_u32(IO addr {:#x}+4 > {:#x})", ofs, s );
				inl(base + ofs as u16)
			},
			IOBinding::Memory(base, _offset) => {
				//println!("read_32({:?}, {:#x}", self, ofs);
				let q = (base + ofs) as *const u32;
				let r = q as u64;
				unsafe {
					*((r) as *const u32)
				}
			},
		}
	}
	/// Writes a single u8 to the binding
	#[inline]
	pub unsafe fn write_8(&self, ofs: u64, val: u8)
	{
		//log_trace!("write_8({:?}, {:#x}, {:#02x})", self, ofs, val);
		match *self{
			IOBinding::IO(base, s) => {
				assert!( ofs+1 <= s.into(), "write_8(IO addr {:#x}+1 > {:#x})", ofs, s );
				outb(base + ofs as u16, val);
			},
			IOBinding::Memory(base, _offset) => {
				let q = (base + ofs) as *mut u32;
				let r = q as u64;
				unsafe{
					*(r as *mut u8).offset(0) = val;
				}
			},
		}
	}
	/// Write a single u32 to the binding
	#[inline]
	pub unsafe fn write_16(&self, ofs: u64, val: u16)
	{
		//log_trace!("write_16({:?}, {:#x}, {:#02x})", self, ofs, val);
		match *self{
			IOBinding::IO(base, s) => {
				assert!(ofs+2 <= s.into(), "write_16(IO addr {:#x}+4 > {:#x})", ofs, s);
				outw(base + ofs as u16, val);
			},
			IOBinding::Memory(base, _offset) => {
				let q = (base + ofs) as *mut u32;
				let r = q as u64;
				unsafe{
					*(r as *mut u16).offset(0) = val;
				}
			},
		}
	}
	/// Write a single u32 to the binding
	#[inline]
	pub unsafe fn write_32(&self, ofs: u64, val: u32)
	{
		//log_trace!("write_32({:?}, {:#x}, {:#02x})", self, ofs, val);
		match *self{
			IOBinding::IO(base, s) => {
				assert!(ofs+4 <= s.into(), "write_32(IO addr {:#x}+4 > {:#x})", ofs, s);
				outl(base + ofs as u16, val);
			},
			IOBinding::Memory(base, _offset) => {
				let q = (base + ofs) as *mut u32;
				let r = q as u64;
				unsafe{
					*(r as *mut u32).offset(0) = val;
				}
			},
		}
	}
}

impl ::core::fmt::Debug for IOBinding
{
	fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
		match *self
		{
			IOBinding::IO(b, s) => write!(f, "IO({:#x}+{:#x})", b, s),
			IOBinding::Memory(b, s) => write!(f, "Memory({:#x}+{:#x})", b, s),
		}
	}
}

/// Internal representation of a device on a bus
pub struct Device
{
	pub bus_dev: Box<BusDevice>,
	pub driver: Option<(Box<DriverInstance>, DriverHandleLevel)>,
	//attribs: Vec<u32>,
}

/// Error type for `Driver::bind`
#[derive(Debug)]
pub enum DriverBindError
{
	OutOfMemory,
	Bug(&'static str),
}

/// Internal representation of a bus
pub struct Bus
{
	pub manager: &'static BusManager,
	pub devices: Vec<Device>,
}

pub type DriverHandleLevel = u32;

/// List of registered busses on the system
#[allow(non_upper_case_globals)]
pub static s_root_busses: Mutex<Vec<Bus>> = Mutex::new(Vec::new());

/// List of registered drivers
#[allow(non_upper_case_globals)]
pub static s_driver_list: Mutex<Vec<&'static Driver>> = Mutex::new(Vec::new());

/// Driver instance (maps directly to a device)
pub trait DriverInstance: Send {
}

/// Abstract driver for a device (creates instances when passed a device)
pub trait Driver:
	Send + Sync
{
	/// Driver's name
	fn name(&self) -> &str;
	/// Bus type the driver binds against (matches value from `BusManager::bus_type`)
	fn bus_type(&self) -> &str;
	/// Return the handling level of this driver for the specified device
	fn handles(&self, bus_dev: &BusDevice) -> DriverHandleLevel;
	/// Requests that the driver bind itself to the specified device
	fn bind(&'static self, bus_dev: &mut BusDevice) -> Box<DriverInstance>;
}

/// Register a bus with the device manager
///
/// Creates a new internal representation of the bus, containg the passed set of devices.
pub fn register_bus(manager: &'static BusManager, devices: Vec<Box<BusDevice>>) //-> BusHandle
{
	let bus = Bus {
		manager: manager,
		// For each device, locate a driver
		devices: devices.into_iter().map(|mut d| Device {
			driver: find_driver(manager, &mut *d),
			//attribs: Vec::new(),
			bus_dev: d,
			}).collect(),
		};
	let mut bus_list_lh = s_root_busses.lock();
	bus_list_lh.push(bus);
	//let ptr: *const _ = bus_list_lh.last().unwrap();
	//BusHandle(ptr)
}

/// Registers a driver with the device manger
pub fn register_driver(driver: &'static Driver)
{
	s_driver_list.lock().push(driver);
	println!("\nRegistering driver {}", driver.name());
	// Iterate known devices and spin up instances if needed
	for bus in s_root_busses.lock().iter_mut()
	{
		println!("bus type {}", bus.manager.bus_type());
		if driver.bus_type() == bus.manager.bus_type()
		{
			for dev in bus.devices.iter_mut()
			{
				let rank = driver.handles(&*dev.bus_dev);
				if rank == 0
				{
					// SKIP!
				}
				else if dev.driver.is_some()
				{
					let bind = dev.driver.as_ref().unwrap();
					let cur_rank = bind.1;
					if cur_rank > rank
					{
						// Existing driver is better
					}
					else if cur_rank == rank
					{
						// Fight!
					}
					else
					{
						// New driver is better
						panic!("TODO: Unbind driver and bind in new one");
					}
				}
				else
				{
					// Bind new driver
					dev.driver = Some( (driver.bind(&mut *dev.bus_dev), rank) );
				}
			}
		}
	}
} 

/**
 * Locate the best registered driver for this device and instanciate it
 */
fn find_driver(bus: &BusManager, bus_dev: &mut BusDevice) -> Option<(Box<DriverInstance>,DriverHandleLevel)>
{
	//println!("Finding driver for {}:{:x}", bus.bus_type(), bus_dev.addr());
	let mut best_ranking = 0;
	let mut best_driver = None;
	for driver in s_driver_list.lock().iter()
	{
		if bus.bus_type() == driver.bus_type()
		{
			let ranking = driver.handles(bus_dev);
			if ranking == 0
			{
				// Doesn't handle this device
			}
			else if ranking > best_ranking
			{
				// Best so far
				best_driver = Some( *driver );
				best_ranking = ranking;
			}
			else if ranking == best_ranking
			{
				// A tie, this is not very good
				//log_warning!("Tie for device {}:{:x} between {} and {}",
				//	bus.bus_type(), bus_dev.addr(), driver, best_driver.unwrap());
			}
			else
			{
				// Not as good as current, move along
			}
		}
	}
	best_driver.map(|d| (d.bind(bus_dev), best_ranking))
}
