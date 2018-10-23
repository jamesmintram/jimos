use memory;
use memory::{Frame, FrameAllocator};

use core::mem::size_of;
use core::slice;
use alloc::boxed::Box;

//use alloc::alloc::{GlobalAlloc, Layout};
use spin::Mutex;
use core::ops::Deref;

use core::fmt;

pub struct PageFrameData {
    in_use: bool,
    spec: bool,
}

struct PageFrameDataArray {
    ptr: *mut PageFrameData,
    len: usize,
}

impl fmt::Debug for PageFrameDataArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:?}", self.ptr)
    }
}

impl Deref for PageFrameDataArray {
    type Target = [PageFrameData];

    fn deref(&self) -> &[PageFrameData] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}


#[derive(Debug)]
pub struct AreaFrameAllocator {
    next_free_frame: Frame,
    last_frame: Frame,
    page_frame_data: PageFrameDataArray,
}

impl AreaFrameAllocator {
    //
    //  Accepts the address from which it can start allocating from
    //
    pub fn new<'a> (physical_start_address: usize, physical_end_address: usize) -> Box<AreaFrameAllocator> {

        let allocator_va = memory::physical_to_kernel(physical_start_address);
        let allocator_ptr = allocator_va as *mut AreaFrameAllocator;
        let mut allocator = unsafe {Box::from_raw(allocator_ptr)};

        let allocator_first_frame = Frame::containing_address(physical_start_address);
        let heap_last_frame = Frame::containing_address(physical_end_address);

        let page_frame_count = heap_last_frame.number - allocator_first_frame.number;
        
        let page_frame_data_address = physical_start_address + size_of::<AreaFrameAllocator>();
        let page_frame_data_array_size = size_of::<PageFrameData>() * page_frame_count;

        let physical_heap_start_address = page_frame_data_address + page_frame_data_array_size;
        let heap_first_frame = Frame::containing_address(physical_heap_start_address);
        
        let allocator_size = page_frame_data_array_size + size_of::<AreaFrameAllocator>();

        //TODO: Nasty hack - but I cannot do maths at this time in the morning
        //      This is to account for the fact that memory is used by the allocator
        let adjusted_heap_last_frame = Frame::containing_address(physical_end_address - allocator_size);
        let adjusted_page_frame_count = adjusted_heap_last_frame.number - allocator_first_frame.number;

        assert!(adjusted_page_frame_count > 0);

        allocator.next_free_frame = heap_first_frame;
        allocator.last_frame = adjusted_heap_last_frame;
        allocator.page_frame_data = PageFrameDataArray{
            ptr: page_frame_data_address as *mut PageFrameData,
            len: adjusted_page_frame_count,
        };


        use kwriter;
        use core::fmt::Write;

        write!(kwriter::WRITER, "Allocator\n",);
        write!(kwriter::WRITER, "\tFirst page frame: {}\n", allocator.next_free_frame.number);
        write!(kwriter::WRITER, "\tLast page frame: {}\n",  allocator.last_frame.number);
        write!(kwriter::WRITER, "\tPage frame count: {}\n", adjusted_page_frame_count);

        write!(kwriter::WRITER, "\tAllocator size (bytes): {}\n", allocator_size);
        write!(kwriter::WRITER, "\tAllocator size (page frames): {}\n", allocator_size / memory::PAGE_SIZE);

        allocator
    }   
}

impl FrameAllocator for AreaFrameAllocator 
{
    fn allocate_frame(&mut self) -> Option<Frame> 
    {
        self.allocate_frames(1)
    }

    fn allocate_frames(&mut self, count:usize) -> Option<Frame> 
    {
        let next_frame_after_alloc = self.next_free_frame.number + count;
        if next_frame_after_alloc < self.last_frame.number 
        {
            let frame = Frame{number: self.next_free_frame.number};
         
            self.next_free_frame.number += count;

            return Some(frame)
        }
        return None
    }

    fn deallocate_frame(&mut self, _frame: Frame) {
        // TODO (see below)
    }
}

//------------------------------------------------------------------------


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


pub fn alloc(
    allocator: &LockedAreaFrameAllocator, 
    frame_count: usize) -> *mut u8
{
    let (start, _end) = alloc_frames(allocator, frame_count);
    let addr = memory::physical_to_kernel(start.start_address());
    return addr as *mut u8;

}

pub fn alloct<T>(
    allocator: &LockedAreaFrameAllocator, 
    frame_count: usize) -> *mut T
{
    return alloc(allocator, frame_count) as *mut T;
}


impl Deref for LockedAreaFrameAllocator {
    type Target = Mutex<Option<Box<AreaFrameAllocator>>>;

    fn deref(&self) -> &Mutex<Option<Box<AreaFrameAllocator>>> {
        &self.0
    }
}
 