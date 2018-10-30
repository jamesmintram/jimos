use alloc::boxed::Box;
use alloc::vec::Vec;

use memory::paging::table;
use memory::va_segment::VASegment;

struct AddressSpaceEntry
{
    pub start: usize,
    pub size: usize,  
}

pub struct AddressSpace<'a>
{
    //List of AddressSpaceEntries
    segments: Vec<&'a VASegment>,
}


//TODO: get_info(VA) -> Option<AddressSpaceEntry>

impl<'a> AddressSpace<'a> 
{
    //TODO: fault()

    pub fn create() -> AddressSpace<'a>
    {
        return AddressSpace{
            segments: Vec::new(),
        };
    }

    pub fn add_segment(&mut self, new_segment: &'a VASegment)
    {
        //TODO: Check for overlaps etc
        self.segments.push(new_segment);
    }

    //TODO: add_segment()
    //TODO: remove_segment()
}

