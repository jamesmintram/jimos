mod area_frame_allocator;
pub mod paging;

use self::paging::PhysicalAddress;
use self::paging::VirtualAddress;

use self::paging::table::{Table, Level4};
use self::paging::Page;
use self::paging::translate_page;

pub use self::area_frame_allocator::AreaFrameAllocator;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

pub const USER_ADDRESS_END: usize = 0x0000FFFF_FFFFFFFF;

pub const KERNEL_ADDRESS_START: usize = 0xFFFF0000_00000000;
pub const KERNEL_ADDRESS_MASK: usize = !KERNEL_ADDRESS_START;

pub const PAGE_SIZE: usize = 4096;
pub const PAGE_MASK: usize = !(PAGE_SIZE -1);

pub const TOTAL_MEMORY: usize = 0x3EFFFFFF;
pub const TOTAL_PAGE_FRAMES: usize = TOTAL_MEMORY / PAGE_SIZE;

impl Frame {
    fn containing_address(address: usize) -> Frame {
        Frame{ number: address / PAGE_SIZE }
    }   

    pub fn start_address(&self) -> PhysicalAddress {
        self.number * PAGE_SIZE
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

pub fn kernel_to_physical(
    virtual_address: VirtualAddress) -> PhysicalAddress 
{
    return virtual_address & KERNEL_ADDRESS_MASK;
}

pub fn physical_to_kernel(
    physical_address: PhysicalAddress) -> VirtualAddress 
{
    return physical_address | KERNEL_ADDRESS_START;
}

pub fn virtual_to_physical(
    page_table: &Table<Level4>, 
    virtual_address: VirtualAddress) -> Option<PhysicalAddress>
{
     let offset = virtual_address % PAGE_SIZE;
     translate_page(page_table, Page::containing_address(virtual_address))
         .map(|frame| frame.number * PAGE_SIZE + offset)
}

pub fn clear_el0() 
{
    unsafe {
        asm!("
            msr ttbr0_el1, xzr
            dsb ish
            isb
        "::);
    }

}