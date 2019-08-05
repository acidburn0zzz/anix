#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use alloc::prelude::String;
use alloc::prelude::ToString;
use volatile::Volatile;

pub const REG_CAP: u32 = 0x00;
pub const REG_GHC: u32 = 0x04;
pub const REG_IS : u32 = 0x08;
pub const REG_PI : u32 = 0x0C;
pub const REG_Px : u32 = 0x100;

pub const CAP_S64A: u32 = (1 << 31);     // Supports 64-bit addressing
pub const CAP_SNCQ: u32 = (1 << 30);     // Supports Native Command Queuing
pub const CAP_NCS : u32 = (31 << 8);     // Number of command slots (mask)
pub const CAP_NCS_ofs: u32 = 8;         //                                 (offset)
pub const CAP_SXS : u32 = (1 <<  5);     // Support External SATA

pub const GHC_AE  : u32 = (1 << 31);     // AHCI Enable
pub const GHC_MRSM: u32 = (1 << 2);     // MSI Revert to Single Message
pub const GHC_IE  : u32 = (1 << 1);     // Interrupt Enable
pub const GHC_HR  : u32 = (1 << 0);     // HBA Reset (Clears once complete)


pub const REG_PxCLB : u32 = 0x00;     // Command List Base Address
pub const REG_PxCLBU: u32 = 0x04;     // (High of above)
pub const REG_PxFB  : u32 = 0x08;     // FIS Base Address
pub const REG_PxFBU : u32 = 0x0C;     // (high of above)
pub const REG_PxIS  : u32 = 0x10;     // Interrupt Status
pub const REG_PxIE  : u32 = 0x14;     // Interrupt Enable
pub const REG_PxCMD : u32 = 0x18;
pub const REG_PxTFD : u32 = 0x20;     // Task File Data
pub const REG_PxSIG : u32 = 0x24;     // Signature
pub const REG_PxSSTS: u32 = 0x28;     // Serial ATA Status
pub const REG_PxSCTL: u32 = 0x2C;     // Serial ATA Control
pub const REG_PxSERR: u32 = 0x30;     // Serial ATA Error
pub const REG_PxSACT: u32 = 0x34;     // Serial ATA Active
pub const REG_PxCI  : u32 = 0x38;     // Command Issue
pub const REG_PxSNTF: u32 = 0x3C;     // Serial ATA Notification
pub const REG_PxFBS : u32 = 0x40;     // FIS-based Switching Control
//pub const REG_PxVS0 : u32 = 0x70;     // 4x Vendor-Specific

pub const PxIS_CPDS: u32 = (1 << 31);     // Cold Port Detect Status
pub const PxIS_TFES: u32 = (1 << 30);     // Task File Error Status
pub const PxIS_HBFS: u32 = (1 << 29);     // Host Bus Fatal error Status
pub const PxIS_HBDS: u32 = (1 << 28);     // Host Bus Data error Status
pub const PxIS_IFS : u32 = (1 << 27);     // Interface Fatal error Status
pub const PxIS_INFS: u32 = (1 << 26);     // Interface Non-Fatal error status
pub const PxIS_OFS : u32 = (1 << 24);     // OverFlow Status
pub const PxIS_IPMS: u32 = (1 << 23);     // Incorrect Port Multipier Status
pub const PxIS_PRCS: u32 = (1 << 22);     // PhyRdy Change Status
pub const PxIS_DMPS: u32 = (1 <<  7);     // Device Mechanical Presence Status
pub const PxIS_PCS : u32 = (1 <<  6);     // Port Connect change Status
pub const PxIS_DPS : u32 = (1 <<  5);     // Descriptor Processed
pub const PxIS_UFI : u32 = (1 <<  4);     // Unknown FIS Interrupt
pub const PxIS_SDBS: u32 = (1 <<  3);     // Set Device Bits Interrupt
pub const PxIS_DSS : u32 = (1 <<  2);     // DMA Setup FIS Interrupt
pub const PxIS_PSS : u32 = (1 <<  1);     // PIO Setup FIS Interrupt
pub const PxIS_DHRS: u32 = (1 <<  0);     // Device to Host Register FIS Interrupt

