//extern void sys_yield();

void __start() {
    // Test out the mapping
    int *ptr = (int*)0x40000000;
    //let data : *mut usize = addr1 as *mut usize;

    //TODO: GET WORKING ON HARDWARE    
    *ptr = 1024;

    // for(;;) {

    // }

    //void (*yield)() = (void*)0x808070;

    //sys_yield();
}
