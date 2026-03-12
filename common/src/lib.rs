//! FeatherCore Common Library
//! FeatherCore 公共库
//! 
//! This library provides common functionality shared between bootloader and kernel.
//! All code in this library must be `no_std` and should not depend on any external
//! libraries except architecture-specific code.
//! 这个库提供了引导加载程序和内核之间共享的通用功能。
//! 这个库中的所有代码必须是 `no_std` 的，除了架构特定的代码外，不应依赖任何外部库。

#![no_std]
#![deny(missing_docs)]

// Enable alloc for dynamic memory allocation support
// 启用 alloc 以支持动态内存分配
extern crate alloc;
#[allow(unused_imports)]
use alloc::{string::String, vec::Vec};

// Re-export architecture modules if features are enabled
// 如果启用了特性，则重新导出架构模块
#[cfg(all(feature = "arm", not(feature = "riscv")))]
pub use feathercore_arch_arm as arch;

#[cfg(all(feature = "riscv", not(feature = "arm")))]
pub use feathercore_arch_riscv as arch;

// Core modules
// 核心模块
pub mod error;
pub mod util;
pub mod devicetree;

// Generated modules
// 生成的模块
#[cfg(feature = "devicetree")]
pub mod generated;

// Optional modules (enabled by features)
// 可选模块（通过特性启用）
#[cfg(feature = "async")]
pub mod async_rt;

#[cfg(feature = "async")]
pub use async_rt::{AsyncExecutor, delay, yield_now};

#[cfg(feature = "mm")]
pub mod mm;

#[cfg(feature = "sync")]
pub mod sync;

#[cfg(feature = "driver")]
pub mod driver;

#[cfg(feature = "fs")]
pub mod fs;

/// Global allocator for no_std environment
/// 用于 no_std 环境的全局分配器
#[cfg(feature = "mm")]
mod heap_alloc {
    use core::alloc::{GlobalAlloc, Layout};
    use core::ptr::null_mut;

    /// Simple global allocator using a static buffer
    /// 使用静态缓冲区的简单全局分配器
    #[repr(align(8))]
    pub struct BumpAlloc {
        buf: [u8; 32 * 1024], // 32KB buffer
    }

    impl BumpAlloc {
        pub const fn new() -> Self {
            Self {
                buf: [0; 32 * 1024],
            }
        }
    }

    unsafe impl GlobalAlloc for BumpAlloc {
        unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
            use core::sync::atomic::{AtomicUsize, Ordering};
            static POS: AtomicUsize = AtomicUsize::new(0);
            static mut BUF: [u8; 32 * 1024] = [0; 32 * 1024];
            
            let pos = POS.load(Ordering::Relaxed);
            let aligned_pos = (pos + _layout.align() - 1) & !(_layout.align() - 1);
            
            if aligned_pos + _layout.size() > BUF.len() {
                null_mut()
            } else {
                POS.store(aligned_pos + _layout.size(), Ordering::Relaxed);
                BUF.as_mut_ptr().add(aligned_pos)
            }
        }

        unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        }
    }
}

/// Global allocator instance
/// 全局分配器实例
#[cfg(feature = "mm")]
#[global_allocator]
static GLOBAL_ALLOC: heap_alloc::BumpAlloc = heap_alloc::BumpAlloc::new();

/// Common error type for FeatherCore
/// FeatherCore 的通用错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Invalid argument
    /// 无效参数
    InvalidArgument,
    /// Operation not supported
    /// 操作不支持
    NotSupported,
    /// Resource busy
    /// 资源忙
    Busy,
    /// Timeout
    /// 超时
    Timeout,
    /// Out of memory
    /// 内存不足
    OutOfMemory,
    /// Device error
    /// 设备错误
    DeviceError,
    /// Filesystem error
    /// 文件系统错误
    FilesystemError,
    /// Other error
    /// 其他错误
    Other,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::InvalidArgument => write!(f, "Invalid argument"),
            Error::NotSupported => write!(f, "Operation not supported"),
            Error::Busy => write!(f, "Resource busy"),
            Error::Timeout => write!(f, "Operation timeout"),
            Error::OutOfMemory => write!(f, "Out of memory"),
            Error::DeviceError => write!(f, "Device error"),
            Error::FilesystemError => write!(f, "Filesystem error"),
            Error::Other => write!(f, "Other error"),
        }
    }
}

/// Result type for FeatherCore operations
/// FeatherCore 操作的 Result 类型
pub type Result<T> = core::result::Result<T, Error>;