pub const PxCMD_ICC  : u32 = (15 << 28);     // Interface Communication Control (mask)
pub const PxCMD_ASP  : u32 = (1 << 27);     // Agressive Slumber / Partial
pub const PxCMD_ALPE : u32 = (1 << 26);     // Agressive Link Power Management Enable
pub const PxCMD_DLAE : u32 = (1 << 25);     // Drive LED on ATAPI Enable
pub const PxCMD_ATAPI: u32 = (1 << 24);     // Device is ATAPI
pub const PxCMD_APSTE: u32 = (1 << 23);     // Automatic Partial to Slumber Transitions Enabled
pub const PxCMD_FBSCP: u32 = (1 << 22);     // FIS-based Switching Capable Port
pub const PxCMD_ESP  : u32 = (1 << 21);     // External SATA Port
pub const PxCMD_CPD  : u32 = (1 << 20);     // Cold Presence Detection
pub const PxCMD_MPSP : u32 = (1 << 19);     // Mechanical Presence Switch attached to Port
pub const PxCMD_HPCP : u32 = (1 << 18);     // Hot Plut Capable Port
pub const PxCMD_PMA  : u32 = (1 << 17);     // Port Multiplier Attached
pub const PxCMD_CPS  : u32 = (1 << 16);     // Cold Presence State
pub const PxCMD_CR    : u32 = (1 << 15);     // Command List Running
pub const PxCMD_FR    : u32 = (1 << 14);     // FIS Receive Running
pub const PxCMD_MPSS : u32 = (1 << 13);     // Mechanical Presence Switch State
pub const PxCMD_CCS  : u32 = (31 << 8);     // Current Command Slot (mask)
pub const PxCMD_FRE  : u32 = (1 << 4);     // FIS Receive Enable
pub const PxCMD_CLO  : u32 = (1 << 3);     // Command List Override
pub const PxCMD_POD  : u32 = (1 << 2);     // Power On Device
pub const PxCMD_SUD  : u32 = (1 << 1);     // Spin-Up Device
pub const PxCMD_ST    : u32 = (1 << 0);     // Start

pub const PxTFD_ERR: u32 = (255 << 8); // = 0xff00
pub const PxTFD_STS: u32 = (255 << 0);     // Status (latest copy of task file status register)
pub const PxTFD_STS_BSY: u32 = (1 << 7);     // Interface is busy
pub const PxTFD_STS_DRQ: u32 = (1 << 3);     // Data transfer requested
pub const PxTFD_STS_ERR: u32 = (1 << 0);     // Error during transfer

pub const PxSSTS_IPM: u32 = (15 << 8);     // Interface Power Management (0=NP,1=Active,2=Partial,6=Slumber)
pub const PxSSTS_IPM_ofs: u32 = 8;
pub const PxSSTS_SPD: u32 = (15 << 4);     // Current Interface Speed (0=NP,Generation n)
pub const PxSSTS_SPD_ofs: u32 = 4;
pub const PxSSTS_DET: u32 = (15 << 0);     // Device Detection (0: None, 1: Present but no PHY yet, 3: Present and PHY, 4: offline)
pub const PxSSTS_DET_ofs: u32 = 0;

// Flags for command header
// Clear Busy flag.
pub const AHCI_FLAGS_BUSY_OK: u16 = 0x0400;

// Write operation flag.
pub const AHCI_FLAGS_WRITE: u16 = 0x0040;

// 2 DW length command flag.
pub const AHCI_FLAGS_2DWCMD: u16 = 0x0002;

// 5 DW length command flag.
pub const AHCI_FLAGS_5DWCMD: u16 = 0x0005;

// Prefetch PRDT
pub const AHCI_FLAGS_PREFETCH: u16 = 0x10;

#[repr(C)]
pub struct CmdHeader
{
     pub flags: u16,
     pub prdtl: u16,     // PRDT Length
     pub prdbc: Volatile<u32>, // PRDT Byte Count
     pub ctba: u64,     // Command Table Base Address, Upper 32 must be 0 if 64-bit not supported
     resvd: [u32; 4],
}

