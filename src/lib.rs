#![feature(panic_implementation)]
#![feature(compiler_builtins_lib, lang_items, asm, pointer_methods, used)]
#![no_builtins]
#![no_std]

pub mod lang_items;
mod memory;
mod uart;
mod user;
mod syscall;
mod kwriter;
mod gpio;
mod mailbox;

use core::fmt;
use core::fmt::Write;

pub use syscall::int_syscall;

extern "C" {
    fn start_userspace();
    fn exit();
}

//--------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn kmain()
{
    //TODO: Map the HardwareIO
    uart::uart_init();

//    exit();
    //start_userspace();
}

//------------------------------------------------------------------------
//------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn user_main()
{
    //panic!();

    // User space code - need to move this into a separate context
    let result = user::syscall::syscall(user::SYS_WRITE);

    //user::print();

    write!(kwriter::WRITER, "Syscall result: {}\n", result == user::SYSCALL_OK);
    user::syscall::syscall(user::SYS_EXIT);
}

//------------------------------------------------------------------------
//------------------------------------------------------------------------

const STDOUT:       u32 = 0;
