use alloc::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicUsize, Ordering};

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
}

impl BumpAllocator 
{
    pub const fn new() -> Self {
        // NOTE: requires adding #![feature(const_atomic_usize_new)] to lib.rs
        Self {
            heap_start: 0,
            heap_end: 0,
            next: AtomicUsize::new(0),
        }
    }
    pub fn init(&mut self, heap_start: usize, heap_end: usize) 
    {
        //TODO: Assert that we only init once
        self.heap_start = heap_start;
        self.heap_end = heap_end;
        self.next = AtomicUsize::new(heap_start);
    }

    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        loop {
            // load current state of the `next` field
            let current_next = self.next.load(Ordering::Relaxed);
            let alloc_start = align_up(current_next, layout.align());
            let alloc_end = alloc_start.saturating_add(layout.size());

            if alloc_end <= self.heap_end {
                // update the `next` pointer if it still has the value `current_next`
                let next_now = self.next.compare_and_swap(current_next, alloc_end,
                    Ordering::Relaxed);
                if next_now == current_next {
                    // next address was successfully updated, allocation succeeded
                    return alloc_start as *mut u8;
                }
            } else {
                return 0 as *mut u8; //Err(AllocErr::Exhausted{ request: layout })
            }
        }
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


pub struct LockedHeap(Mutex<BumpAllocator>);

impl LockedHeap {
    /// Creates an empty heap. All allocate calls will return `None`.
    pub const fn empty() -> LockedHeap {
        LockedHeap(Mutex::new(BumpAllocator::new()))
    }
}


unsafe impl GlobalAlloc for LockedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0
            .lock()
            .alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0
            .lock()
            .deallocate(ptr, layout)
    }
}

impl Deref for LockedHeap {
    type Target = Mutex<BumpAllocator>;

    fn deref(&self) -> &Mutex<BumpAllocator> {
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