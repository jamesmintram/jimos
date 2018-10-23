use alloc::alloc::{Layout, GlobalAlloc};

use memory::LockedAreaFrameAllocator;

use core::ops::Deref;
use spin::Mutex;

// use kwriter;
// use core::fmt::Write;

mod bucket;
mod paged_vector;
mod linked_list_allocator;
mod slab_allocator;

use self::linked_list_allocator::LinkedListAllocator;
use self::slab_allocator::SlabAllocator;

pub trait HeapAllocator 
{
    fn allocate(&mut self, size: usize) -> *mut u8;
    fn release(&mut self, ptr: *mut u8);
    fn release_unused(&mut self);
}
//-------------------------------------------

pub struct HeapSlabAllocator
{
    slab64: SlabAllocator,
    slab128: SlabAllocator,
    slab256: SlabAllocator,
    slab512: SlabAllocator,
    slab1024: SlabAllocator,
    slab2048: SlabAllocator,
    slab4096: SlabAllocator,

    large: LinkedListAllocator,
}

enum ObjectSize {
    Size64Bytes,
    Size128Bytes,
    Size256Bytes,
    Size512Bytes,
    Size1024Bytes,
    Size2048Bytes,
    Size4096Bytes,
    
    SizeLarge,
}

impl HeapSlabAllocator 
{
    pub fn new(frame_allocator: &'static LockedAreaFrameAllocator) -> HeapSlabAllocator {
        HeapSlabAllocator {
            slab64: SlabAllocator::new(frame_allocator, 64),
            slab128: SlabAllocator::new(frame_allocator, 128),
            slab256: SlabAllocator::new(frame_allocator, 256),
            slab512: SlabAllocator::new(frame_allocator, 512),
            slab1024: SlabAllocator::new(frame_allocator, 1024),
            slab2048: SlabAllocator::new(frame_allocator, 2048),
            slab4096: SlabAllocator::new(frame_allocator, 4096),

            large: LinkedListAllocator::new(frame_allocator),
        }
    }

    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let size = HeapSlabAllocator::layout_to_object_size(&layout);
        let allocator = self.object_size_to_allocator(size);

        return allocator.allocate(layout.size());
    }

    unsafe fn deallocate(&mut self, ptr: *mut u8, layout: Layout) {
        let size = HeapSlabAllocator::layout_to_object_size(&layout);
        let allocator = self.object_size_to_allocator(size);

        return allocator.release(ptr);
    }

    fn layout_to_object_size(layout: &Layout) -> ObjectSize 
    {
        if layout.size() > 4096 {
            ObjectSize::SizeLarge
        } else if layout.size() <= 64 && layout.align() <= 64 {
            ObjectSize::Size64Bytes
        } else if layout.size() <= 128 && layout.align() <= 128 {
            ObjectSize::Size128Bytes
        } else if layout.size() <= 256 && layout.align() <= 256 {
            ObjectSize::Size256Bytes
        } else if layout.size() <= 512 && layout.align() <= 512 {
            ObjectSize::Size512Bytes
        } else if layout.size() <= 1024 && layout.align() <= 1024 {
            ObjectSize::Size1024Bytes
        } else if layout.size() <= 2048 && layout.align() <= 2048 {
            ObjectSize::Size2048Bytes
        } else {
            ObjectSize::Size4096Bytes
        }
    }

    fn object_size_to_allocator(&mut self, size: ObjectSize) -> &mut HeapAllocator 
    {
        match size {
            ObjectSize::Size64Bytes => &mut self.slab64,
            ObjectSize::Size128Bytes => &mut self.slab128,
            ObjectSize::Size256Bytes => &mut self.slab256,
            ObjectSize::Size512Bytes => &mut self.slab512,
            ObjectSize::Size1024Bytes => &mut self.slab1024,
            ObjectSize::Size2048Bytes => &mut self.slab2048,
            ObjectSize::Size4096Bytes => &mut self.slab4096,
            ObjectSize::SizeLarge => &mut self.large,
        }
    }
}

pub struct LockedSlabHeap(Mutex<Option<HeapSlabAllocator>>);

impl LockedSlabHeap {
    /// Creates an empty heap. All allocate calls will return `None`.
    pub const fn empty() -> LockedSlabHeap {
        LockedSlabHeap(Mutex::new(None))
    }

    //NOTE: NOT THREAD SAFE!
    pub fn init(&mut self, allocator:  &'static LockedAreaFrameAllocator) {
        self.0 = Mutex::new(Some(HeapSlabAllocator::new(allocator)));
    }
}

unsafe impl GlobalAlloc for LockedSlabHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut lock = self.0.lock();

        if let Some(ref mut allocator) = *lock {
            return allocator.alloc(layout);
        }

        panic!();
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut lock = self.0.lock();
        if let Some(ref mut allocator) = *lock {
            allocator.deallocate(ptr, layout);
        }
    }
}

impl Deref for LockedSlabHeap {
    type Target = Mutex<Option<HeapSlabAllocator>>;

    fn deref(&self) -> &Mutex<Option<HeapSlabAllocator>> {
        &self.0
    }
}