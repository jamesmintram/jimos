use core::panic::PanicInfo;

// #![macro_use]
// use kwriter;

//use user::syscall;
//use user;
//use core::fmt::Write;


extern "C" {
    fn exit() -> !;
}

// This function is called on panic.
#[panic_handler]
pub fn panic_fmt(panic_info: &PanicInfo) -> !
{
    println!("\n");
    println!("|=========================|");
    println!("|    GURU MEDITATION      |");
    println!("|=========================|");
    
    println!("\n");

    let cause = panic_info.payload()
        .downcast_ref::<i32>()
        .unwrap_or(&-1);

    println!( "System Code: {:?}", cause);
    println!( "Panic Message: {:?}", panic_info.message());

    //println!("\n");

    if let Some(location) = panic_info.location() {
         println!(
            "Location: '{}' at line {}", 
            location.file(),
            location.line());
    } else {
        println!("Location: unknown");
    }

    println!("\n"); 
    unsafe {exit()};
}