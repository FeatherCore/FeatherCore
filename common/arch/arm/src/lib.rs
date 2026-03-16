#![no_std]

/// ARM architecture initialization
pub fn init() {
    // Architecture initialization is done via feature flags
}

/// Task context structure
#[repr(C)]
pub struct TaskContext {
    // Architecture-specific task context
    pub stack_pointer: usize,
}

/// Create initial task context
pub fn create_task_context(stack_pointer: usize) -> TaskContext {
    TaskContext {
        stack_pointer,
    }
}

/// Start the first task
pub unsafe fn start_first_task(context: &TaskContext) {
    #[cfg(any(feature = "armv6-m", feature = "armv7-m", feature = "armv7-em", feature = "armv8-m-base", feature = "armv8-m-main"))]
    {
        // Cortex-M: Set stack pointer and jump to main
        core::arch::asm!(
            "mov sp, {0}",
            "b main",
            in(reg) context.stack_pointer,
        );
    }
    
    #[cfg(any(feature = "armv7-a", feature = "armv8-a", feature = "armv9-a"))]
    {
        // Cortex-A: Set stack pointer and jump to main
        core::arch::asm!(
            "mov sp, {0}",
            "b main",
            in(reg) context.stack_pointer,
        );
    }
}

/// Switch task context
pub unsafe fn switch_context(_from: &TaskContext, _to: &TaskContext) {
    // TODO: Implement context switching for different ARM architectures
}

/// Architecture-specific utilities
pub mod util {
    /// Enable interrupts
    pub unsafe fn enable_interrupts() {
        #[cfg(any(feature = "armv6-m", feature = "armv7-m", feature = "armv7-em", feature = "armv8-m-base", feature = "armv8-m-main"))]
        { 
            // Cortex-M: Enable interrupts
            core::arch::asm!("cpsie i");
        }
        
        #[cfg(any(feature = "armv7-a", feature = "armv8-a", feature = "armv9-a"))]
        { 
            // Cortex-A: Enable interrupts
            let mut cpsr: u32;
            core::arch::asm!("mrs {}, cpsr", out(reg) cpsr);
            cpsr &= !0x80; // Clear I bit
            core::arch::asm!("msr cpsr_c, {}", in(reg) cpsr);
        }
    }
    
    /// Disable interrupts
    pub unsafe fn disable_interrupts() {
        #[cfg(any(feature = "armv6-m", feature = "armv7-m", feature = "armv7-em", feature = "armv8-m-base", feature = "armv8-m-main"))]
        { 
            // Cortex-M: Disable interrupts
            core::arch::asm!("cpsid i");
        }
        
        #[cfg(any(feature = "armv7-a", feature = "armv8-a", feature = "armv9-a"))]
        { 
            // Cortex-A: Disable interrupts
            let mut cpsr: u32;
            core::arch::asm!("mrs {}, cpsr", out(reg) cpsr);
            cpsr |= 0x80; // Set I bit
            core::arch::asm!("msr cpsr_c, {}", in(reg) cpsr);
        }
    }
    
    /// Get stack pointer
    pub unsafe fn get_stack_pointer() -> usize {
        let sp: usize;
        core::arch::asm!("mov {}, sp", out(reg) sp);
        sp
    }
    
    /// Set stack pointer
    pub unsafe fn set_stack_pointer(sp: usize) {
        core::arch::asm!("mov sp, {}", in(reg) sp);
    }
}