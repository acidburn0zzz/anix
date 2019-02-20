pub const SYS_CLASS: usize =    0xF000_0000;
pub const SYS_CLASS_PATH: usize=0x1000_0000;
pub const SYS_CLASS_FILE: usize=0x2000_0000;

pub const SYS_ARG: usize =      0x0F00_0000;
pub const SYS_ARG_SLICE: usize =0x0100_0000;
pub const SYS_ARG_MSLICE: usize=0x0200_0000;
pub const SYS_ARG_PATH: usize = 0x0300_0000;

pub const SYS_RET: usize =      0x00F0_0000;
pub const SYS_RET_FILE: usize = 0x0010_0000;

pub const SYS_LINK: usize =     SYS_CLASS_PATH | SYS_ARG_PATH | 9;
pub const SYS_OPEN: usize =     SYS_CLASS_PATH | SYS_RET_FILE | 5;
pub const SYS_CHMOD: usize =    SYS_CLASS_PATH | 15;
pub const SYS_RMDIR: usize =    SYS_CLASS_PATH | 84;
pub const SYS_UNLINK: usize =   SYS_CLASS_PATH | 10;

pub const SYS_CLOSE: usize =    SYS_CLASS_FILE | 6;
pub const SYS_DUP: usize =      SYS_CLASS_FILE | SYS_RET_FILE | 41;
pub const SYS_DUP2: usize =     SYS_CLASS_FILE | SYS_RET_FILE | 63;
pub const SYS_READ: usize =     SYS_CLASS_FILE | SYS_ARG_MSLICE | 3;
pub const SYS_WRITE: usize =    SYS_CLASS_FILE | SYS_ARG_SLICE | 4;
pub const SYS_LSEEK: usize =    SYS_CLASS_FILE | 19;
pub const SYS_FCHMOD: usize =   SYS_CLASS_FILE | 94;
pub const SYS_FCHOWN: usize =   SYS_CLASS_FILE | 207;
pub const SYS_FCNTL: usize =    SYS_CLASS_FILE | 55;
pub const SYS_FEVENT: usize =   SYS_CLASS_FILE | 927;
pub const SYS_FEXEC: usize =    SYS_CLASS_FILE | 11;
pub const SYS_FMAP: usize =     SYS_CLASS_FILE | SYS_ARG_SLICE | 90;
pub const SYS_FUNMAP: usize =   SYS_CLASS_FILE | 91;
pub const SYS_FPATH: usize =    SYS_CLASS_FILE | SYS_ARG_MSLICE | 928;
pub const SYS_FRENAME: usize =  SYS_CLASS_FILE | SYS_ARG_PATH | 38;
pub const SYS_FSTAT: usize =    SYS_CLASS_FILE | SYS_ARG_MSLICE | 28;
pub const SYS_FSTATVFS: usize = SYS_CLASS_FILE | SYS_ARG_MSLICE | 100;
pub const SYS_FSYNC: usize =    SYS_CLASS_FILE | 118;
pub const SYS_FTRUNCATE: usize =SYS_CLASS_FILE | 93;
pub const SYS_FUTIMENS: usize = SYS_CLASS_FILE | SYS_ARG_SLICE | 320;

pub const SYS_BRK: usize =      45;
pub const SYS_CHDIR: usize =    12;
pub const SYS_CLOCK_GETTIME: usize = 265;
pub const SYS_CLONE: usize =    120;
pub const SYS_EXIT: usize =     1;
pub const SYS_FUTEX: usize =    240;
pub const SYS_GETCWD: usize =   183;
pub const SYS_GETEGID: usize =  202;
pub const SYS_GETENS: usize =   951;
pub const SYS_GETEUID: usize =  201;
pub const SYS_GETGID: usize =   200;
pub const SYS_GETNS: usize =    950;
pub const SYS_GETPID: usize =   20;
pub const SYS_GETPGID: usize =  132;
pub const SYS_GETPPID: usize =  64;
pub const SYS_GETUID: usize =   199;
pub const SYS_IOPL: usize =     110;
pub const SYS_KILL: usize =     37;
pub const SYS_MPROTECT: usize = 125;
pub const SYS_MKNS: usize =     984;
pub const SYS_NANOSLEEP: usize =162;
pub const SYS_PHYSALLOC: usize =945;
pub const SYS_PHYSFREE: usize = 946;
pub const SYS_PHYSMAP: usize =  947;
pub const SYS_PHYSUNMAP: usize =948;
pub const SYS_VIRTTOPHYS: usize=949;
pub const SYS_PIPE2: usize =    331;
pub const SYS_SETPGID: usize =  57;
pub const SYS_SETREGID: usize = 204;
pub const SYS_SETRENS: usize =  952;
pub const SYS_SETREUID: usize = 203;
pub const SYS_SIGACTION: usize =67;
pub const SYS_SIGPROCMASK:usize=126;
pub const SYS_SIGRETURN: usize =119;
pub const SYS_UMASK: usize =    60;
pub const SYS_WAITPID: usize =  7;
pub const SYS_YIELD: usize =    158;

