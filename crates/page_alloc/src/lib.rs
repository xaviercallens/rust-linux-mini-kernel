#![no_std]
#![cfg_attr(not(test), no_main)]
//! Physical page allocator (buddy system)
//!
//! Phase 2: Memory Allocator - Page-level allocation
//! Implements a simple buddy allocator for physical memory management

use core::ffi::{c_int, c_ulong, c_void};
use core::ptr;
use core::sync::atomic::{AtomicUsize, Ordering};

#[cfg(not(test))]
use core::panic::PanicInfo;

// Constants
const PAGE_SHIFT: usize = 12;
const PAGE_SIZE: usize = 1 << PAGE_SHIFT; // 4096 bytes
const MAX_ORDER: usize = 11; // Support up to 2^11 pages (8MB)
const TOTAL_MEMORY: usize = 128 * 1024 * 1024; // 128 MB
const TOTAL_PAGES: usize = TOTAL_MEMORY / PAGE_SIZE;

// Page flags
const PG_RESERVED: u32 = 1 << 0;
const PG_ALLOCATED: u32 = 1 << 1;
const PG_SLAB: u32 = 1 << 2;

// Global state
static PAGE_ALLOC_INITIALIZED: AtomicUsize = AtomicUsize::new(0);
static mut FREE_AREA: [[PageList; MAX_ORDER]; 1] = [[PageList::new(); MAX_ORDER]; 1];
static TOTAL_FREE_PAGES: AtomicUsize = AtomicUsize::new(0);

// Page descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Page {
    flags: u32,
    count: u32,
    order: u8,
    next: *mut Page,
}

impl Page {
    const fn new() -> Self {
        Page {
            flags: 0,
            count: 0,
            order: 0,
            next: ptr::null_mut(),
        }
    }

    fn is_free(&self) -> bool {
        (self.flags & PG_ALLOCATED) == 0
    }

    fn mark_allocated(&mut self) {
        self.flags |= PG_ALLOCATED;
        self.count = 1;
    }

    fn mark_free(&mut self) {
        self.flags &= !PG_ALLOCATED;
        self.count = 0;
    }
}

// Free list for each order
#[repr(C)]
#[derive(Copy, Clone)]
struct PageList {
    head: *mut Page,
    count: usize,
}

impl PageList {
    const fn new() -> Self {
        PageList {
            head: ptr::null_mut(),
            count: 0,
        }
    }

    unsafe fn add_page(&mut self, page: *mut Page) {
        (*page).next = self.head;
        self.head = page;
        self.count += 1;
    }

    unsafe fn remove_page(&mut self) -> *mut Page {
        if self.head.is_null() {
            return ptr::null_mut();
        }
        let page = self.head;
        self.head = (*page).next;
        (*page).next = ptr::null_mut();
        self.count -= 1;
        page
    }
}

// Memory region (simulated physical memory)
static mut MEMORY_POOL: [u8; TOTAL_MEMORY] = [0; TOTAL_MEMORY];
static mut PAGE_ARRAY: [Page; TOTAL_PAGES] = [Page::new(); TOTAL_PAGES];

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Initialize the page allocator
///
/// # Safety
/// Must be called once during kernel boot
#[no_mangle]
pub unsafe extern "C" fn page_alloc_init() -> c_int {
    // Check if already initialized
    if PAGE_ALLOC_INITIALIZED.load(Ordering::Acquire) != 0 {
        return 0;
    }

    // Initialize all pages as free
    for i in 0..TOTAL_PAGES {
        PAGE_ARRAY[i] = Page::new();
    }

    // Add pages to free lists by order
    // Start with largest possible blocks
    let mut page_idx = 0;
    while page_idx < TOTAL_PAGES {
        let mut order = MAX_ORDER - 1;

        // Find largest block that fits
        while order > 0 && page_idx + (1 << order) > TOTAL_PAGES {
            order -= 1;
        }

        if page_idx + (1 << order) <= TOTAL_PAGES {
            let page = &mut PAGE_ARRAY[page_idx] as *mut Page;
            (*page).order = order as u8;
            FREE_AREA[0][order].add_page(page);
            page_idx += 1 << order;
        } else {
            break;
        }
    }

    TOTAL_FREE_PAGES.store(TOTAL_PAGES, Ordering::Release);
    PAGE_ALLOC_INITIALIZED.store(1, Ordering::Release);

    0
}

