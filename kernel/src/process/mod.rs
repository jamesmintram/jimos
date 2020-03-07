use memory;
use memory::address_space;


use arch::aarch64::arm;
use arch::aarch64::frame::TrapFrame;

pub struct Process<'a>
{
    pub pid: usize,

    pub address_space: address_space::AddressSpace<'a>,
    pub heap: address_space::AddressSegmentId,
    pub stack: address_space::AddressSegmentId,
    pub text: address_space::AddressSegmentId,


    // Context
    pub frame: TrapFrame,
    pub kernel_stack: memory::VirtualAddress,
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


pub const ONE_MB: usize = 0x10000;
pub const ONE_GB: usize = 0x40000000;

pub const DEFAULT_TEXT_BASE: usize = ONE_MB;
pub const DEFAULT_TEXT_SIZE: usize = 0;

pub const DEFAULT_HEAP_BASE: usize = ONE_GB;
pub const DEFAULT_HEAP_SIZE: usize = ONE_MB * 32;
pub const DEFAULT_HEAP_MAX_SIZE: usize = ONE_GB * 8;

pub const DEFAULT_STACK_BASE: usize = ONE_GB * 32;
pub const DEFAULT_STACK_SIZE: usize = ONE_MB * 2;
pub const DEFAULT_STACK_MAX_SIZE: usize = ONE_MB * 8;

impl<'a> Process<'a>
{
    pub fn new() -> Process<'a>
    {
        /*
            TODO: Add Result type to this and plenty of ?
            Enforce Range: Start < End
        */
        let mut new_as = address_space::AddressSpace::new();

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

        let kern_stack_frame = memory::kalloc::alloc_frame();
        let kernel_stack = memory::physical_to_kernel(kern_stack_frame.start_address());

        let new_process: Process<'a> = Process{
            pid: 0,
            
            heap: head_seg_id,
            stack: stack_seg_id,
            text: text_seg_id,

            address_space: new_as,
            frame: Default::default(),

            kernel_stack: kernel_stack,
        };

        return new_process;
    }

    pub fn fork(&self) -> bool {
        //Spawn new process
        //Register PID with the scheduler
        return false;
    }

    pub fn exec(&mut self, elf: &elf::Elf) //TODO: Pass in an "image" to execute
    {
        println!("Exec process");
        //TODO: Assert that this process is current

        let stack_range = self.address_space.get_segment_range(self.stack);
        let text_range = self.address_space.get_segment_range(self.text);

        //TODO: Reset the text segment size to the image size
        //TODO: Pre-fault the whole text segment
        //TODO: Copy the executable image into the text area


        for section in elf.sections_iter()
        {
            if section.section_type == 1  && section.addr == text_range.start {
                let data = elf.get_section_data(section).unwrap();
                let dest = section.addr as *mut u8;

                for i in 0..data.len() {
                    //HACK: Copy executable data in RAM
                    unsafe {
                        *dest.offset(i as isize) = data[i];
                    }

                }
            }
        }


        //TODO(Later): Reset the HEAP segment (release all physical pages)
        //TODO(Later): Release all of the text segments physical pages


        // Prepare our frame structure for a later call to return_to_userspace
        let ref mut frame = &mut self.frame;
        frame.tf_sp = stack_range.end as u64;
        frame.tf_lr = 0x0;
        frame.tf_elr = text_range.start as u64;

        // let stack_addr = stack_range.start as *mut u64;
        // unsafe { *stack_addr = 0x4321; }

        let mut spsr : u32 = 0;

        //spsr |= 1 << 2;     // .M[3:2] = 0b100 -->  Return to EL1
        spsr |= 1 << 6;     // FIQ masked
        spsr |= 1 << 7;     // IRQ masked
        spsr |= 1 << 8;     // SError (System Error) masked
        spsr |= 1 << 9;

        frame.tf_spsr = spsr;

        println!("Everything set");
        //Spsr
        //esr
        //tf_x -> 0

        //No need to set x29/x30 as there have been no BL
    }
}

// pub fn resume_current_process()
// {
//     println!("Resuming process");
//     arm::resume_process(&get_current_process().frame);
// }

pub fn return_to_userspace()
{
    //Restore process registers
    //Call eret
    println!("About to return to userspace");
    arm::exception_return(&get_current_process().frame);

    //Now jump to the code we have just copied into memory
    //type EntryPoint = extern fn() -> ();
    //let ep = (&DEFAULT_TEXT_BASE as *const usize) as *const EntryPoint;
    //unsafe {(*ep)()};
}

pub fn switch_process(next_process: &mut Process)
{
    println!("Switching process");
    //Set the thread pointer to this
    let _process_ptr = (next_process as *const _) as usize;
    // arm::set_thread_ptr(process_ptr);

    //Switch the page table
    memory::activate_address_space(&next_process.address_space);
}

//TODO: Fix lifetime hack
pub fn get_current_process() -> &'static mut Process<'static>
{
    // TODO: We need to read this from the current thread
    // let process_ptr_value = arm::get_thread_id();
    // let process_ptr = process_ptr_value as *mut Process;

    // //TODO: We need to validate this process address (Debug only maybe?)

    // let process: &mut Process = unsafe { &mut *process_ptr };
    // process
}