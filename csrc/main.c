extern void sys_yield();

void __start() {
    for(;;) {

    }

    //void (*yield)() = (void*)0x808070;


    sys_yield();
}
