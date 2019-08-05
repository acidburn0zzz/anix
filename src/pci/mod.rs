use alloc::prelude::Box;
use alloc::prelude::Vec;
use x86::current::paging::PAddr;
use x86::io::*;
use spin::Mutex;
use drivers::DriverInstance;
use alloc::prelude::String;
use crate::drivers::IOBinding;
use drivers::*;

static S_PCI_LOCK: Mutex<PCICfgSpace> = Mutex::new(PCICfgSpace);

struct PCICfgSpace;
impl PCICfgSpace
{
	fn read(&mut self, addr: u32) -> u32 {
		// SAFE: (from accessing the wrong place)
		unsafe {
			outl(0xCF8, 0x80000000 | addr);
			inl(0xCFC)
		}
	}
	fn write(&mut self, addr: u32, val: u32) {
		// SAFE: (from accessing the wrong place)
		unsafe {
			outl(0xCF8, 0x80000000 | addr);
			outl(0xCFC, val)
		}
	}
}

/// Read a word from a pre-calculated PCI address
pub fn read(addr: u32) -> u32
{
	S_PCI_LOCK.lock().read(addr)
}

/// Write a word to a pre-calculated PCI address
pub fn write(addr: u32, val: u32)
{
	S_PCI_LOCK.lock().write(addr, val);
}

/// Attrbute on a bus device
#[derive(Debug)]
pub enum AttrValue<'a>
{
	/// Invalid attribute name
	None,
	/// 32-bit integer
	U32(u32),
	/// String value
	String(&'a str),
}
impl<'a> AttrValue<'a> {
	pub fn unwrap_u32(self) -> u32 {
		if let AttrValue::U32(v) = self {
			v
		}
		else {
			panic!("AttrValue::unwrap_u32 - {:?}", self);
		}
	}
	pub fn unwrap_str(self) -> &'a str {
		if let AttrValue::String(v) = self {
			v
		}
		else {
			panic!("AttrValue::unwrap_str - {:?}", self);
		}
	}
}

/// Interface to a device on a bus
pub trait BusDevice:
	Send
{
	/// Returns the device's address on the parent bus
	fn addr(&self) -> u32;
	/// Returns the specified attribute (or 0, if invalid)
	fn get_attr(&self, name: &str) -> AttrValue {
		self.get_attr_idx(name, 0)
	}
	fn get_attr_idx(&self, name: &str, idx: usize) -> AttrValue;
	/// Set the specified attribute
	fn set_attr(&mut self, name: &str, value: AttrValue) {
		self.set_attr_idx(name, 0, value)
	}
	fn set_attr_idx(&mut self, name: &str, idx: usize, value: AttrValue);
	/// Set the power state of this device
	fn set_power(&mut self, state: bool);	// TODO: Power state enum for Off,Standby,Low,On
	/// Bind to the specified IO block (meaning of `block_id` depends on the bus)
	fn base_slice(&mut self, block_id: usize) -> u32 {
		self.base(block_id, None)
	}
	fn base(&mut self, block_id: usize, slice: Option<(usize,usize)>) -> u32;
	/// Obtain the specified interrupt vector
	fn get_irq(&mut self, idx: usize) -> u32;
}

/// Interface a bus manager instance
pub trait BusManager:
	Send + Sync
{
	/// Returns the textual name of the bus type (e.g. "pci")
	fn bus_type(&self) -> &str;
	/// Returns a list of valid attributes for BusDevice::get_attr
	fn get_attr_names(&self) -> &[&str];
}

const MAX_FUNC: u8 = 8;	// Address restriction
const MAX_DEV: u8 = 32;	// Address restriction
const CONFIG_WORD_IDENT: u8 = 0;
const CONFIG_WORD_CLASS: u8 = 2;

struct PCIDev
{
	addr: u16,	// Bus,Slot,Fcn
	vendor: u16,
	device: u16,
	class: u32,

	// TODO: Include bound status, and BAR mappings
	config: [u32; 16],
}

#[derive(Debug)]
enum BAR
{
	None,
	IO(u16, u16),	// base, size
	Mem(u64,u32,bool),	// Base, size, prefetchable
}

struct PCIBusManager;
struct PCIChildBusDriver;

