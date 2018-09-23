use memory::KERNEL_ADDRESS_START;
use memory::USER_ADDRESS_END;

use core::fmt;

#[derive(Copy, Clone, PartialEq)]
pub enum VirtualAddress {
    Kernel(usize),
    User(usize),
}

impl fmt::Debug for VirtualAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        match self {
            VirtualAddress::Kernel(addr) => write!(f, "0x{:016X}", addr),
            VirtualAddress::User(addr) => write!(f, "0x{:016X}", addr),
        }
    }
}

pub fn from_usize(address: usize) -> Option<VirtualAddress> 
{    
    if address <= USER_ADDRESS_END
    {
        return Some(VirtualAddress::User(address));
    }

    if address >= KERNEL_ADDRESS_START
    {
        return Some(VirtualAddress::Kernel(address));
    }
    
    None
}

pub fn from_u64(address: u64) -> Option<VirtualAddress> 
{
    from_usize(address as usize)
}