/// Allocate pages of specified order
///
/// # Safety
/// Caller must ensure order is valid
#[no_mangle]
pub unsafe extern "C" fn alloc_pages(order: c_int) -> *mut c_void {
    if PAGE_ALLOC_INITIALIZED.load(Ordering::Acquire) == 0 {
        return ptr::null_mut();
    }

    let order = order as usize;
    if order >= MAX_ORDER {
        return ptr::null_mut();
    }

    // Find a free block of the requested order or larger
    let mut current_order = order;
    while current_order < MAX_ORDER {
        if !FREE_AREA[0][current_order].head.is_null() {
            break;
        }
        current_order += 1;
    }

    if current_order >= MAX_ORDER {
        return ptr::null_mut(); // Out of memory
    }

    // Remove page from free list
    let mut page = FREE_AREA[0][current_order].remove_page();
    if page.is_null() {
        return ptr::null_mut();
    }

    // Split larger blocks if necessary
    while current_order > order {
        current_order -= 1;
        let buddy_idx = page_to_idx(page) + (1 << current_order);
        if buddy_idx < TOTAL_PAGES {
            let buddy = &mut PAGE_ARRAY[buddy_idx] as *mut Page;
            (*buddy).order = current_order as u8;
            FREE_AREA[0][current_order].add_page(buddy);
        }
    }

    // Mark page as allocated
    (*page).mark_allocated();
    (*page).order = order as u8;

    let pages_allocated = 1 << order;
    TOTAL_FREE_PAGES.fetch_sub(pages_allocated, Ordering::AcqRel);

    // Return pointer to actual memory
    page_to_addr(page)
}

/// Free previously allocated pages
///
/// # Safety
/// ptr must have been returned by alloc_pages
#[no_mangle]
pub unsafe extern "C" fn free_pages(ptr: *mut c_void, order: c_int) {
    if ptr.is_null() || PAGE_ALLOC_INITIALIZED.load(Ordering::Acquire) == 0 {
        return;
    }

    let order = order as usize;
    if order >= MAX_ORDER {
        return;
    }

    let page = addr_to_page(ptr);
    if page.is_null() {
        return;
    }

    (*page).mark_free();

    let pages_freed = 1 << order;
    TOTAL_FREE_PAGES.fetch_add(pages_freed, Ordering::AcqRel);

    // Try to merge with buddy
    let mut current_page = page;
    let mut current_order = order;

    while current_order < MAX_ORDER - 1 {
        let page_idx = page_to_idx(current_page);
        let buddy_idx = page_idx ^ (1 << current_order);

        if buddy_idx >= TOTAL_PAGES {
            break;
        }

        let buddy = &mut PAGE_ARRAY[buddy_idx] as *mut Page;

        // Check if buddy is free and same order
        if !(*buddy).is_free() || (*buddy).order != current_order as u8 {
            break;
        }

        // Remove buddy from free list (simplified - would need to traverse list)
        // For now, just add current page back
        break;
    }

    (*current_page).order = current_order as u8;
    FREE_AREA[0][current_order].add_page(current_page);
}

/// Get total free memory in pages
#[no_mangle]
pub unsafe extern "C" fn nr_free_pages() -> c_ulong {
    TOTAL_FREE_PAGES.load(Ordering::Acquire) as c_ulong
}

/// Get page size
#[no_mangle]
pub unsafe extern "C" fn page_size() -> c_ulong {
    PAGE_SIZE as c_ulong
}

// Helper functions
unsafe fn page_to_idx(page: *const Page) -> usize {
    let base = PAGE_ARRAY.as_ptr();
    ((page as usize) - (base as usize)) / core::mem::size_of::<Page>()
}

unsafe fn page_to_addr(page: *const Page) -> *mut c_void {
    let idx = page_to_idx(page);
    MEMORY_POOL.as_mut_ptr().add(idx * PAGE_SIZE) as *mut c_void
}

unsafe fn addr_to_page(addr: *const c_void) -> *mut Page {
    let base = MEMORY_POOL.as_ptr() as usize;
    let ptr = addr as usize;
    if ptr < base || ptr >= base + TOTAL_MEMORY {
        return ptr::null_mut();
    }
    let idx = (ptr - base) / PAGE_SIZE;
    if idx >= TOTAL_PAGES {
        return ptr::null_mut();
    }
    &mut PAGE_ARRAY[idx] as *mut Page
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn page_alloc_exit() {
    PAGE_ALLOC_INITIALIZED.store(0, Ordering::Release);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_alloc_init() {
        unsafe {
            let result = page_alloc_init();
            assert_eq!(result, 0);
            assert_eq!(PAGE_ALLOC_INITIALIZED.load(Ordering::Acquire), 1);
        }
    }

    #[test]
    fn test_alloc_single_page() {
        unsafe {
            page_alloc_init();
            let ptr = alloc_pages(0);
            assert!(!ptr.is_null());
            free_pages(ptr, 0);
        }
    }

    #[test]
    fn test_alloc_multiple_pages() {
        unsafe {
            page_alloc_init();
            let ptr = alloc_pages(2); // 4 pages
            assert!(!ptr.is_null());
            free_pages(ptr, 2);
        }
    }

    #[test]
    fn test_page_size() {
        unsafe {
            assert_eq!(page_size(), 4096);
        }
    }
}
