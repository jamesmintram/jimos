#![feature(lang_items, asm)]
#![no_builtins]
#![no_std]
#![feature(panic_info_message)]

#![feature(alloc_error_handler)]
#![feature(allocator_api)]

//Temporary
#![allow(dead_code)]

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate alloc;
extern crate spin;
extern crate elf;
//extern crate hashmap_core;

pub mod lang_items;

#[macro_use]
mod kwriter;

mod arch;
mod memory;
mod process;
mod thread;
mod syscall;

mod uart;
mod gpio;
mod mailbox;
mod panic;
mod rootprocess;
mod scheduler;
mod reboot;

mod test;

//use memory::LockedAreaFrameAllocator;
use memory::slab_allocator::LockedSlabHeap;


//Temp
use arch::aarch64::arm;
// use spin::Mutex;

// Required here to make them accessable to ASM
pub use syscall::int_syscall;
pub use arch::aarch64::trap::do_el1h_sync;

extern "C" {
    fn exit();
}

#[global_allocator]
static mut HEAP_ALLOCATOR: LockedSlabHeap = LockedSlabHeap::empty();

//--------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn kmain()
{ 
    arm::reset_ttbr0_el1(); //Turn off identity mapping in EL0
    arm::cache_setup();

    uart::uart_init();

    println!("UART init");

    // arm::print_cache_info();

    println!("Building kernel page tables");
    
    //TODO: Need to build a page table which is the same as what we generate in ASM
    //0 - kernel_start           :: unmapped
    //kernel_start -> bss start  :: read only
    //bss_end -> bss_end +1      :: unmapped
    //last 16MB                  :: PT_DEV + PT_OSH

/*
    //TODO: Enforce W^X using types?
    //TODO: AS will create/update an ARCH specific page table

    new_as.add_segment(
        address_space::AddressSegmentDesc{
            range: address_space::AddressRange{
                start: KERNEL_TEXT_START, 
                end: KERNEL_TEXT_END,
            },
            perm:  X 
            flags: NONE,
            name: "Kernel Text"
        },
        address_space::AddressSegmentDesc{
            range: address_space::AddressRange{
                start: KERNEL_BSS_START, 
                end: KERNEL_BSS_END
            },
            perm:  W
            flags: NONE,
            name: "Kernel BSS"
        },

        //Ensure there is a 1 page gap here to catch stack overrun

        address_space::AddressSegmentDesc{
            range: address_space::AddressRange{
                start: KERNEL_STACK_START = KERNEL_BSS_END + 1 page, 
                end: KERNEL_STACK_END
            },
            perm:  W
            flags: NONE,
            name: "Kernel Stack"
        },

        address_space::AddressSegmentDesc{
            range: address_space::AddressRange{
                start: DEFAULT_HEAP_BASE, 
                end: DEFAULT_HEAP_BASE + DEFAULT_HEAP_SIZE
            },
            perm:  W no Exec //Means we cannot execute code in EL1 that lives in the heap 
            flags: NONE,
            name: "Kernel HEAP"
        },
        address_space::AddressSegmentDesc{
            range: address_space::AddressRange{
                start: 0x3F000000, 
                end: 0x3FFFFFFF
            },
            perm:  W, 
            flags: MMIO,
            name: "MMIO"
        },
    }
*/

    //TODO: Consider how rust allocs on the stack
    memory::init();
    thread::init();


    //test::heap();
    //test::deadlock();
    //test::thread_custom_trampoline();

    //TODO: Get inside a thread context
    //thread::create_first_thread();
    //thread::switch_to_first_thread(first_thread_id)
    //     This just sets the correct CPU registers

    //TODO: Create and schedule another thread
    //TODO: How are stacks managed for threads with no Process?

    //TODO: How do we pass in the function we want to run?

    let proc1 = process::create_process();
    let proc2 = process::create_process();

    let thread1 = thread::create_thread(proc1, thread::idle::idle1, None);
    thread::start_thread(thread1);

    let thread2 = thread::create_thread(proc2, thread::idle::idle2, None);
    thread::start_thread(thread2);

    //thread::switch_to_initial(thread1);

    //let mut root_process = process::Process::new(&KERNEL_FRAME_ALLOCATOR);
    //rootprocess::boot_root_process(root_process);

    //process1.exec(&elf);

    //Start process 2
    //process::return_to_userspace();

    //TODO: Implement this via an eret/return to user space
    //TODO: Load two processes and run one after the other - ensure they are using different memory
    //TODO: Refactor the above


    println!("Exiting jimOS\n");
    //exit();

        // match elf::Elf::from_data(&slice) {
    //     Err(err) => println!("ELF: {:#?}\n", err),
    //     _ => println!("ELF Loaded\n"),
    // };


    // Create a new process
    // Schedule process
}
    //TODO: Wrap HeapSlabAllocator in a Mutex and replace HEAP_ALLOCATOR
    //      Implement alloc and release
    //      Churn memory with old and see crash
    //      Churn memory with new and see no crash
    //      Periodically print stats
    //
    //
    //  Book keeping for SlabAllocator
    //  Naive way to release page frames
    //
    //
    //TODO: Look below for the Address space stuff
    //slab_allocator::HeapSlabAllocator::new(&KERNEL_FRAME_ALLOCATOR);

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





    // write!(
    //     kwriter::WRITER,
    //     "UPT1: Data at data: 0x{:X?}\n",
    //     *data);

    // memory::activate_el0(user_table2);

    // write!(
    //     kwriter::WRITER,
    //     "Data at data: 0x{:X?}\n",
    //     *data);
    //TODO GET WORKING ON HARDWARE



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
    // let addr1 = 42 * 512 * 512 * 4096;
    // let addr2 = 12 * 512 * 512 * 4096;

    //TODO: Create entry in process address space
    //      Trigger a page fault
    //      Fault handler
    //          finds page frame to back memory
    //          updates process page table
    //          continues
    //
    //      Unmapping a region of VA space?
    //          Need to release refs to page frames
    //          Need to remove from page table

    // let add_space = memory::address_space::new();
    // let create_table = || {
    //     let mut lock = KERNEL_FRAME_ALLOCATOR.lock();

    //     if let Some(ref mut allocator) = *lock
    //     {
    //         //TODO: Refactor this so that we use memory::alloc(allocator)
    //         return memory::paging::table::new(&mut **allocator);
    //     }

    //     panic!()
    // };




    // let create_process = |table| {
    //     let newprocess = process::Process{page_table: table};
    //     return newprocess;
    // };

    //TODO: Refactor this so that we use memory::alloc(allocator)
    // let map_address = |process: &mut process::Process, address| {
    //     let mut lock = KERNEL_FRAME_ALLOCATOR.lock();

    //     if let Some(ref mut allocator) = *lock
    //     {
    //         //Map a page into that memory (TODO: Move this)
    //         let page = Page::containing_address(address);
    //         let frame = allocator
    //             .allocate_frame()
    //             .expect("no more frames");

    //         memory::paging::map_to(
    //             process.page_table,
    //             page,
    //             frame,
    //             EntryFlags::empty(),
    //             &mut **allocator
    //             );
    //     }
    //     else
    //     {
    //         panic!();
    //     }
    // };

    // let user_table1 = create_table();
    // let mut process1 = process::Process::create(user_table1);
    // map_address(&mut process1, addr1);
    // map_address(&mut process1, addr2);

    // let user_table2 = create_table();

    //https://shop.pimoroni.com/products/hdmi-8-lcd-screen-kit-1024x768#description

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
    // let data : *mut usize = addr1 as *mut usize;

    // process::switch_process(&mut process1);
    // memory::activate_el0(process1.page_table);

    // // TODO GET WORKING ON HARDWARE
    // *data = 1024;
    // write!(
    //     kwriter::WRITER,
    //     "UPT1: Data at data: 0x{:X?}\n",
    //     *data);

    // memory::activate_el0(user_table2);

    // write!(
    //     kwriter::WRITER,
    //     "Data at data: 0x{:X?}\n",
    //     *data);
    //TODO GET WORKING ON HARDWARE


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
    // write!(kwriter::WRITER,"\n");

    // {
    //     let page = Page::containing_address(addr);
    //     memory::paging::unmap(
    //         user_table1,
    //         page,
    //         frame_allocator);

    //     memory::flush_tlb();
    // }