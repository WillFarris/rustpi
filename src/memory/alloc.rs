
use crate::synchronization::{SpinLock, interface::Mutex};
use core::alloc::GlobalAlloc;

extern "C" {
    static heap_start: u8;
    static heap_end: u8;
}

enum BuddyBlockState {
    Free,
    Allocated,
    SplitLeft,
    SplitRight,
    SplitBoth,
}

struct KernelAllocator {
    inner: SpinLock<KernelAllocatorInner>,
}

const NUM_HEAP_PAGES: usize = 1024;
struct KernelAllocatorInner {
    map: [u8; NUM_HEAP_PAGES],
}

#[global_allocator]
//#[link_section = ".locks"]
static KERNEL_ALLOCATOR: KernelAllocator = KernelAllocator {
    inner: SpinLock::new(KernelAllocatorInner { map: [0; 1024] })
};

unsafe impl GlobalAlloc for KernelAllocator {
unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
    let numbytes = layout.size();
    let mut pages = numbytes / 65536;
    let extra = numbytes - pages * 65536;
    if extra > 0 {
        pages += 1;
    }
    
    let mut inner = self.inner.lock().unwrap();
    inner.get_pages(pages)
}

unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
    let numbytes = layout.size();
    let mut pages = numbytes / 65536;
    let extra = numbytes - pages * 65536;
    if extra > 0 {
        pages += 1;
    }

    let mut inner = self.inner.lock().unwrap();
    inner.free_pages(ptr, pages);
}
}

impl KernelAllocatorInner {
    fn get_pages(&mut self, num_pages: usize) -> *mut u8 {
        let base_addr = unsafe { &heap_start as *const u8 as usize };
        
        let iter = NUM_HEAP_PAGES / num_pages;
        for i in 0..iter {
            let mut valid = true;
            for j in 0..num_pages {
                if self.map[i] == 1 {
                    valid = false;
                }
            }
            if valid {
                for j in 0..num_pages {
                    self.map[i+j] = 1;
                }
                let addr = (base_addr + 65536 * i) as *mut u8;
                return addr;
            }
        }
        panic!("No available page!");
    }

    fn free_pages(&mut self, ptr: *const u8, num_pages: usize) {
        let base_addr = unsafe { &heap_start as *const u8 as usize };
        let offset = ptr as usize - base_addr;
        let index = offset / 65536;
        for i in 0..num_pages {
            if self.map[index+i] == 0 {
                panic!("Double free error at {:x}", ptr as usize);
            }
            self.map[index+i] = 0;
        }
    }
}