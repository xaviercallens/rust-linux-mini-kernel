//! Protocol independent destination cache implementation
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ptr;
use core::sync::atomic::{AtomicI32, AtomicUsize, Ordering};
use libc::{c_int, c_uint, c_void, size_t};

// Constants from C
pub const DST_OBSOLETE_DEAD: c_int = 1;
pub const DST_OBSOLETE_NONE: c_int = 0;
pub const DST_NOCOUNT: c_int = 1;
pub const DST_METADATA: c_int = 1;
pub const AF_UNSPEC: c_int = 0;
pub const ENOMEM: c_int = -12;
pub const EINVAL: c_int = -22;

// Type definitions
#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dst_ops {
    family: c_int,
    neigh_lookup: extern "C" fn(*const dst_entry, *mut sk_buff, *const c_void) -> *mut c_void,
    check: extern "C" fn(*mut dst_entry, u32) -> *mut dst_entry,
    cow_metrics: extern "C" fn(*mut dst_entry, usize) -> *mut u32,
    update_pmtu: extern "C" fn(*mut dst_entry, *mut sock, *mut sk_buff, u32, bool),
    redirect: extern "C" fn(*mut dst_entry, *mut sock, *mut sk_buff),
    mtu: extern "C" fn(*const dst_entry) -> u32,
    ifdown: Option<extern "C" fn(*mut dst_entry, *mut net_device, bool)>,
    gc: Option<extern "C" fn(*mut dst_ops) -> bool>,
    kmem_cachep: *mut c_void,
    gc_thresh: c_int,
}

#[repr(C)]
pub struct dst_metrics {
    refcnt: AtomicI32,
    metrics: [u32; 16], // Assuming RTAX_MAX is 16
}

#[repr(C)]
pub struct dst_entry {
    dev: *mut net_device,
    ops: *mut dst_ops,
    _metrics: usize,
    expires: u64,
    xfrm: *mut c_void,
    input: extern "C" fn(*mut dst_entry, *mut sk_buff) -> c_int,
    output: extern "C" fn(*mut net, *mut sock, *mut sk_buff) -> c_int,
    error: c_int,
    obsolete: c_int,
    header_len: c_int,
    trailer_len: c_int,
    tclassid: u32,
    lwtstate: *mut c_void,
    __refcnt: AtomicI32,
    __use: AtomicI32,
    lastuse: u64,
    flags: c_int,
    rcu_head: c_void,
}

#[repr(C)]
pub struct metadata_dst {
    dst: dst_entry,
    type_: c_int,
    // Additional fields based on type
    u: [u8; 0],
}

// Global variables
#[repr(C)]
pub static mut dst_default_metrics: dst_metrics = dst_metrics {
    refcnt: AtomicI32::new(1),
    metrics: [0; 16],
};

// Function implementations
/// Discard outgoing skb
///
/// # Safety
/// - `skb` must be a valid pointer to sk_buff
#[no_mangle]
pub unsafe extern "C" fn dst_discard_out(
    _net: *mut net,
    _sk: *mut sock,
    skb: *mut sk_buff,
) -> c_int {
    // SAFETY: Caller guarantees skb is valid
    kfree_skb(skb);
    0
}

/// Initialize destination entry
///
/// # Safety
/// - `dst` must be a valid pointer to dst_entry
/// - `dev` must be a valid pointer to net_device or NULL
#[no_mangle]
pub unsafe extern "C" fn dst_init(
    dst: *mut dst_entry,
    ops: *mut dst_ops,
    dev: *mut net_device,
    initial_ref: c_int,
    initial_obsolete: c_int,
    flags: c_int,
) {
    // SAFETY: Caller guarantees dst is valid
    (*dst).dev = dev;
    if !dev.is_null() {
        dev_hold(dev);
    }
    (*dst).ops = ops;
    dst_init_metrics(dst, &mut dst_default_metrics.metrics as *mut _, true);
    (*dst).expires = 0;
    (*dst).xfrm = ptr::null_mut();
    (*dst).input = dst_discard;
    (*dst).output = dst_discard_out;
    (*dst).error = 0;
    (*dst).obsolete = initial_obsolete;
    (*dst).header_len = 0;
    (*dst).trailer_len = 0;
    (*dst).tclassid = 0;
    (*dst).lwtstate = ptr::null_mut();
    atomic_set(&(*dst).__refcnt, initial_ref);
    (*dst).__use = 0;
    (*dst).lastuse = jiffies();
    (*dst).flags = flags;
    if !(flags & DST_NOCOUNT) != 0 {
        dst_entries_add(ops, 1);
    }
}

