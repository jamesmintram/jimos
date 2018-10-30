use memory;
use memory::kalloc;
use memory::FrameAllocator;
use memory::LockedAreaFrameAllocator;
use memory::physical_to_kernel;

use memory::paging::entry::*;
use memory::paging::ENTRY_COUNT;

use core::marker::PhantomData;

pub trait TableLevel {}

pub enum Level4 {}
pub enum Level3 {}
pub enum Level2 {}
pub enum Level1 {}

impl TableLevel for Level4 {}
impl TableLevel for Level3 {}
impl TableLevel for Level2 {}
impl TableLevel for Level1 {}

pub trait HierarchicalLevel: TableLevel {
    type NextLevel: TableLevel;
}

impl HierarchicalLevel for Level4 {
    type NextLevel = Level3;
}

impl HierarchicalLevel for Level3 {
    type NextLevel = Level2;
}

impl HierarchicalLevel for Level2 {
    type NextLevel = Level1;
}

pub struct Table<L: TableLevel> {
    entries: [Entry; ENTRY_COUNT],
    level: PhantomData<L>,
}

impl<L> Table<L> where L: TableLevel {
    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.set_unused();
        }
    }   
}

impl<L> Table<L> where L: HierarchicalLevel {
    pub fn next_table_address(&self, index: usize) -> Option<usize> {
        let entry_flags = self[index].flags();

        if entry_flags.contains(PRESENT) { 
            let table_address = self[index].test() & memory::PAGE_MASK;
            let kernel_address = physical_to_kernel(table_address);

            Some(kernel_address)
        } else {
            None
        }
    }

    pub fn next_table(&self, index: usize) -> Option<&Table<L::NextLevel>> {
        self.next_table_address(index)
            .map(|address| unsafe { &*(address as *const _) })
    }

    pub fn next_table_mut(
        &mut self, 
        index: usize) -> Option<&mut Table<L::NextLevel>> 
    {
        self.next_table_address(index)
            .map(|address| unsafe { &mut *(address as *mut _) })
    }

    pub fn next_table_create<A>(
        &mut self,
        index: usize,
        allocator: &mut A) -> &mut Table<L::NextLevel>
            where A: FrameAllocator
    {
        if self.next_table(index).is_none() {
            
            let frame = allocator
                .allocate_frame()
                .expect("no frames available");

            self.entries[index]
                .set(frame, PRESENT | TABLE_DESCRIPTOR);

            self.next_table_mut(index)
                .unwrap()
                .zero();
        }
        self.next_table_mut(index)
            .unwrap()
    }
}

use core::ops::{Index, IndexMut};

impl<L> Index<usize> for Table<L> where L: TableLevel {
    type Output = Entry;

    fn index(&self, index: usize) -> &Entry {
        &self.entries[index]
    }
}

impl<L> IndexMut<usize> for Table<L> where L: TableLevel {
    fn index_mut(&mut self, index: usize) -> &mut Entry {
        &mut self.entries[index]
    }
}

pub fn new (frame_allocator: &LockedAreaFrameAllocator) -> &mut Table<Level4>
{
    let new_pgt = kalloc::alloc_page(frame_allocator);
    let new_pgt_ptr: *mut Table<Level4> = new_pgt as *mut _;
    let new_pgt = unsafe { &mut (*new_pgt_ptr) };

    new_pgt
}