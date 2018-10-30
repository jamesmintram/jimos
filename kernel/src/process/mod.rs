use memory::address_space::AddressSpace;
use memory::va_segment::VASegment;
use memory::paging::table;
use arch::aarch64::arm;
use arch::aarch64::frame::TrapFrame;
use alloc::boxed::Box;

pub struct Process<'a> 
{
    pub page_table: &'a mut table::Table<table::Level4>,

    pub address_space: AddressSpace<'a>,
    pub heap: VASegment,
    pub stack: VASegment,
    pub text: VASegment,

    // Context
    pub frame: TrapFrame,

    //TODO:
    /*
        Current status
        Wait Queues?
        Everything retains a reference to a process via a PID
            Process has to be requested using a PID and unwrapped

        Process::create -> ProcessManager::create()
    */
}

/*
    
*/

impl<'a> Process<'a>
{
    pub fn create(page_table: &'a mut table::Table<table::Level4>)// -> Process<'a>
    {
        // let address_space = AddressSpace::create();
        
        // // Remove all of the explicit lifetimes (once we create our own page table)
        // // Creates its own page table using page_alloc/KERNEL_ALLOCATOR
        // // Owns its segments, which can be borrowed
        // // Create_segment returns an AddressSegmentID

        // let heap = address_space.create_segment(seg_desc);

        // {
        //     Check out the ? operator - used in 
        //     if let Some(heap_ref) = address_space.get_mut(heap)
        //     {
        //         //Do some stuff
        //     }
        // }

        // let mut new_process: Process<'a> = Process{
        //     page_table: page_table,

        //     heap: VASegment{},
        //     stack: VASegment{},
        //     text: VASegment{},

        //     address_space: address_space,
        //     frame: Default::default(),
        // };

        // new_process.address_space.add_segment(&new_process.heap);
        // new_process.address_space.add_segment(&new_process.stack);
        // new_process.address_space.add_segment(&new_process.text);


        // return new_process;
    }


}

pub fn switch_process(next_process: &mut Process) 
{
    //Set the thread pointer to this
    let process_ptr = (next_process as *const _) as usize;
    arm::set_thread_ptr(process_ptr);
}

//TODO: Fix lifetime hack
pub fn get_current_process() -> &'static Process<'static>
{
    let process_ptr_value = arm::get_thread_ptr();
    let process_ptr = process_ptr_value as *const Process;

    //TODO: We need to validate this process address (Debug only maybe?)

    let process: &Process = unsafe { &*process_ptr };
    process
}