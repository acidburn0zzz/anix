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

use x86::msr;

use crate::gdt;
use crate::syscall::number::*;
use crate::errors::*;
// use crate::processes::processes_RUNNING;

pub unsafe fn init() {
    msr::wrmsr(msr::IA32_STAR, ((gdt::GDT_KERNEL_CODE as u64) << 3) << 32);
    msr::wrmsr(msr::IA32_LSTAR, syscall_instruction as u64);
    msr::wrmsr(msr::IA32_FMASK, 1 << 9);
    msr::wrmsr(msr::IA32_KERNEL_GSBASE, &gdt::TSS as *const _ as u64);

    let efer = msr::rdmsr(msr::IA32_EFER);
    msr::wrmsr(msr::IA32_EFER, efer | 1);
}

pub unsafe extern fn do_syscall(num: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize,
                                arg5: usize, _bp: usize, _stack: &mut SyscallStack) -> usize {
    match num {
        SYS_EXIT => {
            // TODO: Kill or stop the processes
            use crate::processes::scheduler::{switch, kill};
            println!("exit({}) = 0", arg1);
            kill();
            switch();
            0
        },
        // INPUT: arg1 -> buffer as DateTime
        SYS_TIME => {
            use core::slice::from_raw_parts_mut;

            use crate::device::rtc::Rtc;
            use crate::time::DateTime;

            let pointer = from_raw_parts_mut(arg1 as *mut DateTime, arg2);
            pointer[0] = Rtc::new().date();
            println!("time({:#x}) = 0", arg1);
            0
        },
        // INPUT: arg1 -> string
        //        arg2 -> len of string
        SYS_DEBUG => {
            use core::slice::from_raw_parts;
            use core::str::from_utf8;

            print!("{}", from_utf8(from_raw_parts(arg1 as *const u8, arg2)).expect("cannot transform to utf8"));
            0
        },
        // INPUT: arg1 -> path
        //        arg2 -> len of path
        //        arg3 -> flags
        SYS_OPEN => {
            // use core::slice::from_raw_parts;
            // use core::str::from_utf8;
            // use crate::fs::ext2::file::File;

            /*let path = from_utf8(from_raw_parts(arg1 as *const u8, arg2)).expect("cannot transform to utf8");
            let id = processes_RUNNING.to_owned().unwrap().next_file_id();
            let file = File::open(path, arg3);
            processes_RUNNING.to_owned().unwrap().add_new_file(file);
            println!("open({}, {:#x}) = {}", path, arg3, id);
            id*/
            0
        },
        // INPUT: arg1 -> file descriptor num
        //        arg2 -> pointer of buffer
        //        arg3 -> len of buffer
        SYS_READ => {
            // use core::ptr::copy_nonoverlapping;
            // TODO: from_raw_slice_mut?
            /*let src = processes_RUNNING.to_owned().unwrap().fds.lock()[arg1].content_ptr;
            copy_nonoverlapping(src as *const u8,
                arg2 as *mut u8,
                arg3);
            println!("read({}, {:#x}, {}) = {}", arg1, arg2, arg3, 0);*/
            0
        },
        // INPUT: arg1 -> file descriptor num
        //        arg2 -> pointer of buffer
        //        arg3 -> len of buffer
        SYS_WRITE => {
            const STDIN_FILENO: usize = 0;
            const STDOUT_FILENO: usize = 1;
            const STDERR_FILENO: usize = 2;

             match arg1 {
                STDIN_FILENO => {
                    use core::slice::from_raw_parts;
                    use core::str::from_utf8;

                    // Write to Stdin
                    print!("STDIN: {}", from_utf8(from_raw_parts(arg2 as *const u8, arg3)).expect("cannot transform to utf8"));
                    arg3
                },
                STDOUT_FILENO => {
                    use core::slice::from_raw_parts;
                    use core::str::from_utf8;

                    // Write to Stdout
                    // TODO: Use strlen() instead of arg3
                    print!("{}", from_utf8(from_raw_parts(arg2 as *const u8, arg3)).expect("cannot transform to utf8"));
                    arg3
                },
                STDERR_FILENO => {
                    use core::slice::from_raw_parts;
                    use core::str::from_utf8;

                    // Write to Error Stdout
                    println!("ERROR: {}", from_utf8(from_raw_parts(arg2 as *const u8, arg3)).expect("cannot transform to utf8"));
                    arg3
                },
                _ => {
                    println!("Write files not implemented (fd num: {})", arg1);
                    0
                },
             }
        },
        // INPUT: arg1 -> addr
        //        arg2 -> length
        //        arg3 -> prot
        //        arg4 -> flags
        //        arg5 -> file
        //        arg6 -> offset (do not used for now because we only get 5 parameters)
        SYS_MMAP => {
            use bitflags::bitflags;
            use crate::processes::scheduler::SCHEDULER;
            use crate::memory::{map, paging::EntryFlags};

            print!("mmap({:#x}, {}, {}, {}, {:#x}) = ", arg1, arg2, arg3, arg4, arg5);
            bitflags! {
                struct MmapFlags: u32 {
                    const SHARED = 1 << 0;
                    const PRIVATE = 1 << 1;
                    const FIXED = 1 << 4;
                    const ANONYMOUS = 1 << 5;
                }
            }
            bitflags! {
                struct MmapProt: u32 {
                    const NONE = 0;
                    const READ = 1 << 0;
                    const WRITE = 1 << 1;
                    const EXEC = 1 << 2;
                }
            }

            impl From<MmapProt> for EntryFlags {
                fn from(flags: MmapProt) -> Self {
                    let mut result = EntryFlags::PRESENT;
                    if !flags.contains(MmapProt::NONE) {
                        result = EntryFlags::empty();
                    }
                    if flags.contains(MmapProt::READ) {
                        result |= EntryFlags::USER_ACCESSIBLE;
                    }
                    if flags.contains(MmapProt::WRITE) {
                        result |= EntryFlags::WRITABLE;
                    }
                    if !flags.contains(MmapProt::EXEC) {
                        result |= EntryFlags::NO_EXECUTE;
                    }
                    result
                }
            }

            let prot = MmapProt::from_bits_truncate(arg3 as u32);
            let flags = MmapFlags::from_bits_truncate(arg4 as u32);

            if flags.contains(MmapFlags::ANONYMOUS) {
                if arg1 == 0 {
                    SCHEDULER.force_write_unlock();
                    let addr =
                        SCHEDULER.try_write().unwrap().get_current_process_mut().heap.try_lock().unwrap().bottom() - 4096;
                    map(
                        addr - 4096,
                        addr,
                        EntryFlags::from(prot)
                    );
                    // Fill memory area with zeros
                    // TODO: Create a helper function in the common.rs file
                    //       (use it also in src/elf/mod.rs)
                    for i in 0..4096 {
                        SCHEDULER.force_write_unlock();
                        let ptr = SCHEDULER.try_write().unwrap().get_current_process_mut().heap.try_lock().unwrap().bottom() as *mut u8;
                        *ptr.offset(i as isize) = 0;
                    }
                    SCHEDULER.force_write_unlock();
                    println!("{:#x}", SCHEDULER.try_write().unwrap().get_current_process_mut().heap.try_lock().unwrap().bottom());
                    SCHEDULER.try_write().unwrap().get_current_process_mut().heap.try_lock().unwrap().bottom()
                }
                else {
                    Error::mux(Err(Error::new(EINVAL)))
                }
            }
            else {
                Error::mux(Err(Error::new(EINVAL)))
            }
        },
        SYS_BRK => {
            use crate::processes::scheduler::SCHEDULER;
            print!("brk({:#x}) = ", arg1);
            if arg1 == 0 {
                // Get brk
                SCHEDULER.force_write_unlock();
                println!("{:#x}", SCHEDULER.try_write().unwrap().get_current_process_mut().heap.try_lock().unwrap().bottom());
                SCHEDULER.try_write().unwrap().get_current_process_mut().heap.try_lock().unwrap().bottom()
            }
            else {
                // Set brk
                let current_brk = SCHEDULER.try_write().unwrap().get_current_process_mut().heap.try_lock().unwrap().bottom();

                SCHEDULER.force_write_unlock();
                SCHEDULER.try_write().unwrap().get_current_process_mut().heap.force_unlock();
                SCHEDULER.force_write_unlock();

                SCHEDULER.try_write().unwrap().get_current_process_mut().heap.try_lock().unwrap().extend(arg1 - current_brk);

                SCHEDULER.force_write_unlock();
                SCHEDULER.try_write().unwrap().get_current_process_mut().heap.force_unlock();
                SCHEDULER.force_write_unlock();

                println!("{:#x}", SCHEDULER.try_write().unwrap().get_current_process_mut().heap.try_lock().unwrap().bottom());
                SCHEDULER.try_write().unwrap().get_current_process_mut().heap.try_lock().unwrap().bottom()
            }
        },
        SYS_SET_TID_ADDR => {
            // let value = processes_RUNNING.to_owned().unwrap().getpid();
            // println!("set_tid_addr({:#x}) = {}", arg1, value);
            // value as usize
            0
        },
        SYS_SIGACTION => {
            println!("rt_sigaction({:#x}) = 0", arg1);
            0
        },
        SYS_SIGPROCMASK => {
            println!("rt_sigprocmask({:#x}) = 0", arg1);
            0
        },
        // INPUT: arg1 -> new ss (used to change ss)
        //        arg2 -> old ss (used to get ss)
        SYS_SIGALSTACK => {
            use crate::memory::consts::USER_OFFSET;

            #[derive(Debug)]
            struct SigaltStack {
                ss_sp: usize,
                ss_flags: u32,
                ss_size: usize,
            }
            println!("sigalstack({:#x}, {:#x}) = 0", arg1, arg2);

            if arg1 != 0 {}
            if arg2 != 0 {
                *(arg2 as *mut SigaltStack) = SigaltStack {
                    ss_sp: USER_OFFSET.start,
                    ss_flags: 2,
                    ss_size: 1024,
                };
            }
            0
        },
        // INPUT: arg1 -> option
        //        arg2 -> addr
        SYS_ARCHPRCTL => {
            use x86_64::{
                instructions::segmentation::{
                    load_fs,
                    load_gs
                },
                PrivilegeLevel::Ring3,
                structures::gdt::SegmentSelector
            };

            use crate::memory::consts::USER_OFFSET;
            pub const ARCH_SET_GS: usize = 0x1001;
            pub const ARCH_SET_FS: usize = 0x1002;
            pub const _ARCH_GET_FS: usize = 0x1003;
            pub const ARCH_GET_GS: usize = 0x1004;

            if arg2 > USER_OFFSET.end { return EPERM as usize; }
            else if arg1 < ARCH_SET_FS || arg1 > ARCH_GET_GS{ return EINVAL as usize; }
            // TODO: EFAULT if addr points to an unmapped address or is outside the process address space.
            else {
                match arg1 {
                    ARCH_SET_GS => {
                        /*
                         * ARCH_SET_GS has always overwritten the index
                         * and the base. Zero is the most sensible value
                         * to put in the index, and is the only value that
                         * makes any sense if FSGSBASE is unavailable.
                         */
                        load_fs(SegmentSelector::new(0, Ring3));
                        msr::wrmsr(msr::IA32_GS_BASE, arg2 as u64);
                    },
                    ARCH_SET_FS => {
                        /*
                         * ARCH_SET_FS has always overwritten the index
                         * and the base. Zero is the most sensible value
                         * to put in the index, and is the only value that
                         * makes any sense if FSGSBASE is unavailable.
                         */
                        load_gs(SegmentSelector::new(0, Ring3));
                        msr::wrmsr(msr::IA32_FS_BASE, arg2 as u64);
                    },
                    _ => println!("sys_arch_prctl unknown option") // TODO: Return EINVAL,
                };
            }

            println!("arch_prctl({:#x} {:#x}) = 0", arg1, arg2);
            0
        },
        SYS_TKILL => {
            use crate::processes::scheduler::{kill, switch};
            println!("tkill({}, {}) = 0", arg1, arg2);
            kill();
            switch();
            0
        }
        _ => {
            println!("Unknown syscall: {}", num);
            Error::mux(Err(Error::new(ENOSYS)))
        },
    }
}

