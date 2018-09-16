#![feature(panic_implementation)]
#![feature(compiler_builtins_lib, lang_items, asm, used)]
#![no_builtins]
#![no_std]

#[macro_use]
extern crate bitflags;

pub mod lang_items;

mod memory;
mod uart;
mod syscall;
mod kwriter;
mod gpio;
mod mailbox;
mod panic;

use core::fmt::Write;

use memory::FrameAllocator;
use memory::paging::table::{Table, Level4};

pub use syscall::int_syscall;

extern "C" {
    fn exit();
    static mut kernel_end: u8;
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
    
    // Read our bootstrap page table
    extern "C" {
        static mut __page_tables_start: u8;
    }   
    let page_table_addr =  (& __page_tables_start as *const _ as usize) | memory::KERNEL_ADDRESS_START;
    let page_table_ptr: *mut Table<Level4> = page_table_addr as *mut _;
    let page_table = &(*page_table_ptr);
    
    let p3 = page_table.next_table(42)
      .and_then(|p3| p3.next_table(1337));
      

    let addr1 = memory::translate(page_table, 0x3EFFFFFF);
    let addr2 = memory::translate(page_table, 0x3EADBEEF);
    //let addr3 = memory::translate(page_table, 0xDEADBEEF);


    write!(kwriter::WRITER, "PGT 0x{:X?}\n", memory::PAGE_MASK);
    write!(kwriter::WRITER, "ADDR 0x{:X?}\n", addr2);


    //TODO: Clear out the user

    let kern = memory::physical_to_kernel(0xDEADBEEF);
    let phys = memory::kernel_to_physical(kern);

    write!(kwriter::WRITER, "KERN 0x{:X?}\n", kern);
    write!(kwriter::WRITER, "PHYS 0x{:X?}\n", phys);


    //.and_then(|p1| p1.next_table(0xcafebabe));


    // write!(kwriter::WRITER, "PGT 0x{:X?}\n", page_table_addr);

    // if let Some(table2) = (*page_table).next_table(0) {
    //     write!(kwriter::WRITER, "Second: 0x{:X?}\n", table2.test_my_addr());
    //     write!(kwriter::WRITER, "    -: 0x{:X?}\n", table2[0].uflags());

    //     if let Some(table3) = table2.next_table(0) {

    //         write!(kwriter::WRITER, "Third: 0x{:X?}\n", table3.test_my_addr());

    //         for i in 0..10 {
    //             if let Some(frame) = table3[i].pointed_frame() {

    //                 write!(kwriter::WRITER, "    -: 0x{:X?} :: 0x{:X?}\n",  frame.start_address(), table3[i].uflags());    
    //             }
    //         }
    //     }
    // }

    //TODO:
    //
    //  Recreate identity map (into pre-allocated memory)
    //  Update register to point to new map
    //  Flush everything
    //  Hopefully not data aborts
    //  

    // Call switch_task

    write!(kwriter::WRITER, "Exiting jimOS\n");
    exit();
}