
https://access.redhat.com/documentation/en-US/Red_Hat_Enterprise_Linux/4/html/Using_ld_the_GNU_Linker/sections.html

- Find out information about the system and populate a struct
    - Amount of RAM
    - End address of Kernel binary

    https://www.raspberrypi.org/forums/viewtopic.php?f=72&t=219677

- Split apart into Userspace and Kernelspace code using Rust modules
    
- Catch and properly handle illegal instruction
    https://github.com/s-matyukevich/raspberry-pi-os/blob/master/docs/lesson06/rpi-os.md#allocating-new-pages-on-demand

    - Print out some sort of dump

- All non EL0 exception handler should kernel panic with dump

- Build another rust program which uses shared Userspace rust modules
    - Load and run at a fixed place

- Memory and stack management
    https://github.com/s-matyukevich/raspberry-pi-os/blob/master/docs/lesson06/rpi-os.md
    - Enable MMU 
        - Fixed table to start with
        - Load in other program and run (Using a fixed array of process entries)
        - No allocations

- Context management around threads
    - Load above program twice using MMU + per process page tables
    - Add a yield method to pass control onto the next process

See the same process running twice in its own memory address space
with task switching.

- Enable new/delete in rust (slab allocator?)
- Mapping of kernel space and userspace
    - Relates to KAlloc and MAlloc
    - Enable page protection so User cannot read Kernel
    - Check linux kernel whether kernel can read user? (or how)
    - Copy memory from userspace to kernel space

- Look at fork
    - Process ownership + parent processes
    - COW for child page tables?
    - Prevent parent from modifying until exec? (Check with others)

- Scheduling
    - Pre-emptive
        - Kernel locking BKL
    - Syscall
    - Blocking reads/writes to IO
        - Wait queues etc

- TTY/Input/Output/Dev filesystem
    - Reading/Writing STDIO
    - Multiplex output over UART
    - Look at how input drivers work in Minix etc
    - Simple terminal program


USB + Network

https://github.com/jamesmintram/csud
https://github.com/rsta2/circle/tree/master/lib/usb

//------------------------------------------------------------------------------



Syscalls should work

Activate MMU and run program 2x in same memory map using cooperative yield
Map memory 
    -> Kernel un lower and Userspace in upper
    -> Test memory protection is working

Simple Memory management? 
Load a basic ELF file and execute

Fork/Exec
Pre-emptive multi tasking


RUST
-----
https://www.youtube.com/watch?time_continue=87&v=7Mzl2HA3TuU

ARM Architecture
----------------
https://quequero.org/2014/04/introduction-to-arm-architecture/
http://arm.ninja/2016/03/07/decoding-syscalls-in-arm64/



Improve Messagebox interface
Basic Framebuffer?

-----------------------------------------------------------------------------------------------------------------------

This works as we explicitly load the binary images into the memory locations we need them to be

qemu-system-aarch64 -M raspi3 -semihosting -serial stdio -device loader,file=build/blinky.bin,addr=0x0 -device loader,file=build/prog1.bin,addr=0x100000

qemu-system-aarch64 -M raspi3 -semihosting -serial stdio -device loader,file=build/blinky.bin,addr=0x0 

Mailbox stuff

http://www.valvers.com/open-software/raspberry-pi/step05-bare-metal-programming-in-c-pt5/


FP Hang Issue

https://stackoverflow.com/questions/24589235/application-hangs-when-calling-printf-to-uart-with-bare-metal-raspberry-pi/27257841#27257841


-----------------------------------------------------------------------------------------------------------------------
-----------------------------------------------------------------------------------------------------------------------
-----------------------------------------------------------------------------------------------------------------------


-device loader,file=<file>[,addr=<addr>][,cpu-num=<cpu-num>][,force-raw=<raw>]

qemu-system-aarch64 -M raspi3 -kernel blinky.img -serial stdio

qemu-system-aarch64 -M raspi3 -semihosting -kernel build/blinky.bin -serial stdio -device loader,file=build/prog1.bin,addr=0x10000,force-raw=on -curses

qemu-system-aarch64 -M raspi3 -semihosting -kernel build/blinky.bin  -device loader,file=build/prog1.bin,addr=0x10000,force-raw=on

qemu-system-aarch64 -M raspi3 -semihosting -kernel build/blinky.elf -serial stdio -device loader,file=build/prog1.bin,addr=0x100000 -curses


//------------------------------------------------------

#[inline(never)]
fn spin_sleep_ms(ms: usize) {
    for _ in 0..(ms * 600) {
        unsafe { asm!("nop" :::: "volatile"); }
    }
}

// macro_rules! print {
//     ($($arg:tt)*) => ({
//         use core::fmt::Write;
//         WRITER.write_fmt(format_args!($($arg)*)).unwrap();
//     });
// }


//------------------------------------------------------
