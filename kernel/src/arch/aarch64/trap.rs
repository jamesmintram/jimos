use arch::aarch64::frame;
use arch::aarch64::arm;

use memory;
use memory::virtual_address;
use process;

use kwriter;
use core::fmt::Write;

#[allow(non_camel_case_types)]

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


//TODO: This is temporary
fn remap(process: &process::Process)
{    
    write!(kwriter::WRITER, "Switching out PGT\n");
    memory::activate_el0(process.page_table);
}

fn data_abort(
    _frame: &frame::TrapFrame, 
    process: &process::Process, 
    far: u64, 
    _low: bool)
{
    /*
	 * According to the ARMv8-A rev. A.g, B2.10.5 "Load-Exclusive
	 * and Store-Exclusive instruction usage restrictions", state
	 * of the exclusive monitors after data abort exception is unknown.
	 */
	arm::clrex();

    //TODO: Handle the "fault status code" - 
    // https://yurichev.com/mirrors/ARMv8-A_Architecture_Reference_Manual_(Issue_A.a).pdf
    // p. 1528

    //dump_regs(&_frame);

    let fault_address = virtual_address::from_u64(far);

    //TODO: Call memory::page::PageFault(darta);
    //TODO: Check for permission issue if page fault
    if let Some(address) = fault_address {
        match address {
            virtual_address::VirtualAddress::User(addr) => {
                write!(
                    kwriter::WRITER,
                    "Fault address: {:?}\n",
                    addr);   
                remap(process);
            },

            virtual_address::VirtualAddress::Kernel(addr) => {
                write!(
                    kwriter::WRITER,
                    "Kernel Page Fault: 0x{:X}\n", far);    
                    panic!("Unkown address");    
            },
        } 
    } else {
        write!(
            kwriter::WRITER,
            "Invalid address: 0x{:X}\n", far);    
            panic!("Unkown address");
    }

    //TODO: Check for success
    //TODO: Rewrite as handle_page_fault(..blah)
}

#[no_mangle]
#[allow(dead_code)]
pub unsafe extern "C" fn do_el1h_sync(
    frame_ptr: *const frame::TrapFrame) -> i32
{
    let frame = &*frame_ptr;
    let exception = exception_from_esr(frame.tf_esr);
    let process = process::get_current_process();



    match exception {
        //TODO: Match on all data aborts
        Exception::DATA_ABORT => {
            let far = arm::read_far_el1();
            data_abort(frame, process, far, false);            
        },
        Exception::DATA_ABORT_L => {
            let far = arm::read_far_el1();
            data_abort(frame, process, far, true);            
        },
        _ => {
            panic!("Unhandled Exception: {:?}", exception);
        }
    }

    //TODO: Fix up the register clobbering memory/mod.rs
    //panic!("Unhandled exception")
    return -1;
}

fn dump_regs(frame: &frame::TrapFrame)
{
    // borrow of packed field is unsafe and requires unsafe function or block
    unsafe {
        write!(kwriter::WRITER,"SP: {:X}  {}\n", frame.tf_sp, frame.tf_sp);
        write!(kwriter::WRITER,"LR: {:X}  {}\n", frame.tf_lr, frame.tf_lr);
        write!(kwriter::WRITER,"ELR: {:X}  {}\n", frame.tf_elr, frame.tf_elr);
        write!(kwriter::WRITER,"SPSR: {:X}  {}\n", frame.tf_spsr, frame.tf_spsr);
        write!(kwriter::WRITER,"ESR: {:X}  {}\n", frame.tf_esr, frame.tf_esr);

        for i in 0..30  {
            write!(kwriter::WRITER,"X{}: {:X}\n", i, frame.tf_x[i]);
        }
    }
}

