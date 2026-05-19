#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
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
pub struct nf_conntrack_man_tcp {
    pub port: u16,
    pub state: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_man_udp {
    pub port: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_man_icmp {
    pub type_: u8,
    pub code: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_man_sctp {
    pub port: u16,
    pub state: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_man_dccp {
    pub port: u16,
    pub state: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_hash {
    pub tuplehash: *mut nf_conntrack_tuple_hash,
    pub tuple: nf_conntrack_tuple,
    pub me: *mut nf_conntrack,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack {
    pub timeout: *mut nf_conntrack_timeout,
    pub tuplehash: [*mut nf_conntrack_tuple_hash; 2],
    pub status: u32,
    pub mark: u32,
    pub use_: u32,
    pub id: u32,
    pub master: *mut nf_conntrack,
    pub helper: *mut nf_conntrack_helper,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_expect {
    pub tuple: nf_conntrack_tuple,
    pub mask: nf_conntrack_tuple,
    pub expectfn: Option<extern "C" fn(*mut nf_conn, *mut nf_conntrack_expect)>,
    pub timeout: u32,
    pub flags: u8,
    pub class: u8,
    pub id: u16,
    pub master: *mut nf_conntrack,
    pub helper: *mut nf_conntrack_helper,
}

#[no_mangle]
pub extern "C" fn nf_ct_timeout_lookup(
    name: *const c_char,
    timeout: u32,
    hook_mask: u8,
) -> *mut nf_conntrack_timeout {
    let mut timeout_ptr = unsafe { nf_ct_timeout_find_get(name) };

    if timeout_ptr.is_null() {
        timeout_ptr = unsafe { nf_ct_timeout_alloc(name, timeout, hook_mask) };
        if timeout_ptr.is_null() {
            return core::ptr::null_mut();
        }
    }

    unsafe { nf_ct_timeout_put(timeout_ptr) };
    timeout_ptr
}

#[no_mangle]
pub extern "C" fn nf_ct_timeout_find_get(name: *const c_char) -> *mut nf_conntrack_timeout {
    let mut timeout_ptr = unsafe { nf_ct_timeout_find(name) };

    if !timeout_ptr.is_null() {
        unsafe { nf_ct_timeout_get(timeout_ptr) };
    }

    timeout_ptr
}

#[no_mangle]
pub extern "C" fn nf_ct_timeout_alloc(
    name: *const c_char,
    timeout: u32,
    hook_mask: u8,
) -> *mut nf_conntrack_timeout {
    let timeout_ptr = unsafe {
        kmalloc(
            core::mem::size_of::<nf_conntrack_timeout>(),
            GFP_KERNEL as c_int,
        ) as *mut nf_conntrack_timeout
    };

    if timeout_ptr.is_null() {
        return core::ptr::null_mut();
    }

    unsafe {
        (*timeout_ptr).name = name;
        (*timeout_ptr).timeout = timeout;
        (*timeout_ptr).hook_mask = hook_mask;
        (*timeout_ptr).next = core::ptr::null_mut();
        (*timeout_ptr).use_ = 1;
    }

    timeout_ptr
}

#[no_mangle]
pub extern "C" fn nf_ct_timeout_find(name: *const c_char) -> *mut nf_conntrack_timeout {
    let mut timeout_ptr = unsafe { NF_CT_TIMEOUT_LIST };

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
        (*timeout).next = NF_CT_TIMEOUT_LIST;
        NF_CT_TIMEOUT_LIST = timeout;
    }

    p
}

#[no_mangle]
pub extern "C" fn nf_ct_timeout_list_del(timeout: *mut nf_conntrack_timeout) {
    let mut prev = core::ptr::null_mut();
    let mut curr = unsafe { NF_CT_TIMEOUT_LIST };

    while !curr.is_null() {
        if curr == timeout {
            if prev.is_null() {
                unsafe { NF_CT_TIMEOUT_LIST = (*curr).next };
            } else {
                unsafe { (*prev).next = (*curr).next };
            }
            break;
        }

        prev = curr;
        curr = unsafe { (*curr).next };
    }
    p
}

static mut NF_CT_TIMEOUT_LIST: *mut nf_conntrack_timeout = core::ptr::null_mut();

#[no_mangle]
pub extern "C" fn nf_ct_timeout_init() {
    unsafe { NF_CT_TIMEOUT_LIST = core::ptr::null_mut() };
}

#[no_mangle]
pub extern "C" fn nf_ct_timeout_cleanup() {
    let mut timeout_ptr = unsafe { NF_CT_TIMEOUT_LIST };

    while !timeout_ptr.is_null() {
        let next = unsafe { (*timeout_ptr).next };
        unsafe { nf_ct_timeout_destroy(timeout_ptr) };
        timeout_ptr = next;
    }

    unsafe { NF_CT_TIMEOUT_LIST = core::ptr::null_mut() };
}
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
