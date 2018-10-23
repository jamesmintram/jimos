use core;

use memory;
use memory::Frame;

use kwriter;
use core::fmt::Write;

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
    pub object_size: usize,
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
            write!(kwriter::WRITER, "Bucket Full\n");
            return 0 as *mut u8;
        }

        //Return a ptr
        let start_address = memory::physical_to_kernel(self.start.start_address());
        let start_ptr = start_address as *mut u8;
        let start_offset = self.first_free * self.object_size;

        unsafe {
            let current_head = start_ptr.add(start_offset) as *mut usize;
            let next_head = *current_head;

            // write!(kwriter::WRITER,  "Current head: {:X}\n", current_head as usize);

            self.first_free = next_head;
            return current_head as *mut u8;
        }
    }

    pub fn release()
    {
        //Takes a ptr and adds it to the freelist
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

            for idx in 0..object_count -1 {                
                let next_ptr = start_ptr.add(offset) as *mut usize;
                offset += object_size;
                next_ptr.write(idx + 1);
            }

            //Now set the final item in the freelist to -1
            let next_ptr = start_ptr.add(offset) as *mut usize;
            next_ptr.write(LIST_END);
        }

        write!(
            kwriter::WRITER, 
            "Object size {} Bucket size: {}  Objects: {}\n", 
            object_size,
            bucket_byte_size, 
            object_count);

        Bucket{
            start: start,
            end: end,
            first_free: 0,
            object_size: object_size,
        }
    }
}