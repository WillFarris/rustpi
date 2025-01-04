pub mod alloc;
pub mod mmu;

use core::{alloc::GlobalAlloc, sync::atomic::{AtomicBool, Ordering}};

use linked_list_allocator::Heap;

use crate::{synchronization::{interface::Mutex, SpinLock}, warn};

#[global_allocator]
static ALLOCATOR: SpinLock<Heap> = SpinLock::new(Heap::empty());

unsafe impl GlobalAlloc for SpinLock<Heap> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut inner = self.lock().unwrap();
        let result = inner.allocate_first_fit(layout).ok();
        match result {
            Some(allocation) => {
                allocation.as_ptr()
            },
            None => core::ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        let mut inner = self.lock().unwrap();
        inner.deallocate(core::ptr::NonNull::new_unchecked(ptr), layout);
    }
}

extern "Rust" {
    static heap_start: core::cell::UnsafeCell<()>;
    static heap_end: core::cell::UnsafeCell<()>;
}

pub fn init_heap() {
    static INIT_DONE: AtomicBool = AtomicBool::new(false);

    if INIT_DONE.load(Ordering::Relaxed) {
        warn!("Attempted to init heap twice");
        return;
    }

    let mut inner = ALLOCATOR.lock().unwrap();

    unsafe {
        let heap_bottom = heap_start.get() as *mut u8;
        let heap_size = heap_end.get() as usize - heap_start.get() as usize;
        inner.init(heap_bottom, heap_size)
    }
    
    INIT_DONE.store(true, Ordering::Relaxed);
}