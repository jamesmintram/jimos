use memory::paging::entry::*;
use memory::paging::ENTRY_COUNT;

pub struct Table {
    entries: [Entry; ENTRY_COUNT],
}

impl Table {
    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.set_unused();
        }
    }

    pub fn test_my_addr(&self) -> usize {
        return self as *const _ as usize;
    }

    pub fn next_table_address(&self, index: usize) -> Option<usize> {
        let entry_flags = self[index].flags();
        if entry_flags.contains(PRESENT) { //&& !entry_flags.contains(HUGE_PAGE) {
            let table_address = self[index].test() & 0x000fffff_fffff00;
            Some(table_address)
        } else {
            None
        }
    }

    pub fn next_table(&self, index: usize) -> Option<&Table> {
        self.next_table_address(index)
            .map(|address| unsafe { &*(address as *const _) })
    }

    pub fn next_table_mut(&mut self, index: usize) -> Option<&mut Table> {
        self.next_table_address(index)
        .map(|address| unsafe { &mut *(address as *mut _) })
    }
}

use core::ops::{Index, IndexMut};

impl Index<usize> for Table {
    type Output = Entry;

    fn index(&self, index: usize) -> &Entry {
        &self.entries[index]
    }
}

impl IndexMut<usize> for Table {
    fn index_mut(&mut self, index: usize) -> &mut Entry {
        &mut self.entries[index]
    }
}