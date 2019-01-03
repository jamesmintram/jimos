use core;

use memory;
use memory::Frame;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BucketStatus {
    Unused,
    Partial,
    Full,
}

pub struct Bucket {
    pub start: Frame,
    pub end: Frame,
    pub first_free: usize,   
}

const  LIST_END: usize = core::usize::MAX;

impl Bucket {
    pub fn status(&self ) -> BucketStatus
    {
        if self.first_free == LIST_END
        {
            return BucketStatus::Full;
        }
        return BucketStatus::Partial;
    }

    pub fn take(&mut self) -> *mut u8
    {
        //TODO: Update this to use Option?
        if self.first_free == LIST_END 
        {
            println!("Bucket Full\n");
            return 0 as *mut u8;
        }

        unsafe {
            let current_head = self.first_free as *mut usize;
            let next_head = *current_head;

            // println!( "Current head: {:X}\n", current_head as usize);

            self.first_free = next_head;
            // println!("Take FF {:X}\n", self.first_free);
            // println!("Take RET {:X}\n", current_head as usize);

            //TODO: Memset to zero before returning
            return current_head as *mut u8;
        }
    }

    pub fn release(&mut self, ptr: *mut u8)
    {
        if self.contains(ptr) == false
        {
            panic!();
        }

        // println!("PreRelease FF {:X}\n", self.first_free);

        // Put this at the start of the freelist
        unsafe {
            let new_head = ptr as *mut usize;
            *new_head = self.first_free;
            self.first_free = ptr as usize;
        }

        // println!("Release PT {:X}\n", ptr as usize);
        // println!("Release FF {:X}\n", self.first_free);
    }

    pub fn contains(&self, ptr: *const u8) -> bool
    {
        let start_address = memory::physical_to_kernel(self.start.start_address());
        let end_address = memory::physical_to_kernel(self.end.start_address());

        let ptr_address = ptr as usize;

        ptr_address >= start_address && ptr_address < end_address
    }

    pub fn new(start: Frame, end: Frame, object_size: usize) -> Bucket
    {
        // Assert that object_size > 16 and pow2
        if object_size & (object_size - 1) != 0 
            || object_size < 16
        {
            panic!();
        }

        //Create a freelist
        let start_address = memory::physical_to_kernel(start.start_address());
        let end_address = memory::physical_to_kernel(end.start_address());

        let bucket_byte_size = end_address - start_address;
        let object_count = bucket_byte_size / object_size;

        unsafe {
            let start_ptr = start_address as *mut u8;
            let mut offset = 0;

            for _idx in 0..object_count -1 {                
                let current_ptr = start_ptr.add(offset) as *mut usize;

                let next_ptr = start_ptr.add(offset + object_size) as *mut usize;
                let next_ptr_address = next_ptr as usize;

                current_ptr.write(next_ptr_address);
                offset += object_size;
            }

            //Now set the final item in the freelist to -1
            let next_ptr = start_ptr.add(offset) as *mut usize;
            next_ptr.write(LIST_END);
        }

        println!( 
            "Object size {} Bucket size: {}  Objects: {}\n", 
            object_size,
            bucket_byte_size, 
            object_count);

        Bucket{
            start: start,
            end: end,
            first_free: start_address,
        }
    }
}