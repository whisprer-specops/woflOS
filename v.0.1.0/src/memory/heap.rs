use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Heap size: 64KB for kernel heap
const HEAP_SIZE: usize = 64 * 1024;

/// Aligned heap storage wrapper
#[repr(C, align(8))]
struct HeapStorage {
    data: [u8; HEAP_SIZE],
}

/// Static heap storage - lives in BSS
static mut HEAP_SPACE: HeapStorage = HeapStorage {
    data: [0; HEAP_SIZE],
};

/// Simple bump allocator for kernel heap
pub struct BumpAllocator {
    next: AtomicUsize,
    initialized: AtomicUsize,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        BumpAllocator {
            next: AtomicUsize::new(0),
            initialized: AtomicUsize::new(0),
        }
    }
    
    pub unsafe fn init(&self) {
        let heap_start = &raw const HEAP_SPACE.data as *const u8 as usize;
        self.next.store(heap_start, Ordering::Release);
        self.initialized.store(1, Ordering::Release);
    }
    
    pub fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        
        // Check if initialized
        let init_val = self.initialized.load(Ordering::Acquire);
        if init_val == 0 {
            // Not initialized!
            return null_mut();
        }
        
        let heap_start = unsafe { &raw const HEAP_SPACE.data as *const u8 as usize };
        let heap_end = heap_start + HEAP_SIZE;
        
        // Check if heap pointers make sense
        if heap_start == 0 || heap_end == 0 {
            return null_mut();
        }
        
        loop {
            let current = self.next.load(Ordering::Acquire);
            
            // Sanity check
            if current < heap_start || current > heap_end {
                return null_mut();
            }
            
            // Align the allocation
            let aligned = (current + align - 1) & !(align - 1);
            let new_next = aligned + size;
            
            // Check if we have space
            if new_next > heap_end {
                return null_mut(); // Out of heap memory
            }
            
            // Try to claim this region atomically
            if self.next.compare_exchange(
                current,
                new_next,
                Ordering::AcqRel,
                Ordering::Acquire
            ).is_ok() {
                return aligned as *mut u8;
            }
            // If compare_exchange failed, retry
        }
    }
    
    pub fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator doesn't support deallocation
    }
    
    #[allow(dead_code)]
    pub fn used(&self) -> usize {
        let heap_start = unsafe { &raw const HEAP_SPACE.data as *const u8 as usize };
        let next = self.next.load(Ordering::Relaxed);
        next.saturating_sub(heap_start)
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.alloc(layout)
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.dealloc(ptr, layout)
    }
}

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator::new();

/// Initialize the heap
pub unsafe fn init() {
    // For debugging - we can't print here easily, but at least call init
    ALLOCATOR.init();
}

/// Get heap usage statistics
#[allow(dead_code)]
pub fn heap_used() -> usize {
    ALLOCATOR.used()
}