#![feature(panic_implementation)]
#![feature(compiler_builtins_lib, lang_items, asm, used)]
#![no_builtins]
#![no_std]
#![feature(alloc)]

#![feature(alloc_error_handler)] 
#![feature(allocator_api)]
#![feature(min_const_fn)]

//Temporary
#![allow(dead_code)]

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate alloc;

pub mod lang_items;

mod arch;
mod memory;
mod process;
mod syscall;

mod uart;
mod gpio;
mod mailbox;
mod panic;


mod kwriter;
use core::fmt::Write;

use memory::FrameAllocator;
use memory::paging::Page;
use memory::paging::entry::EntryFlags;
use memory::heap_allocator::BumpAllocator;

// Required here to make them accessable to ASM
pub use syscall::int_syscall;
pub use arch::aarch64::trap::do_el1h_sync;

extern "C" {
    fn exit();
    static mut kernel_end: u8;
}

//--------------------------------------------------------------------

pub const HEAP_START: usize = 42 * 512 * 512 * 4096;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

//TODO: Update this thing to bring it online after memory system is initialized
#[global_allocator]
static HEAP_ALLOCATOR: BumpAllocator = BumpAllocator::new(
    HEAP_START,
    HEAP_START + HEAP_SIZE);

#[no_mangle]
pub unsafe extern "C" fn kmain()
{
    uart::uart_init();

    write!(kwriter::WRITER, "UART init\n");
    write!(kwriter::WRITER, "Building kernel page tables\n");

    let kernel_end_addr =  (&kernel_end as *const _) as usize;

    write!(kwriter::WRITER, "Kernel ends at {}\n", kernel_end_addr);
    
    let frame_allocator 
        = &mut memory::AreaFrameAllocator::new(kernel_end_addr);

    //Turn off identity mapping!
    memory::clear_el0();

    //TODO: Move this into a static variable - there is only 1 true kernel page table
    // Read our bootstrap page table
    // extern "C" {
    //     static mut __page_tables_start: u8;
    // }   

    // let page_table_addr =  (& __page_tables_start as *const _ as usize) | memory::KERNEL_ADDRESS_START;
    // let page_table_ptr: *mut Table<Level4> = page_table_addr as *mut _;
    // let kernel_page_table = &mut (*page_table_ptr);

    //------------------------------------------------
    //TODO: This should be all fixed up to use UserAddress or KernelAddress
    let addr = 42 * 512 * 512 * 4096; 

    let (user_table1, frame_allocator) 
        = memory::paging::table::new(frame_allocator);
    let mut process1 = process::Process{page_table: user_table1};

    {
        let page = Page::containing_address(addr);
        let frame = frame_allocator
            .allocate_frame()
            .expect("no more frames");

        memory::paging::map_to(
            process1.page_table, 
            page, 
            frame, 
            EntryFlags::empty(), 
            frame_allocator);
    }

    //TODO: Why doesn't borrow checker complain?
    //TODO: This should also activate the "Process" page table
    process::switch_process(&mut process1);
    //process::switch_process(&mut process1);

    let (user_table2, _frame_allocator) 
        = memory::paging::table::new(frame_allocator);

    // {
    //     let page = Page::containing_address(addr);
    //     let frame = frame_allocator
    //         .allocate_frame()
    //         .expect("no more frames");

    //     memory::paging::map_to(
    //         user_table2, 
    //         page, 
    //         frame, 
    //         EntryFlags::empty(), 
    //         frame_allocator);
    // }

    // Question: Are interrupts masked during a Sync Exception?

    // Test out the mapping
    let data : *mut usize = addr as *mut usize;

    memory::activate_el0(process1.page_table);
    
    *data = 1024;
    write!(
        kwriter::WRITER, 
        "UPT1: Data at data: 0x{:X?}\n", 
        *data);

    memory::activate_el0(user_table2);

    write!(
        kwriter::WRITER, 
        "Data at data: 0x{:X?}\n", 
        *data);
    
    // memory::activate_el0(user_table1);

    // write!(
    //     kwriter::WRITER, 
    //     "UPT1: Data at data: 0x{:X?}\n", 
    //     *data);


    // let mut vec_test = vec![1,2,3,4,5,6,7];
    // vec_test[3] = 42;
    // for i in &vec_test {
    //     write!(
    //     kwriter::WRITER,"{} ", i);
    // }


    // {
    //     let page = Page::containing_address(addr);
    //     memory::paging::unmap(
    //         user_table1,
    //         page, 
    //         frame_allocator);

    //     memory::flush_tlb();
    // }

    write!(kwriter::WRITER, "Exiting jimOS\n");
    exit();
    
    // Create a new process
    // Schedule process
}

