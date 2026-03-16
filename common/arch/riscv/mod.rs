//! RISC-V Architecture Module
//! 
//! Supports:
//! - RV32IMAC: 32-bit with Integer, Multiply, Atomic, Compressed extensions
//! - RV64GC: 64-bit with General Purpose, Compressed extensions (includes M, A, F, D)
//! 
//! # Features
//! 
//! - `riscv32imac`: RV32IMAC (32-bit embedded profile)
//! - `riscv64gc`: RV64GC (64-bit application profile)
//! 
//! # Examples
//! 
//! For RV32IMAC (embedded):
//! ```toml
//! [dependencies]
//! feathercore-arch = { path = "../common/arch", features = ["riscv32imac", "no_mmu"] }
//! ```
//! 
//! For RV64GC (application):
//! ```toml
//! [dependencies]
//! feathercore-arch = { path = "../common/arch", features = ["riscv64gc", "with_mmu"] }
//! ```

// RV32IMAC specific modules
#[cfg(feature = "riscv32imac")]
pub mod rv32imac;

// RV64GC specific modules
#[cfg(feature = "riscv64gc")]
pub mod rv64gc;

// Re-export based on selected variant
#[cfg(feature = "riscv32imac")]
pub use rv32imac::*;

#[cfg(feature = "riscv64gc")]
pub use rv64gc::*;

/// RISC-V-specific task context
/// 
/// This structure holds callee-saved registers and other state
/// that needs to be preserved during context switches.
/// 
/// Register convention:
/// - ra: Return address
/// - sp: Stack pointer
/// - gp: Global pointer
/// - tp: Thread pointer
/// - t0-t6: Temporary registers (caller-saved, but some saved in context)
/// - s0-s11: Saved registers (callee-saved)
/// - a0-a7: Argument/return registers
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RiscvTaskContext {
    // Callee-saved registers
    pub ra: usize,  // Return address (x1)
    pub sp: usize,  // Stack pointer (x2)
    pub gp: usize,  // Global pointer (x3)
    pub tp: usize,  // Thread pointer (x4)
    
    // Temporary registers
    pub t0: usize,  // x5
    pub t1: usize,  // x6
    pub t2: usize,  // x7
    
    // Saved registers
    pub s0: usize,  // x8 (FP)
    pub s1: usize,  // x9
    
    // Additional callee-saved registers (s2-s11)
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
}

impl RiscvTaskContext {
    /// Create a new RISC-V task context with default values
    pub const fn new() -> Self {
        RiscvTaskContext {
            ra: 0,
            sp: 0,
            gp: 0,
            tp: 0,
            t0: 0,
            t1: 0,
            t2: 0,
            s0: 0,
            s1: 0,
            s2: 0,
            s3: 0,
            s4: 0,
            s5: 0,
            s6: 0,
            s7: 0,
            s8: 0,
            s9: 0,
            s10: 0,
            s11: 0,
        }
    }
}

impl Default for RiscvTaskContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize user context for RISC-V
/// 
/// Sets up the initial register state for a new user-mode task.
/// 
/// # Arguments
/// 
/// * `context` - Mutable reference to the architecture context array
/// * `entry_point` - Entry point address of the task
/// * `arg` - First argument to pass to the task (in a0)
pub fn init_user_context(context: &mut [usize; 16], entry_point: usize, arg: usize) {
    #[cfg(feature = "riscv32imac")]
    {
        rv32imac::init_user_context(context, entry_point, arg);
    }
    
    #[cfg(feature = "riscv64gc")]
    {
        rv64gc::init_user_context(context, entry_point, arg);
    }
}

/// RISC-V architecture initialization
/// 
/// Initializes RISC-V-specific components including:
/// - Interrupt controller (PLIC/CLIC)
/// - Timer (MTIME)
/// - FPU (if available)
/// - Vector extensions (if available)
pub fn init() {
    #[cfg(feature = "riscv32imac")]
    {
        rv32imac::init();
    }
    
    #[cfg(feature = "riscv64gc")]
    {
        rv64gc::init();
    }
}

