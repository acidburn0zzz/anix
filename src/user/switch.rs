use crate::gdt;
pub fn usermode(ip: usize, sp: usize, arg: usize){
       unsafe{
		asm!("push r10
			push r11
			push r12
			push r13
			push r14
			push r15"
			: // No output
			:   "{r10}"(gdt::GDT_USER_DATA << 3 | 3), // Data segment
				"{r11}"(sp), // Stack pointer
				"{r12}"(1 << 9), // Flags - Set interrupt enable flag
				"{r13}"(gdt::GDT_USER_CODE << 3 | 3), // Code segment
				"{r14}"(ip), // IP
				"{r15}"(arg) // Argument
			: // No clobbers
			: "intel", "volatile");

		// Go to usermode
		asm!("mov ds, r14d
			mov es, r14d
			mov fs, r15d
			mov gs, r14d
			xor rax, rax
			xor rbx, rbx
			xor rcx, rcx
			xor rdx, rdx
			xor rsi, rsi
			xor rdi, rdi
			xor rbp, rbp
			xor r8, r8
			xor r9, r9
			xor r10, r10
			xor r11, r11
			xor r12, r12
			xor r13, r13
			xor r14, r14
			xor r15, r15
			fninit
			pop rdi
			iretq"
			: // No output because it never returns
			:   "{r14}"(gdt::GDT_USER_DATA << 3 | 3), // Data segment
				"{r15}"(gdt::GDT_USER_TLS << 3 | 3) // TLS segment
			: // No clobbers because it never returns
			: "intel", "volatile");
         }
}

pub fn switch_to_user_mode()
{
        /*set_kernel_stack(current_task->kernel_stack+KERNEL_STACK_SIZE);

        print!("Attempting to switch to user mode.\n");
        asm!("mov $0x23, %ax;
                mov %ax, %ds;
                mov %ax, %es;
                mov %ax, %fs;
                mov %ax, %gs;
                             
                mov %esp, %eax;
                pushl $0x23;
                pushl %eax;
                pushf;
                pushl $0x1B;
                push $1f;
                iret;
        1:                              ");*/

}

/*pub extern fn userspace_init() {
    let path = b"initfs:/bin/init";
    let env = unsafe { INIT_ENV };

    assert_eq!(syscall::chdir(b"initfs:"), Ok(0));

    assert_eq!(syscall::open(b"debug:", syscall::flag::O_RDONLY).map(FileHandle::into), Ok(0));
    assert_eq!(syscall::open(b"debug:", syscall::flag::O_WRONLY).map(FileHandle::into), Ok(1));
    assert_eq!(syscall::open(b"debug:", syscall::flag::O_WRONLY).map(FileHandle::into), Ok(2));

    let fd = syscall::open(path, syscall::flag::O_RDONLY).expect("failed to open init");

    let mut args = Vec::new();
    args.push(path.to_vec().into_boxed_slice());

    let mut vars = Vec::new();
    for var in env.split(|b| *b == b'\n') {
        if ! var.is_empty() {
            vars.push(var.to_vec().into_boxed_slice());
        }
    }

    syscall::fexec_kernel(fd, args.into_boxed_slice(), vars.into_boxed_slice()).expect("failed to execute init");

    panic!("init returned");
}*/

pub fn task1(){
        loop{print!("a");};
        return;
}

pub fn init_user(){
		use crate::mm::*;
		unsafe{
				//println!("Memcpy");
				mmemcpy(0x30000 as *mut u8, func_to_u8(task1), 100);
				
				println!("Switch");
				switch_to_user_mode();
				
				println!("Success !");
				/*asm!("movw 0x38, %ax \n \
				ltr %ax");
				
				asm!("movw %ss, 0 \n \
				movl %esp, 1");

				asm!("cli \n \
				push 0x33 \n \
				push 0x30000 \n \
				pop %eax \n \
				orl 0x200, %eax \n \
				and 0xffffbfff, %eax \n \
				push %eax \n \
				push 0x23 \n \
				push 0x0 \n \
				movl 0x20000, 0 \n \
				movw 0x2B, %ax \n \
				movw %ax, %ds \n \
				iret");*/
		}
}

pub fn func_to_u8(mut f: fn()) -> *const u8 {
		let mut func_mut: &mut FnMut() = &mut f;
		let func_mut_2 = &mut func_mut;

		let func_pointer = func_mut_2 as *mut _;
		return func_pointer as *const u8;
}


