//extern void sys_yield();
// void yield() {
//     //let mut result: i32 = 0;

//     __asm("svc 0");
//         //"svc 0");
    
//             //:
//             //: "r"(call_id)
//             // /);
// }


void __start() {
    // Test out the mapping
    volatile int *ptr = (int*)0x400000FF;
    //let data : *mut usize = addr1 as *mut usize;

    //TODO: GET WORKING ON HARDWARE    
    *ptr = 1024;
    //yield();
    for(;;) {

    }

    //void (*yield)() = (void*)0x808070;

    //sys_yield();
}
