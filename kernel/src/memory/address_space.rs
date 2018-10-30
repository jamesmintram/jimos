use alloc::vec::Vec;

use memory;
use memory::paging::table;
use memory::LockedAreaFrameAllocator;

struct AddressSegment
{
    range: AddressRange,
    //TODO: Add a mapper here
}

pub struct AddressSpace<'a>
{
    //List of AddressSpaceEntries
    segments: Vec<AddressSegment>,

    pub page_table: &'a mut table::Table<table::Level4>,

}

pub struct AddressRange
{
    pub start: usize,
    pub end: usize,  
}

impl AddressRange 
{
    pub fn overlaps(&self, other: &AddressRange) -> bool
    {
        (other.start >= self.start && other.start <= self.end)
        || (other.end >= self.start && other.end <= self.end)
        
    }
}

pub struct AddressSegmentDesc
{
    pub range: AddressRange,
}
pub struct AddressSegmentId(u64);


//TODO: get_info(VA) -> Option<AddressSpaceEntry>

impl<'a> AddressSpace<'a> 
{
    //TODO: fault()

    pub fn new(allocator: &LockedAreaFrameAllocator) -> AddressSpace
    {
        return AddressSpace{
            segments: Vec::new(),
            page_table: memory::paging::table::new(allocator),
        };
    }

    pub fn add_segment(&mut self, desc: &AddressSegmentDesc) -> AddressSegmentId
    {
        //TODO: Validate that this range does not overlap with any existing
        for segment in self.segments.iter() 
        {
            if segment.range.overlaps(&desc.range)
            {
                panic!("Overlapping segments");
            }
        }

        let new_segment = AddressSegment{
            range: AddressRange{..desc.range},
        };

        self.segments.push(new_segment);
        let segment_id = (self.segments.len() -1) as u64;

        return AddressSegmentId(segment_id);
    }

    //TODO: add_segment()
    //TODO: remove_segment()
}