pub const CLONE_VM: usize = 0x100;
pub const CLONE_FS: usize = 0x200;
pub const CLONE_FILES: usize = 0x400;
pub const CLONE_SIGHAND: usize = 0x800;
pub const CLONE_VFORK: usize = 0x4000;
pub const CLONE_THREAD: usize = 0x10000;

pub const CLOCK_REALTIME: usize = 1;
pub const CLOCK_MONOTONIC: usize = 4;

pub const EVENT_NONE: usize = 0;
pub const EVENT_READ: usize = 1;
pub const EVENT_WRITE: usize = 2;

pub const F_DUPFD: usize = 0;
pub const F_GETFD: usize = 1;
pub const F_SETFD: usize = 2;
pub const F_GETFL: usize = 3;
pub const F_SETFL: usize = 4;

pub const FUTEX_WAIT: usize = 0;
pub const FUTEX_WAKE: usize = 1;
pub const FUTEX_REQUEUE: usize = 2;

pub const MAP_SHARED: usize = 0x0001;
pub const MAP_PRIVATE: usize = 0x0002;

pub const MODE_TYPE: u16 = 0xF000;
pub const MODE_DIR: u16 = 0x4000;
pub const MODE_FILE: u16 = 0x8000;
pub const MODE_SYMLINK: u16 = 0xA000;
pub const MODE_FIFO: u16 = 0x1000;
pub const MODE_CHR: u16 = 0x2000;

pub const MODE_PERM: u16 = 0x0FFF;
pub const MODE_SETUID: u16 = 0o4000;
pub const MODE_SETGID: u16 = 0o2000;

pub const O_RDONLY: usize =     0x0001_0000;
pub const O_WRONLY: usize =     0x0002_0000;
pub const O_RDWR: usize =       0x0003_0000;
pub const O_NONBLOCK: usize =   0x0004_0000;
pub const O_APPEND: usize =     0x0008_0000;
pub const O_SHLOCK: usize =     0x0010_0000;
pub const O_EXLOCK: usize =     0x0020_0000;
pub const O_ASYNC: usize =      0x0040_0000;
pub const O_FSYNC: usize =      0x0080_0000;
pub const O_CLOEXEC: usize =    0x0100_0000;
pub const O_CREAT: usize =      0x0200_0000;
pub const O_TRUNC: usize =      0x0400_0000;
pub const O_EXCL: usize =       0x0800_0000;
pub const O_DIRECTORY: usize =  0x1000_0000;
pub const O_STAT: usize =       0x2000_0000;
pub const O_SYMLINK: usize =    0x4000_0000;
pub const O_NOFOLLOW: usize =   0x8000_0000;
pub const O_ACCMODE: usize =    O_RDONLY | O_WRONLY | O_RDWR;

pub const PHYSMAP_WRITE: usize = 1;
pub const PHYSMAP_WRITE_COMBINE: usize = 2;

pub const PROT_NONE: usize = 0x0000_0000;
pub const PROT_EXEC: usize = 0x0001_0000;
pub const PROT_WRITE: usize = 0x0002_0000;
pub const PROT_READ: usize = 0x0004_0000;

pub const SEEK_SET: usize = 0;
pub const SEEK_CUR: usize = 1;
pub const SEEK_END: usize = 2;

