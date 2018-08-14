

#[no_mangle]
pub unsafe extern "C" fn syscall3(
    call_id : u32, 
    param1: usize, 
    param2: usize, 
    param3: usize) -> i32 
{
    let mut result: i32 = 0;

    asm!("
        mov x0, $1
        mov x1, $2
        mov x2, $3
        mov x3, $4
        svc 0
        mov $0, x0
    "
            : "=r"(result)
            : "r"(call_id)
            , "r"(param1)
            , "r"(param2)
            , "r"(param3)
            );


    return result;
}

#[no_mangle]
pub unsafe extern "C" fn syscall(
    call_id : u32) -> i32 
{
    let mut result: i32 = 0;

    asm!("
        mov x0, $1
        svc 0
        mov $0, x0
    "
            : "=r"(result)
            : "r"(call_id)
            );

    return result;
}
