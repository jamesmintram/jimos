use memory::KERNEL_ADDRESS_START;
use memory::USER_ADDRESS_END;

#[derive(Copy, Clone ,Debug, PartialEq)]
pub enum VirtualAddress {
    Kernel(usize),
    User(usize),
}


pub fn from_usize(address: usize) -> Option<VirtualAddress> 
{    
    if (address <= USER_ADDRESS_END)
    {
        return Some(VirtualAddress::User(address));
    }

    if (address >= KERNEL_ADDRESS_START)
    {
        return Some(VirtualAddress::Kernel(address));
    }
    
    None
}

pub fn from_u64(address: u64) -> Option<VirtualAddress> 
{
    from_usize(address as usize)
}