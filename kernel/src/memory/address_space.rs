use alloc::boxed::Box;

struct AddressSpaceEntry
{
    pub start: usize,
    pub size: usize,  
}

pub struct AddressSpace
{
    //List of AddressSpaceEntries
}


//TODO: get_info(VA) -> Option<AddressSpaceEntry>

impl AddressSpace 
{
    //TODO: fault()

    //TODO: add_segment()
    //TODO: remove_segment()
}


pub fn new() -> Box<AddressSpace>
{
    let new_as = Box::new(AddressSpace{});

    new_as
}