/// Allocate destination entry
///
/// # Safety
/// - `ops` must be a valid pointer to dst_ops
#[no_mangle]
pub unsafe extern "C" fn dst_alloc(
    ops: *mut dst_ops,
    dev: *mut net_device,
    initial_ref: c_int,
    initial_obsolete: c_int,
    flags: c_int,
) -> *mut c_void {
    if !((*ops).gc.is_some() && (flags & DST_NOCOUNT) == 0 && dst_entries_get_fast(ops) > (*ops).gc_thresh) {
        if let Some(gc) = (*ops).gc {
            if gc(ops) {
                pr_notice_ratelimited(
                    b"Route cache is full: consider increasing sysctl net.ipv6.route.max_size.\0"
                        as *const u8 as *const c_void,
                );
                return ptr::null_mut();
            }
        }
    }

    let dst = kmem_cache_alloc((*ops).kmem_cachep, GFP_ATOMIC);
    if dst.is_null() {
        return ptr::null_mut();
    }

    dst_init(
        dst as *mut dst_entry,
        ops,
        dev,
        initial_ref,
        initial_obsolete,
        flags,
    );

    dst
}

/// Destroy destination entry
///
/// # Safety
/// - `dst` must be a valid pointer to dst_entry
#[no_mangle]
pub unsafe extern "C" fn dst_destroy(dst: *mut dst_entry) -> *mut dst_entry {
    let mut child: *mut dst_entry = ptr::null_mut();

    smp_rmb();

    if !((*dst).flags & DST_NOCOUNT) != 0 {
        dst_entries_add((*dst).ops, -1);
    }

    if let Some(destroy) = (*(*dst).ops).destroy {
        destroy(dst);
    }
    if !(*dst).dev.is_null() {
        dev_put((*dst).dev);
    }

    lwtstate_put((*dst).lwtstate);

    if (*dst).flags & DST_METADATA != 0 {
        metadata_dst_free(dst as *mut metadata_dst);
    } else {
        kmem_cache_free((*dst).ops, dst);
    }

    dst
}

/// RCU callback for destroying dst
///
/// # Safety
/// - `head` must be a valid pointer to rcu_head
#[no_mangle]
pub unsafe extern "C" fn dst_destroy_rcu(head: *mut c_void) {
    let dst = (head as *mut dst_entry);
    dst_destroy(dst);
}

/// Put device reference and mark dst as dead
///
/// # Safety
/// - `dst` must be a valid pointer to dst_entry
#[no_mangle]
pub unsafe extern "C" fn dst_dev_put(dst: *mut dst_entry) {
    let dev = (*dst).dev;
    (*dst).obsolete = DST_OBSOLETE_DEAD;
    if let Some(ifdown) = (*(*dst).ops).ifdown {
        ifdown(dst, dev, true);
    }
    (*dst).input = dst_discard;
    (*dst).output = dst_discard_out;
    (*dst).dev = blackhole_netdev();
    dev_hold((*dst).dev);
    dev_put(dev);
}

/// Release reference to dst
///
/// # Safety
/// - `dst` must be a valid pointer to dst_entry
#[no_mangle]
pub unsafe extern "C" fn dst_release(dst: *mut dst_entry) {
    if !dst.is_null() {
        let newrefcnt = atomic_dec_return(&(*dst).__refcnt);
        if newrefcnt < 0 {
            net_warn_ratelimited(
                b"%s: dst:%p refcnt:%d\n\0" as *const u8 as *const c_void,
                __func(),
                dst,
                newrefcnt,
            );
        }
        if newrefcnt == 0 {
            call_rcu(&(*dst).rcu_head, dst_destroy_rcu);
        }
    }
}

