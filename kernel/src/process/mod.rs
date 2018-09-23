use memory::paging::table;
use arch::aarch64::arm;

pub struct Process<'a> 
{
    pub page_table: &'a mut table::Table<table::Level4>,
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