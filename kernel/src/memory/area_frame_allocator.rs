use memory;
use memory::{Frame, FrameAllocator};

pub struct AreaFrameAllocator {
    next_free_frame: Frame,
}

impl AreaFrameAllocator {
    //
    //  Accepts the address from which it can start allocating from
    //
    pub fn new<'a> (physical_start_address: usize) -> &'a mut AreaFrameAllocator {

        let allocator_va = memory::physical_to_kernel(physical_start_address);
        let allocator_ptr = allocator_va as *mut AreaFrameAllocator;
        
        let allocator: &'a mut AreaFrameAllocator = unsafe {&mut *allocator_ptr};

        let allocator_memory = 1024 * 1024 * 16;    //16 TEMP

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

