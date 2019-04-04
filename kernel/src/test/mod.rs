use memory; 
use thread;

pub fn heap() 
{
    let mut vec_test = vec![1,2,3,4,5,6,7,1,2,3,4,5,6,7,1,2,3,4,5,6,7];
    vec_test[3] = 42;

    for _i in 0..1098 {
         vec_test.push(1);
    }
}

pub fn deadlock() 
{
    unsafe {
        if let Some(ref mut _allocator1) = *memory::KERNEL_FRAME_ALLOCATOR.lock() {
            if let Some(ref mut _allocator2) = *memory::KERNEL_FRAME_ALLOCATOR.lock() {
                //allocator.allocate_frame();
            }
        }   
    }
}

pub fn thread_custom_trampoline()
{
    let idle_thread = thread::create_thread(
        |fn_param| {},
        Some(|fn_ptr, fn_param|{
            println!("My own tam tam tamborine");
        }));

    thread::start_thread(idle_thread);
}