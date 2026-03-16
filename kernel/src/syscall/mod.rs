//! System Call Interface
//! Unix-like system calls for FeatherCore OS
//! 
//! 系统调用接口
//! FeatherCore OS 的类 Unix 系统调用

#![no_std]

/// System call numbers
/// 系统调用号
#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum SyscallNumber {
    // Process control / 进程控制
    SYS_EXIT = 1,
    SYS_FORK = 2,
    SYS_READ = 3,
    SYS_WRITE = 4,
    SYS_OPEN = 5,
    SYS_CLOSE = 6,
    SYS_WAITPID = 7,
    SYS_CREAT = 8,
    SYS_LINK = 9,
    SYS_UNLINK = 10,
    SYS_EXECVE = 11,
    SYS_CHDIR = 12,
    SYS_TIME = 13,
    SYS_MKNOD = 14,
    SYS_CHMOD = 15,
    SYS_GETPID = 20,
    
    // Memory management / 内存管理
    SYS_MMAP = 90,
    SYS_MUNMAP = 91,
    SYS_MPROTECT = 92,
    SYS_BRK = 93,
    
    // File system operations / 文件系统操作
    SYS_STAT = 4,
    SYS_LSEEK = 8,
    SYS_GETDENTS = 89,
    SYS_FSTAT = 94,
    SYS_ACCESS = 33,
    SYS_READLINK = 88,
    
    // Device control / 设备控制
    SYS_IOCTL = 54,
    SYS_FCNTL = 55,
    
    // Socket operations / 套接字操作
    SYS_SOCKET = 97,
    SYS_CONNECT = 98,
    SYS_ACCEPT = 99,
    SYS_SEND = 100,
    SYS_RECV = 101,
}

/// System call arguments
/// 系统调用参数
#[repr(C)]
pub struct SyscallArgs {
    pub arg0: usize,
    pub arg1: usize,
    pub arg2: usize,
    pub arg3: usize,
    pub arg4: usize,
    pub arg5: usize,
}

/// System call result
/// 系统调用结果
#[repr(C)]
pub struct SyscallResult {
    pub ret0: usize,
    pub ret1: usize,
}

/// System call handler function type
/// 系统调用处理函数类型
pub type SyscallHandler = fn(&SyscallArgs) -> SyscallResult;

/// System call table
/// 系统调用表
pub struct SyscallTable {
    handlers: [Option<SyscallHandler>; 256],
}

impl SyscallTable {
    /// Create a new syscall table
    /// 创建新的系统调用表
    pub const fn new() -> Self {
        SyscallTable {
            handlers: [None; 256],
        }
    }
    
    /// Register a syscall handler
    /// 注册系统调用处理程序
    pub fn register(&mut self, num: usize, handler: SyscallHandler) {
        if num < 256 {
            self.handlers[num] = Some(handler);
        }
    }
    
    /// Get a syscall handler
    /// 获取系统调用处理程序
    pub fn get_handler(&self, num: usize) -> Option<SyscallHandler> {
        if num < 256 {
            self.handlers[num]
        } else {
            None
        }
    }
}

/// Handle a system call
/// 处理系统调用
pub fn handle_syscall(num: usize, args: &SyscallArgs) -> SyscallResult {
    // TODO: Get syscall table from kernel
    match num {
        1 => sys_exit(args),
        3 => sys_read(args),
        4 => sys_write(args),
        5 => sys_open(args),
        6 => sys_close(args),
        20 => sys_getpid(args),
        _ => SyscallResult { ret0: usize::MAX, ret1: 0 },
    }
}

/// SYS_EXIT - Exit current process
/// 退出当前进程
fn sys_exit(args: &SyscallArgs) -> SyscallResult {
    let exit_code = args.arg0;
    // TODO: Implement process exit
    // Clean up resources, close file descriptors, etc.
    // 清理资源，关闭文件描述符等
    crate::info!("Process exit with code: {}", exit_code);
    SyscallResult { ret0: 0, ret1: 0 }
}

/// SYS_READ - Read from file descriptor
/// 从文件描述符读取
fn sys_read(args: &SyscallArgs) -> SyscallResult {
    let fd = args.arg0;
    let buf = args.arg1 as *mut u8;
    let count = args.arg2;
    
    // TODO: Implement read from VFS
    // 从虚拟文件系统读取
    unsafe {
        // Placeholder implementation
        if !buf.is_null() && count > 0 {
            // For now, just return 0 bytes read
            // 目前返回 0 字节
        }
    }
    
    SyscallResult { ret0: 0, ret1: 0 }
}

/// SYS_WRITE - Write to file descriptor
/// 写入文件描述符
fn sys_write(args: &SyscallArgs) -> SyscallResult {
    let fd = args.arg0;
    let buf = args.arg1 as *const u8;
    let count = args.arg2;
    
    // TODO: Implement write to VFS
    // 写入虚拟文件系统
    unsafe {
        if !buf.is_null() && count > 0 {
            // For now, just return count as bytes written
            // 目前返回写入的字节数
        }
    }
    
    SyscallResult { ret0: count, ret1: 0 }
}

/// SYS_OPEN - Open file
/// 打开文件
fn sys_open(args: &SyscallArgs) -> SyscallResult {
    let pathname = args.arg0 as *const u8;
    let flags = args.arg1;
    let mode = args.arg2;
    
    // TODO: Implement open in VFS
    // 在虚拟文件系统中打开
    SyscallResult { ret0: usize::MAX, ret1: 0 }
}

/// SYS_CLOSE - Close file descriptor
/// 关闭文件描述符
fn sys_close(args: &SyscallArgs) -> SyscallResult {
    let fd = args.arg0;
    
    // TODO: Implement close in VFS
    // 在虚拟文件系统中关闭
    SyscallResult { ret0: 0, ret1: 0 }
}

/// SYS_GETPID - Get process ID
/// 获取进程 ID
fn sys_getpid(args: &SyscallArgs) -> SyscallResult {
    // TODO: Get actual process ID
    // 获取实际的进程 ID
    SyscallResult { ret0: 1, ret1: 0 }
}

/// Initialize syscall interface
/// 初始化系统调用接口
pub fn init() {
    crate::info!("Syscall interface initialized");
}
