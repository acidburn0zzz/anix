/* TODO: Copy and paste from https://stackoverflow.com/questions/11739979/ahci-driver-for-own-os
 * For each port supported, switch high bit PxCMD.SUD //DON'T DO
 * Poll on PxSSTS (Page 36) to check the state of the PHY //DON'T DO
 * Check PxSIG -> It means that the first FIS was sent and all are good!!! //DO
 * Print PxCMD.FRE (it must be 0)
 */
use super::hw;
use core::ptr::read_volatile;
use alloc::prelude::String;
use super::driver::{CURRENT_SATA_DEVICE, SATA_DEVICES};

/// This constant value is used to rebase ports
pub const AHCI_BASE: u64 = 0x400000; // 4M

const SATA_READ_DMA: u8 = 0xC8;
const SATA_WRITE_DMA: u8 = 0xCA;
const SATA_READ_DMA_EXT: u8 = 0x25;
const SATA_WRITE_DMA_EXT: u8 = 0x35;

/// Mutable/Immutable data pointer, encoded as host-relative (Send = immutable data)
#[derive(Clone, Copy)]
pub enum DataPtr<'a>
{
    Send(&'a [u8]),
    Recv(&'a &'a mut [u8]),
}
impl<'a> DataPtr<'a> {
    pub fn as_slice(&self) -> &[u8] {
        match self{
            &DataPtr::Send(p) => p,
            &DataPtr::Recv(ref p) => p,
        }
    }
    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
    pub fn is_send(&self) -> bool {
        match self{
            &DataPtr::Send(_) => true,
            &DataPtr::Recv(_) => false,
        }
    }
}
impl<'a> core::fmt::Debug for DataPtr<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self{
            &DataPtr::Send(p) => write!(f, "Send({:p}+{})", p.as_ptr(), p.len()),
            &DataPtr::Recv(ref p) => write!(f, "Recv(mut {:p}+{})", p.as_ptr(), p.len()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum DeviceType{
    SATA = 0x101,
    SATAPI = 0xeb140101, //TODO: Change this to 32 bits
    OTHER = 0,
    NOT_PRESENT = 1,
}

#[derive(Copy, Clone, Debug)]
pub struct Port{
    pub abar: u32,
    pub index: u32,
    pub r#type: DeviceType,
    pub mem: AhciMemory,
}

#[derive(Copy, Clone, Debug)]
pub struct AhciMemory{
    pub idx: u32,
    pub base: u32,
}

impl AhciMemory{
    pub fn new(base: u32, idx: u32) -> Self{
        Self{
            idx: idx,
            base: base,
        }
    }

    pub fn read_value(&self, addr: u32) -> u32{
        let q = (self.base + addr) as *const u32;
        let r = q as u32;
        unsafe{
            *(r as *const u32)
        }
    }

    pub fn write_value(&self, addr: u32, value: u64){
        let q = (self.base + addr) as *mut u64;
        let mut r = q as u64;
        unsafe{
            *(r as *mut u64) = value;
        }
    }

    /// Read a register of the port
    pub fn read_reg(&self, reg: u32) -> u32{
        // To compute address, see https:// www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/serial-ata-ahci-spec-rev1-3-1.pdf#page=30
        self.read_value(hw::REG_Px + self.idx * 0x80 as u32 + reg)
    }

    /// Write a register of the port
    pub fn write_reg(&self, reg: u32, value: u64){
        // To compute address, see https:// www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/serial-ata-ahci-spec-rev1-3-1.pdf#page=30
        self.write_value(hw::REG_Px + self.idx * 0x80 as u32 + reg, value);
    }
}

impl Port{
    pub unsafe fn new(abar: u32, idx: u32) -> Result<Port, crate::drivers::DriverBindError>{
        assert!(idx < 32);
        let memory = AhciMemory::new(abar, idx);
        // TODO: test if sata is supported: if read_value(hw::REG_GHC) == GHC_AE

        // See https:// wiki.osdev.org/AHCI#Detect_attached_SATA_devices
        // Is the device present?
        if memory.read_reg(hw::REG_PxSSTS) & 0x0F == 0x3{
            let device_type = memory.read_reg(hw::REG_PxSIG);

            if device_type == DeviceType::SATAPI as u32{
                // TODO: SATAPI device
                Ok(Port {
                    abar: abar,
                    index: idx,
                    r#type: DeviceType::SATAPI,
                    mem: memory,
                })
            }
            else if device_type == DeviceType::SATA as u32{
                memory.write_reg(hw::REG_PxSERR, 0);
                memory.write_reg(hw::REG_PxIS, 1);
                memory.write_reg(hw::REG_PxIE, (hw::PxIS_CPDS|hw::PxIS_DSS|
                                hw::PxIS_PSS|hw::PxIS_DHRS|
                                hw::PxIS_TFES|hw::PxIS_IFS) as u64);
                memory.write_value(hw::REG_GHC, (hw::GHC_AE|hw::GHC_IE) as u64);

                // Set Command list base (size: 0x20)
                memory.write_reg(hw::REG_PxCLB, AHCI_BASE + (memory.idx * 0x24ff) as u64);
                memory.write_reg(hw::REG_PxCLBU, (AHCI_BASE + (memory.idx * 0x24ff) as u64) >> 32);

                // Set FIS base
                memory.write_reg(hw::REG_PxFB, AHCI_BASE + 0x400);
                memory.write_reg(hw::REG_PxFBU, (AHCI_BASE + 0x400) >> 32);

                let new_port = Port {
                    abar: abar,
                    index: idx,
                    r#type: DeviceType::SATA,
                    mem: memory,
                };

                //Add device in devices list
                CURRENT_SATA_DEVICE = idx as usize;
                SATA_DEVICES.push(new_port);

                Ok(new_port)
            }
            else{
                // TODO: Other devices like SEMB, PM, ...
                // SEMB => 0xC33C0101
                // PM => 0x96690101
                Ok(Port {
                    abar: abar,
                    index: idx,
                    r#type: DeviceType::OTHER,
                    mem: memory,
                })
            }
        }
        else{
            Ok(Port {
                abar: abar,
                index: idx,
                r#type: DeviceType::NOT_PRESENT,
                mem: memory,
            })
        }
        /*
        // Clear PxACT (TODO: not really used here)
        regs.write(hw::REG_PxSACT, 0);
        // Interrupts on
        regs.write(hw::REG_PxSERR, 0x3FF783);
        regs.write(hw::REG_PxIS, !0);
        regs.write(hw::REG_PxIE, hw::PxIS_CPDS|hw::PxIS_DSS|hw::PxIS_PSS|hw::PxIS_DHRS|hw::PxIS_TFES|hw::PxIS_IFS);
        // Start command engine (Start, FIS Rx Enable)
        let cmd = regs.read(hw::REG_PxCMD);
        regs.write(hw::REG_PxCMD, cmd|hw::PxCMD_ST|hw::PxCMD_FRE);
        */
    }

    pub fn get_rcvd_fis(addr: u32) -> &'static mut hw::RcvdFis{
        unsafe { &mut *(addr as *mut hw::RcvdFis) }
    }

    pub fn start_cmd(memory: AhciMemory){
        // while Self::read_reg(hw::REG_PxCMD) == hw::PxCMD_CR{}
        memory.write_reg(hw::REG_PxCMD, (hw::PxCMD_FRE|hw::PxCMD_ST) as u64);
    }

    pub fn stop_cmd(memory: AhciMemory){
        // Clear ST (bit0)
        memory.write_reg(hw::REG_PxCMD, hw::PxCMD_ST as u64);

        // Wait until FR (bit14), CR (bit15) are cleared
        loop{
            if memory.read_reg(hw::REG_PxCMD) == hw::PxCMD_FR{
                continue;
            }
            if memory.read_reg(hw::REG_PxCMD) == hw::PxCMD_CR{
                continue;
            }
            break;
        }

        // Clear FRE (bit4)
        memory.write_reg(hw::REG_PxCMD, hw::PxCMD_FRE as u64);
    }

    pub unsafe fn request_ata_lba48(&self, memory: AhciMemory, lba: u64, count: &mut u16, buf: DataPtr) -> Result<(), &'static str>{
        use alloc::prelude::ToString;
        use  alloc::prelude::Vec;

        // Prepare buffer
        let page = crate::memory::table::ActivePageTable::new();
        let data = buf.as_slice().as_ptr() as usize;
        let data_addr = page.translate(data).unwrap() as u64;

        let capabilities = memory.read_value(hw::REG_CAP);
        let supports_64bit = capabilities & hw::CAP_S64A != 0;
        let max_commands = ((capabilities & hw::CAP_NCS) >> hw::CAP_NCS_ofs) + 1; // TODO: Put this in the Port structure
        // println!("Max commands is: {}", max_commands);

        Self::start_cmd(memory);

        let cmd_slot = CommandSlot::new(memory, max_commands as usize);

        /* println!("Command slot chosen. It is {}. So, the command header is at {:#x}. Finally, command table is at {:#x}.",
             cmd_slot.idx, memory.read_reg(hw::REG_PxCLB + 0x20 * cmd_slot.idx),
             AHCI_BASE + (40 << 10) as u64 + (memory.idx << 13) as u64 + (cmd_slot.idx << 8) as u64);
        */

        //Build command header. See
        //https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/serial-ata-ahci-spec-rev1-3-1.pdf
        //(page 78)
        let cmd_header = cmd_slot.get_cmd_header(
            memory.read_reg(hw::REG_PxCLB + 0x20 * cmd_slot.idx)
        ); //NOTE: Maybe, (((0 | port->clbu) << 32) | port->clb)?
        let mut n_prdt_ents = 0;

        cmd_header.prdtl = *count as u16;
        //                    FIS + Command list
        cmd_header.ctba = AHCI_BASE + 0x4ff + 0x100 as u64 * cmd_slot.idx as u64;
        cmd_header.flags = hw::AHCI_FLAGS_BUSY_OK | hw::AHCI_FLAGS_2DWCMD | hw::AHCI_FLAGS_PREFETCH;
        cmd_header.prdbc.write(0);

        cmd_slot.set_cmd_header(memory.read_reg(hw::REG_PxCLB) + 0x20 as u32 * cmd_slot.idx as u32, cmd_header as &hw::CmdHeader);

        let cmd_table = cmd_slot.get_cmd_table(cmd_header.ctba); //NOTE: Maybe, (((0 | cmdheader->ctbau) << 32)|cmdheader->ctba)?
        // println!("{:#?}", cmd_table);
        while count > &mut 0 {
            cmd_table.prdt[n_prdt_ents as usize].dba = data_addr; // TODO: Physical address!!! (with page.translate)
            cmd_table.prdt[n_prdt_ents as usize].dbc = 8 * 1024 -1;    //  8K bytes (this value should always be set to 1 less than the actual value)
            *count -= 1;
            n_prdt_ents += 1;
        }

        cmd_table.prdt[cmd_header.prdtl as usize].dbc |= 1 << 31;    //  512 bytes per sector

        // Setup command
        let cmd_data = super::hw::sata::FisHost2DevReg {
            ty: hw::sata::FisType::H2DRegister as u8,
            flags: 0x80,
            command: SATA_READ_DMA_EXT,
            sector_num: lba as u8,
            cyl_low: (lba >> 8) as u8,
            cyl_high: (lba >> 16) as u8,
            dev_head: 0x40 | (0 << 4), // 0 is the disk
            sector_num_exp: (lba >> 24) as u8,
            cyl_low_exp: (lba >> 32) as u8,
            cyl_high_exp: (lba >> 40) as u8,
            sector_count: 1 as u8, // 1 is n_sectors
            sector_count_exp: (1 >> 8) as u8, // 1 is n_sectors
            ..Default::default()
        };

        let command = cmd_data.as_ref();
        cmd_table.cmd_fis[..command.len()].clone_from_slice(command);

        // Self::print_all_registers(memory);

        // Send all
        cmd_slot.set_cmd_table(cmd_header.ctba, cmd_table as &hw::CmdTable);

        let mut spin = 0;

        // Test if interface is busy or not
        while (memory.read_reg(hw::REG_PxTFD) >> 7) == 1 {
            println!("Interface busy!!!");
            spin += 1;
        }

        if spin == 1000000{
            return Err("Port is hung");
        }

        // Self::print_all_registers(memory);

        cmd_slot.start(); // Issued command

        // println!("{:#?}", cmd_header);
        // Self::print_all_registers(memory);

        // Wait for completion
        // TODO: Delete this loop
        loop{
            // In some longer duration reads, it may be helpful to spin on the DPS bit
            // in the PxIS port field as well (1 << 5)
            if memory.read_reg(hw::REG_PxCI) & (1 << cmd_slot.idx) == 1{
                break;
            }
            if memory.read_reg(hw::REG_PxIS) == hw::PxIS_TFES{
                // Task file error
                return Err("Read disk error");
            }
        }

        //Check all
        let int_status = memory.read_reg(hw::REG_PxIS);
        let tfd = memory.read_reg(hw::REG_PxTFD);
        let issued_commands = memory.read_reg(hw::REG_PxCI);
        let active_commands = memory.read_reg(hw::REG_PxSACT);

        if memory.read_reg(hw::REG_PxCMD) == hw::PxCMD_CLO{
            return Err("- Read disk error: Command list override");
        }
        else if int_status & hw::PxIS_TFES != 0{
            return Err("- Read disk error: Device pushed error");
        }
        else if int_status & hw::PxIS_CPDS != 0{
            println!("- Presence change");
        }
        else if int_status & hw::PxIS_DHRS != 0{
            println!("- Device register update");
        }
        else if int_status & hw::PxIS_PSS != 0{
            println!("- PIO setup status update");
        }

        for cmd in 0 .. 32{
            let mask = 1 << cmd;
            if tfd & 0x01 != 0 {
                //println!("Command was sent!!!");
            }
            else if issued_commands & mask == 0 || active_commands & mask == 0 {
                //println!("Command was sent!!!");
            }
            else if active_commands & mask != 0{
                // println!("Command {} active, but not used :(", cmd);
            }
        }

        // Self::print_all_registers(memory);

        // let rcvd_fis = Self::get_rcvd_fis(memory.read_reg(hw::REG_PxFB));
        // println!("Received FIS: {:#?}", rcvd_fis.RFIS);

        let cmd_header = cmd_slot.get_cmd_header(
            memory.read_reg(hw::REG_PxCLB + 0x20 * cmd_slot.idx)
        );

        // println!("Byte count: {}", cmd_header.prdbc.read());

        // Self::print_all_registers(memory);
        Ok(())
    }

