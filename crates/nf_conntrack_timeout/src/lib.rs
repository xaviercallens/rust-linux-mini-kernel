#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_char, c_void};
use kernel_types::*;

#[repr(C)]
pub struct nf_conn {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct nf_conntrack {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct nf_conntrack_helper {
    _priv: [u8; 0],
}
#[repr(C)]
pub struct nf_conntrack_expect {
    _priv: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_inet_addr {
    pub all: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_man_tcp {
    pub port: u16,
    pub state: u8,
    pub _pad: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_man_udp {
    pub port: u16,
    pub _pad: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_man_icmp {
    pub type_: u8,
    pub code: u8,
    pub _pad: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_man_sctp {
    pub port: u16,
    pub state: u8,
    pub _pad: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_man_dccp {
    pub port: u16,
    pub state: u8,
    pub _pad: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_conntrack_man_proto {
    pub all: [u32; 2],
    pub tcp: nf_conntrack_man_tcp,
    pub udp: nf_conntrack_man_udp,
    pub icmp: nf_conntrack_man_icmp,
    pub sctp: nf_conntrack_man_sctp,
    pub dccp: nf_conntrack_man_dccp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple {
    pub src: nf_inet_addr,
    pub dst: nf_inet_addr,
    pub src_u: nf_conntrack_man_proto,
    pub dst_u: nf_conntrack_man_proto,
    pub src_l3num: u8,
    pub dst_l3num: u8,
    pub src_protonum: u8,
    pub dst_protonum: u8,
}

#[repr(C)]
pub struct nf_conntrack_timeout {
    pub name: *const c_char,
    pub timeout: u32,
    pub hook_mask: u8,
    pub _pad: [u8; 3],
    pub next: *mut nf_conntrack_timeout,
    pub use_: u32,
}

unsafe extern "C" {
    pub static mut nf_ct_timeout_list: *mut nf_conntrack_timeout;

    pub fn kmalloc(size: usize, flags: gfp_t) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}

#[no_mangle]
pub extern "C" fn nf_ct_timeout_find(name: *const c_char) -> *mut nf_conntrack_timeout {
    let mut cur = unsafe { nf_ct_timeout_list };

    while !cur.is_null() {
        let a = unsafe { core::ffi::CStr::from_ptr((*cur).name) };
        let b = unsafe { core::ffi::CStr::from_ptr(name) };
        if a.to_bytes() == b.to_bytes() {
            return cur;
        }
        cur = unsafe { (*cur).next };
    }

    core::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn nf_ct_timeout_get(timeout: *mut nf_conntrack_timeout) {
    if timeout.is_null() {
        return;
    }
    unsafe {
        (*timeout).use_ = (*timeout).use_.wrapping_add(1);
    }
}

#[no_mangle]
pub extern "C" fn nf_ct_timeout_put(timeout: *mut nf_conntrack_timeout) {
    if timeout.is_null() {
        return;
    }

    unsafe {
        if (*timeout).use_ <= 1 {
            kfree(timeout as *mut c_void);
        } else {
            (*timeout).use_ -= 1;
        }
    }
}

#[no_mangle]
pub extern "C" fn nf_ct_timeout_find_get(name: *const c_char) -> *mut nf_conntrack_timeout {
    let p = nf_ct_timeout_find(name);
    if !p.is_null() {
        nf_ct_timeout_get(p);
    }
    p
}

#[no_mangle]
pub extern "C" fn nf_ct_timeout_alloc(
    name: *const c_char,
    timeout: u32,
    hook_mask: u8,
) -> *mut nf_conntrack_timeout {
    let p =
        unsafe { kmalloc(core::mem::size_of::<nf_conntrack_timeout>(), GFP_KERNEL as gfp_t) }
            as *mut nf_conntrack_timeout;

    if p.is_null() {
        return core::ptr::null_mut();
    }

    unsafe {
        (*p).name = name;
        (*p).timeout = timeout;
        (*p).hook_mask = hook_mask;
        (*p)._pad = [0; 3];
        (*p).next = core::ptr::null_mut();
        (*p).use_ = 1;
    }

    p
}

#[no_mangle]
pub extern "C" fn nf_ct_timeout_lookup(
    name: *const c_char,
    timeout: u32,
    hook_mask: u8,
) -> *mut nf_conntrack_timeout {
    let mut p = nf_ct_timeout_find_get(name);
    if p.is_null() {
        p = nf_ct_timeout_alloc(name, timeout, hook_mask);
    }
    p
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}