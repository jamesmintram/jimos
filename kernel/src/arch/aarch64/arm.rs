pub fn read_far_el1() -> u64
{
    let mut val: u64 = 0;
    unsafe {
        asm!("mrs $0, far_el1": "=&r" (val):);
    };
    val
}



pub fn set_thread_ptr(ptr_value: usize) 
{
    unsafe {
        asm!("
            mov	x18, $0
            msr tpidr_el1, x18
        "
        :
        : "r"(ptr_value)
        : 
        );
    };
}

pub fn get_thread_ptr()  -> usize
{
    let mut ptr_value: usize;

    unsafe {
        asm!("mrs $0, tpidr_el1": "=&r" (ptr_value):);
    };

    ptr_value
}