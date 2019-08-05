use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;
use crate::drivers::*;
use pci::BusDevice;
use alloc::boxed::Box;

lazy_static! {
    /// A global `Writer` instance that can be used for printing to the VGA text buffer.
    ///
    /// Used by the `print!` and `println!` macros.
    pub static ref DRAWER: Mutex<Drawer> = Mutex::new(Drawer {
        buffer: unsafe { &mut *(0xa0000 as *mut Buffer) },
    });
}

/// The standard color palette in VGA text mode.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

/// A screen character in the VGA frame buffer, consisting of a `ColorCode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    color_code: Color,
}

struct VgaDevice {}

pub const BUFFER_HEIGHT: usize = 240;

pub const BUFFER_WIDTH: usize = 320;

/// A structure representing the VGA frame buffer.
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// Driver object for the PCI
struct VgaPciDriver;

pub struct Drawer {
    buffer: &'static mut Buffer,
}

impl Drawer {
    pub fn pixel(&mut self, row: usize, col: usize, color: Color) {
        if col >= BUFFER_WIDTH || row >= BUFFER_HEIGHT{
            println!("Not in screen!!!"); // FIXME: When Anix will use only frame buffer, change println! by panic!
        }

        self.buffer.chars[row][col].write(ScreenChar {
            color_code: color,
        });
    }
}


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
    fn handles(&self, bus_dev: &BusDevice) -> u32
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
    fn bind(&self, bus_dev: &mut BusDevice) -> Box<DriverInstance+'static> {
        box VgaDevice::new()
    }

}

impl VgaDevice {
    pub fn new() -> VgaDevice {
        Self::test();
        Self{}
    }
    pub fn test() {
        for w in 0..10 {
            for h in 0..10 {
                DRAWER.lock().pixel(w, h, Color::Blue);
            }
        }
    }
}

impl DriverInstance for VgaDevice {
}
