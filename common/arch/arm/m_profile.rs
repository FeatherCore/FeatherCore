//! ARM M-Profile Implementation
//! 
//! This module provides ARM M-profile (Microcontroller) specific implementations
//! for ARMv6-M, ARMv7-M, ARMv7-EM, and ARMv8-M architectures.
//! 
//! # Supported Cores
//! 
//! - Cortex-M0/M0+ (ARMv6-M)
//! - Cortex-M3/M4 (ARMv7-M)
//! - Cortex-M7 (ARMv7-EM)
//! - Cortex-M23/M33/M55 (ARMv8-M)

use crate::TaskContext;

/// M-Profile exception handler setup
pub fn setup_exception_handlers() {
    // Vector table setup is platform-specific
    // This is typically done in the platform crate
}

/// Initialize M-profile architecture
pub fn init() {
    // Enable FPU on Cortex-M4F/M7F if available
    #[cfg(any(feature = "armv7-em", feature = "armv8-m-main"))]
    {
        unsafe {
            // CPACR is at address 0xE000ED88
            const CPACR: *mut u32 = 0xE000_ED88 as *mut u32;
            let mut cpacr = core::ptr::read_volatile(CPACR);
            // Enable CP10 and CP11 (FPU)
            cpacr |= (0xF << 20);
            core::ptr::write_volatile(CPACR, cpacr);
            // Memory barrier
            core::arch::asm!("dsb", options(nomem, nostack, preserves_flags));
            core::arch::asm!("isb", options(nomem, nostack, preserves_flags));
        }
    }
}

/// Initialize user context for M-profile
pub fn init_user_context(context: &mut [usize; 16], entry_point: usize, arg: usize) {
    // M-profile context layout:
    // R4-R11: callee-saved registers
    // SP, LR, PC, xPSR: exception return frame
    
    context[0] = 0; // R4
    context[1] = 0; // R5
    context[2] = 0; // R6
    context[3] = 0; // R7
    context[4] = 0; // R8
    context[5] = 0; // R9
    context[6] = 0; // R10
    context[7] = 0; // R11
    context[8] = 0; // SP (will be set by caller)
    context[9] = entry_point as usize; // LR (return address, but we'll use it as PC)
    context[10] = entry_point as usize; // PC (entry point)
    // xPSR: Thumb bit (bit 24) must be set
    context[11] = 1 << 24;
    context[12] = arg; // R0 (first argument)
    context[13] = 0; // R1
    context[14] = 0; // R2
    context[15] = 0; // R3
}

/// Context switching for M-profile
/// 
/// This function saves the current task's context and restores the target task's context.
/// 
/// # Implementation Details
/// 
/// For ARMv7-M and ARMv8-M, this uses the PendSV exception for context switching.
/// For ARMv6-M, a simpler software interrupt approach is used.
/// 
/// # Safety
/// 
/// This function is unsafe because it directly manipulates the stack pointer
/// and CPU registers.
#[cfg(any(feature = "armv7-m", feature = "armv7-em", feature = "armv8-m-main"))]
pub unsafe fn switch_context(_from: &TaskContext, _to: &TaskContext) {
    // Trigger PendSV exception to perform context switch
    // The actual context save/restore happens in the PendSV handler
    const NVIC_ICSR: *mut u32 = 0xE000_ED04 as *mut u32;
    const PENDSVSET: u32 = 1 << 28;
    
    // Set PendSV pending bit
    core::ptr::write_volatile(NVIC_ICSR, PENDSVSET);
    
    // Memory barrier to ensure the write completes
    core::arch::asm!("dsb");
    core::arch::asm!("isb");
    
    // The PendSV handler will perform the actual context switch
    // This is a simplified version - full implementation requires assembly
}

#[cfg(feature = "armv6-m")]
pub unsafe fn switch_context(_from: &TaskContext, _to: &TaskContext) {
    // ARMv6-M implementation (Cortex-M0/M0+)
    // Simpler approach without PendSV
    todo!("ARMv6-M context switching implementation")
}

/// Start first task on M-profile
/// 
/// This function restores the task context and starts execution in user mode.
/// 
/// # Safety
/// 
/// This function is unsafe because it sets up the initial stack frame
/// and triggers exception return to start the task.
#[cfg(any(feature = "armv7-m", feature = "armv7-em", feature = "armv8-m-main"))]
pub unsafe fn start_first_task(context: &TaskContext) -> ! {
    // Restore context from the provided TaskContext
    // Then trigger exception return to start task execution
    
    // For M-profile, we need to set up the stack frame and use
    // exception return mechanism (BX LR with EXC_RETURN value)
    
    // This is a placeholder - full implementation requires inline assembly
    core::arch::asm!(
        "ldr sp, [{context}, 8]", // Load stack pointer
        "ldr r0, [{context}, 40]", // Load argument
        "mov lr, #0xFFFFFFFD", // EXC_RETURN for MSP and Thread mode
        "bx lr",
        context = in(reg) context,
        options(noreturn)
    );
}

#[cfg(feature = "armv6-m")]
pub unsafe fn start_first_task(_context: &TaskContext) -> ! {
    todo!("ARMv6-M first task start implementation")
}

/// Jump to kernel on M-profile
/// 
/// Transfers control to the kernel entry point.
/// 
/// # Safety
/// 
/// This function is unsafe because it transfers control to arbitrary
/// memory address.
pub unsafe fn jump_to_kernel(vector_table_addr: usize) -> ! {
    const VTOR: *mut u32 = 0xE000_ED08 as *mut u32;
    
    // Set vector table offset
    core::ptr::write_volatile(VTOR, vector_table_addr as u32);
    core::arch::asm!("dsb");
    core::arch::asm!("isb");
    
    // Get reset vector from the table
    let reset_vector = *(vector_table_addr as *const u32).add(1);
    
    // Jump to reset handler
    let reset_handler = unsafe {
        core::ptr::read_volatile(&reset_vector as *const u32 as *const unsafe fn() -> !)
    };
    reset_handler()
}

/// Memory barrier for M-profile
#[inline(always)]
pub fn memory_barrier() {
    unsafe {
        core::arch::asm!("dsb sy");
        core::arch::asm!("isb sy");
    }
}

/// Wait for interrupt for M-profile
#[inline(always)]
pub fn wfi() {
    unsafe {
        core::arch::asm!("wfi");
    }
}
