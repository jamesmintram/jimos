use alloc::vec::Vec;

use memory;
use memory::paging;
use memory::paging::table;
use memory::LockedAreaFrameAllocator;

struct AddressSegment
{
    range: AddressRange,
    //TODO: Add a mapper here
}

pub struct AddressSpace<'a>
{
    allocator: &'a LockedAreaFrameAllocator,

    //List of AddressSpaceEntries
    segments: Vec<AddressSegment>,
    pub page_table: &'a mut table::Table<table::Level4>,
}

#[derive(Clone,Copy)]
pub struct AddressRange
{
    pub start: usize,
    pub end: usize,  
    //TODO: Change to VirtualAddress type
    //TODO: start < end
    //TODO: start and end are page aligned
}

impl AddressRange 
{
    pub fn overlaps(&self, other: &AddressRange) -> bool
    {
        (other.start >= self.start && other.start <= self.end)
        || (other.end >= self.start && other.end <= self.end)
        
    }

    pub fn contains(&self, address: usize) -> bool
    {
        return address >= self.start && address <= self.end;
    }

    pub fn size(&self) -> usize 
    {
        self.end - self.start
    }
    pub fn page_count(&self) -> usize
    {
        self.size() >> memory::PAGE_SHIFT
    }
}

pub struct AddressSegmentDesc
{
    pub range: AddressRange,
}

#[derive(Clone, Copy)]
pub struct AddressSegmentId(usize);


impl<'a> AddressSpace<'a> 
{

    pub fn new(allocator: &LockedAreaFrameAllocator) -> AddressSpace
    {
        return AddressSpace{
            allocator: allocator,
            segments: Vec::new(),
            page_table: memory::paging::table::new(allocator),
        };
    }

    //TODO: Update error handling to Result
    pub fn add_segment(&mut self, desc: &AddressSegmentDesc) -> AddressSegmentId
    {
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

        //TODO: Clean this up
        let segment_id = AddressSegmentId((self.segments.len() -1) as usize);
        //let new_segment = self.get_segment(segment_id);
        let new_segment = &self.segments[segment_id.0];
        map_segment(self.allocator, new_segment, self.page_table);

        return segment_id; 
    }

    pub fn get_segment_range(&self, segment_id: AddressSegmentId) -> AddressRange
    {
        return self.segments[segment_id.0].range;
    }

    fn get_segment(&self, segment_id: AddressSegmentId) -> &AddressSegment
    {
        return &self.segments[segment_id.0];
    }

    fn contains_address(&self, address: usize) -> bool 
    {
        for segment in self.segments.iter() 
        {
            if segment.range.contains(address)
            {
                return true;
            }
        }
        return false;
    }

    pub fn handle_fault(&mut self, fault_address: usize) -> bool
    {
        if self.contains_address(fault_address) == false 
        {
            return false;
        }

        let allocator = &self.allocator;
        let page_table = &mut self.page_table;

        //TODO: This should come from the ANON HEAP instead of the KERNEL HEAP
        //      but that config should come in through the Segment Mapper desc
        let frame = memory::kalloc::alloc_frame(allocator);
        let page = paging::Page::containing_address(fault_address);

        memory::paging::map_to(
            allocator,
            page_table, 
            page, 
            frame, 
            paging::EntryFlags::empty());

        return true;
    }
    //TODO: remove_segment()
    //TODO: fault()
}


//------------------------------------------------------------------------
//TODO: This should be moved into a mapper
//------------------------------------------------------------------------
fn map_segment(
    allocator: &LockedAreaFrameAllocator, 
    segment: &AddressSegment, 
    page_table: &mut table::Table<table::Level4>)
{
    let page_count = segment.range.page_count();
    let page = paging::Page::containing_address(segment.range.start);

    for page_idx in 0..page_count 
    {
        let current_page = page.offset_by(page_idx);
        memory::paging::add_page(allocator, page_table, current_page, paging::EntryFlags::empty())
    }
}

//TODO: unmap_segment for Mapper