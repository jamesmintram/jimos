use core::panic::PanicInfo;
use kwriter;
//use user::syscall;
//use user;

use core::fmt::Write;


extern "C" {
    fn exit() -> !;
}

// This function is called on panic.
#[panic_handler]
pub fn panic_fmt(panic_info: &PanicInfo) -> !
{
    write!(kwriter::WRITER,"\n\n");
    write!(kwriter::WRITER,"|=========================|\n");
    write!(kwriter::WRITER,"|    GURU MEDITATION      |\n");
    write!(kwriter::WRITER,"|=========================|\n");
    
    write!(kwriter::WRITER,"\n\n");

    let cause = panic_info.payload()
        .downcast_ref::<i32>()
        .unwrap_or(&-1);

    write!(kwriter::WRITER, "System Code: {:?}\n", cause);
    write!(kwriter::WRITER, "Panic Message: {:?}\n", panic_info.message());

    //write!(kwriter::WRITER,"\n\n");

    if let Some(location) = panic_info.location() {
        write!(
            kwriter::WRITER,
            "Location: '{}' at line {}", 
            location.file(),
            location.line());
    } else {
        write!(kwriter::WRITER,"Location: unknown");
    }

    write!(kwriter::WRITER,"\n\n"); 
    unsafe {exit()};
}