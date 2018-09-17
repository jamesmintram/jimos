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
mod gpio;
mod mailbox;
mod panic;

mod kwriter;
use core::fmt::Write;

use memory::FrameAllocator;
use memory::paging::table::{Table, Level4};
use memory::paging::Page;
use memory::paging::entry::EntryFlags;

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

    let kernel_end_addr =  (&kernel_end as *const _) as usize;

    write!(kwriter::WRITER, "Kernel ends at {}\n", kernel_end_addr);
    
    let mut frame_allocator 
        = memory::AreaFrameAllocator::new(kernel_end_addr);

    //Turn off identity mapping!
    memory::clear_el0();

    //Setup "proper" Kernel Page Table
    //Unmap all pages above the kernel_end_addr
    
 

    //Map some memory
    // Create 2 user space page tables
    // Map same VA to different page frames
    // Test that we are actually writing to different memory
    // Map another VA to alias a page frame
    // Test that different addresses can access same memory
    // Unmap memory
    // Test data abort



    // Read our bootstrap page table
    extern "C" {
        static mut __page_tables_start: u8;
    }   

    let page_table_addr =  (& __page_tables_start as *const _ as usize) | memory::KERNEL_ADDRESS_START;
    let page_table_ptr: *mut Table<Level4> = page_table_addr as *mut _;
    let page_table = &mut (*page_table_ptr);

    let _addr1 = memory::virtual_to_physical(page_table, memory::KERNEL_ADDRESS_START);
    let addr2 = memory::virtual_to_physical(page_table, 0x3EADBEEF);
    //let addr3 = memory::virtual_to_physical(page_table, 0xDEADBEEF);


    write!(kwriter::WRITER, "PGT 0x{:X?}\n", memory::PAGE_MASK);
    write!(kwriter::WRITER, "ADDR 0x{:X?}\n", addr2);

    let kern = memory::physical_to_kernel(0xDEADBEEF);
    let phys = memory::kernel_to_physical(kern);

    write!(kwriter::WRITER, "KERN 0x{:X?}\n", kern);
    write!(kwriter::WRITER, "PHYS 0x{:X?}\n", phys);

    //------------------------------------------------

    //TODO: This should be all fixed up to use UserAddress or KernelAddress
    let addr = memory::physical_to_kernel(42 * 512 * 512 * 4096); // 42th P3 entry
    let page = Page::containing_address(addr);
    let frame = frame_allocator
        .allocate_frame()
        .expect("no more frames");

    write!(kwriter::WRITER, "Frame start address: {:X?}\n", frame.start_address());

    write!(kwriter::WRITER, "Mapping: {:X?}\n", addr);

    write!(kwriter::WRITER, "None = {:?}, map to {:?}\n",
            memory::virtual_to_physical(page_table, addr),
            frame);
    
    memory::paging::map_to(
        page_table, 
        page, 
        frame, 
        EntryFlags::empty(), 
        &mut frame_allocator);

    write!(
        kwriter::WRITER, 
        "Some = {:?}\n", 
        memory::virtual_to_physical(page_table, addr));

    write!(
        kwriter::WRITER, 
        "next free frame: {:?}\n", 
        frame_allocator.allocate_frame());

    memory::flush_tlb();

    let pgt : *const usize = memory::physical_to_kernel(0x29D000) as *const usize;
    write!(
        kwriter::WRITER, 
        "Data at PGT: 0x{:X?}\n", 
        *pgt);

    // TODO: Test out the mapping
    

    let data : *mut usize = addr as *mut usize;

    write!(
        kwriter::WRITER, 
        "Data at data: 0x{:X?}\n", 
        *data);

    *data = 1024;

    write!(
        kwriter::WRITER, 
        "Data at data: 0x{:X?}\n", 
        *data);

    write!(kwriter::WRITER, "Exiting jimOS\n");
    exit();
}