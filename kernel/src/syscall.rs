use kwriter;
use core::fmt::Write;

//TODO: Remove this later
use uart;


//const SYSCALL_OK:   i32 = 0;

const SYS_EXIT:     u32 = 100;
const SYS_PRINT:    u32 = 101;
const SYS_WRITE:    u32 = 102;

//const SYS_INVALID:  u32 = 1000;


extern "C" {
    fn exit();
}


//------------------------------------------------------

fn k_print_el()
{
    let c: i32;
    unsafe {
        asm!("mrs $0, CurrentEL"
             : "=r"(c)
             :
             );
    }
    write!(kwriter::WRITER, "Exception level: {}\n", c);
}

fn k_exit()
{
    unsafe {exit();}
}

#[no_mangle]
#[allow(dead_code)]
pub unsafe extern "C" fn int_syscall(
    call_id : u32,
    p1: usize,
    p2: usize,
    p3: usize) -> i32
{
    write!(kwriter::WRITER, "Syscall: {}\n", call_id);

    if call_id == SYS_PRINT {
        k_print_el();
        return 0;
    }
    if call_id == SYS_EXIT {
        k_exit();
    }
    if call_id == SYS_WRITE {

         write!(kwriter::WRITER, "PRINT: {} :: {} :: {}\n", p1, p2, p3);

        let str_data = p2 as  *const u8;
        let str_len = p3 as isize;

        for idx in 0..str_len {
            uart::uart_send_byte(*str_data.offset(idx));
        }

        return 0;
    }

    return -1;
}

