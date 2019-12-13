mod area_frame_allocator;
pub mod paging;
pub mod heap_allocator;
pub mod virtual_address;
pub mod address_space;
pub mod va_segment;
pub mod slab_allocator;
pub mod kalloc;

use memory;

use arch::aarch64::arm;

pub use self::paging::PhysicalAddress;
pub use self::paging::VirtualAddress;

use self::paging::table::{Table, Level4};
use self::paging::Page;
use self::paging::translate_page;

pub use self::kalloc::{alloc_pages, alloct_pages, alloc_frames};
pub use self::area_frame_allocator::AreaFrameAllocator;
pub use self::area_frame_allocator::LockedAreaFrameAllocator;

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

//--------------------------------------------------------------------

//pub const HEAP_START: usize = 1024 * 1024 * 256; // 256MB for now
pub const HEAP_SIZE: usize = 1024 * 1024 * 256; // 256MB for now

pub static mut KERNEL_FRAME_ALLOCATOR: LockedAreaFrameAllocator = LockedAreaFrameAllocator::empty();
pub static mut ANON_FRAME_ALLOCATOR: LockedAreaFrameAllocator = LockedAreaFrameAllocator::empty();

//static mut HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();
extern "C" {
    static mut kernel_end: u8;
}

pub fn init() {
    //kernel_end comes from linker scripts
    let kernel_end_addr =  unsafe{(&kernel_end as *const _) as usize};

    // Initialise the heaps
    let heap_start = kernel_end_addr + memory::KERNEL_ADDRESS_START;
    let heap_end = heap_start + HEAP_SIZE;
    
    let anon_mem_start = heap_end;
    let anon_mem_end = anon_mem_start + HEAP_SIZE;

    unsafe {
        KERNEL_FRAME_ALLOCATOR.init(memory::AreaFrameAllocator::new(kernel_end_addr, anon_mem_start));
        ANON_FRAME_ALLOCATOR.init(memory::AreaFrameAllocator::new(anon_mem_start, anon_mem_end));

        ::HEAP_ALLOCATOR.init();
    }
}

//--------------------------------------------------------------------

impl Frame {
    fn containing_address(address: usize) -> Frame {
        let masked_address = address & ADDRESS_MASK;
        let number = masked_address >> PAGE_SHIFT;

        Frame{ number: number }
    }   

    pub fn start_address(&self) -> PhysicalAddress {
        self.number << PAGE_SHIFT
    }
    pub fn end_address(&self) -> PhysicalAddress {
        ((self.number + 1) << PAGE_SHIFT) -1
    }
}


pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn allocate_frames(&mut self, count:usize) -> Option<Frame>;
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

pub fn activate_address_space(address_space: &address_space::AddressSpace) 
{
    //TODO: Do we need an ASID? If so, get one. 
    let as_table: &Table<Level4> = &address_space.page_table;

    let as_table_ptr = as_table as *const _;
    let as_table_address = as_table_ptr as usize;
    let as_physical_table_address = memory::kernel_to_physical(as_table_address);

    arm::set_ttbr0_el1(as_physical_table_address);
    arm::flush_tlb();
}