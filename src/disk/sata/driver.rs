use crate::drivers::*;
use alloc::boxed::Box;
use alloc::vec::Vec;
use pci::BusDevice;
use super::port::DeviceType;

pub static S_PCI_DRIVER: PciDriver = PciDriver;
pub static mut SATA_DEVICES: Vec<super::port::Port> = Vec::new();
pub static mut CURRENT_SATA_DEVICE: usize = 0;

/// Standard PCI bus binding (Class 1, Subclass 6, IF 1)
pub struct PciDriver;

impl Driver for PciDriver
{
    fn name(&self) -> &str {
        "ahci-pci"
    }
    fn bus_type(&self) -> &str {
        "pci"
    }
    fn handles(&self, bus_dev: &BusDevice) -> u32
    {
        let classcode = bus_dev.get_attr("class").unwrap_u32();
        // [class] [subclass] [IF] [ver]
        if classcode & 0xFFFFFF00 == 0x01060100{
            1    // Handle as weakly as possible (vendor-provided drivers bind higher)
        }
        else {
            0
        }
    }
    fn bind(&self, bus_dev: &mut BusDevice) -> Box<DriverInstance+'static>
    {
        let irq = bus_dev.get_irq(0);
        let base = bus_dev.base_slice(5);

        Box::new(SATAController::new(base))
    }
}

struct SATAController{

}

impl SATAController{
    pub fn new<'a>(base: u32) -> Self{
        for i in 0..32{
            unsafe{
                let port_result = super::port::Port::new(base, i);

                match port_result{
                    Ok(port) => {
                        //For find the type of device attached to the port
                        match port.r#type{
                            //Now, we only support SATA devices
                            DeviceType::SATA => {
                                let types = [/*"atapi", */"lba48", "lba28"];

                                let mut array = [0, 0, 0, 0, 0, 0, 0, 0];
                                let mut slice = &mut (array)[..];
                                let buf: super::port::DataPtr = super::port::DataPtr::Recv(&slice);
                                let result = port.request_ata_lba48(port.mem, 1, &mut 1, buf); //Read the first sector
                                if *buf.as_slice().as_ptr() != 0{
                                    println!("SUCCESS LBA48!!! Data: {}", *buf.as_slice().as_ptr());
                                    break;
                                }

                                /*for cmd in &commands{
                                    for t in &types{
                                        let mut array = [0, 0, 0, 0, 0, 0, 0, 0];
                                        let mut slice = &mut (array)[0..7];
                                        let buf: super::port::DataPtr = super::port::DataPtr::Recv(&slice);
                                        /*match result{
                                            Ok(r) => println!("Content: {:#?}, data: {}", buf, *buf.as_slice().as_ptr()),
                                            Err(e) => println!("READ ERROR: {:#?}", e),
                                        }*/
                                        match t{
                                            /*&"atapi" => {
                                                let result = port.request_atapi(port.mem, 0, &[*cmd], buf); //Read the first sector
                                                if *buf.as_slice().as_ptr() != 0{
                                                    println!("SUCCESS ATAPI!!! Data: {}, command: {}", *buf.as_slice().as_ptr(), cmd);
                                                    break;
                                                }
                                            },*/
                                            &"lba48" => {
                                                let result = port.request_ata_lba48(port.mem, 1, 1, buf); //Read the first sector
                                                if *buf.as_slice().as_ptr() != 0{
                                                    println!("SUCCESS LBA48!!! Data: {}, command: {}", *buf.as_slice().as_ptr(), cmd);
                                                    break;
                                                }
                                            },
                                            &"lba28" => {
                                                let result = port.request_ata_lba28(port.mem, 1, 1, buf); //Read the first sector
                                                if *buf.as_slice().as_ptr() != 0{
                                                    println!("SUCCESS LBA28!!! Data: {}, command: {}", *buf.as_slice().as_ptr(), cmd);
                                                    break;
                                                }
                                            },
                                            _ => {
                                                println!("Strange");
                                            }
                                        }
                                    }
                                }*/
                            }
                            _ => ()/*println!("Device type not supported")*/,
                        }
                    },
                    Err(e) => println!("{:?}", e),
                }
            }
        }

        Self{

        }
    }
}

impl crate::drivers::DriverInstance for SATAController{

}
