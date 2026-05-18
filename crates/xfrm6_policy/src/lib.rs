//! IPv6 XFRM Policy Module
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]

use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_void;
use core::mem;
use core::ptr;
use core::ptr::null_mut;
use kernel_types::*;

// Constants from C
pub const EHOSTUNREACH: c_int = -13;
pub const ENODEV: c_int = -19;
pub const ENOMEM: c_int = -12;

// Type definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_address_t {
    pub in6: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_dst {
    pub u: dst_entry,
    pub rt6: rt6_info,
    pub route_cookie: *mut c_void,
    pub route: *mut dst_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_policy_afinfo {
    pub dst_ops: *const dst_ops,
    pub dst_lookup: Option<
        unsafe extern "C" fn(
            net: *mut net,
            tos: c_int,
            oif: c_int,
            saddr: *const xfrm_address_t,
            daddr: *const xfrm_address_t,
            mark: u32,
        ) -> *mut dst_entry,
    >,
    pub get_saddr: Option<
        unsafe extern "C" fn(
            net: *mut net,
            oif: c_int,
            saddr: *mut xfrm_address_t,
            daddr: *const xfrm_address_t,
            mark: u32,
        ) -> c_int,
    >,
    pub fill_dst: Option<
        unsafe extern "C" fn(xdst: *mut xfrm_dst, dev: *mut net_device, fl: *const c_void) -> c_int,
    >,
    pub blackhole_route: Option<unsafe extern "C" fn(dst: *mut dst_entry, net: *mut net)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flowi6 {
    pub flowi6_oif: c_int,
    pub flowi6_flags: c_int,
    pub flowi6_mark: u32,
    pub daddr: in6_addr,
    pub saddr: in6_addr,
}

// Function implementations
/// Lookup IPv6 destination entry
///
/// # Safety
/// - `net` must be a valid pointer to network namespace
/// - `saddr` and `daddr` must be valid pointers if non-null
#[no_mangle]
pub unsafe extern "C" fn xfrm6_dst_lookup(
    net: *mut net,
    tos: c_int,
    oif: c_int,
    saddr: *const xfrm_address_t,
    daddr: *const xfrm_address_t,
    mark: u32,
) -> *mut dst_entry {
    let mut fl6 = flowi6 {
        flowi6_oif: l3mdev_master_ifindex_by_index(net, oif),
        flowi6_flags: 0x01, // FLOWI_FLAG_SKIP_NH_OIF
        flowi6_mark: mark,
        daddr: in6_addr { in6_u: in6_addr_union { u6_addr8: [0; 16] } },
        saddr: in6_addr { in6_u: in6_addr_union { u6_addr8: [0; 16] } },
    };

    // SAFETY: fl6 is properly initialized
    ptr::copy_nonoverlapping(
        &(*daddr).in6,
        &mut fl6.daddr as *mut _ as *mut u8,
        mem::size_of::<in6_addr>(),
    );
    if !saddr.is_null() {
        ptr::copy_nonoverlapping(
            &(*saddr).in6,
            &mut fl6.saddr as *mut _ as *mut u8,
            mem::size_of::<in6_addr>(),
        );
    }

    let dst = ip6_route_output(net, null_mut(), &fl6);

    if (*dst).error != 0 {
        dst_release(dst);
        return dst;
    }

    dst
}

/// Get source address for IPv6
///
/// # Safety
/// - `saddr` and `daddr` must be valid pointers
#[no_mangle]
pub unsafe extern "C" fn xfrm6_get_saddr(
    net: *mut net,
    oif: c_int,
    saddr: *mut xfrm_address_t,
    daddr: *const xfrm_address_t,
    mark: u32,
) -> c_int {
    let dst = xfrm6_dst_lookup(net, 0, oif, null_mut(), daddr, mark);
    if dst.is_null() {
        return EHOSTUNREACH;
    }

    let dev = (*(*dst).dev).dev;
    ipv6_dev_get_saddr(dev_net(dev), dev, &(*daddr).in6, 0, &mut (*saddr).in6);
    dst_release(dst);
    0
}

/// Fill IPv6 destination information
///
/// # Safety
/// - `xdst` and `dev` must be valid pointers
#[no_mangle]
pub unsafe extern "C" fn xfrm6_fill_dst(
    xdst: *mut xfrm_dst,
    dev: *mut net_device,
    fl: *const c_void,
) -> c_int {
    (*xdst).u.dev = dev;
    dev_hold(dev);

    (*xdst).rt6.rt6i_idev = in6_dev_get(dev);
    if (*xdst).rt6.rt6i_idev.is_null() {
        dev_put(dev);
        return ENODEV;
    }

    let rt = (*xdst).route as *mut rt6_info;
    (*xdst).rt6.rt6i_flags = (*rt).rt6i_flags & (0x01 | 0x02); // RTF_ANYCAST | RTF_LOCAL
    (*xdst).rt6.rt6i_gateway = (*rt).rt6i_gateway;
    (*xdst).rt6.rt6i_dst = (*rt).rt6i_dst;
    (*xdst).rt6.rt6i_src = (*rt).rt6i_src;
    INIT_LIST_HEAD(&mut (*xdst).rt6.rt6i_uncached);
    rt6_uncached_list_add(&mut (*xdst).rt6);
    atomic_inc(&mut (*dev_net(dev)).ipv6.rt6_stats.fib_rt_uncache);

    0
}

/// Update PMTU for IPv6
///
/// # Safety
/// - `dst` must be valid pointer to xfrm_dst
#[no_mangle]
pub unsafe extern "C" fn xfrm6_update_pmtu(
    dst: *mut dst_entry,
    sk: *mut c_void,
    skb: *mut c_void,
    mtu: u32,
    confirm_neigh: bool,
) {
    let xdst = dst as *mut xfrm_dst;
    let path = (*xdst).route;

    if let Some(update_pmtu) = (*(*dst).ops).update_pmtu {
        update_pmtu(path, sk, skb, mtu, confirm_neigh);
    }
}

/// Handle IPv6 redirect
///
/// # Safety
/// - `dst` must be valid pointer to xfrm_dst
#[no_mangle]
pub unsafe extern "C" fn xfrm6_redirect(dst: *mut dst_entry, sk: *mut c_void, skb: *mut c_void) {
    let xdst = dst as *mut xfrm_dst;
    let path = (*xdst).route;

    if let Some(redirect) = (*(*dst).ops).redirect {
        redirect(path, sk, skb);
    }
}

/// Destroy IPv6 destination entry
///
/// # Safety
/// - `dst` must be valid pointer to xfrm_dst
#[no_mangle]
pub unsafe extern "C" fn xfrm6_dst_destroy(dst: *mut dst_entry) {
    let xdst = dst as *mut xfrm_dst;

    if !(*xdst).rt6.rt6i_idev.is_null() {
        in6_dev_put((*xdst).rt6.rt6i_idev);
    }

    dst_destroy_metrics_generic(dst);

    if !(*xdst).rt6.rt6i_uncached.next.is_null() {
        rt6_uncached_list_del(&mut (*xdst).rt6);
    }

    xfrm_dst_destroy(dst);
}

/// Handle IPv6 interface down
///
/// # Safety
/// - `dst` must be valid pointer to xfrm_dst
#[no_mangle]
pub unsafe extern "C" fn xfrm6_dst_ifdown(
    dst: *mut dst_entry,
    dev: *mut net_device,
    unregister: c_int,
) {
    if unregister == 0 {
        return;
    }

    let xdst = dst as *mut xfrm_dst;
    if (*(*xdst).rt6.rt6i_idev).dev == dev {
        let loopback_idev = in6_dev_get(dev_net(dev).loopback_dev);
        let mut current_xdst = xdst;

        loop {
            in6_dev_put((*current_xdst).rt6.rt6i_idev);
            (*current_xdst).rt6.rt6i_idev = loopback_idev;
            in6_dev_hold(loopback_idev);

            let child = xfrm_dst_child(&(*current_xdst).u);
            if child.is_null() || (*child).xfrm.is_null() {
                break;
            }
            current_xdst = child as *mut xfrm_dst;
        }

        __in6_dev_put(loopback_idev);
    }

    xfrm_dst_ifdown(dst, dev);
}

/// Initialize IPv6 policy module
#[no_mangle]
pub unsafe extern "C" fn xfrm6_policy_init() -> c_int {
    xfrm_policy_register_afinfo(&xfrm6_policy_afinfo, 10 /* AF_INET6 */)
}

/// Cleanup IPv6 policy module
#[no_mangle]
pub unsafe extern "C" fn xfrm6_policy_fini() {
    xfrm_policy_unregister_afinfo(&xfrm6_policy_afinfo);
}

// External functions (assumed to exist in C)
extern "C" {
    fn l3mdev_master_ifindex_by_index(net: *mut net, oif: c_int) -> c_int;
    fn ip6_route_output(net: *mut net, sk: *mut c_void, fl: *const flowi6) -> *mut dst_entry;
    fn dst_release(dst: *mut dst_entry);
    fn dev_hold(dev: *mut net_device);
    fn in6_dev_get(dev: *mut net_device) -> *mut inet6_dev;
    fn dev_put(dev: *mut net_device);
    fn ipv6_dev_get_saddr(
        net: *mut net,
        dev: *mut net_device,
        daddr: *const in6_addr,
        flags: c_int,
        saddr: *mut in6_addr,
    );
    fn INIT_LIST_HEAD(head: *mut ListHead);
    fn rt6_uncached_list_add(rt: *mut rt6_info);
    fn atomic_inc(counter: *mut c_int);
    fn xfrm_policy_register_afinfo(afinfo: *const xfrm_policy_afinfo, family: c_int) -> c_int;
    fn xfrm_policy_unregister_afinfo(afinfo: *const xfrm_policy_afinfo);
    fn xfrm_dst_destroy(dst: *mut dst_entry);
    fn xfrm_dst_ifdown(dst: *mut dst_entry, dev: *mut net_device);
    fn dev_net(dev: *mut net_device) -> *mut net;
    fn in6_dev_put(idev: *mut inet6_dev);
    fn in6_dev_hold(idev: *mut inet6_dev);
    fn __in6_dev_put(idev: *mut inet6_dev);
    fn dst_destroy_metrics_generic(dst: *mut dst_entry);
    fn xfrm_dst_child(dst: *mut dst_entry) -> *mut dst_entry;
}

// Static data
#[no_mangle]
pub static mut xfrm6_dst_ops_template: dst_ops = dst_ops {
    family: 10, // AF_INET6
    update_pmtu: Some(xfrm6_update_pmtu as _),
    redirect: Some(xfrm6_redirect as _),
    cow_metrics: Some(dst_cow_metrics_generic as _),
    destroy: Some(xfrm6_dst_destroy as _),
    ifdown: Some(xfrm6_dst_ifdown as _),
    local_out: Some(__ip6_local_out as _),
    gc_thresh: 32768,
};

#[no_mangle]
pub static mut xfrm6_policy_afinfo: xfrm_policy_afinfo = xfrm_policy_afinfo {
    dst_ops: &xfrm6_dst_ops_template,
    dst_lookup: Some(xfrm6_dst_lookup as _),
    get_saddr: Some(xfrm6_get_saddr as _),
    fill_dst: Some(xfrm6_fill_dst as _),
    blackhole_route: Some(ip6_blackhole_route as _),
};

// External functions for dst_ops
extern "C" {
    fn dst_cow_metrics_generic(dst: *mut dst_entry, new_metrics: *mut c_void) -> *mut dst_entry;
    fn __ip6_local_out(skb: *mut c_void) -> c_int;
    fn ip6_blackhole_route(dst: *mut dst_entry, net: *mut net);
}