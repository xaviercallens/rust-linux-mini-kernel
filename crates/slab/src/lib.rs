#![no_std]
#![cfg_attr(not(test), no_main)]
//! SLAB allocator for kernel objects
//!
//! Phase 2: Memory Allocator - Object-level allocation
//! Implements kmalloc/kfree for kernel memory allocation

use core::ffi::{c_int, c_ulong, c_void};
use core::ptr;
use core::sync::atomic::{AtomicUsize, Ordering};

#[cfg(not(test))]
use core::panic::PanicInfo;

// External page allocator
extern "C" {
    fn alloc_pages(order: c_int) -> *mut c_void;
    fn free_pages(ptr: *mut c_void, order: c_int);
    fn page_size() -> c_ulong;
}

// Constants
const KMALLOC_MIN_SIZE: usize = 32;
const KMALLOC_MAX_SIZE: usize = 8192;
const NUM_CACHES: usize = 8;

// Cache sizes: 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192
const CACHE_SIZES: [usize; NUM_CACHES] = [32, 64, 128, 256, 512, 1024, 2048, 4096];

// Slab object header
#[repr(C)]
struct SlabObject {
    next: *mut SlabObject,
}

// Slab descriptor
#[repr(C)]
struct Slab {
    free_list: *mut SlabObject,
    num_free: usize,
    num_objects: usize,
    next: *mut Slab,
}

// Cache descriptor
#[repr(C)]
struct KmemCache {
    object_size: usize,
    slab_order: usize,
    slab_list: *mut Slab,
    total_slabs: usize,
    total_objects: usize,
    allocated_objects: usize,
}

impl KmemCache {
    const fn new(object_size: usize) -> Self {
        KmemCache {
            object_size,
            slab_order: 0,
            slab_list: ptr::null_mut(),
            total_slabs: 0,
            total_objects: 0,
            allocated_objects: 0,
        }
    }
}

// Global state
static SLAB_INITIALIZED: AtomicUsize = AtomicUsize::new(0);
static mut KMALLOC_CACHES: [KmemCache; NUM_CACHES] = [
    KmemCache::new(32),
    KmemCache::new(64),
    KmemCache::new(128),
    KmemCache::new(256),
    KmemCache::new(512),
    KmemCache::new(1024),
    KmemCache::new(2048),
    KmemCache::new(4096),
];

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Initialize the SLAB allocator
///
/// # Safety
/// Must be called after page_alloc_init
#[no_mangle]
pub unsafe extern "C" fn slab_init() -> c_int {
    if SLAB_INITIALIZED.load(Ordering::Acquire) != 0 {
        return 0;
    }

    // Initialize each cache
    for i in 0..NUM_CACHES {
        let cache = &mut KMALLOC_CACHES[i];
        cache.object_size = CACHE_SIZES[i];

        // Determine slab order based on object size
        cache.slab_order = if cache.object_size <= 512 {
            0 // 1 page (4096 bytes)
        } else if cache.object_size <= 2048 {
            1 // 2 pages (8192 bytes)
        } else {
            2 // 4 pages (16384 bytes)
        };
    }

    SLAB_INITIALIZED.store(1, Ordering::Release);
    0
}

/// Allocate a slab for the given cache
///
/// # Safety
/// Cache must be valid
unsafe fn kmem_cache_grow(cache: *mut KmemCache) -> c_int {
    if cache.is_null() {
        return -1;
    }

    let slab_order = (*cache).slab_order as c_int;
    let slab_mem = alloc_pages(slab_order);
    if slab_mem.is_null() {
        return -1; // Out of memory
    }

    let page_sz = page_size() as usize;
    let slab_size = page_sz * (1 << (*cache).slab_order);
    let object_size = (*cache).object_size;

    // Place slab descriptor at beginning of slab
    let slab = slab_mem as *mut Slab;
    (*slab).num_objects = (slab_size - core::mem::size_of::<Slab>()) / object_size;
    (*slab).num_free = (*slab).num_objects;
    (*slab).next = (*cache).slab_list;

    // Initialize free list
    let objects_start = (slab as usize + core::mem::size_of::<Slab>()) as *mut u8;
    let mut prev_obj: *mut SlabObject = ptr::null_mut();

    for i in 0..(*slab).num_objects {
        let obj = objects_start.add(i * object_size) as *mut SlabObject;
        (*obj).next = ptr::null_mut();

        if i == 0 {
            (*slab).free_list = obj;
        } else {
            (*prev_obj).next = obj;
        }
        prev_obj = obj;
    }

    // Add slab to cache
    (*cache).slab_list = slab;
    (*cache).total_slabs += 1;
    (*cache).total_objects += (*slab).num_objects;

    0
}

