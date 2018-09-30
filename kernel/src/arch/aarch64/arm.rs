pub fn read_far_el1() -> u64
{
    let val: u64;
    unsafe {
        asm!("mrs $0, far_el1": "=&r" (val):);
    };
    val
}

pub fn set_thread_ptr(ptr_value: usize) 
{
    unsafe {
        asm!("
            mov	x18, $0
            msr tpidr_el1, x18
        "
        :
        : "r"(ptr_value)
        : 
        );
    };
}

pub fn get_thread_ptr()  -> usize
{
    let ptr_value: usize;

    unsafe {
        asm!("mrs $0, tpidr_el1": "=&r" (ptr_value):);
    };

    ptr_value
}

pub fn clrex()
{
    /*
	 * Ensure compiler barrier, otherwise the monitor clear might
	 * occur too late for us ?
	 */
    unsafe { asm!("clrex"::: "memory"); };
}

pub fn reset_ttbr0_el1() 
{
    set_ttbr0_el1(0);
}

pub fn set_ttbr0_el1(value: usize) 
{
    //TODO: Fix up the register clobbering
    unsafe {
        asm!("
            mov x18, $0
            msr ttbr0_el1, x18
        "
        :
        : "r"(value)
        : 
        );
    };
}

pub fn flush_tlb()
{
    unsafe {
        asm!("
            TLBI VMALLE1
            dsb ish
            isb
        "::);
    };
}

// Cache

/* CTR_EL0 - Cache Type Register */
const CTR_DLINE_SHIFT: usize =	16;
const CTR_DLINE_MASK: usize =	0xf << CTR_DLINE_SHIFT;

const CTR_ILINE_SHIFT: usize =	0;
const CTR_ILINE_MASK: usize =	0xf << CTR_ILINE_SHIFT;

fn ctr_dline_size(reg: usize) -> usize {
    ((reg) & CTR_DLINE_MASK) >> CTR_DLINE_SHIFT
}

fn ctr_iline_size(reg: usize) -> usize {
    ((reg) & CTR_ILINE_MASK) >> CTR_ILINE_SHIFT
}

pub fn get_ctr_el0()  -> usize
{
    let ctr_value: usize;

    unsafe {
        asm!("mrs $0, ctr_el0": "=&r" (ctr_value):);
    };

    ctr_value
}

//TODO: Better way to handle this?
static mut D_CACHE_LINE_SIZE: usize = 0;
static mut I_CACHE_LINE_SIZE: usize = 0;

pub fn dcache_line_size() -> usize 
{
    //TODO: Debug only
    let size = unsafe{D_CACHE_LINE_SIZE};
    if size == 0 {
        panic!("Must call cache_setup")
    }

    size
}
pub fn icache_line_size() -> usize 
{
    //TODO: Debug only
    let size = unsafe{I_CACHE_LINE_SIZE};
    if size == 0 {
        panic!("Must call cache_setup")
    }

    size
}

use core;

pub fn cache_setup()
{
    //TODO: Ensure only called once

    //We modify a static
    unsafe {
        let ctr_el0 = get_ctr_el0();

        // Read the log2 words in each D cache line
        let dcache_line_shift = ctr_dline_size(ctr_el0);
        // Get the D cache line size 
        D_CACHE_LINE_SIZE = core::mem::size_of::<usize>() << dcache_line_shift;

        // And the same for the I cache 
        let icache_line_shift = ctr_iline_size(ctr_el0);
        I_CACHE_LINE_SIZE = core::mem::size_of::<usize>() << icache_line_shift;
        I_CACHE_LINE_SIZE = core::cmp::min(D_CACHE_LINE_SIZE, I_CACHE_LINE_SIZE);

       
    }

}

pub fn print_cache_info()
{
    unsafe {
        use kwriter;
        use core::fmt::Write;

        write!(kwriter::WRITER, "DCache size: {}\n", D_CACHE_LINE_SIZE);
        write!(kwriter::WRITER, "ICache size: {}\n", I_CACHE_LINE_SIZE);
    }
}

pub fn cache_clean(addr: usize)
{
    unsafe  {
        asm!("
            dc civac, $0
            dsb	ish
        " : : "r" (addr) : "memory");
    }
}

pub fn cache_clean_range(addr: usize, size: usize)
{
    let cache_line_size = dcache_line_size();
    let cache_line_count = (size / cache_line_size) + 1;

    for i in 0..cache_line_count {
        let cache_addr = addr + i * cache_line_size;

        unsafe  {
            asm!("
                dc civac, $0
                dsb	ish
            " : : "r" (cache_addr) : "memory");
        } //TODO: Can we defer "dsb ish" until after the loop?
    }
}