#[naked]
pub unsafe extern fn syscall_instruction() {
    #[inline(never)]
    unsafe fn inner(stack: &mut SyscallStack) -> usize {
        let rbp;
        asm!("" : "={rbp}"(rbp) : : : "intel", "volatile");

        do_syscall(stack.rax, stack.rdi, stack.rsi, stack.rdx, stack.r10, stack.r8, rbp, stack)
    }

    // Yes, this is magic. No, you don't need to understand
    asm!("xchg bx, bx
          swapgs                    // Set gs segment to TSS
          mov gs:[28], rsp          // Save userspace rsp
          mov rsp, gs:[4]           // Load kernel rsp
          push 5 * 8 + 3            // Push userspace data segment
          push qword ptr gs:[28]    // Push userspace rsp
          mov qword ptr gs:[28], 0  // Clear userspace rsp
          push r11                  // Push rflags
          push 4 * 8 + 3            // Push userspace code segment
          push rcx                  // Push userspace return pointer
          swapgs                    // Restore gs
          "
          :
          :
          :
          : "intel", "volatile");

      // Push scratch registers
      asm!("push rax
           push rbx
           push rcx
           push rdx
           push rdi
           push rsi
           push r8
           push r9
           push r10
           push r11
           push fs
           mov r11, 0x18
           mov fs, r11"
           : : : : "intel", "volatile");

      // Get reference to stack variables
      let rsp: usize;
      asm!("" : "={rsp}"(rsp) : : : "intel", "volatile");

      let a = inner(&mut *(rsp as *mut SyscallStack));

      asm!("" : : "{rax}"(a) : : "intel", "volatile");

      // Interrupt return
      asm!("pop fs
            pop r11
            pop r10
            pop r9
            pop r8
            pop rsi
            pop rdi
            pop rdx
            pop rcx
            pop rbx
            add rsp, 8
            iretq"
            : : : : "intel", "volatile");
}