    pub fn print_all_registers(memory: AhciMemory){
        println!("GHC: {:#x} CLB: {:#x} CLBU: {:#x} FB: {:#x}\nFBU: {:#x} IS: {:#x} IE: {:#x} CMD: {:#x}\nTFD: {:#x} SIG: {:#x} SSTS: {:#x} SCTL: {:#x}\nSERR: {:#x} SACT: {:#x} CI: {:#x} SNTF: {:#x} FBS: {:#x}", memory.read_value(hw::REG_GHC), memory.read_reg(hw::REG_PxCLB), memory.read_reg(hw::REG_PxCLBU), memory.read_reg(hw::REG_PxFB), memory.read_reg(hw::REG_PxFBU), memory.read_reg(hw::REG_PxIS), memory.read_reg(hw::REG_PxIE), memory.read_reg(hw::REG_PxCMD), memory.read_reg(hw::REG_PxTFD), memory.read_reg(hw::REG_PxSIG), memory.read_reg(hw::REG_PxSSTS), memory.read_reg(hw::REG_PxSCTL), memory.read_reg(hw::REG_PxSERR), memory.read_reg(hw::REG_PxSACT), memory.read_reg(hw::REG_PxCI), memory.read_reg(hw::REG_PxSNTF), memory.read_reg(hw::REG_PxFBS));
    }
}

