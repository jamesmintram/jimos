//use kwriter;
//use core::fmt::Write;

use memory;
use memory::LockedAreaFrameAllocator;
use memory::slab_allocator::HeapAllocator;
use memory::slab_allocator::bucket::Bucket;
use memory::slab_allocator::bucket::BucketStatus;
use memory::slab_allocator::paged_vector::PagedVector;

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
        // write!(kwriter::WRITER, "Alloc from Slab: {:X}\n", self.object_size); 

        //TODO: Track wastage using _size
        let object = self.bucket_data.update_one(
            |bucket|  { bucket.status() != BucketStatus::Full },
            |bucket|  { bucket.take() })
        .or_else(
            || {
                let allocator = self.allocator;
                let object_size = self.object_size;

                self.bucket_data.add_one(
                    || {
                        let (start, end) = memory::alloc_frames(1);
                        Bucket::new(start, end, object_size)
                    },
                    |bucket| { bucket.take() }
                )})
        .unwrap();
        
        object
    }
    fn release(&mut self, ptr: *mut u8) {
        // write!(kwriter::WRITER, "Release to Slab: {:X}\n", ptr as usize);

        self.bucket_data.update_one(
            |bucket|  { bucket.contains(ptr) },
            |bucket|  { bucket.release(ptr) })
        .expect("Invalid ptr address");
    }
    fn release_unused(&mut self) {
        //TODO: Implement this
    }
}

impl SlabAllocator 
{
    pub fn new(allocator: &'static LockedAreaFrameAllocator, object_size: usize) -> SlabAllocator 
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