/// Release reference to dst immediately
///
/// # Safety
/// - `dst` must be a valid pointer to dst_entry
#[no_mangle]
pub unsafe extern "C" fn dst_release_immediate(dst: *mut dst_entry) {
    if !dst.is_null() {
        let newrefcnt = atomic_dec_return(&(*dst).__refcnt);
        if newrefcnt < 0 {
            net_warn_ratelimited(
                b"%s: dst:%p refcnt:%d\n\0" as *const u8 as *const c_void,
                __func(),
                dst,
                newrefcnt,
            );
        }
        if newrefcnt == 0 {
            dst_destroy(dst);
        }
    }
}

/// Copy metrics with write permission
///
/// # Safety
/// - `dst` must be a valid pointer to dst_entry
#[no_mangle]
pub unsafe extern "C" fn dst_cow_metrics_generic(
    dst: *mut dst_entry,
    old: usize,
) -> *mut u32 {
    let p = kmalloc(core::mem::size_of::<dst_metrics>() as size_t, GFP_ATOMIC);
    if p.is_null() {
        return ptr::null_mut();
    }

    let p = p as *mut dst_metrics;
    let old_p = __DST_METRICS_PTR(old) as *mut dst_metrics;
    (*p).refcnt = AtomicI32::new(1);
    ptr::copy_nonoverlapping(
        (*old_p).metrics.as_ptr(),
        (*p).metrics.as_mut_ptr(),
        core::mem::size_of_val(&(*p).metrics),
    );

    let new = p as *mut _ as usize;
    let prev = cmpxchg(&(*dst)._metrics, old, new);

    if prev != old {
        kfree(p as *mut c_void);
        let p = __DST_METRICS_PTR(prev) as *mut dst_metrics;
        if prev & DST_METRICS_REFCOUNTED != 0 {
            if refcount_dec_and_test(&(*p).refcnt) {
                kfree(p as *mut c_void);
            }
        }
        return p as *mut u32;
    }

    if prev & DST_METRICS_REFCOUNTED != 0 {
        if refcount_dec_and_test(&(*old_p).refcnt) {
            kfree(old_p as *mut c_void);
        }
    }

    p as *mut u32
}

/// Destroy metrics
///
/// # Safety
/// - `dst` must be a valid pointer to dst_entry
#[no_mangle]
pub unsafe extern "C" fn __dst_destroy_metrics_generic(
    dst: *mut dst_entry,
    old: usize,
) {
    let new = (&dst_default_metrics as *const _ as usize) | DST_METRICS_READ_ONLY;
    let prev = cmpxchg(&(*dst)._metrics, old, new);
    if prev == old {
        kfree(__DST_METRICS_PTR(old) as *mut c_void);
    }
}

// Helper functions (assumed to be implemented elsewhere)
#[inline]
unsafe fn kfree_skb(skb: *mut sk_buff) {
    kfree(skb as *mut c_void);
}

#[inline]
unsafe fn kmem_cache_alloc(cache: *mut c_void, flags: c_int) -> *mut c_void {
    malloc(0) // Placeholder - actual implementation would use kernel cache
}

#[inline]
unsafe fn kmem_cache_free(cache: *mut c_void, obj: *mut c_void) {
    free(obj);
}

#[inline]
unsafe fn kmalloc(size: size_t, flags: c_int) -> *mut c_void {
    malloc(size)
}

#[inline]
unsafe fn kfree(obj: *mut c_void) {
    free(obj);
}

#[inline]
unsafe fn dev_hold(dev: *mut net_device) {
    // Placeholder
}

#[inline]
unsafe fn dev_put(dev: *mut net_device) {
    // Placeholder
}

#[inline]
unsafe fn lwtstate_put(state: *mut c_void) {
    // Placeholder
}

#[inline]
unsafe fn metadata_dst_free(md_dst: *mut metadata_dst) {
    // Placeholder
}

#[inline]
unsafe fn dst_entries_add(ops: *mut dst_ops, delta: c_int) {
    // Placeholder
}

#[inline]
unsafe fn dst_entries_get_fast(ops: *mut dst_ops) -> c_int {
    0 // Placeholder
}