/// Allocate memory of specified size
///
/// # Safety
/// Size must be non-zero and <= KMALLOC_MAX_SIZE
#[no_mangle]
pub unsafe extern "C" fn kmalloc(size: c_ulong) -> *mut c_void {
    if size == 0 || size > KMALLOC_MAX_SIZE as c_ulong {
        return ptr::null_mut();
    }

    if SLAB_INITIALIZED.load(Ordering::Acquire) == 0 {
        return ptr::null_mut();
    }

    let size = size as usize;

    // Find appropriate cache
    let mut cache_idx = 0;
    for i in 0..NUM_CACHES {
        if CACHE_SIZES[i] >= size {
            cache_idx = i;
            break;
        }
    }

    let cache = &mut KMALLOC_CACHES[cache_idx] as *mut KmemCache;

    // Try to allocate from existing slabs
    let mut slab = (*cache).slab_list;
    while !slab.is_null() {
        if (*slab).num_free > 0 {
            // Allocate from this slab
            let obj = (*slab).free_list;
            if !obj.is_null() {
                (*slab).free_list = (*obj).next;
                (*slab).num_free -= 1;
                (*cache).allocated_objects += 1;

                // Clear the object memory
                ptr::write_bytes(obj as *mut u8, 0, (*cache).object_size);

                return obj as *mut c_void;
            }
        }
        slab = (*slab).next;
    }

    // Need to grow cache
    if kmem_cache_grow(cache) < 0 {
        return ptr::null_mut();
    }

    // Retry allocation
    slab = (*cache).slab_list;
    if !slab.is_null() && (*slab).num_free > 0 {
        let obj = (*slab).free_list;
        if !obj.is_null() {
            (*slab).free_list = (*obj).next;
            (*slab).num_free -= 1;
            (*cache).allocated_objects += 1;

            ptr::write_bytes(obj as *mut u8, 0, (*cache).object_size);

            return obj as *mut c_void;
        }
    }

    ptr::null_mut()
}

/// Free previously allocated memory
///
/// # Safety
/// ptr must have been returned by kmalloc
#[no_mangle]
pub unsafe extern "C" fn kfree(ptr: *mut c_void) {
    if ptr.is_null() || SLAB_INITIALIZED.load(Ordering::Acquire) == 0 {
        return;
    }

    // Find which cache this object belongs to
    for i in 0..NUM_CACHES {
        let cache = &mut KMALLOC_CACHES[i] as *mut KmemCache;
        let mut slab = (*cache).slab_list;

        while !slab.is_null() {
            let slab_start = slab as usize;
            let slab_size = (page_size() as usize) * (1 << (*cache).slab_order);
            let slab_end = slab_start + slab_size;
            let ptr_addr = ptr as usize;

            if ptr_addr >= slab_start && ptr_addr < slab_end {
                // Found the slab
                let obj = ptr as *mut SlabObject;
                (*obj).next = (*slab).free_list;
                (*slab).free_list = obj;
                (*slab).num_free += 1;
                (*cache).allocated_objects -= 1;
                return;
            }

            slab = (*slab).next;
        }
    }
}

/// Allocate zeroed memory
///
/// # Safety
/// Same as kmalloc
#[no_mangle]
pub unsafe extern "C" fn kzalloc(size: c_ulong) -> *mut c_void {
    let ptr = kmalloc(size);
    if !ptr.is_null() {
        ptr::write_bytes(ptr as *mut u8, 0, size as usize);
    }
    ptr
}

/// Get SLAB statistics
#[no_mangle]
pub unsafe extern "C" fn kmem_cache_stat(cache_idx: c_int) -> c_int {
    if cache_idx < 0 || cache_idx >= NUM_CACHES as c_int {
        return -1;
    }

    let cache = &KMALLOC_CACHES[cache_idx as usize];
    cache.allocated_objects as c_int
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn slab_exit() {
    if SLAB_INITIALIZED.load(Ordering::Acquire) == 0 {
        return;
    }

    // Free all slabs
    for i in 0..NUM_CACHES {
        let cache = &mut KMALLOC_CACHES[i];
        let mut slab = cache.slab_list;

        while !slab.is_null() {
            let next_slab = (*slab).next;
            free_pages(slab as *mut c_void, cache.slab_order as c_int);
            slab = next_slab;
        }

        cache.slab_list = ptr::null_mut();
        cache.total_slabs = 0;
        cache.total_objects = 0;
        cache.allocated_objects = 0;
    }

    SLAB_INITIALIZED.store(0, Ordering::Release);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slab_init() {
        unsafe {
            let result = slab_init();
            assert_eq!(result, 0);
            assert_eq!(SLAB_INITIALIZED.load(Ordering::Acquire), 1);
        }
    }

    #[test]
    fn test_cache_sizes() {
        for i in 0..NUM_CACHES {
            assert!(CACHE_SIZES[i] >= KMALLOC_MIN_SIZE);
            assert!(CACHE_SIZES[i] <= KMALLOC_MAX_SIZE);
        }
    }
}
