#![no_std]
#![no_main]
#![allow(non_camel_case_types)]

use core::ffi::c_void;
use core::ptr;
use kernel_types::*;

pub const EINVAL: c_int = 22;
pub const ENOMEM: c_int = 12;
pub const ENOSYS: c_int = 38;

pub const IP6_SIT_HASH_SIZE: usize = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_device {
    _priv: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iphdr {
    _priv: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_tunnel_parm {
    pub iph: iphdr,
    pub i_flags: __u32,
    pub link: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_tunnel_prl {
    pub addr: __be32,
    pub datalen: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_tunnel_prl_entry {
    pub addr: __be32,
    pub flags: c_int,
    pub next: *mut ip_tunnel_prl_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_tunnel {
    pub parms: ip_tunnel_parm,
    pub dev: *mut net_device,
    pub next: *mut ip_tunnel,
    pub prl: *mut ip_tunnel_prl_entry,
    pub prl_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sit_net {
    pub tunnels_r_l: [*mut ip_tunnel; IP6_SIT_HASH_SIZE],
    pub tunnels_r: [*mut ip_tunnel; IP6_SIT_HASH_SIZE],
    pub tunnels_l: [*mut ip_tunnel; IP6_SIT_HASH_SIZE],
    pub tunnels_wc: [*mut ip_tunnel; 1],
    pub tunnels: [*mut ip_tunnel; 4],
    pub fb_tunnel_dev: *mut net_device,
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

#[unsafe(no_mangle)]
pub extern "C" fn ipip6_tunnel_init(dev: *mut net_device) -> c_int {
    if dev.is_null() {
        return -EINVAL;
    }
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn ipip6_tunnel_setup(dev: *mut net_device) {
    if dev.is_null() {
        return;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ipip6_dev_free(dev: *mut net_device) {
    if dev.is_null() {
        return;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ipip6_tunnel_lookup(
    net: *mut c_void,
    _dev: *mut net_device,
    _remote: __be32,
    _local: __be32,
    _sifindex: c_int,
) -> *mut ip_tunnel {
    if net.is_null() {
        return ptr::null_mut();
    }
    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn __ipip6_bucket(
    sitn: *mut sit_net,
    parms: *mut ip_tunnel_parm,
) -> *mut *mut ip_tunnel {
    if sitn.is_null() || parms.is_null() {
        return ptr::null_mut();
    }
    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn ipip6_bucket(sitn: *mut sit_net, t: *mut ip_tunnel) -> *mut *mut ip_tunnel {
    if sitn.is_null() || t.is_null() {
        return ptr::null_mut();
    }
    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn ipip6_tunnel_unlink(sitn: *mut sit_net, t: *mut ip_tunnel) {
    if sitn.is_null() || t.is_null() {
        return;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ipip6_tunnel_link(sitn: *mut sit_net, t: *mut ip_tunnel) {
    if sitn.is_null() || t.is_null() {
        return;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ipip6_tunnel_clone_6rd(dev: *mut net_device, sitn: *mut sit_net) {
    if dev.is_null() || sitn.is_null() {
        return;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn ipip6_tunnel_create(dev: *mut net_device) -> c_int {
    if dev.is_null() {
        return -EINVAL;
    }
    -ENOSYS
}