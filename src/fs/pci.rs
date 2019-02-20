use x86::io::*;

pub unsafe fn pci_read_word(bus: u16, slot: u16, func: u16, offset: u16) -> u32 {
	let (mut address, lbus, lslot, lfunc, mut tmp) = (0, bus, slot, func, 0);
    
    address = lbus << 16 | lslot << 11 | lfunc << 8 | offset & 0xfc | 0x80000000;
    outl(0xCF8, address.into());
    tmp = (inl(0xCFC) >> ((offset & 2) * 8)) & 0xffff;
    tmp
}

pub unsafe fn get_device_id(bus: u16, device: u16, function: u16) -> u32 {
        let r0: u32 = pci_read_word(bus, device, function, 2);
        r0
}
