use memory;
use memory::address_space;

use alloc::sync::Arc;
use arch::aarch64::arm;
use spin::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use thread;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct ProcessId(pub usize);

pub struct Process<'a>
{
    pub pid: ProcessId,

    pub address_space: address_space::AddressSpaceRef<'a>,
    pub heap: address_space::AddressSegmentId,
    pub stack: address_space::AddressSegmentId, //TODO: Remove this - there is 1 stack per thread
    pub text: address_space::AddressSegmentId,


    // Context
    // pub frame: TrapFrame,
    // pub kernel_stack: memory::VirtualAddress,
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

pub const DEFAULT_PROCESS : Process = Process {
    pid: ProcessId(0),
    address_space: None,
    heap: address_space::AddressSegmentId(0),
    stack: address_space::AddressSegmentId(0),
    text: address_space::AddressSegmentId(0),
};


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

//TODO: Reimplement the below using pub fn
// impl Process<'_>
// {
//     // pub fn fork(&self) -> bool {
//     //     //Spawn new process
//     //     //Register PID with the scheduler
//     //     return false;
//     // }

//     // pub fn exec(&mut self, elf: &elf::Elf) //TODO: Pass in an "image" to execute
//     // {
//     //     println!("Exec process");
   
//     //     for section in elf.sections_iter()
//     //     {
//     //         if section.section_type == 1  && section.addr == text_range.start {
//     //             let data = elf.get_section_data(section).unwrap();
//     //             let dest = section.addr as *mut u8;

//     //             for i in 0..data.len() {
//     //                 //HACK: Copy executable data in RAM
//     //                 unsafe {
//     //                     *dest.offset(i as isize) = data[i];
//     //                 }

//     //             }
//     //         }
//     //     }
//     // }
// }


struct ProcessSystem<'a> {
    pub processes: [Process<'a>;1024],
    pub current_id: usize,
}

impl ProcessSystem<'_> {
    pub fn create(&mut self) -> ProcessId
    {
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

        //TODO: Check we have been able to create required resources - if not, early return

        self.current_id += 1;
        let new_process_id = ProcessId(self.current_id);
        
        {
            let mut new_process = &mut self.processes[self.current_id];
            new_process.pid = new_process_id;
            new_process.heap = head_seg_id;
            new_process.stack = stack_seg_id;
            new_process.text = text_seg_id;

            new_process.address_space = Some(Arc::new(RwLock::new(new_as))); 
        }
        
        new_process_id
    }

    fn with<F>(&self, pid: ProcessId, to_call: F) 
        where F: Fn(&Process) -> ()
    {
        let new_process = &self.processes[pid.0];
        to_call(new_process);
    }

    pub const fn new() -> ProcessSystem<'static> {
        return ProcessSystem {
            processes: [DEFAULT_PROCESS;1024],   //QUESTION: Does this ensure space is allocated?
            current_id: 1,
        }
    }
}
static PROCESS_SYS: RwLock<ProcessSystem> = RwLock::new(ProcessSystem::new());

fn process_sys() -> RwLockReadGuard<'static, ProcessSystem<'static>> {
    PROCESS_SYS.read()
}
fn process_sys_mut() -> RwLockWriteGuard<'static, ProcessSystem<'static>> {
    PROCESS_SYS.write()
}

pub fn create_process() -> ProcessId
{
    process_sys_mut().create()
}

pub fn with<F>(pid: ProcessId, to_call: F) 
        where F: Fn(&Process) -> ()
{
    let new_process = &process_sys().processes[pid.0];
    to_call(new_process);
}

pub fn with_mut<F>(pid: ProcessId, to_call: F) 
        where F: Fn(&mut Process) -> ()
{
    let new_process = &mut process_sys_mut().processes[pid.0];
    to_call(new_process);
}

pub fn activate_address_space(pid: ProcessId) 
{
    with(pid, |process| {
        address_space::with(&process.address_space, |address_space| {
            memory::activate_address_space(address_space);
        });
    });
}


pub fn exec(pid: ProcessId, elf: &elf::Elf) -> thread::ThreadId //TODO: Pass in an "image" to execute
{
    println!("Exec process");

    with(pid, |process| {
        address_space::with_mut(&process.address_space, |address_space| {
            
            for section in elf.sections_iter()
            {
                println!("Section type: {}", section.section_type);
                if section.section_type == 1  && section.addr == DEFAULT_TEXT_BASE {
                    let data = elf.get_section_data(section).unwrap();
                    
                    //Map in the pages required
                    //WARNING: In the future this data could get paged out
                    //          and would not be demand paged in on PF
                    address_space.map_range(DEFAULT_TEXT_BASE, data.len());

                    println!("Kernel: {:X}", 
                        memory::as_to_kernel(
                            address_space, 
                            DEFAULT_TEXT_BASE).unwrap());

                    // let dest = section.addr as *mut u8;
                    //TODO: 
                    let dest = memory::as_to_kernel(
                        address_space, 
                        DEFAULT_TEXT_BASE).unwrap() as *mut u8;

                    println!("Copy data: {:X}", dest as usize);
                    for i in 0..data.len() {
                        //HACK: Copy executable data in RAM
                        unsafe {
                             *dest.offset(i as isize) = data[i];
                        }
                    }
                }
            }
        });

        let process_thread = thread::create_thread(pid, DEFAULT_TEXT_BASE, None);
        thread::start_thread(process_thread);
    });

    //TODO: Fix this hard coded hack
    return 1;
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
    //arm::exception_return(&get_current_process().frame);

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

    address_space::with(&next_process.address_space, |address_space| {
        memory::activate_address_space(address_space);
    });
}

// //TODO: Fix lifetime hack
// pub fn get_current_process() -> &'static mut Process<'static>
// {
//     //TODO: We need to read this from the current thread
//     let process_ptr_value = arm::get_thread_id();
//     let process_ptr = process_ptr_value as *mut Process;

//     //TODO: We need to validate this process address (Debug only maybe?)

//     let process: &mut Process = unsafe { &mut *process_ptr };
//     process
// }