/*
 * Copyright (C) 2018-2020 Nicolas Fouquet
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see https://www.gnu.org/licenses.
 */
use core::panic::PanicInfo;
use alloc::alloc::Layout;
use x86_64::{
    structures::paging::{
        mapper::MapToError,
        FrameAllocator,
        Mapper,
        Page,
        PageTableFlags as Flags,
        Size4KiB,
    },
    VirtAddr,
};

use crate::*;

pub mod memory;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("System->Panic! Debug informations: {:#?}", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[alloc_error_handler]
fn handle_alloc_error(layout: Layout) -> ! {
    println!("Heap->Out of memory! The size {:#x} bytes is too big", layout.size());
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[cfg(test)]
use bootloader::BootInfo;

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    screen::logo_screen();

    println!("Welcome!\nAnix v0.0.3 is starting...");

    println!("DEBUG: enable SSE");
    sse::enable_sse();

    unsafe {
        println!("DEBUG: init GDT");
        gdt::init();
        gdt::init_paging(rsp());
    }

    unsafe {
        irq::syscalls::init();
    }

    println!("DEBUG: init IDT");
    idt::init_idt();

    println!("DEBUG: init pics");
    unsafe { PICS.lock().initialize() };

    println!("DEBUG: Init memory");
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {
        memory::init(phys_mem_offset)
    };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    map(&mut mapper,
        &mut frame_allocator,
        0x10000001000,
        0x10000001000,
        Flags::PRESENT | Flags::WRITABLE | Flags::USER_ACCESSIBLE
    ).expect("cannot map");
    map(&mut mapper,
        &mut frame_allocator,
        0x8000000,
        0x8000500,
        Flags::PRESENT | Flags::WRITABLE | Flags::USER_ACCESSIBLE
    ).expect("cannot map");
    /*enable_nxe_bit();
    enable_write_protect_bit();

    unsafe {
        let boot_info = multiboot2::load(multiboot_information_address);

        memory::init(
            boot_info.start_address() as u64,
            boot_info.end_address() as u64,
            boot_info.elf_sections_tag().expect("cannot get elf sections tag"),
            boot_info.memory_map_tag().expect("Memory map tag required").memory_areas()
        );
        *VESA_BUFFER.lock() = boot_info.vbe_info_tag().unwrap().mode_info.framebuffer_base_ptr;
    }*/

    println!("DEBUG: Init the heap");
    crate::memory::heap::init();

    println!("DEBUG: Start SATA driver");
    disk::sata::init();

    // println!("DEBUG: Start VBE driver");
    // graphics::vesa::init();

    println!("DEBUG: Start PCI driver");
    pci::init();

    println!("INFO: List of PCI devices:");
    pci::list_devices();

    // TODO: Use the multiboot crate to determinate the disk which will be read and write
    // println!("DEBUG: Test Ext2 filesystem");
    // fs::ext2::init();

    println!("DEBUG: Start tasking system");

    // Map usermode tasks
    // map(system as *const () as u64, system as *const () as u64 + 15,
    // EntryFlags::PRESENT | EntryFlags::WRITABLE | EntryFlags::USER_ACCESSIBLE);
    Process::new(
        String::from("system"),
        system as *const () as u64,
        Box::new(&["Hello world with arguments!".as_bytes()])
    );
    Process::new(
        String::from("terminal"),
        user::input::terminal as *const () as u64,
        Box::new(&[])
    );

    // println!("DEBUG: Start elf loader");
    // elf::init();

    test_main();

    loop {}
}

pub fn map(mapper: &mut impl Mapper<Size4KiB>,
           frame_allocator: &mut impl FrameAllocator<Size4KiB>,
           start_address: u64,
           end_address: u64,
           flags: Flags) -> Result<(), MapToError> {
        let page_range = {
        let start = VirtAddr::new(start_address);
        let end = VirtAddr::new(end_address);
        let start_page = Page::containing_address(start);
        let end_page = Page::containing_address(end);
        Page::range_inclusive(start_page, end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        mapper.map_to(page, frame, flags, frame_allocator)?.flush();
    }
    Ok(())
}
