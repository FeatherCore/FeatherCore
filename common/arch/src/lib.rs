#![no_std]

//! FeatherCore Architecture Support Library
//! 
//! This library provides multi-architecture support for FeatherCore OS.
//! It supports ARM (M-profile and A-profile) and RISC-V architectures.
//! 
//! # Supported Architectures
//! 
//! ## ARM M-profile (Microcontroller)
//! - ARMv6-M: Cortex-M0, Cortex-M0+
//! - ARMv7-M: Cortex-M3, Cortex-M4
//! - ARMv7-EM: Cortex-M7, Cortex-M4 with FPU
//! - ARMv8-M: Cortex-M23, Cortex-M33, Cortex-M55
//! 
//! ## ARM A-profile (Application Processor)
//! - ARMv7-A: Cortex-A5, Cortex-A7, Cortex-A8, Cortex-A15
//! - ARMv8-A: Cortex-A53, Cortex-A72
//! - ARMv9-A: Latest application processors
//! 
//! ## RISC-V
//! - RV32IMAC: 32-bit with Integer, Multiply, Atomic, Compressed
//! - RV64GC: 64-bit General Purpose with Compressed extensions
//! 
//! # Usage
//! 
//! Enable architecture via Cargo features:
//! ```toml
//! [dependencies]
//! feathercore-arch = { path = "../common/arch", features = ["armv7-m"] }
//! # or
//! feathercore-arch = { path = "../common/arch", features = ["armv7-a", "with_mmu"] }
//! # or
//! feathercore-arch = { path = "../common/arch", features = ["riscv32imac"] }
//! ```
//! 
//! # Examples
//! 
//! ```rust,no_run
//! #![no_std]
//! use feathercore_arch::{init, TaskContext};
//! 
//! #[no_mangle]
//! pub extern "C" fn arch_init() {
//!     init();
//! }
//! ```

// ARM architecture module (located at crate root level, not in src/)
#[cfg(feature = "arm")]
#[path = "../arm/mod.rs"]
pub mod arm;

// RISC-V architecture module (located at crate root level, not in src/)
#[cfg(feature = "riscv")]
#[path = "../riscv/mod.rs"]
pub mod riscv;

// Re-export current architecture
#[cfg(feature = "arm")]
pub use arm::*;

#[cfg(feature = "riscv")]
pub use riscv::*;

/// Architecture initialization
/// 
/// This function must be called early in the boot process to initialize
/// architecture-specific components.
pub fn init() {
    #[cfg(feature = "arm")]
    arm::init();
    
    #[cfg(feature = "riscv")]
    riscv::init();
}

/// Task context structure for context switching
/// 
/// This structure holds the CPU register state that needs to be preserved
/// during task switching. The exact layout depends on the architecture.
#[repr(C)]
pub struct TaskContext {
    /// Stack pointer
    pub stack_pointer: usize,
    /// Architecture-specific register context
    pub arch_context: [usize; 16],
}

impl TaskContext {
    /// Create a new task context with default values
    pub const fn new() -> Self {
        TaskContext {
            stack_pointer: 0,
            arch_context: [0; 16],
        }
    }
    
    /// Initialize task context for user stack
    pub fn init_user_stack(&mut self, stack_top: usize, entry_point: usize, arg: usize) {
        self.stack_pointer = stack_top;
        #[cfg(feature = "arm")]
        arm::init_user_context(&mut self.arch_context, entry_point, arg);
        
        #[cfg(feature = "riscv")]
        riscv::init_user_context(&mut self.arch_context, entry_point, arg);
    }
}

impl Default for TaskContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Context switching function
/// 
/// Switches execution from one task to another. This is a low-level function
/// that saves the current CPU state and restores the target task's state.
/// 
/// # Safety
/// 
/// This function is unsafe because it:
/// - Directly manipulates CPU registers
/// - Can cause undefined behavior if contexts are invalid
/// - Never returns in the traditional sense (jumps to target task)
pub unsafe fn switch_context(_from: &TaskContext, _to: &TaskContext) {
    #[cfg(feature = "arm")]
    arm::switch_context(_from, _to);
    
    #[cfg(feature = "riscv")]
    riscv::switch_context(_from, _to);
}

/// Start the first task
/// 
/// This function starts execution of the first task in the system.
/// It never returns.
/// 
/// # Safety
/// 
/// This function is unsafe because it:
/// - Sets up initial CPU state
/// - Transitions to user mode (if supported)
/// - Never returns
pub unsafe fn start_first_task(context: &TaskContext) -> ! {
    #[cfg(feature = "arm")]
    return arm::start_first_task(context);
    
    #[cfg(feature = "riscv")]
    return riscv::start_first_task(context);
    
    #[cfg(not(any(feature = "arm", feature = "riscv")))]
    loop {}
}

/// Jump to kernel entry point
/// 
/// Used by bootloaders to jump to the kernel with proper architecture setup.
/// 
/// # Safety
/// 
/// This function is unsafe because it:
/// - Transfers control to arbitrary memory address
/// - Expects valid kernel image at target address
/// - Never returns
pub unsafe fn jump_to_kernel(vector_table_addr: usize) -> ! {
    #[cfg(feature = "arm")]
    return arm::jump_to_kernel(vector_table_addr);
    
    #[cfg(feature = "riscv")]
    return riscv::jump_to_kernel(vector_table_addr);
    
    #[cfg(not(any(feature = "arm", feature = "riscv")))]
    loop {}
}

/// Architecture-specific exception handler setup
#[cfg(feature = "arm")]
pub use arm::{setup_exception_handlers, ExceptionFrame};

#[cfg(feature = "riscv")]
pub use riscv::{setup_exception_handlers, ExceptionFrame};

/// Memory barrier macro for architecture
#[macro_export]
macro_rules! memory_barrier {
    () => {
        #[cfg(feature = "arm")]
        arm::memory_barrier();
        
        #[cfg(feature = "riscv")]
        riscv::memory_barrier();
    };
}

/// Wait for interrupt macro
#[macro_export]
macro_rules! wfi {
    () => {
        #[cfg(feature = "arm")]
        arm::wfi();
        
        #[cfg(feature = "riscv")]
        riscv::wfi();
    };
}
