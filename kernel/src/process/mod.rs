use memory::LockedAreaFrameAllocator;  
use memory::address_space;


use arch::aarch64::arm;
use arch::aarch64::frame::TrapFrame;

pub struct Process<'a> 
{
    pub address_space: address_space::AddressSpace<'a>,
    pub heap: address_space::AddressSegmentId,
    pub stack: address_space::AddressSegmentId,
    pub text: address_space::AddressSegmentId,

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
    pub fn new(allocator: &'a LockedAreaFrameAllocator) -> Process<'a>
    {
        /*
            TODO: Add Result type to this and plenty of ?
            Enforce Range: Start < End
        */
        let mut new_as = address_space::AddressSpace::new(allocator);    
        
        let heap_desc = address_space::AddressSegmentDesc{
            range: address_space::AddressRange{start: 0x10000, end: 0xFFFFF},
        };
        let text_desc = address_space::AddressSegmentDesc{
            range: address_space::AddressRange{start: 0x100000, end: 0xFFFFFF},
        };
        let stack_desc = address_space::AddressSegmentDesc{
            range: address_space::AddressRange{start: 0x1000000, end: 0xFFFFFFF},
        };

        let head_seg_id = new_as.add_segment(&heap_desc);
        let text_seg_id = new_as.add_segment(&text_desc);
        let stack_seg_id = new_as.add_segment(&stack_desc);

        let new_process: Process<'a> = Process{
            heap: head_seg_id,
            stack: text_seg_id,
            text: stack_seg_id,

            address_space: new_as,
            frame: Default::default(),
        };

        return new_process;
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