use arch::aarch64::frame;

use kwriter;
use core::fmt::Write;

#[no_mangle]
#[allow(dead_code)]
pub unsafe extern "C" fn do_el1h_sync(
    thread: usize,
    frame: *const frame::TrapFrame) -> i32
{
    //TODO: Figure out what kind of exception we have

    // Page fault
    {
        //TODO: Remap the memory that we need for this call
        // Hack by calling a fn
        // Semi-hack by getting pgt from param0
        // Eventually from a process struct
        ::remap();
        //return -1;
    }   

    //TODO: Fix up the register clobbering memory/mod.rs
    panic!();
    //TODO: Fix panic!(string)
    //panic!("Unhandled exception")
}

fn dump_regs(frame: &frame::TrapFrame)
{
    write!(kwriter::WRITER,"SP: {:X?}  {}\n", frame.tf_sp, frame.tf_sp);
    write!(kwriter::WRITER,"LR: {:X?}  {}\n", frame.tf_lr, frame.tf_lr);
    write!(kwriter::WRITER,"ELR: {:X?}  {}\n", frame.tf_elr, frame.tf_elr);
    write!(kwriter::WRITER,"SPSR: {:X?}  {}\n", frame.tf_spsr, frame.tf_spsr);
    write!(kwriter::WRITER,"ESR: {:X?}  {}\n", frame.tf_esr, frame.tf_esr);

    for i in 0..30  {
        write!(kwriter::WRITER,"X{}: {:X?}\n", i, frame.tf_x[i]);
    }
}

