#![feature(panic_implementation)]
#![feature(compiler_builtins_lib, lang_items, asm, used)]
#![no_builtins]
#![no_std]

pub mod lang_items;

mod memory;

mod uart;
//mod user;
mod syscall;
mod kwriter;
mod gpio;
mod mailbox;
mod panic;

use core::fmt;
use core::fmt::Write;

use memory::FrameAllocator;


pub use syscall::int_syscall;



extern "C" {
    //fn start_userspace();
    fn exit();
    static mut kernel_end: u8;
    //static mut __tbss_end: u8;
}

//--------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn kmain()
{
    //TODO: Map the HardwareIO
    uart::uart_init();

    write!(kwriter::WRITER, "UART init\n");

    write!(kwriter::WRITER, "Building kernel page tables\n");

    // Create a new process
    // Schedule process

    let kernel_end_addr =  & kernel_end as *const _ as usize;

    write!(kwriter::WRITER, "Kernel ends at {}\n", kernel_end_addr);
    
    let mut frame_allocator = memory::AreaFrameAllocator::new(kernel_end_addr);

    for i in 0.. {
        if let None = frame_allocator.allocate_frame() {
            write!(kwriter::WRITER, "allocated {} frames\n", i + 1);
            break;
        }
    }

    // Call switch_task

    write!(kwriter::WRITER, "Exiting jimOS\n");
    exit();
    //start_userspace();
}

//------------------------------------------------------------------------
//------------------------------------------------------------------------

// #[no_mangle]
// pub unsafe extern "C" fn user_main()
// {
//     //panic!();

//     // User space code - need to move this into a separate context
//     let result = user::syscall::syscall(user::SYS_WRITE);

//     //user::print();

//     write!(kwriter::WRITER, "Syscall result: {}\n", result == user::SYSCALL_OK);
//     user::syscall::syscall(user::SYS_EXIT);
// }

//------------------------------------------------------------------------
//------------------------------------------------------------------------

//const STDOUT:       u32 = 0;
