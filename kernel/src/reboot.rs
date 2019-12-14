use memory;

pub const PM_RSTC: *mut u32 = (memory::KERNEL_ADDRESS_START + 0x3F10001c) as *mut u32;
pub const PM_WDOG: *mut u32 = (memory::KERNEL_ADDRESS_START + 0x3F100024) as *mut u32;
pub const PM_RSTS: *mut u32 = (memory::KERNEL_ADDRESS_START + 0x3F100020) as *mut u32;

pub const PM_PASSWORD: u32 = 0x5a000000;
pub const PM_RSTC_WRCFG_FULL_RESET: u32 = 0x00000020;

pub unsafe fn reboot()
{
    let mut r = PM_RSTS.read_volatile();
    
    r &= !0xfffffaaa;

    PM_RSTS.write_volatile(r);
    PM_WDOG.write_volatile(PM_PASSWORD | 1);
    PM_RSTC.write_volatile(PM_PASSWORD | PM_RSTC_WRCFG_FULL_RESET);


    loop {

    }
}
