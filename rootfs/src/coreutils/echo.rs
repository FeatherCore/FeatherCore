//! echo - display a line of text

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
    // TODO: Implement echo command using kernel syscalls
    loop {}
}
