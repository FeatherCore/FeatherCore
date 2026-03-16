//! ARM Architecture Module
//! 
//! Supports:
//! - ARMv6-M (Cortex-M0, Cortex-M0+)
//! - ARMv7-M (Cortex-M3, Cortex-M4)
//! - ARMv7-EM (Cortex-M7, Cortex-M4 with FPU)
//! - ARMv8-M (Cortex-M23, Cortex-M33, Cortex-M55)
//! - ARMv7-A (Cortex-A5, Cortex-A7, Cortex-A8, Cortex-A15)
//! - ARMv8-A (Cortex-A53, Cortex-A72)
//! - ARMv9-A
//! 
//! # Features
//! 
//! - `armv6-m`: ARMv6-M profile (Cortex-M0/M0+)
//! - `armv7-m`: ARMv7-M profile (Cortex-M3/M4)
//! - `armv7-em`: ARMv7-EM profile (Cortex-M7/M4 with FPU)
//! - `armv8-m-base`: ARMv8-M Baseline (Cortex-M23)
//! - `armv8-m-main`: ARMv8-M Mainline (Cortex-M33/M55)
//! - `armv7-a`: ARMv7-A profile (Application processors)
//! - `armv8-a`: ARMv8-A profile (64-bit application processors)
//! - `armv9-a`: ARMv9-A profile (Latest application processors)
//! 
//! # Examples
//! 
//! For ARMv7-M (Cortex-M3/M4):
//! ```toml
//! [dependencies]
//! feathercore-arch = { path = "../common/arch", features = ["armv7-m", "no_mmu"] }
//! ```
//! 
//! For ARMv7-A (Cortex-A7/A15):
//! ```toml
//! [dependencies]
//! feathercore-arch = { path = "../common/arch", features = ["armv7-a", "with_mmu"] }
//! ```

// M-profile specific modules
#[cfg(any(
    feature = "armv6-m",
    feature = "armv7-m",
    feature = "armv7-em",
    feature = "armv8-m-base",
    feature = "armv8-m-main"
))]
pub mod m_profile;

// A-profile specific modules
#[cfg(any(
    feature = "armv7-a",
    feature = "armv8-a",
    feature = "armv9-a"
))]
pub mod a_profile;

// Re-export based on selected profile
#[cfg(any(
    feature = "armv6-m",
    feature = "armv7-m",
    feature = "armv7-em",
    feature = "armv8-m-base",
    feature = "armv8-m-main"
))]
pub use m_profile::*;

#[cfg(any(
    feature = "armv7-a",
    feature = "armv8-a",
    feature = "armv9-a"
))]
pub use a_profile::*;

/// ARM-specific task context
/// 
/// This structure holds all callee-saved registers and control registers
/// that need to be preserved during context switches.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ArmTaskContext {
    // Callee-saved registers (R4-R11)
    pub r4: usize,
    pub r5: usize,
    pub r6: usize,
    pub r7: usize,
    pub r8: usize,
    pub r9: usize,
    pub r10: usize,
    pub r11: usize,
    
    // Stack pointer (R13)
    pub sp: usize,
    
    // Link register (R14)
    pub lr: usize,
    
    // Program counter (R15)
    pub pc: usize,
    
    // Program status register
    pub xpsr: usize,
}

impl ArmTaskContext {
    /// Create a new ARM task context with default values
    pub const fn new() -> Self {
        ArmTaskContext {
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            sp: 0,
            lr: 0,
            pc: 0,
            xpsr: 0,
        }
    }
}

impl Default for ArmTaskContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize user context for ARM
/// 
/// Sets up the initial register state for a new user-mode task.
/// 
/// # Arguments
/// 
/// * `context` - Mutable reference to the architecture context array
/// * `entry_point` - Entry point address of the task
/// * `arg` - First argument to pass to the task
pub fn init_user_context(context: &mut [usize; 16], entry_point: usize, arg: usize) {
    #[cfg(any(
        feature = "armv6-m",
        feature = "armv7-m",
        feature = "armv7-em",
        feature = "armv8-m-base",
        feature = "armv8-m-main"
    ))]
    {
        m_profile::init_user_context(context, entry_point, arg);
    }
    
    #[cfg(any(
        feature = "armv7-a",
        feature = "armv8-a",
        feature = "armv9-a"
    ))]
    {
        a_profile::init_user_context(context, entry_point, arg);
    }
}

/// ARM architecture initialization
/// 
/// Initializes ARM-specific components including:
/// - Vector table
/// - Exception handlers
/// - Floating point unit (if available)
/// - Memory protection unit (if available)
pub fn init() {
    #[cfg(any(
        feature = "armv6-m",
        feature = "armv7-m",
        feature = "armv7-em",
        feature = "armv8-m-base",
        feature = "armv8-m-main"
    ))]
    {
        m_profile::init();
    }
    
    #[cfg(any(
        feature = "armv7-a",
        feature = "armv8-a",
        feature = "armv9-a"
    ))]
    {
        a_profile::init();
    }
}