/*-------------------------------------RUX----------------------------------
use arch::init;

/// Interrupt handler function type.
pub type HandlerFunc = unsafe extern "C" fn();

#[derive(Debug, Clone)]
#[repr(C)]
pub struct ExceptionStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}

#[derive(Debug, Clone)]
pub struct ExceptionInfo {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
    pub error_code: Option<u64>,
    pub exception_code: u64
}

#[derive(Debug, Clone)]
pub struct Registers {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
}

impl Default for Registers {
    fn default() -> Registers {
        Registers {
            rax: 0, rbx: 0, rcx: 0, rdx: 0, rsi: 0, rdi: 0,
            r8: 0, r9: 0, r10: 0, r11: 0, r12: 0, r13: 0, r14: 0, r15: 0,
            rbp: 0,
        }
    }
}

pub static mut RSP_AFTER_SAVING_REGISTERS: u64 = 0;

unsafe extern "C" fn set_kernel_stack(addr: u64) {
    init::set_kernel_stack(addr);
}

pub unsafe fn switch_to_raw(stack_vaddr: u64, code_start: u64, cpu_flags: u64, code_seg: u64, data_seg: u64) {
    switch_to_raw_naked(stack_vaddr, code_start, cpu_flags, code_seg, data_seg);
}

#[naked]
#[inline(never)]
pub unsafe extern "C" fn switch_to_raw_naked(stack_vaddr: u64, code_start: u64, cpu_flags: u64, code_seg: u64, data_seg: u64) {
    asm!("
       /* save registers */
       push rax
       push rbx
       push rcx
       push rdx
       push rbp
       push rsi
       push rdi
       push r8
       push r9
       push r10
       push r11
       push r12
       push r13
       push r14
       push r15
       mov [$1], rsp

       push r8 /* data seg */
       push rdi /* stack vaddr */
       push rdx /* cpu flags */
       push rcx /* code seg */
       push rsi /* code start */

       mov rdi, rsp
       call $0

       mov rax, [$2]
       mov rbx, [$3]
       mov rcx, [$4]
       mov rdx, [$5]
       mov rsi, [$6]
       mov rdi, [$7]
       mov r8, [$8]
       mov r9, [$9]
       mov r10, [$10]
       mov r11, [$11]
       mov r12, [$12]
       mov r13, [$13]
       mov r14, [$14]
       mov r15, [$15]
       mov rbp, [$16]

       iretq
    "
    ::
         "i"(set_kernel_stack as unsafe extern "C" fn(u64)),
         "i"(&RSP_AFTER_SAVING_REGISTERS),

         "i"(&CUR_REGISTERS.rax),
         "i"(&CUR_REGISTERS.rbx),
         "i"(&CUR_REGISTERS.rcx),
         "i"(&CUR_REGISTERS.rdx),
         "i"(&CUR_REGISTERS.rsi),
         "i"(&CUR_REGISTERS.rdi),
         "i"(&CUR_REGISTERS.r8),
         "i"(&CUR_REGISTERS.r9),
         "i"(&CUR_REGISTERS.r10),
         "i"(&CUR_REGISTERS.r11),
         "i"(&CUR_REGISTERS.r12),
         "i"(&CUR_REGISTERS.r13),
         "i"(&CUR_REGISTERS.r14),
         "i"(&CUR_REGISTERS.r15),
         "i"(&CUR_REGISTERS.rbp),

         "{r8}"(data_seg),
         "{rdi}"(stack_vaddr),
         "{rdx}"(cpu_flags),
         "{rcx}"(code_seg),
         "{rsi}"(code_start)
    ::
    "volatile", "intel");
}

static mut CUR_EXCEPTION_STACK_FRAME: Option<ExceptionStackFrame> = None;
static mut CUR_EXCEPTION_ERROR_CODE: Option<u64> = None;
static mut CUR_EXCEPTION_CODE: Option<u64> = None;
pub static mut CUR_REGISTERS: Registers = Registers {
    rax: 0, rbx: 0, rcx: 0, rdx: 0, rsi: 0, rdi: 0,
    r8: 0, r9: 0, r10: 0, r11: 0, r12: 0, r13: 0, r14: 0, r15: 0, rbp: 0
};

pub unsafe fn set_cur_registers(registers: Registers) {
    CUR_REGISTERS = registers;
}

pub unsafe fn cur_registers() -> Registers {
    CUR_REGISTERS.clone()
}

pub unsafe extern "C" fn store_exception_stack(exception_raw: *const ExceptionStackFrame, exception_code: u64) {
    let exception = &*exception_raw;
    CUR_EXCEPTION_STACK_FRAME = Some(exception.clone());
    CUR_EXCEPTION_ERROR_CODE = None;
    CUR_EXCEPTION_CODE = Some(exception_code);
}

#[allow(dead_code)]
pub unsafe extern "C" fn store_error_exception_stack(exception_raw: *const ExceptionStackFrame, error_code: u64, exception_code: u64) {
    let exception = &*exception_raw;
    CUR_EXCEPTION_STACK_FRAME = Some(exception.clone());
    CUR_EXCEPTION_ERROR_CODE = Some(error_code);
    CUR_EXCEPTION_CODE = Some(exception_code);
}

