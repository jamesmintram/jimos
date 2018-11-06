pub const ONE_GB: usize = 0x40000000;
pub const HEAP_BASE: usize = ONE_GB; 

pub unsafe fn main() {
    // Test out the mapping
    let addr1 = HEAP_BASE;
    let data : *mut usize = addr1 as *mut usize;

    //TODO: GET WORKING ON HARDWARE    
    *data = 1024;
}