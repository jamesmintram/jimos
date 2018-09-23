pub fn read_far_el1() -> u64
{
    let val: u64;
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
    let ptr_value: usize;

    unsafe {
        asm!("mrs $0, tpidr_el1": "=&r" (ptr_value):);
    };

    ptr_value
}

pub fn clrex()
{
    /*
	 * Ensure compiler barrier, otherwise the monitor clear might
	 * occur too late for us ?
	 */
    unsafe { asm!("clrex"::: "memory"); };
}