macro_rules! return_to_raw_fn {
    ($name: ident, $exception_code: expr) => (
        #[naked]
        #[inline(never)]
        pub unsafe extern "C" fn $name() {
            use ::arch::interrupt::switch::{RSP_AFTER_SAVING_REGISTERS, CUR_REGISTERS};

            asm!("mov [$2], rax
                  mov [$3], rbx
                  mov [$4], rcx
                  mov [$5], rdx
                  mov [$6], rsi
                  mov [$7], rdi
                  mov [$8], r8
                  mov [$9], r9
                  mov [$10], r10
                  mov [$11], r11
                  mov [$12], r12
                  mov [$13], r13
                  mov [$14], r14
                  mov [$15], r15
                  mov [$16], rbp

                  mov rsi, $17
                  mov rdi, rsp
                  sub rsp, 8
                  call $0

                  mov rsp, [$1]
                  pop r15
                  pop r14
                  pop r13
                  pop r12
                  pop r11
                  pop r10
                  pop r9
                  pop r8
                  pop rdi
                  pop rsi
                  pop rbp
                  pop rdx
                  pop rcx
                  pop rbx
                  pop rax"
                 ::

                 "i"(::arch::interrupt::switch::store_exception_stack as unsafe extern "C" fn(*const ::arch::interrupt::switch::ExceptionStackFrame, u64)),
                 "i"(&RSP_AFTER_SAVING_REGISTERS),

                 "i"(&CUR_REGISTERS.rax),
                 "i"(&CUR_REGISTERS.rbx),
                 "i"(&CUR_REGISTERS.rcx),
                 "i"(&CUR_REGISTERS.rdx),
                 "i"(&CUR_REGISTERS.rsi),
                 "i"(&CUR_REGISTERS.rdi),
                 "i"(&CUR_REGISTERS.r8),
                 "i"(&CUR_REGISTERS.r9),
                 "i"(&CUR_REGISTERS.r10),
                 "i"(&CUR_REGISTERS.r11),
                 "i"(&CUR_REGISTERS.r12),
                 "i"(&CUR_REGISTERS.r13),
                 "i"(&CUR_REGISTERS.r14),
                 "i"(&CUR_REGISTERS.r15),
                 "i"(&CUR_REGISTERS.rbp),

                 "i"($exception_code)
                 :: "volatile", "intel");
        }
    )
}

#[allow(unused_macros)]
macro_rules! return_error_to_raw_fn {
    ($name: ident, $exception_code: expr) => (
        #[naked]
        #[inline(never)]
        pub unsafe extern "C" fn $name() {
            use ::arch::interrupt::switch::{RSP_AFTER_SAVING_REGISTERS, CUR_REGISTERS};

            asm!("mov [$2], rax
                  mov [$3], rbx
                  mov [$4], rcx
                  mov [$5], rdx
                  mov [$6], rsi
                  mov [$7], rdi
                  mov [$8], r8
                  mov [$9], r9
                  mov [$10], r10
                  mov [$11], r11
                  mov [$12], r12
                  mov [$13], r13
                  mov [$14], r14
                  mov [$15], r15
                  mov [$16], rbp

                  mov rdx, $17
                  pop rsi
                  mov rdi, rsp
                  sub rsp, 8
                  call $0

                  mov rsp, [$1]
                  pop r15
                  pop r14
                  pop r13
                  pop r12
                  pop r11
                  pop r10
                  pop r9
                  pop r8
                  pop rdi
                  pop rsi
                  pop rbp
                  pop rdx
                  pop rcx
                  pop rbx
                  pop rax"
                 ::

                 "i"(::arch::interrupt::switch::store_error_exception_stack as unsafe extern "C" fn(*const ::arch::interrupt::switch::ExceptionStackFrame, u64, u64)),
                 "i"(&RSP_AFTER_SAVING_REGISTERS),

                 "i"(&CUR_REGISTERS.rax),
                 "i"(&CUR_REGISTERS.rbx),
                 "i"(&CUR_REGISTERS.rcx),
                 "i"(&CUR_REGISTERS.rdx),
                 "i"(&CUR_REGISTERS.rsi),
                 "i"(&CUR_REGISTERS.rdi),
                 "i"(&CUR_REGISTERS.r8),
                 "i"(&CUR_REGISTERS.r9),
                 "i"(&CUR_REGISTERS.r10),
                 "i"(&CUR_REGISTERS.r11),
                 "i"(&CUR_REGISTERS.r12),
                 "i"(&CUR_REGISTERS.r13),
                 "i"(&CUR_REGISTERS.r14),
                 "i"(&CUR_REGISTERS.r15),
                 "i"(&CUR_REGISTERS.rbp),

                 "i"($exception_code)
                 : "rdi" : "volatile", "intel");
        }
    )
}

pub fn last_exception_return_value() -> Option<ExceptionInfo> {
    unsafe {
        CUR_EXCEPTION_STACK_FRAME.clone().map(|exp| {
            ExceptionInfo {
                instruction_pointer: exp.instruction_pointer,
                code_segment: exp.code_segment,
                cpu_flags: exp.cpu_flags,
                stack_pointer: exp.stack_pointer,
                stack_segment: exp.stack_segment,
                error_code: CUR_EXCEPTION_ERROR_CODE,
                exception_code: CUR_EXCEPTION_CODE.unwrap()
            }
        })
    }
}
*/
