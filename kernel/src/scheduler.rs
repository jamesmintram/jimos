use thread::{ThreadId};
use thread;
use spin::{Once, RwLock, RwLockReadGuard, RwLockWriteGuard};


struct SchedulerSystem {
    pub current_id: ThreadId,
    pub max_id: ThreadId,
}

impl SchedulerSystem {
    pub fn new() -> SchedulerSystem {
        return SchedulerSystem {
            current_id: 0,
            max_id: 0,
        }
    }

    fn register_thread(&mut self, thread_id: ThreadId) {
        //TODO: Make this proper
        self.current_id = thread_id;
        self.max_id = thread_id;
    }

    fn switch_to_next(&mut self) -> ThreadId {
        let current_thread_id = thread::get_thread_id();

        //TODO: There is lots more we can do here respect to checking thread statuses
        //TODO: and priorities and other stuff

        self.current_id = 1 + (current_thread_id % self.max_id);
        return self.current_id;

        //TODO: This will bottom out with ERET
        //register PID with scheduler

        //process::switch_process(&mut process);
        //process::resume_current_process();


        // DISABLED FOR NOW
        // println!("arm::set_thread_ptr");
        // arm::set_thread_ptr(self.current_id);

        // println!("thread::resume");
        // thread::resume(self.current_id);


        //TODO: (if required) Switch the page table
        //memory::activate_address_space(&next_process.address_space)
    }
}

static SCHED_SYS: Once<RwLock<SchedulerSystem>> = Once::new();

/// Initialize contexts, called if needed
fn init_sched_sys() -> RwLock<SchedulerSystem> {
    RwLock::new(SchedulerSystem::new())
}
fn sched_sys() -> RwLockReadGuard<'static, SchedulerSystem> {
    SCHED_SYS.call_once(init_sched_sys).read()
}
fn sched_sys_mut() -> RwLockWriteGuard<'static, SchedulerSystem> {
    SCHED_SYS.call_once(init_sched_sys).write()
}

pub fn register_thread(thread_id: ThreadId) {
    sched_sys_mut().register_thread(thread_id);
}

pub fn switch_to_next() {
    let next_thread = sched_sys_mut().switch_to_next();

    //This is split apart to prevent deadlocking the scheduler
    // arm::set_thread_ptr(next_thread);
    thread::switch_to(next_thread);
}