/// Setup exception handlers
/// 
/// Configures the vector table and exception handlers for the ARM core.
pub fn setup_exception_handlers() {
    #[cfg(any(
        feature = "armv6-m",
        feature = "armv7-m",
        feature = "armv7-em",
        feature = "armv8-m-base",
        feature = "armv8-m-main"
    ))]
    {
        m_profile::setup_exception_handlers();
    }
    
    #[cfg(any(
        feature = "armv7-a",
        feature = "armv8-a",
        feature = "armv9-a"
    ))]
    {
        a_profile::setup_exception_handlers();
    }
}

/// Exception frame structure
/// 
/// This structure represents the stack frame created by the CPU
/// when an exception occurs.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ExceptionFrame {
    pub r0: usize,
    pub r1: usize,
    pub r2: usize,
    pub r3: usize,
    pub r12: usize,
    pub lr: usize,
    pub pc: usize,
    pub xpsr: usize,
}

/// Context switching for ARM
/// 
/// Switches execution from one task to another by saving the current
/// task's context and restoring the target task's context.
/// 
/// # Safety
/// 
/// This function is unsafe because it directly manipulates CPU registers
/// and can cause undefined behavior if contexts are invalid.
pub unsafe fn switch_context(_from: &crate::TaskContext, _to: &crate::TaskContext) {
    #[cfg(any(
        feature = "armv6-m",
        feature = "armv7-m",
        feature = "armv7-em",
        feature = "armv8-m-base",
        feature = "armv8-m-main"
    ))]
    {
        m_profile::switch_context(_from, _to);
    }
    
    #[cfg(any(
        feature = "armv7-a",
        feature = "armv8-a",
        feature = "armv9-a"
    ))]
    {
        a_profile::switch_context(_from, _to);
    }
}

/// Start first task on ARM
/// 
/// Starts execution of the first task in the system. This function
/// never returns as it transitions to user mode.
/// 
/// # Safety
/// 
/// This function is unsafe because it sets up the initial CPU state
/// and transitions to user mode.
pub unsafe fn start_first_task(_context: &crate::TaskContext) -> ! {
    #[cfg(any(
        feature = "armv6-m",
        feature = "armv7-m",
        feature = "armv7-em",
        feature = "armv8-m-base",
        feature = "armv8-m-main"
    ))]
    {
        return m_profile::start_first_task(_context);
    }
    
    #[cfg(any(
        feature = "armv7-a",
        feature = "armv8-a",
        feature = "armv9-a"
    ))]
    {
        return a_profile::start_first_task(_context);
    }
    
    #[cfg(not(any(
        feature = "armv6-m",
        feature = "armv7-m",
        feature = "armv7-em",
        feature = "armv8-m-base",
        feature = "armv8-m-main",
        feature = "armv7-a",
        feature = "armv8-a",
        feature = "armv9-a"
    )))]
    {
        loop {}
    }
}

/// Jump to kernel on ARM
/// 
/// Transfers control to the kernel entry point with proper ARM setup.
/// This is typically used by bootloaders.
/// 
/// # Arguments
/// 
/// * `vector_table_addr` - Address of the kernel vector table
/// 
/// # Safety
/// 
/// This function is unsafe because it transfers control to arbitrary
/// memory address and never returns.
pub unsafe fn jump_to_kernel(_vector_table_addr: usize) -> ! {
    #[cfg(any(
        feature = "armv6-m",
        feature = "armv7-m",
        feature = "armv7-em",
        feature = "armv8-m-base",
        feature = "armv8-m-main"
    ))]
    {
        return m_profile::jump_to_kernel(_vector_table_addr);
    }
    
    #[cfg(any(
        feature = "armv7-a",
        feature = "armv8-a",
        feature = "armv9-a"
    ))]
    {
        return a_profile::jump_to_kernel(_vector_table_addr);
    }
    
    #[cfg(not(any(
        feature = "armv6-m",
        feature = "armv7-m",
        feature = "armv7-em",
        feature = "armv8-m-base",
        feature = "armv8-m-main",
        feature = "armv7-a",
        feature = "armv8-a",
        feature = "armv9-a"
    )))]
    {
        loop {}
    }
}

/// Memory barrier for ARM
/// 
/// Ensures all memory operations before this point complete before
/// any operations after this point begin.
#[inline(always)]
pub fn memory_barrier() {
    #[cfg(any(
        feature = "armv6-m",
        feature = "armv7-m",
        feature = "armv7-em",
        feature = "armv8-m-base",
        feature = "armv8-m-main"
    ))]
    {
        unsafe {
            core::arch::asm!("dsb sy", options(nomem, nostack));
            core::arch::asm!("isb sy", options(nomem, nostack));
        }
    }
    
    #[cfg(any(
        feature = "armv7-a",
        feature = "armv8-a",
        feature = "armv9-a"
    ))]
    {
        unsafe {
            core::arch::asm!("dsb sy", options(nomem, nostack));
            core::arch::asm!("isb sy", options(nomem, nostack));
        }
    }
}

/// Wait for interrupt (WFI)
/// 
/// Puts the CPU into low-power mode until an interrupt occurs.
#[inline(always)]
pub fn wfi() {
    #[cfg(any(
        feature = "armv6-m",
        feature = "armv7-m",
        feature = "armv7-em",
        feature = "armv8-m-base",
        feature = "armv8-m-main"
    ))]
    {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
    
    #[cfg(any(
        feature = "armv7-a",
        feature = "armv8-a",
        feature = "armv9-a"
    ))]
    {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}
