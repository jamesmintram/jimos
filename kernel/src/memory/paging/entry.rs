use memory::Frame; 
use memory::PAGE_SIZE;
use memory::ADDRESS_MASK;
//use memory::ADDRESS_FLAGS_MASK;

pub struct Entry(u64);

bitflags! {
    pub struct EntryFlags: u64 {
        const PRESENT           = 1 << 0;
        const TABLE_DESCRIPTOR  = 1 << 1;
        const ACCESS            = 1 << 10;
        const RW                = 1 << 6;
    }
}

impl Entry {
    pub fn test(&self) -> usize{
        self.0 as usize
    }
    pub fn is_unused(&self) -> bool {
        self.0 == 0
    }

    pub fn set_unused(&mut self) {
        self.0 = 0;
    }

    pub fn flags(&self) -> EntryFlags {
        EntryFlags::from_bits_truncate(self.0)
    }

    // pub fn uflags(&self) -> u64 {
    //     self.0 & ADDRESS_FLAGS_MASK
    // }

    // Only makes sense in the final table (TODO: Update with proper name from arm doc)
    pub fn pointed_frame(&self) -> Option<Frame> {
        if self.flags().contains(PRESENT) {
            Some(Frame::containing_address(
                self.0 as usize & ADDRESS_MASK
            ))
        } else {
            None
        }
    }

    pub fn set(&mut self, frame: Frame, flags: EntryFlags) {
        assert!(frame.start_address() % PAGE_SIZE == 0);
        self.0 = (frame.start_address() as u64) | flags.bits();

        // write!(kwriter::WRITER, "SETF 0x{:X?}  {}\n",  self.0,  self.0);
        // write!(kwriter::WRITER, "SETA 0x{:X?}  {}\n",  frame.start_address(),  frame.start_address());
    }
}