use arch::aarch64::frame;
use arch::aarch64::arm;

use memory::virtual_address;
use process;

#[allow(non_camel_case_types)]

// ESR - Instruction Fault Status Code
// -----------------------------------
// 0b000000 Address size fault in TTBR0 or TTBR1.
// 0b000101 Translation fault, 1st level.
// 00b00110 Translation fault, 2nd level.
// 00b00111 Translation fault, 3rd level.
// 0b001001 Access flag fault, 1st level.
// 0b001010 Access flag fault, 2nd level.
// 0b001011 Access flag fault, 3rd level.
// 0b001101 Permission fault, 1st level.
// 0b001110 Permission fault, 2nd level.
// 0b001111 Permission fault, 3rd level.
// .. and more here: http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0488c/CIHIDFFE.html
//-------------------------------------

#[derive(Copy, Clone ,Debug, PartialEq)]
pub enum FaultStatusCode {
    ADDR_SIZE_FAULT,
    TRANS_FAULT,
    ACCESS_FAULT,
    PERM_FAULT,

    UNKNOWN,
}

fn fault_status_code_from_u32(code: u32) -> FaultStatusCode {
    match code {
        0x0 => FaultStatusCode::ADDR_SIZE_FAULT,
        0x1 => FaultStatusCode::TRANS_FAULT,
        0x2 => FaultStatusCode::ACCESS_FAULT,
        0x3 => FaultStatusCode::PERM_FAULT,

        _ => FaultStatusCode::UNKNOWN,
    }
}

#[derive(Copy, Clone ,Debug, PartialEq)]
pub enum FaultStage {
    STAGE_1,
    STAGE_2,
    STAGE_3,

    UNKNOWN,
}

fn fault_stage_from_u32(stage: u32) -> FaultStage {
    match stage {
        0x1 => FaultStage::STAGE_1,
        0x2 => FaultStage::STAGE_2,
        0x3 => FaultStage::STAGE_3,
        _ => FaultStage::UNKNOWN,
    }
}

#[derive(Copy, Clone, Debug)]
pub struct FaultInfo { 
    code: FaultStatusCode,
    stage: FaultStage,
}

fn fault_info_from_esr(esr: u32) -> FaultInfo {
    const ESR_STAGE_SHIFT: u32 = 0x0;
    const ESR_STAGE_MASK: u32 = 0b11;

    const ESR_CODE_SHIFT: u32 = 0x2;
    const ESR_CODE_MASK: u32 = 0b11;

    let stage = (esr >> ESR_STAGE_SHIFT) & ESR_STAGE_MASK;
    let code = (esr >> ESR_CODE_SHIFT) & ESR_CODE_MASK;

    FaultInfo{
        code: fault_status_code_from_u32(code),
        stage: fault_stage_from_u32(stage),
    }   
}



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

fn data_abort(
    frame: &frame::TrapFrame, 
    process: &mut process::Process, 
    far: u64, 
    low: bool)
{
    /*
	 * According to the ARMv8-A rev. A.g, B2.10.5 "Load-Exclusive
	 * and Store-Exclusive instruction usage restrictions", state
	 * of the exclusive monitors after data abort exception is unknown.
	 */
	arm::clrex();

    let status = fault_info_from_esr(frame.tf_esr);

    //TODO: Temporary, this will need changing when there are legitimate
    //      reasons - ie COW
    if status.code == FaultStatusCode::PERM_FAULT
        || status.code == FaultStatusCode::ACCESS_FAULT 
    {
        panic!("ACCESS_FAULT");
    }

    let fault_address = virtual_address::from_u64(far);

    if let Some(address) = fault_address {
        match address {
            virtual_address::VirtualAddress::User(addr) => {
                println!(
                    "Fault address: {:?}",
                    address); 

                if process.address_space.handle_fault(addr) == false {
                    if low 
                    {
                        panic!("Process SEGFAULT");
                        //TODO: Handle a process segfault
                        //
                        //  Set the process status to DEAD
                        //  Return
                        //
                        //  During the "pre-return" check for another process
                        //  to schedule in
                        //
                    }
                    else
                    {
                        panic!("Unable to satisfy kernel page fault");
                    }
                    
                }
                arm::flush_tlb();
            },

            virtual_address::VirtualAddress::Kernel(_addr) => {
                println!(
                    "Kernel Page Fault: 0x{:X}", far);    
                panic!("Unkown address");    
            },
        } 
    } else {
        println!(
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
            arm::dump_frame(&*frame_ptr);
            panic!("Unhandled Exception: {:?}", exception);
        }
    }

    //TODO: Run scheduler check (we could get swapped out here)

    //TODO: Fix up the register clobbering memory/mod.rs
    //panic!("Unhandled exception")
    return -1;
}



#[no_mangle]
#[allow(dead_code)]
pub unsafe extern "C" fn do_el0_sync(
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
        Exception::INSN_ABORT_L => {
            let far = arm::read_far_el1();
            data_abort(frame, process, far, true);            
        },
        _ => {
            arm::dump_frame(&*frame_ptr);
            panic!("Unhandled Exception: {:?}", exception);
        }
    }

    //TODO: Fix up the register clobbering memory/mod.rs
    //panic!("Unhandled exception")
    return -1;
}
