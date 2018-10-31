ASID improvements

When ASID wrap around, flush TLB, increment generation -> will force process to request 
a new ASID next time they are switched in.

Set pages to non-global to enable ASID in TLB

Set process ASID here:
http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0500e/CIHFFGFG.html

Select ASID here:
http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0500e/CIHFFGFG.html

Test a failure case where TLB flush means we read stale data. Then test non-global, ASID switch etc.



Address Space management

MapperT provides Page Table + Cache flags 

MMIO mapping 
    let seg_desc = {
        va_range: {start: 0, end: 0xdeadbeef},
        mapperT: MMIO{driver:MMIO, from, to},
    }

MMap one range to another
    let seg_desc = {
        va_range: {start: 0, end: 0xdeadbeef},
        mapperT: MMAP{driver:MMAP, va_src_from, va_src_to},
    }

Normal Address Space assignment
    let seg_desc = {
        va_range: {start: 0, end: 0xdeadbeef},
        mapperT: ANON{driver:ANON},
    }

where T is a Mapper

    let head_id = as.add_segment(seg_desc)
        which then calls
    mapperT::CreateRange(self, pgt, new_range)

    as.update_segment(seg_id, |va_range| {va_range.add(1024)});
        same as
    as.update_segment(seg_id, VARange::Add(1024));
        which then calls
    mapperT::UpdateRange(self, pgt, old_range, new_range)


    as.drop_segment(seg_id)
        mapperT::DropRange(self, pgt, old_range)


similar to pmap API in BSD to manage protection bits
    add_prot_read()
    add_prot_write()
    remove_prod_read()
    remove_prod_write()



Memory driver management

let mem_seg_desc1 = {
    "mmio",
    start_addr,
    end_addr,
    ACL = [MMIO, ROOT]
}

let mem_seg_desc2 = {
    "mailbox",
    start_addr,
    end_addr,
    ACL = [MBOX, ROOT]
}

memfs::init();
memfs::register(mem_seg_desc1);
memfs::register(mem_seg_desc2);


Driver process registration

let driver = create_process();
driver.acl.add_group("MMIO");
elf::load_image("MMIO_DRIVER", driver.address_space);
scheduler::schedule(driver);

Driver process code

let fd = fopen("/dev/mem/mmio", "rw")
let mmio_mem = mmap(null, fd, fd_size, RW);

let mmio = mmio::create(mmio_mem);

//.. Start doing stuff with mmio