#[allow(non_upper_case_globals)]
static s_pci_bus_manager: PCIBusManager = PCIBusManager;
#[allow(non_upper_case_globals)]
static s_pci_child_bus_driver: PCIChildBusDriver = PCIChildBusDriver;
static S_ATTR_NAMES: [&'static str; 3] = ["vendor", "device", "class"];

//TODO: Function to print all devices but not in init
pub fn init()
{
	crate::drivers::register_driver(&s_pci_child_bus_driver);

	// 1. Enumerate PCI bus(es)
	let devs = scan_bus(0);
	
	crate::drivers::register_bus(&s_pci_bus_manager, devs);
	// - All drivers that have PCI bindings should be waiting on this to load
}

impl BusManager for PCIBusManager
{
	fn bus_type(&self) -> &str { "pci" }
	fn get_attr_names(&self) -> &[&str]
	{
		&S_ATTR_NAMES
	}
}

impl crate::drivers::Driver for PCIChildBusDriver
{
	fn name(&self) -> &str {
		"bus-pci"
	}
	fn bus_type(&self) -> &str {
		"pci"
	}
	fn handles(&self, bus_dev: &BusDevice) -> u32
	{
		let addr = bus_dev.addr() as u16;
		let bridge_type = (read_word(addr, 3) >> 16) & 0x7F;
		// 0x00 == Normal device, 0x01 = PCI-PCI Bridge
		// -> There should only be one PCI bridge handler, but bind low just in case
		if bridge_type == 0x01 { 1 } else { 0 }
	}
	fn bind(&'static self, bus_dev: &mut BusDevice) -> Box<crate::drivers::DriverInstance>
	{
		let addr = bus_dev.addr() as u16;
		let bridge_type = (read_word(addr, 3) >> 16) & 0x7F;
		assert!(bridge_type == 0x01);
		// Get sub-bus number
		let sec_bus_id = (read_word(addr, 6) >> 8) & 0xFF;
		//println!("PCI Bridge Bind: sec_bus_id = {:#02x}", sec_bus_id);

		Box::new(self)
	}
}

impl DriverInstance for &PCIChildBusDriver{}

impl BusDevice for PCIDev
{
	fn addr(&self) -> u32 {
		self.addr as u32
	}
	fn get_attr_idx(&self, name: &str, idx: usize) -> AttrValue {
		match name
		{
		"vendor" => AttrValue::U32(self.vendor as u32),
		"device" => AttrValue::U32(self.device as u32),
		"class" => AttrValue::U32(self.class),
		"bus_master" => AttrValue::U32(if self.config[1] & 4 == 0 { 0 } else { 1 }),
		"raw_config" => {
			if idx >= 256 || idx % 4 != 0 {
				AttrValue::None
			}
			else {
				AttrValue::U32(read_word(self.addr, idx as u8 / 4))
			}
			},
		_ => {
			println!("Request for non-existant attr '{}' on device 0x{:05x}", name, self.addr);
			AttrValue::None
			},
		}
	}
	fn set_attr_idx(&mut self, name: &str, _idx: usize, value: AttrValue) {
		match (name,value)
		{
		("vendor", _)|
		("device", _)|
		("class", _) => {
			println!("Attempting to set read-only attr '{}' on device {:#05x}", name, self.addr);
			},
		// Enable/Disable PCI bus-mastering support
		("bus_master", AttrValue::U32(value)) => {
			if value != 0 {
				self.config[1] |= 4;
			}
			else {
				self.config[1] &= !4;
			}
			write_word(self.addr, 1, self.config[1]);
			},
		_ => {
			println!("Attempting to set non-existant attr '{}' on device 0x{:05x}", name, self.addr);
			},
		}
	}
	fn set_power(&mut self, state: bool)
	{
		// Nope
	}
	fn base(&mut self, block_id: usize, slice: Option<(usize,usize)>) -> u32{
		if block_id > 6 {
			panic!("PCI bind_io - block_id out of range (max 5, got {})", block_id);
		}
		if block_id % 1 == 1 {
			if self.config[4+block_id-1] & 7 == 4 {
				// Accessing the second word of a 64-bit BAR, this is an error.
				panic!("PCI bind_io - Requesting second word of a 64-bit BAR");
			}
		}

		match parse_bar(self.addr, 4 + block_id as u8){
			BAR::None => {
				println!("ERROR: PCI bind_io - Request for BAR{} of {:#x} which isn't populated", block_id, self.addr);
				0
			},
			BAR::IO(base, size) => {
				if let Some(slice) = slice {
					if slice.0 >= size as usize || slice.1 + slice.0 > size as usize {
						println!("slice.0 >= s as usize || slice.1 + slice.0 > s as usize: slice.0={}, slice.1={}, s={}", slice.0, slice.1, size);
						0
					}
					else {
						println!("IOBinding::IO(b + slice.0 as u16, slice.1 as u16)={:#?}", IOBinding::IO(base + slice.0 as u16, slice.1 as u16));
						(base + slice.0 as u16) as u32
					}
				}
				else {
					println!("IOBinding::IO(b,s)={:#?}", IOBinding::IO(base, size));
					base as u32
				}
			},

			BAR::Mem(base, size, prefetchable) => {
				base as u32
			}
		}
	}
	fn get_irq(&mut self, idx: usize) -> u32
	{
		if idx == 0
		{
			self.config[0x3C / 4] & 0xFF
		}
		else
		{
			0
		}
	}
}

fn scan_bus(bus_id: u8) -> Vec<Box<BusDevice+'static>>
{
	println!("\nPCI scan_bus({})", bus_id);
	let mut ret: Vec<Box<BusDevice>> = Vec::new();
	for devidx in 0 .. MAX_DEV
	{
		match get_device(bus_id, devidx, 0)
		{
		Some(devinfo) => {
			let is_multifunc = (devinfo.config[3] & 0x0080_0000) != 0;
			let class = devinfo.class;

			// Increase device count
			ret.push(box devinfo);

			// Handle multi-function devices (iterate from 1 onwards)
			if is_multifunc
			{
				for fcnidx in 1 .. MAX_FUNC
				{
					if let Some(devinfo) = get_device(bus_id, devidx, fcnidx) {
						ret.push(box devinfo);
					}
				}
			}
			},
		None => {
			// Move along, nothing to see here
			},
		}
	}
	ret
}

pub fn list_devices() {
	use core::ops::Deref;
	print!("\n");
	
	for device in &s_root_busses.lock().deref()[0].devices {
		println!("{} {}", device.bus_dev.addr(), get_device_type(device.bus_dev.get_attr_idx("class", 0).unwrap_u32()));
	}
}

fn get_device_type(class: u32) -> String{
	use alloc::prelude::ToString;

	let mut r#type = format!("{:#x}", class);
	if r#type.ends_with("0003"){
		//Delete the 0003 at the end of the class
		let mut vec: Vec<char> = r#type.chars().collect();
		let len = vec.len();
		vec.remove(len - 1);
		vec.remove(len - 2);
		vec.remove(len - 3);
		vec.remove(len - 4);
		r#type = vec.iter().cloned().collect::<String>();
	}
	else if r#type.ends_with("003"){
		//Delete the 003 at the end of the class
		let mut vec: Vec<char> = r#type.chars().collect();
		let len = vec.len();
		vec.remove(len - 1);
		vec.remove(len - 2);
		vec.remove(len - 3);
		r#type = vec.iter().cloned().collect::<String>();
	}
	else if r#type.ends_with("0103"){
		//Delete the 0103 at the end of the class
		let mut vec: Vec<char> = r#type.chars().collect();
		let len = vec.len();
		vec.remove(len - 1);
		vec.remove(len - 2);
		vec.remove(len - 3);
		vec.remove(len - 4);
		r#type = vec.iter().cloned().collect::<String>();
	}
	else if r#type.ends_with("00007"){
		//Delete the 00007 at the end of the class
		let mut vec: Vec<char> = r#type.chars().collect();
		let len = vec.len();
		vec.remove(len - 1);
		vec.remove(len - 2);
		vec.remove(len - 3);
		vec.remove(len - 4);
		vec.remove(len - 5);
		r#type = vec.iter().cloned().collect::<String>();
	}
	else if r#type.ends_with("0193"){
		//Delete the 00007 at the end of the class
		let mut vec: Vec<char> = r#type.chars().collect();
		let len = vec.len();
		vec.remove(len - 1);
		vec.remove(len - 2);
		vec.remove(len - 3);
		vec.remove(len - 4);
		r#type = vec.iter().cloned().collect::<String>();
	}
	let r#type_str = r#type.as_str();

	match r#type_str{
		//PCI Bridge
		"0x60" => return "PCI Bridge Host".to_string(),
		"0x601" => return "PCI Bridge ISA".to_string(),
		"0x604" => return "PCI Bridge".to_string(),

		//Display
		"0x38" => return "Unknown display".to_string(),
		"0x30" => return "VGA display".to_string(),

		//USB
		"0xc03" => return "Unknown USB".to_string(),
		"0xc032" => return "USB EHCI (USB2)".to_string(),

		//Others
		"0x403" => return "Audio device".to_string(),
		"0xc05" => return "SMBUS".to_string(),
		"0x106" => return "SATA".to_string(),
		_ => return format!("Unknown type: {}", r#type),
	};
}