#[naked]
pub unsafe extern fn syscall() {
    #[inline(never)]
    unsafe fn inner(stack: &mut SyscallStack) -> usize {
        let rbp;
        asm!("" : "={rbp}"(rbp) : : : "intel", "volatile");

        do_syscall(stack.rax, stack.rbx, stack.rcx, stack.rdx,
                stack.rsi, stack.rdi, rbp, stack)
    }

    // Push scratch registers
    asm!("push rax
         push rbx
         push rcx
         push rdx
         push rdi
         push rsi
         push r8
         push r9
         push r10
         push r11
         push fs
         mov r11, 0x18
         mov fs, r11"
         : : : : "intel", "volatile");

    // Get reference to stack variables
    let rsp: usize;
    asm!("" : "={rsp}"(rsp) : : : "intel", "volatile");

    let a = inner(&mut *(rsp as *mut SyscallStack)) as i32;

    asm!("" : : "{rax}"(a) : : "intel", "volatile");

    // Interrupt return
    asm!("pop fs
          pop r11
          pop r10
          pop r9
          pop r8
          pop rsi
          pop rdi
          pop rdx
          pop rcx
          pop rbx
          add rsp, 8
          iretq"
          : : : : "intel", "volatile");
}

#[allow(dead_code)]
#[repr(packed)]
pub struct SyscallStack {
    pub fs: usize,
    pub r11: usize,
    pub r10: usize,
    pub r9: usize,
    pub r8: usize,
    pub rsi: usize,
    pub rdi: usize,
    pub rdx: usize,
    pub rcx: usize,
    pub rbx: usize,
    pub rax: usize,
    pub rip: usize,
    pub cs: usize,
    pub rflags: usize,
}
