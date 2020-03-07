use memory;
use memory::Frame;
use memory::FrameAllocator;

pub fn alloc_frames(frame_count: usize) -> (Frame, Frame)
{
    let mut lock = unsafe{memory::KERNEL_FRAME_ALLOCATOR.try_lock()};
    match lock {
        Some(locked_allocator) =>
        {    
            let ref mut allocator = *locked_allocator;
            let start = allocator
                .allocate_frames(frame_count)
                .expect("No more darta");

            let end = Frame{number: start.number + frame_count};

            return (start, end)
        }
        None => panic!("Failed to acquire KERNEL_FRAME_ALLOCATOR lock")
    };
}

pub fn alloc_frame() -> Frame
{
    let (frame, _) = alloc_frames(1);
    return frame;
}

pub fn alloc_pages(frame_count: usize) -> *mut u8
{
    let (start, _end) = alloc_frames(frame_count);
    let addr = memory::physical_to_kernel(start.start_address());
    return addr as *mut u8;
}

pub fn alloct_pages<T>(
    frame_count: usize) -> *mut T
{
    return alloc_pages(frame_count) as *mut T;
}

pub fn alloc_page() -> *mut u8
{
    return alloc_pages(1);
}

pub fn deallocate_frame(frame: Frame)
{
    let mut lock = unsafe {memory::KERNEL_FRAME_ALLOCATOR.lock()};
    if let Some(ref mut allocator) = *lock 
    {
        allocator.deallocate_frame(frame);
    }
    panic!()
}