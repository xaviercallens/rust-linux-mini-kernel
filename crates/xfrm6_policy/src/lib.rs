#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]

use core::ffi::{c_int, c_void};
use core::mem;
use core::ptr;
use core::ptr::null_mut;
use kernel_types::*;

#[repr(C)]
pub struct net {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct net_device {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct inet6_dev {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct dst_ops {
    _priv: [u8; 0],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ListHead {
    pub next: *mut ListHead,
    pub prev: *mut ListHead,
}

pub const EHOSTUNREACH: c_int = -113;
pub const ENODEV: c_int = -19;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_address_t {
    pub in6: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt6_info {
    pub rt6i_flags: c_int,
    pub rt6i_gateway: in6_addr,
    pub rt6i_dst: in6_addr,
    pub rt6i_src: in6_addr,
    pub rt6i_idev: *mut inet6_dev,
    pub rt6i_uncached: ListHead,
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
pub struct flowi6 {
    pub flowi6_oif: c_int,
    pub flowi6_flags: c_int,
    pub flowi6_mark: u32,
    pub daddr: in6_addr,
    pub saddr: in6_addr,
}

#[repr(C)]
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

unsafe extern "C" {
    fn l3mdev_master_ifindex_by_index(net: *mut net, ifindex: c_int) -> c_int;
    fn ip6_route_output(net: *mut net, sk: *mut c_void, fl6: *const flowi6) -> *mut dst_entry;
    fn dst_release(dst: *mut dst_entry);
    fn dev_net(dev: *mut net_device) -> *mut net;
    fn ipv6_dev_get_saddr(
        net: *mut net,
        dev: *mut net_device,
        daddr: *const in6_addr,
        prefs: c_int,
        saddr: *mut in6_addr,
    ) -> c_int;
    fn dev_hold(dev: *mut net_device);
    fn dev_put(dev: *mut net_device);
    fn in6_dev_get(dev: *mut net_device) -> *mut inet6_dev;
    fn INIT_LIST_HEAD(list: *mut ListHead);
    fn rt6_uncached_list_add(rt: *mut rt6_info);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn xfrm6_dst_lookup(
    net: *mut net,
    _tos: c_int,
    oif: c_int,
    saddr: *const xfrm_address_t,
    daddr: *const xfrm_address_t,
    mark: u32,
) -> *mut dst_entry {
    let mut fl6 = flowi6 {
        flowi6_oif: l3mdev_master_ifindex_by_index(net, oif),
        flowi6_flags: 0x01,
        flowi6_mark: mark,
        daddr: in6_addr {
            in6_u: in6_addr_union { u6_addr8: [0; 16] },
        },
        saddr: in6_addr {
            in6_u: in6_addr_union { u6_addr8: [0; 16] },
        },
    };

    ptr::copy_nonoverlapping(
        &(*daddr).in6 as *const _ as *const u8,
        &mut fl6.daddr as *mut _ as *mut u8,
        mem::size_of::<in6_addr>(),
    );

    if !saddr.is_null() {
        ptr::copy_nonoverlapping(
            &(*saddr).in6 as *const _ as *const u8,
            &mut fl6.saddr as *mut _ as *mut u8,
            mem::size_of::<in6_addr>(),
        );
    }

    ip6_route_output(net, null_mut(), &fl6)
}

#[no_mangle]
pub unsafe extern "C" fn xfrm6_get_saddr(
    net: *mut net,
    oif: c_int,
    saddr: *mut xfrm_address_t,
    daddr: *const xfrm_address_t,
    mark: u32,
) -> c_int {
    let dst = xfrm6_dst_lookup(net, 0, oif, ptr::null(), daddr, mark);
    if dst.is_null() {
        return EHOSTUNREACH;
    }

    let dev = (*dst).dev as *mut net_device;
    let ret = ipv6_dev_get_saddr(dev_net(dev), dev, &(*daddr).in6, 0, &mut (*saddr).in6);
    dst_release(dst);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn xfrm6_fill_dst(
    xdst: *mut xfrm_dst,
    dev: *mut net_device,
    _fl: *const c_void,
) -> c_int {
    (*xdst).u.dev = dev as *mut c_void;
    dev_hold(dev);

    (*xdst).rt6.rt6i_idev = in6_dev_get(dev);
    if (*xdst).rt6.rt6i_idev.is_null() {
        dev_put(dev);
        return ENODEV;
    }

    let rt = (*xdst).route as *mut rt6_info;
    (*xdst).rt6.rt6i_flags = (*rt).rt6i_flags & (0x01 | 0x02);
    (*xdst).rt6.rt6i_gateway = (*rt).rt6i_gateway;
    (*xdst).rt6.rt6i_dst = (*rt).rt6i_dst;
    (*xdst).rt6.rt6i_src = (*rt).rt6i_src;
    INIT_LIST_HEAD(&mut (*xdst).rt6.rt6i_uncached);
    rt6_uncached_list_add(&mut (*xdst).rt6);

    0
}