#[derive(Copy, Clone)]
pub struct CommandSlot{
    pub mem: AhciMemory,
    pub idx: u32,
}
impl CommandSlot{
    pub fn new(mem: AhciMemory, max_commands: usize) -> Self{
        let mut available: u32 = 0;
        for i in 0 .. max_commands{
            let slot = mem.read_reg(hw::REG_PxSACT) | mem.read_reg(hw::REG_PxCI);
            if slot & 1 << i == 0 {
                available = i as u32;
                break;
            }
        }
        Self{
            mem: mem,
            idx: available,
        }
    }
    pub fn start(&self){
        self.mem.write_reg(hw::REG_PxSACT, 1);
        self.mem.write_reg(hw::REG_PxCI, 1);
    }
    pub fn get_cmd_table(&self, addr: u64) -> &'static mut hw::CmdTable{
        //TODO: Delete addr and calculate address in this function
        unsafe { &mut *(addr as *mut hw::CmdTable) }
    }

    pub fn set_cmd_table(&self, addr: u64, cmd_table: &hw::CmdTable){
        unsafe{
            *(addr as *mut &hw::CmdTable) = cmd_table;
        }
    }

    pub fn get_cmd_header(&self, addr: u32) -> &'static mut hw::CmdHeader{
        unsafe { &mut *(addr as *mut hw::CmdHeader) }
    }

    pub fn set_cmd_header(&self, addr: u32, cmd_header: &hw::CmdHeader){
        unsafe{
            *(addr as *mut &hw::CmdHeader) = cmd_header;
        }
    }

    pub fn set_data_size(&self, addr: u32, size: u16) {
        //TODO: Remove addr such as at the top of this function
        let cmd_header = self.get_cmd_header(addr);
        cmd_header.prdtl = size;
        self.set_cmd_header(addr, cmd_header);
    }
}
