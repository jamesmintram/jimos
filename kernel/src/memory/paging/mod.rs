pub mod entry;
pub mod table;

use memory::PAGE_SIZE; 
use memory::Frame;
use memory::FrameAllocator;
use memory::LockedAreaFrameAllocator;
use memory;

pub use self::entry::*;

const ENTRY_COUNT: usize = 512;

use self::table::{Table, Level4};

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;


pub struct Page {
   number: usize,
}

impl Page {
    pub fn containing_address(address: VirtualAddress) -> Page {
        //assert!(address <= USER_ADDRESS_END || address >= KERNEL_ADDRESS_START, "Invalid address");
        Page{number: address / PAGE_SIZE}
    }

    pub fn offset_by(&self, idx: usize) -> Page {
        Page{number: self.number + idx}
    }

    pub fn start_address(&self) -> usize {
        self.number * PAGE_SIZE
    }

    pub fn p4_index(&self) -> usize {
        (self.number) >> 27 & 0o777
    }
    pub fn p3_index(&self) -> usize {
        (self.number) >> 18 & 0o777
    }
    pub fn p2_index(&self) -> usize {
        (self.number) >> 9 & 0o777
    }
    pub fn p1_index(&self) -> usize {
        (self.number) >> 0 & 0o777
    }
}

pub fn translate_page(
    page_table: &Table<Level4>, 
    page: Page) -> Option<Frame> 
{
    page_table.next_table(page.p4_index())
    .and_then(|p3| p3.next_table(page.p3_index()))
    .and_then(|p2| p2.next_table(page.p2_index()))
    .and_then(|p1| p1[page.p1_index()].pointed_frame())
}

pub fn unmap<A>(
    page_table: &mut Table<Level4>, 
    page: Page, 
    allocator: &LockedAreaFrameAllocator)
        where A: FrameAllocator
{
    assert!(memory::virtual_to_physical(page_table, page.start_address()).is_some());

    let p1 = page_table
                 .next_table_mut(page.p4_index())
                 .and_then(|p3| p3.next_table_mut(page.p3_index()))
                 .and_then(|p2| p2.next_table_mut(page.p2_index()))
                 .unwrap();

    let frame = p1[page.p1_index()].pointed_frame().unwrap();
    p1[page.p1_index()].set_unused();
    // TODO free p(1,2,3) table if empty
    memory::kalloc::deallocate_frame(allocator, frame);
}

pub fn add_page(
    allocator: &LockedAreaFrameAllocator,
    page_table: &mut Table<Level4>,
    page: Page, 
    flags: EntryFlags) 
{
    let p3 = page_table.next_table_create(page.p4_index(), allocator);
    let p2 = p3.next_table_create(page.p3_index(), allocator);
    let p1 = p2.next_table_create(page.p2_index(), allocator);

    //TODO: Reinstate with proper semantics
    //assert!(p1[page.p1_index()].is_unused());

    let zero_frame = Frame{number: 0};
    p1[page.p1_index()].set(zero_frame, flags | ACCESS | TABLE_DESCRIPTOR);
}

pub fn map_to(
    allocator: &LockedAreaFrameAllocator,
    page_table: &mut Table<Level4>,
    page: Page,
    frame: Frame, 
    flags: EntryFlags)
{
    //print!("P3 [P4 Index: {:X?}]\n", page.p4_index());
    let p3 = page_table.next_table_create(page.p4_index(), allocator);

    //print!("P2 [P3 Index: {}]\n", page.p3_index());
    let p2 = p3.next_table_create(page.p3_index(), allocator);

    //print!("P1 [P2 Index: {}]\n", page.p2_index());
    let p1 = p2.next_table_create(page.p2_index(), allocator);

    //TODO: Reinstate with proper semantics
    //assert!(p1[page.p1_index()].is_unused());
    
    //print!("VA [P1 Index: {}]\n", page.p1_index());
    let new_flags = flags | PRESENT | ACCESS | TABLE_DESCRIPTOR;
    let new =  (frame.start_address() as u64) | new_flags.bits();


    //print!("CURRENT: {:X?}\n", p1[page.p1_index()].test());
    //print!("NEW: {:X?}\n", new);
    p1[page.p1_index()].set(frame, flags | PRESENT | ACCESS | TABLE_DESCRIPTOR);
}