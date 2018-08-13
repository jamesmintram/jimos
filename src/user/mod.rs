pub mod panic;
pub mod syscall;

pub const SYSCALL_OK:   i32 = 0;

pub const SYS_EXIT:     u32 = 100;
pub const SYS_PRINT:    u32 = 101;
pub const SYS_WRITE:    u32 = 102;

pub const SYS_INVALID:  u32 = 1000;


pub const STDOUT:       u32 = 0;

//------------------------------------------------------------------------
//------------------------------------------------------------------------



pub fn print() {
    let data = "Hello world!\n";

    let ptr = data.as_ptr();
    let len = data.len();

    //TODO: Not sure about this, need to make the 
    //point that stuff passed to syscall needs to be mutable

    //TODO: i.e where do the boundaries of safe -> unsafe occur?

    //TODO: Also consider a libc implementation, needs to be extern "C" etc
    //TODO: But then we will have a bunch of user code in Rust that could
    //      benefit from a safe System layer. (Like rust stdlib?)

    unsafe {
        syscall::syscall3(
            SYS_WRITE, 
            STDOUT as usize, 
            ptr as usize, 
            len as usize);
    }
}