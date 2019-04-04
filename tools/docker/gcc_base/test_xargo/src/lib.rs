#![no_builtins]
#![no_std]

use core::panic::PanicInfo;


#[panic_handler]
pub fn panic_fmt(_panic_info: &PanicInfo) -> !
{
    loop {}
}