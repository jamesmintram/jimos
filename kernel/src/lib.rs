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

extern crate spin;

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
use memory::heap_allocator::LockedHeap;
use memory::LockedAreaFrameAllocator;



//Temp
use arch::aarch64::arm;
use spin::Mutex;

// Required here to make them accessable to ASM
pub use syscall::int_syscall;
pub use arch::aarch64::trap::do_el1h_sync;

extern "C" {
    fn exit();
    static mut kernel_end: u8;
}

//--------------------------------------------------------------------
//pub const HEAP_START: usize = 1024 * 1024 * 256; // 256MB for now
pub const HEAP_SIZE: usize = 1024 * 1024 * 256; // 256MB for now

static mut KERNEL_FRAME_ALLOCATOR: LockedAreaFrameAllocator = LockedAreaFrameAllocator::empty();
static mut ANON_FRAME_ALLOCATOR: LockedAreaFrameAllocator = LockedAreaFrameAllocator::empty();


#[global_allocator]
static mut HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

#[no_mangle]
pub unsafe extern "C" fn kmain()
{
    arm::reset_ttbr0_el1(); //Turn off identity mapping in EL0
    arm::cache_setup();     
    
    uart::uart_init();

    write!(kwriter::WRITER, "UART init\n");
    
    arm::print_cache_info();

    write!(kwriter::WRITER, "Building kernel page tables\n");

    let kernel_end_addr =  (&kernel_end as *const _) as usize;

    write!(kwriter::WRITER, "Kernel ends at {}\n", kernel_end_addr);

    // Initialise the heaps
    let heap_start = kernel_end_addr + memory::KERNEL_ADDRESS_START;
    let heap_end = heap_start + HEAP_SIZE;
    
    let anon_mem_start = heap_end;
    let anon_mem_end = anon_mem_start + HEAP_SIZE;

    KERNEL_FRAME_ALLOCATOR.init(memory::AreaFrameAllocator::new(kernel_end_addr, anon_mem_start));
    ANON_FRAME_ALLOCATOR.init(memory::AreaFrameAllocator::new(anon_mem_start, anon_mem_end));

    HEAP_ALLOCATOR.init(&KERNEL_FRAME_ALLOCATOR);

    // TODO: Support for deallocate
    //       Slab allocator 
    //       Test to churn the heap 
    //          Should crash before we implement dealloc
    //          Should not crash after we implement dealloc
    //
    //      Virtual Address Space manager
    //          Add/Remove areas
    //          Split/Merge
    //          Fault handler
    //
    //      Test
    //          Kernel address fault = kernel panic

    

    // Heap Test
    //----------------------
    // let mut vec_test = vec![1,2,3,4,5,6,7];
    // vec_test[3] = 42;

    // for i in 0..1098 {
    //     vec_test.push(1);
    // }

    // for i in &vec_test {
    //     write!(kwriter::WRITER,"{} ", i);
    // }

    // write!(
    //     kwriter::WRITER, 
    //     "Pushed some vec\n");

    // // Deadlock test TODO: Make this pass
    // if let Some(ref mut allocator) = *KERNEL_FRAME_ALLOCATOR.lock() {
    //     if let Some(ref mut _allocator) = *KERNEL_FRAME_ALLOCATOR.lock() {
    //         allocator.allocate_frame();
    //     }
    // }   

    //----------------------


    // Global Kernel Page Table
    //----------------------
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
    let addr1 = 42 * 512 * 512 * 4096; 
    let addr2 = 12 * 512 * 512 * 4096; 

    // This manages pageable memory
    // let frame_allocator
    //     = &mut *memory::AreaFrameAllocator::new(anon_mem_start);

    let add_space = memory::address_space::new();


    let create_table = || {
        let mut lock = ANON_FRAME_ALLOCATOR.lock();
        
        if let Some(ref mut allocator) = *lock 
        {
            return memory::paging::table::new(&mut **allocator);
        }

        panic!()
    };


    let mut create_process = |table| {
        let mut lock = ANON_FRAME_ALLOCATOR.lock();
        
        if let Some(ref mut allocator) = *lock 
        {
            let mut newprocess = process::Process{page_table: table};
            return newprocess;
        }
         panic!()
    };          

    let mut map_address = |process: &mut process::Process, address| {
        let mut lock = ANON_FRAME_ALLOCATOR.lock();
        
        if let Some(ref mut allocator) = *lock 
        {
            //Map a page into that memory (TODO: Move this)
            let page = Page::containing_address(address);
            let frame = allocator
                .allocate_frame()
                .expect("no more frames");

            memory::paging::map_to(
                process.page_table, 
                page, 
                frame, 
                EntryFlags::empty(), 
                &mut **allocator
                );
        }
        else
        {
            panic!();
        }
    };          

    let user_table1 = create_table();
    let mut process1 = create_process(user_table1);
    map_address(&mut process1, addr1);
    map_address(&mut process1, addr2);

    let user_table2 = create_table();
    // //TODO: Why doesn't borrow checker complain?
    // //TODO: This should also activate the "Process" page table
    
    // //process::switch_process(&mut process1);

    // let (user_table2, _frame_allocator) 
    //     = memory::paging::table::new(frame_allocator);

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

    // // Question: Are interrupts masked during a Sync Exception?

    // Test out the mapping
    let data : *mut usize = addr1 as *mut usize;
    
    process::switch_process(&mut process1);
    memory::activate_el0(process1.page_table);

    // TODO GET WORKING ON HARDWARE    
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
    //TODO GET WORKING ON HARDWARE


    // memory::activate_el0(user_table1);

    // write!(
    //     kwriter::WRITER, 
    //     "UPT1: Data at data: 0x{:X?}\n", 
    //     *data);


    let mut vec_test = vec![1,2,3,4,5,6,7];
    vec_test[3] = 42;
    for i in &vec_test {
        write!(
        kwriter::WRITER,"{} ", i);
    }
    write!(kwriter::WRITER,"\n");

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