fn get_device(bus_id: u8, devidx: u8, function: u8) -> Option<PCIDev>
{
	let addr = get_pci_addr(bus_id, devidx, function);
	let idword = read_word(addr, CONFIG_WORD_IDENT);

	if idword & 0xFFFF == 0xFFFF {
		None
	}
	else {
		Some(PCIDev {
			addr: addr,
			vendor: (idword & 0xFFFF) as u16,
			device: (idword >> 16) as u16,
			class: read_word(addr, CONFIG_WORD_CLASS),
			config: [
				idword            , read_word(addr, 1),
				read_word(addr, 2), read_word(addr, 3),
				read_word(addr, 4), read_word(addr, 5),
				read_word(addr, 6), read_word(addr, 7),
				read_word(addr, 8), read_word(addr, 9),
				read_word(addr,10), read_word(addr,11),
				read_word(addr,12), read_word(addr,13),
				read_word(addr,14), read_word(addr,15),
				],
			})
	}
}

fn parse_bar(addr: u16, word: u8) -> BAR
{
	let value = read_word(addr, word);
	//log_trace!("parse_bar({}) value={:#x}", word-4, value);
	if value == 0
	{
		//log_debug!("parse_bar: None");
		BAR::None
	}
	else if value & 1 == 0
	{
		write_word(addr, word, 0xFFFFFFFF);
		let one_value = read_word(addr, word);
		let size = !(one_value & 0xFFFF_FFF0) + 1;
		write_word(addr, word, value);
		//log_debug!("parse_bar: (memory) one_value={:#x}, size={:#x}, value={:#x}", one_value, size, value);
		// memory BAR
		let pf = (value >> 3) & 1;
		let ty = (value >> 1) & 3;
		match ty
		{
		0 => BAR::Mem(value as u64, size, pf == 1),	// 32-bit
		1 => BAR::None,	// reserved
		2 => {	// 64-bit
			assert!(word % 2 == 0);
			let value2 = read_word(addr, word+1);
			write_word(addr, word+1, !0);
			let size2 = !read_word(addr, word+1);	// No +1
			write_word(addr, word+1, value2);
			assert!(size2 == 0, "TODO: Support 64-bit BARs with sizes >4GB - size={},size2={}", size, size2);
			let addr = (value2 as u64) << 32 | (value as u64 & !0xF);
			//log_debug!("parse_bar: (memory 64) addr={:#x} size={:#x}", addr, size);

			BAR::Mem( addr, size, pf == 1 )
			},
		3 => BAR::None,	// reserved
		_ => unreachable!()
		}
	}
	else
	{
		// IO BAR
		write_word(addr, word, 0xFFFF);
		let one_value = read_word(addr, word);
		let size = ( !(one_value & 0xFFFC) + 1 ) & 0xFFFF;
		//log_debug!("parse_bar: (IO) one_value = {:#x}, size={:#x}, value={:#x}", one_value, size, value);
		write_word(addr, word, value);
		BAR::IO( (value & 0xFFFC) as u16, size as u16 )
	}
}

fn get_pci_addr(bus_id: u8, dev: u8, fcn: u8) -> u16
{
	assert!(dev < MAX_DEV);
	assert!(fcn < MAX_FUNC);
	((bus_id as u16) << 8) | ((dev as u16) << 3) | (fcn as u16)
}

pub fn read_word(bus_addr: u16, wordidx: u8) -> u32
{
	let addr = ((bus_addr as u32) << 8) | ((wordidx as u32) << 2);
	//println!("read_word(bus_addr={:x},idx={}) addr={:#x}", bus_addr, wordidx, addr);
	read(addr)
}
pub fn write_word(bus_addr: u16, wordidx: u8, value: u32)
{
	let addr = ((bus_addr as u32) << 8) | ((wordidx as u32) << 2);
	//println!("read_word(bus_addr={:x},idx={}) addr={:#x}", bus_addr, wordidx, addr);
	write(addr, value)
}


impl ::core::fmt::Debug for PCIDev
{
	fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::result::Result<(),::core::fmt::Error>
	{
		write!(f, "{:#x} Ven:{:#x} Dev:{:#x} Class {:#x}", self.addr, self.vendor, self.device, self.class)
	}
}
