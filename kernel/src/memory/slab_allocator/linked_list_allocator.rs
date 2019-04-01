use memory;
use memory::slab_allocator::HeapAllocator;

pub struct LinkedListAllocator 
{
    count: usize,
}

impl LinkedListAllocator 
{
    pub fn new() -> LinkedListAllocator 
    {
        LinkedListAllocator {
            count: 0,
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