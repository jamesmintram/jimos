target remote localhost:1234
file build/blinky.elf

b _initial_thread_start
b _ctx_switch
b do_el1h_sync


layout next 
layout next 