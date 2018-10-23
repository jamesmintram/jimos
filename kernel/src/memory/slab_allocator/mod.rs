use alloc::alloc::{Layout, GlobalAlloc};
use alloc::boxed::Box;

use memory;
use memory::Frame;


use memory::LockedAreaFrameAllocator;
use core::mem::size_of;
use core::ops::Deref;
use core::slice;
use spin::Mutex;

use kwriter;
use core::fmt::Write;

pub trait HeapAllocator 
{
    fn allocate(&mut self, size: usize) -> *mut u8;
    fn release(&mut self, ptr: *mut u8);
    fn release_unused(&mut self);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BucketStatus {
    Unused,
    Partial,
    Full,
}

//TODO: Make copyable?
struct Bucket {
    start: Frame,
    end: Frame,
    first_free: u64,
    status: BucketStatus,
}

//----------------------------------------

struct PagedVectorPage {
    next: *mut PagedVectorPage,
    count: usize,
}

impl PagedVectorPage {
    fn max(&self) -> usize
    {
        //TODO: Implement properly
        return 4;
    }
    fn is_full(&self) -> bool
    {
        return self.count == self.max();
    }

    fn new(frame_allocator: &'static LockedAreaFrameAllocator) -> Box<PagedVectorPage> 
    {
        let page = memory::alloct::<PagedVectorPage>(frame_allocator, 1);
        unsafe {
            (*page).next = 0 as *mut PagedVectorPage;
            (*page).count = 0;
            
            return Box::from_raw(page)
        }
    }
    fn first_bucket(&mut self) -> *mut Bucket 
    {
        let self_raw = self as *mut PagedVectorPage;
        return unsafe {self_raw.add(1) as *mut Bucket};
    }

    fn get_buckets(&mut self) -> &mut [Bucket]
    {
        let first_bucket = self.first_bucket();
        unsafe { slice::from_raw_parts_mut(first_bucket, self.count) }        
    }

    fn add_bucket(&mut self, bucket: Bucket) 
    {
        if self.is_full()
        {
            panic!();
        }

        let first_bucket = self.first_bucket();
        
        unsafe {
            let target_bucket = first_bucket.add(self.count);
            target_bucket.write(bucket);
        }
        self.count += 1;
    }
}

struct PagedVector {
    head: Box<PagedVectorPage>,

    //TODO: Fix the lifetime
    allocator: &'static LockedAreaFrameAllocator,
}

impl PagedVector 
{
    fn new(allocator: &'static LockedAreaFrameAllocator) -> PagedVector 
    {
        let first_page = PagedVectorPage::new(allocator);

        return PagedVector{
            head: first_page,
            allocator: allocator,
        }
    }

    fn update_one<
        Pred: Fn(&Bucket) -> bool, 
        Update: Fn(&mut Bucket) -> ()> 
            (&mut self, pred: Pred, update: Update) -> bool
    {
        let buckets = self.head.get_buckets();

        //TODO: Go page by page
        for ref mut bucket in buckets.iter_mut() {
            write!(kwriter::WRITER, "Iter bucket!\n");
            if (pred(bucket))
            {
                write!(kwriter::WRITER, "Found bucket!\n");
                update(bucket);
                return true;
            }
        }

        return false;
    }

    fn add_one<
        Create: Fn() -> Bucket, 
        Update: Fn(&mut Bucket) -> ()> 
            (&mut self, create: Create, update: Update) -> bool
    {
        if self.head.is_full() == false 
        {
            write!(kwriter::WRITER, "Add bucket!\n");

            let mut new_bucket = create();
            update(&mut new_bucket);
            self.head.add_bucket(new_bucket);
            
            return true;
        }
        
        //If I can not add a bucket (because the descriptor page is full?)
        write!(kwriter::WRITER, "Failed to create a new bucket\n");
        return false;
    }
}

//----------------------------------------

pub struct SlabAllocator 
{
    count: usize,
    object_size: usize,

    bucket_data: PagedVector,
    //TODO: Fix the lifetime
    allocator: &'static LockedAreaFrameAllocator,
}

impl HeapAllocator for SlabAllocator {
    fn allocate(&mut self, _size: usize) -> *mut u8 
    {    
        //TODO: Track wastage using _size

        let updated = self.bucket_data.update_one(
            |bucket|  {bucket.status != BucketStatus::Full},
            |bucket|  {bucket.status = BucketStatus::Full});

        if updated == false 
        {
            self.bucket_data.add_one(
                || {Bucket{
                        start: Frame{number: 0},
                        end: Frame{number: 0},
                        first_free: 0,
                        status: BucketStatus::Unused,
                    }},
                |bucket| {bucket.status = BucketStatus::Partial}
            );
        }

        //Find a bucket with space
        //  Get the first free object in the bucket
        //Otherwise write and panic

    
        //TODO: Remove all of this
        write!(kwriter::WRITER, "Alloc from Slab: {}\n", self.object_size);
        let frame_count = (self.object_size / memory::PAGE_SIZE) + 1;
        memory::alloc(self.allocator, frame_count)
    }
    fn release(&mut self, _ptr: *mut u8) {
        write!(kwriter::WRITER, "Release to Slab: {}\n", self.object_size);
    }
    fn release_unused(&mut self) {

    }
}

impl SlabAllocator 
{
    fn new(allocator: &'static LockedAreaFrameAllocator, object_size: usize) -> SlabAllocator 
    {
        SlabAllocator {
            //head: 0 as *mut Bucket,
            count: 0,
            object_size: object_size,
            bucket_data: PagedVector::new(allocator),
            allocator: allocator,
        }
    }    
}

//-------------------------------------------


pub struct LinkedListAllocator 
{
    count: usize,

    //TODO: Fix the lifetime
    allocator: &'static LockedAreaFrameAllocator,
}

impl LinkedListAllocator 
{
    fn new(frame_allocator: &'static LockedAreaFrameAllocator) -> LinkedListAllocator 
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

//------------------------------
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
            slab2048: SlabAllocator::new(frame_allocator, 2046),
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