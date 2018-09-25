mod area_frame_allocator;
pub mod paging;
pub mod heap_allocator;
pub mod virtual_address;
pub mod address_space;

use memory;
use arch::aarch64::arm;

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

pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const PAGE_MASK: usize = !(PAGE_SIZE -1);

pub const ADDRESS_MASK: usize = 0x0000FFFF_FFFFF000;
// pub const INV_ADDRESS_MASK: usize = !ADDRESS_MASK;

// pub const ADDRESS_FLAGS_MASK: u64 = 0x00000000_00000FFF;

pub const TOTAL_MEMORY: usize = 0x3EFFFFFF;
pub const TOTAL_PAGE_FRAMES: usize = TOTAL_MEMORY / PAGE_SIZE;

impl Frame {
    fn containing_address(address: usize) -> Frame {
        let masked_address = address & ADDRESS_MASK;
        let number = masked_address >> PAGE_SHIFT;

        Frame{ number: number }
    }   

    pub fn start_address(&self) -> PhysicalAddress {
        self.number << PAGE_SHIFT
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

pub fn activate_el0(user_table: &Table<Level4>) 
{
    let user_table_ptr = user_table as *const _;
    let user_table_address = user_table_ptr as usize;
    let user_physical_table_address = memory::kernel_to_physical(user_table_address);

    arm::set_ttbr0_el1(user_physical_table_address);
    arm::flush_tlb();
}