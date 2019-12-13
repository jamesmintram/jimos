use arch::aarch64::arm;
use arch::aarch64::frame::TrapFrame;

use memory;
use spin::{Once, RwLock, RwLockReadGuard, RwLockWriteGuard};
use scheduler;

pub mod idle;

#[derive(Default, Clone, Copy)]
struct Thread {
    pub frame: TrapFrame,
    pub id: ThreadId,
    pub kernel_stack: memory::VirtualAddress,

    /*
        Move stuff over from the Process module to here
    */

}

struct ThreadSystem {
    pub threads: [Thread;64],
    pub current_id: usize,
}

impl ThreadSystem {
    pub fn new() -> ThreadSystem {
        //println!("New thread sys");
        return ThreadSystem {
            threads: [Default::default();64],
            current_id: 0,
        }
    }

    pub fn create<F>(&mut self, init: F) -> ThreadId
        where F: Fn(&mut Thread) -> ()
    {
        let kern_stack_frame = memory::kalloc::alloc_frame();
        let kernel_stack_bottom = memory::physical_to_kernel(kern_stack_frame.start_address());
        let kernel_stack_top = memory::physical_to_kernel(kern_stack_frame.end_address());


        //TODO: Check we have been able to create required resources - if not, early return

        self.current_id += 1;
        let new_thread_id = self.current_id;

        if let Some(mut new_thread) = self.get_mut(self.current_id)
        {
            new_thread.id = new_thread_id;
            new_thread.kernel_stack = kernel_stack_top;

            //NOTE: Could fail? If so, return invalid ThreadId + free resources
            init(&mut new_thread);
        }
        
        new_thread_id
    }

    pub fn update<F>(&mut self, thread_id: ThreadId, update_fn: F)
        where F: Fn(&mut Thread) -> ()
    {
        //TODO: Check docks + match
        if let Some(mut current_thread) = self.get_mut(thread_id)
        {
            update_fn(&mut current_thread)
        }
        else
        {
            //TODO: Could not find thread.. panic
        }
    }

    fn get_mut(&mut self, thread_id: ThreadId) -> Option<&mut Thread> {
        Some(&mut self.threads[thread_id])
    }

    fn get(&self, thread_id: ThreadId) -> Option<&Thread> {
        Some(&self.threads[thread_id])
    }
}

static THREAD_SYS: Once<RwLock<ThreadSystem>> = Once::new();

/// Initialize contexts, called if needed
fn init_thread_sys() -> RwLock<ThreadSystem> {
    RwLock::new(ThreadSystem::new())
}
fn thread_sys() -> RwLockReadGuard<'static, ThreadSystem> {
    THREAD_SYS.call_once(init_thread_sys).read()
}
fn thread_sys_mut() -> RwLockWriteGuard<'static, ThreadSystem> {
    THREAD_SYS.call_once(init_thread_sys).write()
}

pub type ThreadId = usize;

pub type Trampoline = fn(fn_ptr: u64, fn_param: u64) -> ();
pub type ThreadFn = fn(param: u64) -> ();


pub fn create_thread(
    thread_fn: ThreadFn,
    trampoline: Option<Trampoline>) -> ThreadId
{
    let trampoline_fn = match trampoline {
        Some(func) => func,
        None => default_trampoline
    };

    thread_sys_mut().create(
        |new_thread| {
            // let frame = &mut new_thread.frame;

            // Create a new frame layout, then write
            // it to the stack space and make sure SP
            // is correctly set.

            // TODO: Update when using process AS - get the physical address of SP and convert it to a kernel address

            let frame_ptr = new_thread.kernel_stack as usize as *mut TrapFrame;
            let frame_ptr = unsafe {frame_ptr.offset(-1)};
            let frame = unsafe {&mut *frame_ptr};

            // X0 - thread func
            frame.tf_x[0] = thread_fn as u64;
            // X1 - thread parameter
            frame.tf_x[1] = 0xBEEF;

            frame.tf_elr = 0;
            frame.tf_lr = trampoline_fn as u64;//text_range.start as u64;


            // TODO: Needed to set initial state, not sure about where this should live
            let mut spsr : u32 = 0;

            spsr |= 1 << 0;     // Dunno what this does..
            spsr |= 1 << 2;     // .M[3:2] = 0b100 -->  Return to EL1
            spsr |= 1 << 6;     // FIQ masked
            spsr |= 1 << 7;     // IRQ masked
            spsr |= 1 << 8;     // SError (System Error) masked
            spsr |= 1 << 9;

            frame.tf_spsr = spsr;
        })
}

pub fn start_thread(thread_id: ThreadId) {
    thread_sys_mut().update(
        thread_id,
        |_current_thread| {
            //TODO: Update thread status

            scheduler::register_thread(thread_id);
        });
}

fn default_trampoline(fn_ptr: u64, fn_param: u64) {
    // println!("Tramampoline\n");
    // println!("fn    {:X}", fn_ptr);
    // println!("Param {}", fn_param);


    //TODO: This is a hack mess
    let func_ptr_ptr = &fn_ptr as *const u64 as usize;
    let func = func_ptr_ptr as *const ThreadFn;

    unsafe {
        (*func)(fn_param);
    }

    // println!("Trampoline end");

    scheduler::switch_to_next();

    // panic!("Fallen through a trampoline switchback");
}

//-------------------------

pub fn get_thread_id()  -> usize
{
    arm::get_thread_id()
}

pub fn get_thread_frame(thread_id: ThreadId) -> usize {
    //TODO: This is broken, should work with stacking
    if let Some(thread) = thread_sys().get(thread_id) {

        let frame_ptr = thread.kernel_stack as usize as *mut TrapFrame;
        let frame_ptr = unsafe {frame_ptr.offset(-1)};

        // let frame_ptr = &thread.frame as *const _ as usize;
        frame_ptr as usize
    } else {
        panic!("Invalid thread id: {}", thread_id)
    }
}

pub fn switch_to(next_thread_id: ThreadId) {
    let current_thread_id = get_thread_id();

    // Precondition check
    if current_thread_id == next_thread_id {
        panic!(
            "thread::switch_to thread IDs are equal: {} : to-> : {}",
            current_thread_id,
            next_thread_id);
    }

    let current_thread_frame_addr = get_thread_frame(current_thread_id);
    let next_thread_frame_addr = get_thread_frame(next_thread_id);

    println!("ResumeProcess:current_addr {:X}", current_thread_frame_addr);
    println!("ResumeProcess:next_addr {:X}", next_thread_frame_addr);

    arm::set_thread_id(next_thread_id);
    arm::switch_thread(
       current_thread_frame_addr,
       next_thread_frame_addr);
}

pub fn switch_to_initial(next_thread_id: ThreadId) {
    let next_thread_frame_addr = get_thread_frame(next_thread_id);

    println!("Switch initial");
    println!("SwitchToInitialProcess:frame_addr {:X}", next_thread_frame_addr);

    arm::set_thread_id(next_thread_id);
    arm::switch_to_initial(
        next_thread_frame_addr);
}