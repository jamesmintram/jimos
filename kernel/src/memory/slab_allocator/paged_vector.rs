use alloc::boxed::Box;

use memory;

use memory::LockedAreaFrameAllocator;
use core::slice;
use core::mem::size_of;

use kwriter;
use core::fmt::Write;

use memory::slab_allocator::bucket::Bucket;

struct PagedVectorPage {
    next: *mut PagedVectorPage,
    count: usize,
}

impl PagedVectorPage {
    fn max(&self) -> usize
    {
        let available_size = memory::PAGE_SIZE - size_of::<PagedVectorPage>();
        return  available_size / size_of::<Bucket>();
    }
    fn is_full(&self) -> bool
    {
        return self.count == self.max();
    }

    fn new(frame_allocator: &'static LockedAreaFrameAllocator) -> Box<PagedVectorPage> 
    {
        let page = memory::alloct_pages::<PagedVectorPage>(frame_allocator, 1);
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

pub struct PagedVector {
    head: Box<PagedVectorPage>,

    //TODO: Fix the lifetime
    allocator: &'static LockedAreaFrameAllocator,
}

impl PagedVector 
{
    pub fn new(allocator: &'static LockedAreaFrameAllocator) -> PagedVector 
    {
        let first_page = PagedVectorPage::new(allocator);

        write!(kwriter::WRITER, "PagedVector created, {} descriptors per Page\n", first_page.max());

        return PagedVector{
            head: first_page,
            allocator: allocator,
        }
    }

    pub fn update_one<
        Pred: Fn(&Bucket) -> bool, 
        Update: Fn(&mut Bucket) -> T,
        T> 
            (&mut self, pred: Pred, update: Update) -> Option<T>
    {
        let buckets = self.head.get_buckets();

        //TODO: Go page by page
        for ref mut bucket in buckets.iter_mut() {
            //write!(kwriter::WRITER, "Iter bucket!\n");
            if pred(bucket)
            {
                //write!(kwriter::WRITER, "Found bucket!\n");
                return Some(update(bucket));
            }
        }

        None
    }

    pub fn add_one<
        Create: Fn() -> Bucket, 
        Update: Fn(&mut Bucket) -> T,
        T> 
            (&mut self, create: Create, update: Update) -> Option<T>
    {
        if self.head.is_full() == false 
        {
            //write!(kwriter::WRITER, "Add bucket!\n");

            let mut new_bucket = create();
            let result = update(&mut new_bucket);
            self.head.add_bucket(new_bucket);
            
            return Some(result)
        }
        
        //If I can not add a bucket (because the descriptor page is full?)
        write!(kwriter::WRITER, "Failed to create a new bucket\n");
        None
    }
}