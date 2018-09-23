use arch::aarch64::frame;

use kwriter;
use core::fmt::Write;
#[derive(Copy, Clone ,Debug, PartialEq)]
pub enum Exception {
    UNKNOWN = 0x00,     /* Unkwn exception */
    FP_SIMD = 0x07,     /* VFP/SIMD trap */
    ILL_STATE = 0x0e,   /* Illegal execution state */
    SVC32 = 0x11,       /* SVC trap for AArch32 */
    SVC64 = 0x15,       /* SVC trap for AArch64 */
    MSR = 0x18,         /* MSR/MRS trap */
    INSN_ABORT_L = 0x20,/* Instruction abort, from lower EL */
    INSN_ABORT = 0x21,  /* Instruction abort, from same EL */
    PC_ALIGN = 0x22,    /* PC alignment fault */
    DATA_ABORT_L = 0x24,/* Data abort, from lower EL */
    DATA_ABORT = 0x25,  /* Data abort, from same EL */ 
    SP_ALIGN = 0x26,    /* SP slignment fault */
    TRAP_FP = 0x2c,     /* Trapped FP exception */
    SERROR = 0x2f,      /* SError interrupt */
    SOFTSTP_EL0 = 0x32, /* Software Step, from lower EL */
    SOFTSTP_EL1 = 0x33, /* Software Step, from same EL */
    WATCHPT_EL1 = 0x35, /* Watchpoint, from same EL */
    BRK = 0x3c,         /* Breakpoint */
}

fn exception_from_esr(esr: u32) -> Exception {
    const ESR_ELX_EC_SHIFT: u32 = 26;
    const ESR_ELX_EC_MASK: u32 = (0x3f << 26);

    let code = ((esr) & ESR_ELX_EC_MASK) >> ESR_ELX_EC_SHIFT;

    match code {
        0x07 => Exception::FP_SIMD,
        0x0e => Exception::ILL_STATE,
        0x11 => Exception::SVC32,
        0x15 => Exception::SVC64,
        0x18 => Exception::MSR,
        0x20 => Exception::INSN_ABORT_L,
        0x21 => Exception::INSN_ABORT,
        0x22 => Exception::PC_ALIGN,
        0x24 => Exception::DATA_ABORT_L,
        0x25 => Exception::DATA_ABORT,
        0x26 => Exception::SP_ALIGN,
        0x2c => Exception::TRAP_FP,
        0x2f => Exception::SERROR,
        0x32 => Exception::SOFTSTP_EL0,
        0x33 => Exception::SOFTSTP_EL1,
        0x35 => Exception::WATCHPT_EL1,
        _ => Exception::UNKNOWN,
    }
}

#[no_mangle]
#[allow(dead_code)]
pub unsafe extern "C" fn do_el1h_sync(
    thread: usize,
    frame_ptr: *const frame::TrapFrame) -> i32
{
    //TODO: Figure out what kind of exception we have
    let frame = &*frame_ptr;

    let exception = exception_from_esr(frame.tf_esr);

    // Page fault
    match exception {
        Exception::DATA_ABORT => {
            ::remap();
        },
        _ => {
            panic!("Unhandled Exception: {:?}", exception);
        }
    }


    //TODO: Remap the memory that we need for this call
    // Hack by calling a fn
    // Semi-hack by getting pgt from param0
    // Eventually from a process struct

    //TODO: Fix up the register clobbering memory/mod.rs
    //panic!();
    //TODO: Fix panic!(string)
    //panic!("Unhandled exception")
    return -1;
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

