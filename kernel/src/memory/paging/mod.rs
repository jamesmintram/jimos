pub mod entry;
pub mod table;

use memory::KERNEL_ADDRESS_START;
use memory::USER_ADDRESS_END;
use memory::PAGE_SIZE; 
use memory::Frame;

const ENTRY_COUNT: usize = 512;

use self::table::{Table, Level4};

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

pub struct Page {
   number: usize,
}

impl Page {
    pub fn containing_address(address: VirtualAddress) -> Page {
        assert!(address <= USER_ADDRESS_END || address >= KERNEL_ADDRESS_START, "Invalid address");
        Page{number: address / PAGE_SIZE}
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