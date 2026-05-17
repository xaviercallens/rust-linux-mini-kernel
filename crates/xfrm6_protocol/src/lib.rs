//! xfrm6_protocol - Generic xfrm protocol multiplexer for ipv6
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_void;
use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};
use kernel_types::*;

// Constants from C
pub const IPPROTO_ESP: u8 = 50;
pub const IPPROTO_AH: u8 = 51;
pub const IPPROTO_COMP: u8 = 108;

pub const INET6_PROTO_NOPOLICY: c_int = 1 << 0;
pub const ICMPV6_DEST_UNREACH: c_int = 1;
pub const ICMPV6_PORT_UNREACH: c_int = 4;

pub const EINVAL: c_int = -22;
pub const ENOENT: c_int = -2;
pub const EEXIST: c_int = -17;
pub const ENOMEM: c_int = -12;
pub const EAGAIN: c_int = -35;

// Type definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm6_protocol {
    pub next: *mut xfrm6_protocol,
    pub priority: c_int,
    pub handler: extern "C" fn(*mut sk_buff) -> c_int,
    pub input_handler: extern "C" fn(*mut sk_buff, c_int, u32, c_int) -> c_int,
    pub cb_handler: extern "C" fn(*mut sk_buff, c_int) -> c_int,
    pub err_handler: extern "C" fn(*mut sk_buff, *mut sk_buff, u8, u8, c_int, u32) -> c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet6_protocol {
    pub handler: extern "C" fn(*mut sk_buff) -> c_int,
    pub err_handler: extern "C" fn(*mut sk_buff, *mut sk_buff, u8, u8, c_int, u32) -> c_int,
    pub flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_input_afinfo {
    pub family: c_int,
    pub callback: extern "C" fn(*mut sk_buff, u8, c_int) -> c_int,
}

// Static variables
static mut esp6_handlers: AtomicPtr<xfrm6_protocol> = AtomicPtr::new(ptr::null_mut());
static mut ah6_handlers: AtomicPtr<xfrm6_protocol> = AtomicPtr::new(ptr::null_mut());
static mut ipcomp6_handlers: AtomicPtr<xfrm6_protocol> = AtomicPtr::new(ptr::null_mut());

// Mutex implementation (simplified for kernel compatibility)
#[repr(C)]
struct Mutex {
    // In real kernel code, this would use proper kernel mutexes
    // Here we use a simplified version for demonstration
    locked: bool,
}

impl Mutex {
    const fn new() -> Self {
        Self { locked: false }
    }

    unsafe fn lock(&mut self) {
        while self.locked {}
        self.locked = true;
    }

    unsafe fn unlock(&mut self) {
        self.locked = false;
    }
}

static mut xfrm6_protocol_mutex: Mutex = Mutex::new();

// Helper functions
unsafe fn proto_handlers(protocol: u8) -> *mut *mut xfrm6_protocol {
    match protocol {
        IPPROTO_ESP => &esp6_handlers as *mut AtomicPtr<xfrm6_protocol> as *mut *mut xfrm6_protocol,
        IPPROTO_AH => &ah6_handlers as *mut AtomicPtr<xfrm6_protocol> as *mut *mut xfrm6_protocol,
        IPPROTO_COMP => {
            &ipcomp6_handlers as *mut AtomicPtr<xfrm6_protocol> as *mut *mut xfrm6_protocol
        }
        _ => ptr::null_mut(),
    }
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn xfrm6_rcv_cb(skb: *mut sk_buff, protocol: u8, err: c_int) -> c_int {
    let head = proto_handlers(protocol);
    if head.is_null() {
        return 0;
    }

    let mut handler = (*head)
        .as_ref()
        .map(|p| p as *const xfrm6_protocol as *mut xfrm6_protocol)
        .unwrap_or(ptr::null_mut());

    while !handler.is_null() {
        let ret = ((*(*handler).cb_handler)(skb, err));
        if ret <= 0 {
            return ret;
        }
        handler = (*handler).next;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn xfrm6_rcv_encap(
    skb: *mut sk_buff,
    nexthdr: c_int,
    spi: u32,
    encap_type: c_int,
) -> c_int {
    let head = proto_handlers(nexthdr as u8);
    let mut ret = 0;

    if !head.is_null() {
        let mut handler = (*head)
            .as_ref()
            .map(|p| p as *const xfrm6_protocol as *mut xfrm6_protocol)
            .unwrap_or(ptr::null_mut());

        while !handler.is_null() {
            ret = ((*(*handler).input_handler)(skb, nexthdr, spi, encap_type));
            if ret != -EINVAL {
                return ret;
            }
            handler = (*handler).next;
        }
    }

    // Send ICMPv6 destination unreachable
    icmpv6_send(skb, ICMPV6_DEST_UNREACH, ICMPV6_PORT_UNREACH, 0);

    kfree_skb(skb);
    0
}

#[no_mangle]
pub unsafe extern "C" fn xfrm6_protocol_register(
    handler: *mut xfrm6_protocol,
    protocol: u8,
) -> c_int {
    let head = proto_handlers(protocol);
    if head.is_null() || netproto(protocol).is_null() {
        return EINVAL;
    }

    let mut mutex = &mut xfrm6_protocol_mutex;
    mutex.lock();

    let mut pprev = head;
    let mut t = (*pprev)
        .as_ref()
        .map(|p| p as *const xfrm6_protocol as *mut xfrm6_protocol)
        .unwrap_or(ptr::null_mut());
    let mut add_netproto = t.is_null();
    let mut ret = 0;

    while !t.is_null() {
        if (*t).priority < (*handler).priority {
            break;
        }
        if (*t).priority == (*handler).priority {
            ret = EEXIST;
            break;
        }
        pprev = &mut (*t).next;
        t = (*pprev)
            .as_ref()
            .map(|p| p as *const xfrm6_protocol as *mut xfrm6_protocol)
            .unwrap_or(ptr::null_mut());
    }

    if ret == 0 {
        (*handler).next = *pprev;
        *pprev = handler;
    }

    mutex.unlock();

    if add_netproto && ret == 0 {
        if inet6_add_protocol(netproto(protocol), protocol) != 0 {
            pr_err(b"xfrm6_protocol_register: can't add protocol\n".as_ptr() as *const c_char);
            ret = EAGAIN;
        }
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn xfrm6_protocol_deregister(
    handler: *mut xfrm6_protocol,
    protocol: u8,
) -> c_int {
    let head = proto_handlers(protocol);
    if head.is_null() || netproto(protocol).is_null() {
        return EINVAL;
    }

    let mut mutex = &mut xfrm6_protocol_mutex;
    mutex.lock();

    let mut pprev = head;
    let mut t = (*pprev)
        .as_ref()
        .map(|p| p as *const xfrm6_protocol as *mut xfrm6_protocol)
        .unwrap_or(ptr::null_mut());
    let mut ret = ENOENT;

    while !t.is_null() {
        if t == handler {
            *pprev = (*handler).next;
            ret = 0;
            break;
        }
        pprev = &mut (*t).next;
        t = (*pprev)
            .as_ref()
            .map(|p| p as *const xfrm6_protocol as *mut xfrm6_protocol)
            .unwrap_or(ptr::null_mut());
    }

    mutex.unlock();

    if ret == 0 {
        let empty = (*head)
            .as_ref()
            .map(|p| p as *const xfrm6_protocol as *mut xfrm6_protocol)
            .unwrap_or(ptr::null_mut())
            .is_null();
        if empty {
            if inet6_del_protocol(netproto(protocol), protocol) < 0 {
                pr_err(
                    b"xfrm6_protocol_deregister: can't remove protocol\n".as_ptr() as *const c_char,
                );
                ret = EAGAIN;
            }
        }
    }

    synchronize_net();
    ret
}

// Helper functions (simplified for kernel compatibility)
unsafe fn netproto(protocol: u8) -> *mut inet6_protocol {
    match protocol {
        IPPROTO_ESP => &esp6_protocol as *const inet6_protocol as *mut inet6_protocol,
        IPPROTO_AH => &ah6_protocol as *const inet6_protocol as *mut inet6_protocol,
        IPPROTO_COMP => &ipcomp6_protocol as *const inet6_protocol as *mut inet6_protocol,
        _ => ptr::null_mut(),
    }
}

#[repr(C)]
static mut esp6_protocol: inet6_protocol = inet6_protocol {
    handler: xfrm6_esp_rcv,
    err_handler: xfrm6_esp_err,
    flags: INET6_PROTO_NOPOLICY,
};

#[repr(C)]
static mut ah6_protocol: inet6_protocol = inet6_protocol {
    handler: xfrm6_ah_rcv,
    err_handler: xfrm6_ah_err,
    flags: INET6_PROTO_NOPOLICY,
};

#[repr(C)]
static mut ipcomp6_protocol: inet6_protocol = inet6_protocol {
    handler: xfrm6_ipcomp_rcv,
    err_handler: xfrm6_ipcomp_err,
    flags: INET6_PROTO_NOPOLICY,
};

#[repr(C)]
static mut xfrm6_input_afinfo: xfrm_input_afinfo = xfrm_input_afinfo {
    family: AF_INET6,
    callback: xfrm6_rcv_cb,
};

#[no_mangle]
pub unsafe extern "C" fn xfrm6_protocol_init() -> c_int {
    xfrm_input_register_afinfo(&xfrm6_input_afinfo)
}

#[no_mangle]
pub unsafe extern "C" fn xfrm6_protocol_fini() {
    xfrm_input_unregister_afinfo(&xfrm6_input_afinfo)
}

// Dummy implementations for required kernel functions
#[no_mangle]
pub unsafe extern "C" fn icmpv6_send(skb: *mut sk_buff, _type: c_int, code: c_int, info: c_int) {
    // Dummy implementation
}

#[no_mangle]
pub unsafe extern "C" fn kfree_skb(skb: *mut sk_buff) {
    // Dummy implementation
}

#[no_mangle]
pub unsafe extern "C" fn pr_err(fmt: *const c_char) {
    // Dummy implementation
}

#[no_mangle]
pub unsafe extern "C" fn synchronize_net() {
    // Dummy implementation
}

#[no_mangle]
pub unsafe extern "C" fn inet6_add_protocol(proto: *mut inet6_protocol, protocol: u8) -> c_int {
    // Dummy implementation
    0
}

#[no_mangle]
pub unsafe extern "C" fn inet6_del_protocol(proto: *mut inet6_protocol, protocol: u8) -> c_int {
    // Dummy implementation
    0
}

#[no_mangle]
pub unsafe extern "C" fn xfrm_input_register_afinfo(afinfo: *mut xfrm_input_afinfo) -> c_int {
    // Dummy implementation
    0
}

#[no_mangle]
pub unsafe extern "C" fn xfrm_input_unregister_afinfo(afinfo: *mut xfrm_input_afinfo) {
    // Dummy implementation
}

// Test cases (conditional compilation)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_registration() {
        // This would be a real test in a kernel environment
        unsafe {
            let handler = ptr::null_mut();
            assert_eq!(xfrm6_protocol_register(handler, IPPROTO_ESP), EINVAL);
        }
    }
}
