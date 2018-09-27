use alloc::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicUsize, Ordering};

use memory;
use memory::FrameAllocator;
use memory::LockedAreaFrameAllocator;
use memory::AreaFrameAllocator;

use alloc::boxed::Box;

//use core::mem;

// #[cfg(feature = "use_spin")]
use core::ops::Deref;
//use core::ptr::NonNull;

// #[cfg(feature = "use_spin")]
use spin::Mutex;

/// A simple allocator that allocates memory linearly and ignores freed memory.
#[derive(Debug)]
pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: AtomicUsize,

    allocator: LockedAreaFrameAllocator,
}

impl BumpAllocator 
{
    pub fn new(allocator: Box<AreaFrameAllocator>) -> Self 
    {
        Self {
            heap_start: 0,
            heap_end: 0,
            next: AtomicUsize::new(0),
            allocator: LockedAreaFrameAllocator::new(allocator),
        }
    }

    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        
        //let fa = &mut self.allocator.lock();
        // let ff = fa.expect("Mem not inited");
        let new_frame = self.allocator
            .lock()
            .allocate_frame()
            .expect("No more darta");

        let addr = memory::physical_to_kernel(new_frame.start_address());
        return addr as *mut u8;


        // loop {
        //     // load current state of the `next` field
        //     let current_next = self.next.load(Ordering::Relaxed);
        //     let alloc_start = align_up(current_next, layout.align());
        //     let alloc_end = alloc_start.saturating_add(layout.size());

        //     if alloc_end <= self.heap_end {
        //         // update the `next` pointer if it still has the value `current_next`
        //         let next_now = self.next.compare_and_swap(current_next, alloc_end,
        //             Ordering::Relaxed);
        //         if next_now == current_next {
        //             // next address was successfully updated, allocation succeeded
        //             return alloc_start as *mut u8;
        //         }
        //     } else {
        //         return 0 as *mut u8; //Err(AllocErr::Exhausted{ request: layout })
        //     }
        // }
    }

    unsafe fn deallocate(&self, _ptr: *mut u8, _layout: Layout) {
        // do nothing, leak memory
    }
}


unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        0 as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        
    }
}


#[alloc_error_handler]
fn foo(_: Layout) -> ! {
    panic!("Out of memory")
}


pub struct LockedHeap(Mutex<Option<BumpAllocator>>);

impl LockedHeap {
    /// Creates an empty heap. All allocate calls will return `None`.
    pub const fn empty() -> LockedHeap {
        LockedHeap(Mutex::new(None))
    }

    //NOTE: NOTE THREAD SAFE!
    pub fn init(&mut self, allocator: Box<AreaFrameAllocator>) {
        self.0 = Mutex::new(Some(BumpAllocator::new(allocator)));
    }
}

unsafe impl GlobalAlloc for LockedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut lock = self.0.lock();

        if let Some(ref mut allocator) = *lock {
            return allocator.alloc(layout);
        }

        panic!();
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut lock = self.0.lock();
        if let Some(ref mut allocator) = *lock {
            allocator.deallocate(ptr, layout);
        }
    }
}

impl Deref for LockedHeap {
    type Target = Mutex<Option<BumpAllocator>>;

    fn deref(&self) -> &Mutex<Option<BumpAllocator>> {
        &self.0
    }
}
 

/// Align downwards. Returns the greatest x with alignment `align`
/// so that x <= addr. The alignment must be a power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {
    if align.is_power_of_two() {
        addr & !(align - 1)
    } else if align == 0 {
        addr
    } else {
        panic!("`align` must be a power of 2");
    }
}

/// Align upwards. Returns the smallest x with alignment `align`
/// so that x >= addr. The alignment must be a power of 2.
pub fn align_up(addr: usize, align: usize) -> usize {
    align_down(addr + align - 1, align)
}