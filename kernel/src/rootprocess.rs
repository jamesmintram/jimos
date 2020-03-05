use process;
//use scheduler;
//use arch::aarch64::arm;

pub fn boot_root_process (mut process: process::Process ) {
    //let stack_range = process.address_space.get_segment_range(process.stack);
    let _text_range = process.address_space.get_segment_range(process.text);

    // Prepare our frame structure for a later call to return_to_userspace
    {
        let ref mut frame = &mut process.frame;
        frame.tf_sp = process.kernel_stack as u64;
        frame.tf_lr = 0x0;
        frame.tf_elr = root_process as *const () as u64;

        let mut spsr : u32 = 0;

        spsr |= 1 << 2;     // .M[3:2] = 0b100 -->  Return to EL1
        //spsr |= 1 << 6;     // FIQ masked
        //spsr |= 1 << 7;     // IRQ masked
        //spsr |= 1 << 8;     // SError (System Error) masked
        spsr |= 1 << 9;

        frame.tf_spsr = spsr;
    }

    //register PID with scheduler
    process::switch_process(&mut process);

    println!("Everything set - starting root_process");

    // process::resume_current_process();
    //arm::eret();
}

fn root_process() {
    println!("Entered root process");

    let process = process::get_current_process();

    if process.fork() {
        println!("I am forked");
    } else {
        println!("I am root");
    }

    loop {
        //scheduler::sleep();
    }

    //  // //Dump program (Hacky hex print)
    // let phys_addr = 0x3F000000;
    // let kva_addr = memory::physical_to_kernel(phys_addr);
    // let kva_addr_ptr = kva_addr as *const u8;
    // let slice = slice::from_raw_parts(kva_addr_ptr, 1024 * 128);

    // //Load elf
    // let elf = elf::Elf::from_data(&slice).ok().unwrap();

    // let mut process2 = process::Process::new(&KERNEL_FRAME_ALLOCATOR);
    // process2.exec(&elf);
}