/// Setup exception handlers
/// 
/// Configures the trap vector and exception handlers for the RISC-V core.
pub fn setup_exception_handlers() {
    #[cfg(feature = "riscv32imac")]
    {
        rv32imac::setup_exception_handlers();
    }
    
    #[cfg(feature = "riscv64gc")]
    {
        rv64gc::setup_exception_handlers();
    }
}

/// Exception frame structure
/// 
/// This structure represents the stack frame created by the CPU
/// when an exception or interrupt occurs.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ExceptionFrame {
    // General purpose registers saved on trap
    pub ra: usize,
    pub sp: usize,
    pub gp: usize,
    pub tp: usize,
    pub t0: usize,
    pub t1: usize,
    pub t2: usize,
    pub s0: usize,
    pub s1: usize,
    pub a0: usize,
    pub a1: usize,
    pub a2: usize,
    pub a3: usize,
    pub a4: usize,
    pub a5: usize,
    pub a6: usize,
    pub a7: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub t3: usize,
    pub t4: usize,
    pub t5: usize,
    pub t6: usize,
    
    // Control and status registers
    pub pc: usize,      // Program counter
    pub status: usize,  // mstatus/sstatus
    pub cause: usize,   // mcause/scause
    pub tval: usize,    // mtval/stval
}

/// Context switching for RISC-V
/// 
/// Switches execution from one task to another by saving the current
/// task's context and restoring the target task's context.
/// 
/// # Safety
/// 
/// This function is unsafe because it directly manipulates CPU registers
/// and can cause undefined behavior if contexts are invalid.
pub unsafe fn switch_context(_from: &crate::TaskContext, _to: &crate::TaskContext) {
    #[cfg(feature = "riscv32imac")]
    {
        rv32imac::switch_context(_from, _to);
    }
    
    #[cfg(feature = "riscv64gc")]
    {
        rv64gc::switch_context(_from, _to);
    }
}

/// Start first task on RISC-V
/// 
/// Starts execution of the first task in the system. This function
/// never returns as it transitions to user mode (if MMU is enabled).
/// 
/// # Safety
/// 
/// This function is unsafe because it sets up the initial CPU state
/// and may transition to user mode.
pub unsafe fn start_first_task(_context: &crate::TaskContext) -> ! {
    #[cfg(feature = "riscv32imac")]
    {
        return rv32imac::start_first_task(_context);
    }
    
    #[cfg(feature = "riscv64gc")]
    {
        return rv64gc::start_first_task(_context);
    }
    
    #[cfg(not(any(feature = "riscv32imac", feature = "riscv64gc")))]
    {
        loop {}
    }
}

/// Jump to kernel on RISC-V
/// 
/// Transfers control to the kernel entry point.
/// 
/// # Arguments
/// 
/// * `vector_table_addr` - Address of the device tree or kernel parameters
/// 
/// # Safety
/// 
/// This function is unsafe because it transfers control to arbitrary
/// memory address and never returns.
pub unsafe fn jump_to_kernel(_vector_table_addr: usize) -> ! {
    #[cfg(feature = "riscv32imac")]
    {
        return rv32imac::jump_to_kernel(_vector_table_addr);
    }
    
    #[cfg(feature = "riscv64gc")]
    {
        return rv64gc::jump_to_kernel(_vector_table_addr);
    }
    
    #[cfg(not(any(feature = "riscv32imac", feature = "riscv64gc")))]
    {
        loop {}
    }
}

/// Memory barrier for RISC-V
/// 
/// Ensures all memory operations before this point complete before
/// any operations after this point begin.
#[inline(always)]
pub fn memory_barrier() {
    unsafe {
        core::arch::asm!("fence rw,rw", options(nomem, nostack));
    }
}

/// Wait for interrupt (WFI)
/// 
/// Puts the CPU into low-power mode until an interrupt occurs.
#[inline(always)]
pub fn wfi() {
    unsafe {
        core::arch::asm!("wfi");
    }
}