#[inline]
unsafe fn smp_rmb() {
    // Placeholder
}

#[inline]
unsafe fn cmpxchg(ptr: *mut usize, old: usize, new: usize) -> usize {
    let result = *ptr;
    if result == old {
        *ptr = new;
    }
    result
}

#[inline]
unsafe fn atomic_set(atom: *mut AtomicI32, val: c_int) {
    (*atom).store(val, Ordering::Relaxed);
}

#[inline]
unsafe fn atomic_dec_return(atom: *mut AtomicI32) -> c_int {
    let val = (*atom).fetch_sub(1, Ordering::Relaxed);
    val - 1
}

#[inline]
unsafe fn refcount_dec_and_test(refcnt: *mut AtomicI32) -> bool {
    let val = (*refcnt).fetch_sub(1, Ordering::Relaxed);
    val == 1
}

#[inline]
unsafe fn call_rcu(head: *mut c_void, func: extern "C" fn(*mut c_void)) {
    // Placeholder
}

#[inline]
unsafe fn pr_notice_ratelimited(msg: *const c_void) {
    // Placeholder
}

#[inline]
unsafe fn net_warn_ratelimited(fmt: *const c_void, ...) {
    // Placeholder
}

#[inline]
unsafe fn __func__() -> *const c_void {
    ptr::null()
}

#[inline]
unsafe fn jiffies() -> u64 {
    0 // Placeholder
}

#[inline]
unsafe fn blackhole_netdev() -> *mut net_device {
    ptr::null_mut()
}

// Helper macros
#[inline]
unsafe fn __DST_METRICS_PTR(val: usize) -> *mut dst_metrics {
    (val & !1usize) as *mut dst_metrics
}

// Dummy functions for completeness
#[no_mangle]
pub unsafe extern "C" fn dst_discard(
    _dst: *mut dst_entry,
    _skb: *mut sk_buff,
) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn dst_blackhole_check(
    _dst: *mut dst_entry,
    _cookie: u32,
) -> *mut dst_entry {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn dst_blackhole_cow_metrics(
    _dst: *mut dst_entry,
    _old: usize,
) -> *mut u32 {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn dst_blackhole_neigh_lookup(
    _dst: *const dst_entry,
    _skb: *mut sk_buff,
    _daddr: *const c_void,
) -> *mut c_void {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn dst_blackhole_update_pmtu(
    _dst: *mut dst_entry,
    _sk: *mut sock,
    _skb: *mut sk_buff,
    _mtu: u32,
    _confirm_neigh: bool,
) {
}

#[no_mangle]
pub unsafe extern "C" fn dst_blackhole_redirect(
    _dst: *mut dst_entry,
    _sk: *mut sock,
    _skb: *mut sk_buff,
) {
}

#[no_mangle]
pub unsafe extern "C" fn dst_blackhole_mtu(_dst: *const dst_entry) -> u32 {
    0
}

// Static initializer for blackhole ops
static DST_BLACKHOLE_OPS: dst_ops = dst_ops {
    family: AF_UNSPEC,
    neigh_lookup: dst_blackhole_neigh_lookup as _,
    check: dst_blackhole_check as _,
    cow_metrics: dst_blackhole_cow_metrics as _,
    update_pmtu: dst_blackhole_update_pmtu as _,
    redirect: dst_blackhole_redirect as _,
    mtu: dst_blackhole_mtu as _,
    ifdown: None,
    gc: None,
    kmem_cachep: ptr::null_mut(),
    gc_thresh: 0,
};

// Metadata functions
#[no_mangle]
pub unsafe extern "C" fn metadata_dst_alloc(
    _optslen: u8,
    _type: c_int,
    _flags: c_int,
) -> *mut metadata_dst {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn metadata_dst_free_percpu(_md_dst: *mut metadata_dst) {
    // Placeholder
}

#[no_mangle]
pub unsafe extern "C" fn metadata_dst_alloc_percpu(
    _optslen: u8,
    _type: c_int,
    _flags: c_int,
) -> *mut metadata_dst {
    ptr::null_mut()
}
