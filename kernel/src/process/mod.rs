use memory;
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
    //TODO: Kernel Stack

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

pub const ONE_MB: usize = 0x100000;
pub const ONE_GB: usize = 0x40000000;

pub const DEFAULT_TEXT_BASE: usize = ONE_MB; 
pub const DEFAULT_TEXT_SIZE: usize = 0; 

pub const DEFAULT_HEAP_BASE: usize = ONE_GB; 
pub const DEFAULT_HEAP_SIZE: usize = ONE_MB * 32; 
pub const DEFAULT_HEAP_MAX_SIZE: usize = ONE_GB * 8; 

pub const DEFAULT_STACK_BASE: usize = ONE_GB * 32; 
pub const DEFAULT_STACK_SIZE: usize = ONE_MB; 
pub const DEFAULT_STACK_MAX_SIZE: usize = ONE_MB * 8; 

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
            range: address_space::AddressRange{start: DEFAULT_HEAP_BASE, end: DEFAULT_HEAP_BASE + DEFAULT_HEAP_SIZE},
        };
        let text_desc = address_space::AddressSegmentDesc{
            range: address_space::AddressRange{start: DEFAULT_TEXT_BASE, end: DEFAULT_TEXT_BASE + DEFAULT_TEXT_SIZE},
        };
        let stack_desc = address_space::AddressSegmentDesc{
            range: address_space::AddressRange{start: DEFAULT_STACK_BASE, end: DEFAULT_STACK_BASE + DEFAULT_STACK_SIZE},
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

    pub fn exec(&mut self) //TODO: Pass in an "image" to execute
    {
        //TODO(Later): Reset the HEAP segment (release all physical pages)
        //TODO(Later): Release all of the text segments physical pages

        //TODO: Reset the text segment size to the image size
        //TODO: Pre-fault the whole text segment
        //TODO: Copy the executable image into the text area

        // Prepare our frame structure for a later call to return_to_userspace
        let stack_range = self.address_space.get_segment_range(self.heap);
        let text_range = self.address_space.get_segment_range(self.text);

        let ref mut frame = &mut self.frame;
        frame.tf_sp = stack_range.start as u64;
        frame.tf_lr = 0;
        frame.tf_elr = text_range.start as u64;

        //Spsr
        //esr
        //tf_x -> 0

        //No need to set x29/x30 as there have been no BL
    }
}

pub fn return_to_userspace()
{
    //Restore process registers
    //Call eret
}

pub fn switch_process(next_process: &mut Process) 
{
    //Set the thread pointer to this
    let process_ptr = (next_process as *const _) as usize;
    arm::set_thread_ptr(process_ptr);

    //Switch the page table 
    memory::activate_address_space(&next_process.address_space);
}

//TODO: Fix lifetime hack
pub fn get_current_process() -> &'static mut Process<'static>
{
    let process_ptr_value = arm::get_thread_ptr();
    let process_ptr = process_ptr_value as *mut Process;

    //TODO: We need to validate this process address (Debug only maybe?)

    let process: &mut Process = unsafe { &mut *process_ptr };
    process
}