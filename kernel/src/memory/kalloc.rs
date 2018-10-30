use memory;
use memory::Frame;
use memory::FrameAllocator;
use memory::LockedAreaFrameAllocator;

pub fn alloc_frames(
    frame_allocator: &LockedAreaFrameAllocator, 
    frame_count: usize) -> (Frame, Frame)
{
    let mut lock = frame_allocator.lock();
    if let Some(ref mut allocator) = *lock {
        
        let start = allocator
            .allocate_frames(frame_count)
            .expect("No more darta");

        let end = Frame{number: start.number + frame_count};

        return (start, end)
    }
    panic!()
}


pub fn alloc_pages(
    allocator: &LockedAreaFrameAllocator, 
    frame_count: usize) -> *mut u8
{
    let (start, _end) = alloc_frames(allocator, frame_count);
    let addr = memory::physical_to_kernel(start.start_address());
    return addr as *mut u8;
}

pub fn alloct_pages<T>(
    allocator: &LockedAreaFrameAllocator, 
    frame_count: usize) -> *mut T
{
    return alloc_pages(allocator, frame_count) as *mut T;
}

pub fn alloc_page(
    allocator: &LockedAreaFrameAllocator) -> *mut u8
{
    return alloc_pages(allocator, 1);
}