impl fmt::Debug for CmdHeader{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "
CmdHeader{{
    flags: {:#x}, prdtl: {:#x},
    prdbc: {:#x}, ctba: {:#x},
}}", self.flags, self.prdtl, self.prdbc.read(), self.ctba)
    }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct RcvdFis
{
     pub DSFIS: sata::FisDmaSetup,
     _r1: [u32; 1],
     pub PSFIS: sata::FisPioSetup,
     _r2: [u32; 3],
     pub RFIS: sata::FisDev2HostReg,
     _r3: [u32; 1],
     pub SDBFIS: sata::FisSDB,
     pub UDFIS: [u8; 64],
     _r4: [u8; 0x100 - 0xA0],
}

use core::fmt;
impl fmt::Debug for RcvdFis{
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          write!(f, "
RcvdFis{{
    DSFIS: {:#?},
    PSFIS: {:#?},
    RFIS: {:#?},
    SDBFIS: {:#?},
}}", self.DSFIS, self.PSFIS, self.RFIS, self.SDBFIS)
     }
}

// sizeof = 0x40+0x10+0x30+0x80 = 0x100 = 256 bytes
#[repr(C)]
pub struct CmdTable
{
     pub cmd_fis: [u8; 64],     // 64 bytes of CFIS
     pub atapi_cmd: [u8; 16],     // 16 bytes of ACMD
     _pad: [u8; 0x30],
     pub prdt: [CmdEnt; 65536],
}

impl fmt::Debug for CmdTable{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "
CmdTable{{
    cmd_fis: {},
}}", print_prdt(&self.prdt))
    }
}

pub fn print_prdt(prdt: &[CmdEnt; 65536]) -> String{
     let result = "[".to_string();
     for entry in 0..10{
          format!("{}{:#?}", result, prdt[entry]);
     }
     format!("{}]", result);
     result
}

#[repr(C)]
pub struct CmdEnt
{
     pub dba: u64,     // Data base address
     _rsvd: u32,
     pub dbc: u32,     // Data byte count (and flags, [31] = IntOnComplete, [21:0] = count-1)
}

impl fmt::Debug for CmdEnt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "
CmdEnt{{
    dba: {:#x},
    dbc: {:#x},
}}", self.dba, self.dbc)
          }
     }

pub mod sata {
      use core::fmt;

     #[repr(u8)]
     pub enum FisType
     {
          H2DRegister = 0x27,
          D2HRegister = 0x34, // -0x10
          DMASetup = 0x41,
          Data = 0x46,
          PIOSetup = 0x5F,
     }
     #[repr(C)]
     pub struct FisDmaSetup
     {
          pub ty: u8, // = 0x41
          pub flags: u8,     // [6]: Interrupt, [5]: Direction
          _resvd1: [u8; 2],
          pub dma_buf_id_low: u32,
          pub dma_buf_id_high: u32,
          _resvd2: [u32; 1],
          pub dma_buf_ofs: u32,
          pub dma_transfer_count: u32,
          _resvd: [u32; 1],
     }

      impl fmt::Debug for FisDmaSetup{
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                 write!(f, "
FisDmaSetup{{
    ty: {:#x}
    flags: {:#x}
    dma_buf_id_low: {:#x}
    dma_buf_id_high: {:#x}
    dma_buf_ofs: {:#x}
    dma_transfer_count: {:#x}
}}", self.ty, self.flags, self.dma_buf_id_low, self.dma_buf_id_high, self.dma_buf_ofs, self.dma_transfer_count)
            }
      }

     #[repr(C)]
     pub struct FisData
     {
          ty: u8,     // = 0x46
          _resvd: [u8; 3],
          data: [u32; 0],
     }
     #[repr(C)]
     pub struct FisPioSetup
     {
          pub ty: u8,     // = 0x5F
          pub flags: u8,
          pub status: u8,
          pub error: u8,
          pub sector_num: u8,
          pub cyl_low: u8,
          pub cyl_high: u8,
          pub dev_head: u8,
          pub sector_num_exp: u8,
          pub cyl_low_exp: u8,
          pub cyl_high_exp: u8,
          _resvd1: [u8; 1],
          pub sector_count: u8,
          pub sector_count_exp: u8,
          _resvd2: [u8; 1],
          pub e_status: u8,
          pub transfer_count: u32,
          _resvd3: [u8; 2],
     }
     #[repr(C)]
          #[derive(Default, Clone)]
     pub struct FisHost2DevReg{
          pub ty: u8,     // = 0x27
          pub flags: u8,     // [7]: Update to command register
          pub command: u8,
          pub features: u8,
          pub sector_num: u8,
          pub cyl_low: u8,
          pub cyl_high: u8,
          pub dev_head: u8,
          pub sector_num_exp: u8,
          pub cyl_low_exp: u8,
          pub cyl_high_exp: u8,
          pub features_exp: u8,
          pub sector_count: u8,
          pub sector_count_exp: u8,
          pub _resvd1: u8,
          pub control: u8,
          pub _resvd2: [u8; 4],
     }
     impl AsRef<[u8]> for FisHost2DevReg {
          fn as_ref(&self) -> &[u8] {
                // SAFE: Data is POD
                unsafe {
                     ::core::slice::from_raw_parts(self as *const Self as *const u8, ::core::mem::size_of::<Self>())
                }
          }
     }

          impl fmt::Debug for FisHost2DevReg {
              fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "
FisDev2HostReg{{
    ty: {:#x}, fl: {:#x}, co: {:#x}, fe: {:#x}, sec_num: {:#x}, cyl_low: {:#x}, cyl_high: {:#x},
    dev_head: {:#x}, sector_num_exp: {:#x}, features_exp: {:#x}, cyl_low_exp: {:#x},
    cyl_high_exp: {:#x}, sector_count: {:#x}, sector_count_exp: {:#x}, control: {:#x}
}}
", self.ty, self.flags, self.command, self.features, self.sector_num, self.cyl_low, self.cyl_high,
    self.dev_head, self.sector_num_exp, self.features_exp, self.cyl_low_exp, self.cyl_high_exp,
    self.sector_count, self.sector_count_exp, self.control)
          }
     }


     #[repr(C)]
     pub struct FisDev2HostReg
     {
          pub ty: u8,     // = 0x34
          pub int_resvd: u8,     // [6]: Interrupt bit
          pub status: u8,
          pub error: u8,
          pub sector_num: u8,
          pub cyl_low: u8,
          pub cyl_high: u8,
          pub dev_head: u8,
          pub sector_num_exp: u8,
          pub cyl_low_exp: u8,
          pub cyl_high_exp: u8,
          _resvd1: [u8; 1],
          pub sector_count: u8,
          pub sector_count_exp: u8,
          _resvd: [u8; 6],
     }

          impl fmt::Debug for FisDev2HostReg {
          fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "
FisDev2HostReg{{
    ty: {:#x}, int_resvd: {:#x}, status: {:#x}, error: {:#x},
    sector_num: {:#x}, cyl_low: {:#x}, cyl_high: {:#x},
    dev_head: {:#x}, sector_num_exp: {:#x}, cyl_low_exp: {:#x},
    cyl_high_exp: {:#x}, sector_count: {:#x}, sector_count_exp: {:#x},
}}
", self.ty, self.int_resvd, self.status, self.error, self.sector_num, self.cyl_low, self.cyl_high, self.dev_head, self.sector_num_exp, self.cyl_low_exp, self.cyl_high_exp, self.sector_count, self.sector_count_exp)
          }
     }

     #[repr(C)]
     #[derive(Debug)]
     pub struct FisSDB
     {
          ty: u8,     // = 0xA1
          int_resvd: u8,
          status: u8,
          error: u8,
          _resvd: [u32; 1],
     }

     struct RegPair(u8,u8);
     impl fmt::Debug for RegPair {
          fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:02x}-{:02x}", self.0, self.1)
          }
     }

     impl fmt::Debug for FisPioSetup {
          fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "FisPioSetup {{ flags/status/error/e_status: {:02x}/{:02x}/{:02x}/{:02x}, sector_num: {:?}, cyl_low: {:?}, cyl_high: {:?}, sector_count: {:?}, transfer_count: {} }}",
                     self.flags, self.status, self.error, self.e_status,
                     RegPair(self.sector_num_exp, self.sector_num),
                     RegPair(self.cyl_low_exp, self.cyl_low),
                     RegPair(self.cyl_high_exp, self.cyl_high),
                     RegPair(self.sector_count_exp, self.sector_count),
                     self.transfer_count
                )
          }
     }
}
