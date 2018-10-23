use memory::Frame;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BucketStatus {
    Unused,
    Partial,
    Full,
}

//TODO: Make copyable?
pub struct Bucket {
    pub start: Frame,
    pub end: Frame,
    pub first_free: u64,    //FreeList?
    pub object_size: usize,
    pub status: BucketStatus,
}

impl Bucket {
    pub fn new(start: Frame, end: Frame, object_size: usize) -> Bucket
    {
        //TODO: Create a freelist


        Bucket{
            start: start,
            end: end,
            first_free: 0,
            object_size: object_size,
            status: BucketStatus::Unused,
        }
    }
}