pub const SIGHUP: usize =   1;
pub const SIGINT: usize =   2;
pub const SIGQUIT: usize =  3;
pub const SIGILL: usize =   4;
pub const SIGTRAP: usize =  5;
pub const SIGABRT: usize =  6;
pub const SIGBUS: usize =   7;
pub const SIGFPE: usize =   8;
pub const SIGKILL: usize =  9;
pub const SIGUSR1: usize =  10;
pub const SIGSEGV: usize =  11;
pub const SIGUSR2: usize =  12;
pub const SIGPIPE: usize =  13;
pub const SIGALRM: usize =  14;
pub const SIGTERM: usize =  15;
pub const SIGSTKFLT: usize= 16;
pub const SIGCHLD: usize =  17;
pub const SIGCONT: usize =  18;
pub const SIGSTOP: usize =  19;
pub const SIGTSTP: usize =  20;
pub const SIGTTIN: usize =  21;
pub const SIGTTOU: usize =  22;
pub const SIGURG: usize =   23;
pub const SIGXCPU: usize =  24;
pub const SIGXFSZ: usize =  25;
pub const SIGVTALRM: usize= 26;
pub const SIGPROF: usize =  27;
pub const SIGWINCH: usize = 28;
pub const SIGIO: usize =    29;
pub const SIGPWR: usize =   30;
pub const SIGSYS: usize =   31;

pub const SIG_DFL: usize = 0;
pub const SIG_IGN: usize = 1;

pub const SIG_BLOCK: usize = 0;
pub const SIG_UNBLOCK: usize = 1;
pub const SIG_SETMASK: usize = 2;

pub const SA_NOCLDSTOP: usize = 0x00000001;
pub const SA_NOCLDWAIT: usize = 0x00000002;
pub const SA_SIGINFO: usize =   0x00000004;
pub const SA_RESTORER: usize =  0x04000000;
pub const SA_ONSTACK: usize =   0x08000000;
pub const SA_RESTART: usize =   0x10000000;
pub const SA_NODEFER: usize =   0x40000000;
pub const SA_RESETHAND: usize = 0x80000000;

pub const WNOHANG: usize =    0x01;
pub const WUNTRACED: usize =  0x02;
pub const WCONTINUED: usize = 0x08;

/// True if status indicates the child is stopped.
pub fn wifstopped(status: usize) -> bool {
    (status & 0xff) == 0x7f
}

/// If wifstopped(status), the signal that stopped the child.
pub fn wstopsig(status: usize) -> usize {
    (status >> 8) & 0xff
}

/// True if status indicates the child continued after a stop.
pub fn wifcontinued(status: usize) -> bool {
    status == 0xffff
}

/// True if STATUS indicates termination by a signal.
pub fn wifsignaled(status: usize) -> bool {
    ((status & 0x7f) + 1) as i8 >= 2
}

/// If wifsignaled(status), the terminating signal.
pub fn wtermsig(status: usize) -> usize {
    status & 0x7f
}

/// True if status indicates normal termination.
pub fn wifexited(status: usize) -> bool {
    wtermsig(status) == 0
}

/// If wifexited(status), the exit status.
pub fn wexitstatus(status: usize) -> usize {
    (status >> 8) & 0xff
}

/// True if status indicates a core dump was created.
pub fn wcoredump(status: usize) -> bool {
    (status & 0x80) != 0
}

