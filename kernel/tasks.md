Build and run a user process which prints to the screen via syscall
- Configure build to produce callable process within its own VA space
- Data aborts + JIT memory mapping for lower EL


Build and run a user process which prints to the screen via MMIO
- Different page table attributes - driven by VASegment Driver?


Build and run a driver process for UART output, another process to print through it
- Expose itself via /dev/tty
- Respond to fopen/fwrite/fclose
- Processes to have file handles
- Blocking IO, 
    -> Printer blocks
    -> Driver wakes up, does it's stuff then blocks on read
        ?? Somewhere here the Printer becomes unblocked (Ready)
    -> Printer resumes


Signal a process
- SIGKILL
- SIGABORT
- Send sigabort for a segmentation violation
    -> Triggered from a trap
- Send a SIGKILL for any other exception caused by process
- Send a kill signal from one process to another
- Enumerate processes?
    -> /proc/ ?
    -> By user, by group etc (Probably a program which iterates /proc/)


Fork/Exec
- A parent process which spawns and waits for a child process


Input
- Start processes from the simple command line shell
    -> Redirect IO to/from child process
- Pre-baked commands: ps, exit, shell, printhello
- Process can read from /dev/tty0


File system RO
- Launch a flat binary from simple shell
    -> Move ps and printhello into a filesystem
- Simple RO FAT image loaded at specified address


ELF Loader
- Extend flat binary images to ELF files (Statically linked)


Filesystem/VFS
- Enumerate a proc folder
- Enumerate the RO FAT FS


Drivers
- GPIO + PINMUXING
- Mailbox
- USB

Build and run a user process which does floating point
- Handle the abort
- Enable FPU
- Set a flag to save and restore FPU registers

