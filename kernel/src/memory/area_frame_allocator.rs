use memory;
use memory::{Frame, FrameAllocator};

pub struct AreaFrameAllocator {
    next_free_frame: Frame,
}

impl AreaFrameAllocator {
    //
    //  Accepts the address from which it can start allocating from
    //
    pub fn new(start_address: usize) -> AreaFrameAllocator {
        let first_frame = Frame::containing_address(start_address).number;

        let allocator = AreaFrameAllocator {
            next_free_frame: Frame{number: first_frame},
        };
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

