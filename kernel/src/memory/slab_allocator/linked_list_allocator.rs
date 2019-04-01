use memory;
use memory::LockedAreaFrameAllocator;
use memory::slab_allocator::HeapAllocator;

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
        println!("Alloc {} frames from list allocator\n", frame_count);
        memory::alloc_pages(frame_count)
    }
    fn release(&mut self, _ptr: *mut u8) 
    {
        println!("Release to Physical: \n");
    }
    fn release_unused(&mut self) 
    {

    }
}