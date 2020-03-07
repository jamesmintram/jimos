MyCPU, MyProcess



Set memory protection for everything up to BSS
Stick a write protected page between stack and BSS

Look at using armstub8
- https://github.com/raspberrypi/tools/blob/master/armstubs/armstub8.S


Deadlocks
- Locks held when context switch
- Multiple attempts to lock within same callstack
- Interrupted when a lock is held


usbdevice - 

Add a reboot handler inside of the panic FN - reset when P pressed
See why I cannot create 128 thread blocks (memory crash)
Refactor to thread model

Threads should use their own address spaces
- Switching thread should switch out page tables

Re-test loading an elf + running QEMU
- Bundle elf into the kernel image (So we can load it over ethernet)

Drop process to EL0
- Requires fixup of the exception handlers (Store/restore state)

Switch thread from the C/Elf process
- Requires a yield syscall




Frame Allocator 
    Release
    Page management
    Reverse mapping (maybe outside of the Frame Allocator)
    Page frame ownership

Address Space
    Unmapping a segment
    Splitting/Merging etc
    MProtect/W^X
    Use Result type in "new"




