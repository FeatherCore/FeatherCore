#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

pub mod sched;
pub mod task;
pub mod mm;
pub mod irq;
pub mod time;
pub mod sync;
pub mod log;
pub mod future;
pub mod syscall;
pub mod vfs;
pub mod driver;

// Log macros are automatically exported at crate root due to #[macro_export]

/// Kernel initialization
pub fn init() {
    info!("FeatherCore RTOS Initializing...");
    
    // Initialize interrupt system
    irq::init();
    
    // Initialize memory management
    mm::init();
    
    // Initialize time system
    time::init();
    
    // Initialize scheduler
    sched::init();
    
    // Initialize VFS
    vfs::init();
    
    // Initialize syscall interface
    syscall::init();
    
    info!("FeatherCore RTOS Initialized");
}

/// Kernel main loop
pub fn main_loop() -> ! {
    loop {
        // Execute the next task
        sched::schedule();
    }
}


