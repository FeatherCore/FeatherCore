//! FeatherCore Shell
//! 
//! A minimal Unix-like shell for FeatherCore OS

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use feathercore_kernel::syscall;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Shell entry point
    loop {
        // TODO: Implement shell main loop using kernel syscalls
        // Use feathercore_kernel::syscall for system calls
    }
}
