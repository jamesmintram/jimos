use memory;
use memory::{Frame, FrameAllocator};

use alloc::boxed::Box;

#[derive(Debug)]
pub struct AreaFrameAllocator {
    next_free_frame: Frame,
}

impl AreaFrameAllocator {
    //
    //  Accepts the address from which it can start allocating from
    //
    pub fn new<'a> (physical_start_address: usize) -> Box<AreaFrameAllocator> {

        let allocator_va = memory::physical_to_kernel(physical_start_address);
        let allocator_ptr = allocator_va as *mut AreaFrameAllocator;
        
        //We actually need a box here
        let mut allocator = unsafe {Box::from_raw(allocator_ptr)};

        let allocator_memory = 1024 * 16;    //16KB TEMP (Allocator data)

        let physical_heap_start_address = physical_start_address + allocator_memory;
        let first_frame = Frame::containing_address(physical_heap_start_address);

        allocator.next_free_frame = first_frame;

        allocator
    }   
}

impl FrameAllocator for AreaFrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame> {
        if self.next_free_frame.number < memory::TOTAL_PAGE_FRAMES 
        {
            let frame = Frame{number: self.next_free_frame.number};
         
            self.next_free_frame.number += 1;

            return Some(frame)
        }
        return None
    }

    fn deallocate_frame(&mut self, _frame: Frame) {
        // TODO (see below)
    }
}

//------------------------------------------------------------------------
use alloc::alloc::{GlobalAlloc, Layout};
use core::ops::Deref;
use spin::Mutex;

#[derive(Debug)]
pub struct LockedAreaFrameAllocator(Mutex<Option<Box<AreaFrameAllocator>>>);

impl LockedAreaFrameAllocator {
    //NOTE: NOTE THREAD SAFE!
    pub const fn empty() -> LockedAreaFrameAllocator {
        LockedAreaFrameAllocator(Mutex::new(None))
    }

    pub fn init(&mut self, allocator: Box<AreaFrameAllocator>) {
        self.0 = Mutex::new(Some(allocator));
    }
}

impl Deref for LockedAreaFrameAllocator {
    type Target = Mutex<Option<Box<AreaFrameAllocator>>>;

    fn deref(&self) -> &Mutex<Option<Box<AreaFrameAllocator>>> {
        &self.0
    }
}
 