use memory;
use memory::LockedAreaFrameAllocator;
use memory::slab_allocator::HeapAllocator;

use kwriter;
use core::fmt::Write;

pub struct LinkedListAllocator 
{
    count: usize,

    //TODO: Fix the lifetime
    allocator: &'static LockedAreaFrameAllocator,
}

impl LinkedListAllocator 
{
    pub fn new(frame_allocator: &'static LockedAreaFrameAllocator) -> LinkedListAllocator 
    {
        LinkedListAllocator {
            count: 0,
            allocator: frame_allocator,
        }
    }    
}

impl HeapAllocator for LinkedListAllocator {
    fn allocate(&mut self, size: usize) -> *mut u8 
    {
        let frame_count = (size / memory::PAGE_SIZE) + 1;
        write!(kwriter::WRITER, "Alloc {} frames from list allocator\n", frame_count);
        memory::alloc(self.allocator, frame_count)
    }
    fn release(&mut self, _ptr: *mut u8) 
    {
        write!(kwriter::WRITER, "Release to Physical: \n");
    }
    fn release_unused(&mut self) 
    {

    }
}