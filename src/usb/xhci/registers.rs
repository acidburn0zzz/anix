use crate::io::mmio::Mmio;

pub struct CapabilityRegs {
    pub len: Mmio<u8>,
    _rsvd: Mmio<u8>,
    pub hci_ver: Mmio<u16>,
    pub hcs_params1: Mmio<u32>,
    pub hcs_params2: Mmio<u32>,
    pub hcs_params3: Mmio<u32>,
    pub hcc_params1: Mmio<u32>,
    pub db_offset: Mmio<u32>,
    pub rts_offset: Mmio<u32>,
    pub hcc_params2: Mmio<u32>,
}

pub struct OperationalRegs {
    pub usb_cmd: Mmio<u32>,
    pub usb_sts: Mmio<u32>,
    pub page_size: Mmio<u32>,
    _rsvd2: [Mmio<u32>; 2],
    pub dn_ctrl: Mmio<u32>,
    pub crcr: Mmio<u64>,
    _rsvd3: [Mmio<u32>; 4],
    pub dcbaap: Mmio<u64>,
    pub config: Mmio<u32>,
}

#[repr(packed)]
pub struct Interrupter {
    pub iman: Mmio<u32>,
    pub imod: Mmio<u32>,
    pub erstsz: Mmio<u32>,
    _rsvd: Mmio<u32>,
    pub erstba: Mmio<u64>,
    pub erdp: Mmio<u64>,
}

#[repr(packed)]
pub struct RuntimeRegs {
    pub mfindex: Mmio<u32>,
    _rsvd: [Mmio<u32>; 7],
    pub ints: [Interrupter; 1024],
}
