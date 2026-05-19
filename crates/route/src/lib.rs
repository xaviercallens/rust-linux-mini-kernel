#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

extern crate alloc;

use alloc::boxed::Box;
use core::alloc::{GlobalAlloc, Layout};
use core::ffi::{c_int, c_void};
use core::panic::PanicInfo;
use core::ptr;
use core::sync::atomic::AtomicI32;
use kernel_types::*;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENETUNREACH: c_int = -101;
pub const EACCES: c_int = -13;
pub const ENOSYS: c_int = -38;

struct KernelAlloc;

// SAFETY: This allocator forwards to external C allocator functions.
unsafe impl GlobalAlloc for KernelAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe extern "C" {
            fn malloc(size: size_t) -> *mut c_void;
        }
        unsafe { malloc(layout.size() as size_t) as *mut u8 }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe extern "C" {
            fn free(ptr: *mut c_void);
        }
        unsafe { free(ptr as *mut c_void) }
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: KernelAlloc = KernelAlloc;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

#[repr(C)]
pub struct net_device {
    pub flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uncached_list {
    lock: *mut c_void, // spinlock_t
    head: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dst_ops {
    family: c_int,
    gc: Option<extern "C" fn(*mut c_void) -> c_int>,
    gc_thresh: c_int,
    check: Option<extern "C" fn(*mut c_void, u32) -> *mut c_void>,
    default_advmss: Option<extern "C" fn(*const c_void) -> c_int>,
    mtu: Option<extern "C" fn(*const c_void) -> c_int>,
    destroy: Option<extern "C" fn(*mut c_void)>,
    ifdown: Option<extern "C" fn(*mut c_void, *mut net_device, c_int)>,
    negative_advice: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
    link_failure: Option<extern "C" fn(*mut c_void)>,
    update_pmtu: Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, u32, c_int)>,
    redirect: Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void)>,
    local_out: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
    neigh_lookup:
        Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void, *const c_void) -> *mut c_void>,
    confirm_neigh: Option<extern "C" fn(*mut c_void, *const c_void)>,
}

#[repr(C)]
pub struct fib6_info {
    fib6_flags: c_int,
    fib6_protocol: c_int,
    fib6_metric: u32,
    fib6_ref: AtomicI32,
    fib6_type: c_int,
    fib6_metrics: *mut c_void,
}

#[repr(C)]
pub struct inet6_dev {
    dev: *mut net_device,
}

#[repr(C)]
struct per_cpu_data {
    list: uncached_list,
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn ip6_dst_alloc(
    net: *mut c_void,
    dev: *mut net_device,
    flags: c_int,
) -> *mut rt6_info {
    // Allocate memory for rt6_info
    let size = core::mem::size_of::<rt6_info>() as size_t;
    let ptr = libc::malloc(size);
    if ptr.is_null() {
        return ptr as *mut rt6_info;
    }

    // Initialize rt6_info
    let rt = ptr as *mut rt6_info;
    (*rt).rt6i_idev = ptr::null_mut();
    (*rt).rt6i_flags = 0;
    (*rt).rt6i_uncached.next = &mut (*rt).rt6i_uncached;
    (*rt).rt6i_uncached.prev = &mut (*rt).rt6i_uncached;

    // Initialize dst_entry
    (*rt).dst.__refcnt = AtomicI32::new(1);
    (*rt).dst.__use = 1;
    (*rt).dst.obsolete = 1; // DST_OBSOLETE_FORCE_CHK
    (*rt).dst.error = -ENETUNREACH;
    (*rt).dst.input = ip6_pkt_discard;
    (*rt).dst.output = ip6_pkt_discard_out;
    (*rt).dst.dev = dev;

    // Increment allocation counter
    let stats = (*net).cast::<struct {
        ipv6: struct {
            rt6_stats: *mut c_void,
        },
    }>();
    let counter = (*stats).ipv6.rt6_stats;
    // SAFETY: Assuming atomic increment is available
    unsafe {
        (*list).next = list;
        (*list).prev = list;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ip6_dst_alloc(
    _net: *mut c_void,
    dev: *mut net_device,
    _flags: c_int,
) -> *mut rt6_info {
    let rt = Box::into_raw(Box::new(rt6_info {
        dst: dst_entry {
            __refcnt: AtomicI32::new(1),
            __use: 1,
            obsolete: 1,
            error: ENETUNREACH,
            input: ip6_pkt_discard,
            output: ip6_pkt_discard_out,
            dev,
        },
        rt6i_flags: 0,
        rt6i_idev: ptr::null_mut(),
        rt6i_uncached: list_head {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        },
        rt6i_uncached_list: ptr::null_mut(),
    }));

    unsafe { init_list_head(&mut (*rt).rt6i_uncached as *mut list_head) };
    rt
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ip6_dst_check(dst: *mut dst_entry, _cookie: u32) -> *mut dst_entry {
    if dst.is_null() {
        ptr::null_mut()
    } else {
        dst
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ip6_default_advmss(_dst: *const dst_entry) -> c_int {
    1232
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ip6_mtu(_dst: *const dst_entry) -> c_int {
    1500
}

#[unsafe(no_mangle)]
pub extern "C" fn ip6_pkt_discard(_skb: *mut c_void) -> c_int {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn ip6_pkt_discard_out(
    _net: *mut c_void,
    _sk: *mut c_void,
    _skb: *mut c_void,
) -> c_int {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn ip6_pkt_prohibit(_skb: *mut c_void) -> c_int {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn ip6_pkt_prohibit_out(
    _net: *mut c_void,
    _sk: *mut c_void,
    _skb: *mut c_void,
) -> c_int {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn ip6_link_failure(_skb: *mut c_void) {}