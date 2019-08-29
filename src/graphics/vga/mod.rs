use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;
use crate::drivers::*;
use pci::BusDevice;
use alloc::boxed::Box;
use x86::io::*;

struct VgaDevice {}

/// Driver object for the PCI
struct VgaPciDriver;

static S_VGA_PCI_DRIVER: VgaPciDriver = VgaPciDriver;

pub fn init() {
    // 1. Register Driver
    register_driver(&S_VGA_PCI_DRIVER);
}

impl Driver for VgaPciDriver {
    fn name(&self) -> &str {
        "vga"
    }
    fn bus_type(&self) -> &str {
        "pci"
    }
    fn handles(&self, bus_dev: &dyn BusDevice) -> u32
    {
        let classcode = bus_dev.get_attr("class").unwrap_u32();
        // [class] [subclass] [IF] [ver]
        if classcode & 0xFFFF_FF00 == 0x0300_0000 {
            1    // Handle as weakly as possible (vendor-provided drivers bind higher)
        }
        else {
            0
        }
    }
    fn bind(&self, _bus_dev: &mut dyn BusDevice) -> Box<dyn DriverInstance+'static> {
        box VgaDevice::new()
    }

}

impl VgaDevice {
    pub fn new() -> VgaDevice {
        unsafe {
			Self::test();
		}
        Self{}
    }
    pub unsafe fn test() {
		
    }
}

impl DriverInstance for VgaDevice {
}
