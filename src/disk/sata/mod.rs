pub mod hw;
pub mod driver; //Register the driver
pub mod port;

pub fn init(){
	crate::drivers::register_driver(&driver::S_PCI_DRIVER);
}
