use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct LockedHeap {
    heap_start: usize,
    heap_end: usize,
    next: AtomicUsize,
}

impl LockedHeap {
    pub const fn empty() -> Self {
        LockedHeap {
            heap_start: 0,
            heap_end: 0,
            next: AtomicUsize::new(0),
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next.store(heap_start, Ordering::Relaxed);
    }
}

unsafe impl GlobalAlloc for LockedHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        
        let mut current_next = self.next.load(Ordering::Relaxed);
        
        loop {
            let start = (current_next + align - 1) & !(align - 1);
            let end = start + size;
            
            if end > self.heap_end {
                return null_mut();
            }
            
            match self.next.compare_exchange(
                current_next,
                end,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return start as *mut u8,
                Err(actual) => current_next = actual,
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // HSYNC-OS: Non-dissipative logic avoids frequent deallocation
    }
}

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 64 * 1024; // 64 KB