// Because the memory map is so important to not be aliased, it is defined here, in one place
// The lower 256 PML4 entries are reserved for userspace
// Each PML4 entry references up to 512 GB of memory
// The top (511) PML4 is reserved for recursive mapping
// The second from the top (510) PML4 is reserved for the kernel
    /// The size of a single PML4
    pub const PML4_SIZE: usize = 0x0000_0080_0000_0000;
    pub const PML4_MASK: usize = 0x0000_ff80_0000_0000;

    /// Offset of recursive paging
    pub const RECURSIVE_PAGE_OFFSET: usize = (-(PML4_SIZE as isize)) as usize;
    pub const RECURSIVE_PAGE_PML4: usize = (RECURSIVE_PAGE_OFFSET & PML4_MASK)/PML4_SIZE;

    /// Offset of kernel
    pub const KERNEL_OFFSET: usize = RECURSIVE_PAGE_OFFSET - PML4_SIZE;
    pub const KERNEL_PML4: usize = (KERNEL_OFFSET & PML4_MASK)/PML4_SIZE;

    /// Offset to kernel heap
    pub const KERNEL_HEAP_OFFSET: usize = KERNEL_OFFSET - PML4_SIZE;
    pub const KERNEL_HEAP_PML4: usize = (KERNEL_HEAP_OFFSET & PML4_MASK)/PML4_SIZE;
    /// Size of kernel heap
    pub const KERNEL_HEAP_SIZE: usize = 1 * 1024 * 1024; // 1 MB

    /// Offset to kernel percpu variables
    //TODO: Use 64-bit fs offset to enable this pub const KERNEL_PERCPU_OFFSET: usize = KERNEL_HEAP_OFFSET - PML4_SIZE;
    pub const KERNEL_PERCPU_OFFSET: usize = 0xC000_0000;
    /// Size of kernel percpu variables
    pub const KERNEL_PERCPU_SIZE: usize = 64 * 1024; // 64 KB

    /// Offset to user image
    pub const USER_OFFSET: usize = 0;
    pub const USER_PML4: usize = (USER_OFFSET & PML4_MASK)/PML4_SIZE;

    /// Offset to user TCB
    pub const USER_TCB_OFFSET: usize = 0xB000_0000;

    /// Offset to user arguments
    pub const USER_ARG_OFFSET: usize = USER_OFFSET + PML4_SIZE/2;

    /// Offset to user heap
    pub const USER_HEAP_OFFSET: usize = USER_OFFSET + PML4_SIZE;
    pub const USER_HEAP_PML4: usize = (USER_HEAP_OFFSET & PML4_MASK)/PML4_SIZE;

    /// Offset to user grants
    pub const USER_GRANT_OFFSET: usize = USER_HEAP_OFFSET + PML4_SIZE;
    pub const USER_GRANT_PML4: usize = (USER_GRANT_OFFSET & PML4_MASK)/PML4_SIZE;

    /// Offset to user stack
    pub const USER_STACK_OFFSET: usize = USER_GRANT_OFFSET + PML4_SIZE;
    pub const USER_STACK_PML4: usize = (USER_STACK_OFFSET & PML4_MASK)/PML4_SIZE;
    /// Size of user stack
    pub const USER_STACK_SIZE: usize = 1024 * 1024; // 1 MB

    /// Offset to user sigstack
    pub const USER_SIGSTACK_OFFSET: usize = USER_STACK_OFFSET + PML4_SIZE;
    pub const USER_SIGSTACK_PML4: usize = (USER_SIGSTACK_OFFSET & PML4_MASK)/PML4_SIZE;
    /// Size of user sigstack
    pub const USER_SIGSTACK_SIZE: usize = 256 * 1024; // 256 KB

    /// Offset to user TLS
    pub const USER_TLS_OFFSET: usize = USER_SIGSTACK_OFFSET + PML4_SIZE;
    pub const USER_TLS_PML4: usize = (USER_TLS_OFFSET & PML4_MASK)/PML4_SIZE;

    /// Offset to user temporary image (used when cloning)
    pub const USER_TMP_OFFSET: usize = USER_TLS_OFFSET + PML4_SIZE;
    pub const USER_TMP_PML4: usize = (USER_TMP_OFFSET & PML4_MASK)/PML4_SIZE;

    /// Offset to user temporary heap (used when cloning)
    pub const USER_TMP_HEAP_OFFSET: usize = USER_TMP_OFFSET + PML4_SIZE;
    pub const USER_TMP_HEAP_PML4: usize = (USER_TMP_HEAP_OFFSET & PML4_MASK)/PML4_SIZE;

    /// Offset to user temporary page for grants
    pub const USER_TMP_GRANT_OFFSET: usize = USER_TMP_HEAP_OFFSET + PML4_SIZE;
    pub const USER_TMP_GRANT_PML4: usize = (USER_TMP_GRANT_OFFSET & PML4_MASK)/PML4_SIZE;

    /// Offset to user temporary stack (used when cloning)
    pub const USER_TMP_STACK_OFFSET: usize = USER_TMP_GRANT_OFFSET + PML4_SIZE;
    pub const USER_TMP_STACK_PML4: usize = (USER_TMP_STACK_OFFSET & PML4_MASK)/PML4_SIZE;

    /// Offset to user temporary sigstack (used when cloning)
    pub const USER_TMP_SIGSTACK_OFFSET: usize = USER_TMP_STACK_OFFSET + PML4_SIZE;
    pub const USER_TMP_SIGSTACK_PML4: usize = (USER_TMP_SIGSTACK_OFFSET & PML4_MASK)/PML4_SIZE;

    /// Offset to user temporary tls (used when cloning)
    pub const USER_TMP_TLS_OFFSET: usize = USER_TMP_SIGSTACK_OFFSET + PML4_SIZE;
    pub const USER_TMP_TLS_PML4: usize = (USER_TMP_TLS_OFFSET & PML4_MASK)/PML4_SIZE;

    /// Offset for usage in other temporary pages
    pub const USER_TMP_MISC_OFFSET: usize = USER_TMP_TLS_OFFSET + PML4_SIZE;
    pub const USER_TMP_MISC_PML4: usize = (USER_TMP_MISC_OFFSET & PML4_MASK)/PML4_SIZE;
