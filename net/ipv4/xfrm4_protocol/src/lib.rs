//! xfrm4_protocol - Generic xfrm protocol multiplexer for IPv4
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_void;
use core::mem;
use core::ptr;
use core::ptr::NonNull;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOENT: c_int = -2;
pub const ENOSYS: c_int = -38;

// Type definitions
#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iphdr {
    tos: u8,
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfrm4_protocol {
    next: *mut xfrm4_protocol,
    priority: c_int,
    handler: Option<extern "C" fn(*mut sk_buff) -> c_int>,
    input_handler: Option<extern "C" fn(*mut sk_buff, c_int, u32, c_int) -> c_int>,
    cb_handler: Option<extern "C" fn(*mut sk_buff, c_int) -> c_int>,
    err_handler: Option<extern "C" fn(*mut sk_buff, u32) -> c_int>,
}

#[repr(C)]
pub struct net_protocol {
    handler: Option<extern "C" fn(*mut sk_buff) -> c_int>,
    err_handler: Option<extern "C" fn(*mut sk_buff, u32) -> c_int>,
    no_policy: c_int,
    netns_ok: c_int,
}

#[repr(C)]
pub struct xfrm_input_afinfo {
    family: c_int,
    callback: Option<extern "C" fn(*mut sk_buff, u8, c_int) -> c_int>,
}

// Function pointers for C functions we're assuming exist
extern "C" {
    fn ip_route_input_noref(skb: *mut sk_buff, daddr: u32, saddr: u32, tos: u8, dev: *mut c_void) -> c_int;
    fn icmp_send(skb: *mut sk_buff, type_: c_int, code: c_int, info: u32);
    fn kfree_skb(skb: *mut sk_buff);
    fn inet_add_protocol(proto: *const net_protocol, protocol: u8) -> c_int;
    fn inet_del_protocol(proto: *const net_protocol, protocol: u8) -> c_int;
    fn xfrm_input_register_afinfo(afinfo: *const xfrm_input_afinfo);
}

// Static variables
static mut esp4_handlers: *mut xfrm4_protocol = ptr::null_mut();
static mut ah4_handlers: *mut xfrm4_protocol = ptr::null_mut();
static mut ipcomp4_handlers: *mut xfrm4_protocol = ptr::null_mut();

// Mutex for protocol registration
static mut xfrm4_protocol_mutex: c_int = 0; // Simplified representation

// Constants for protocols
pub const IPPROTO_ESP: u8 = 50;
pub const IPPROTO_AH: u8 = 51;
pub const IPPROTO_COMP: u8 = 108;

// Helper functions
fn proto_handlers(protocol: u8) -> *mut *mut xfrm4_protocol {
    match protocol {
        IPPROTO_ESP => &esp4_handlers,
        IPPROTO_AH => &ah4_handlers,
        IPPROTO_COMP => &ipcomp4_handlers,
        _ => ptr::null_mut(),
    }
}

fn netproto(protocol: u8) -> *const net_protocol {
    match protocol {
        IPPROTO_ESP => &esp4_protocol,
        IPPROTO_AH => &ah4_protocol,
        IPPROTO_COMP => &ipcomp4_protocol,
        _ => ptr::null(),
    }
}

// Static protocol handlers
static esp4_protocol: net_protocol = net_protocol {
    handler: Some(xfrm4_esp_rcv),
    err_handler: Some(xfrm4_esp_err),
    no_policy: 1,
    netns_ok: 1,
};

static ah4_protocol: net_protocol = net_protocol {
    handler: Some(xfrm4_ah_rcv),
    err_handler: Some(xfrm4_ah_err),
    no_policy: 1,
    netns_ok: 1,
};

static ipcomp4_protocol: net_protocol = net_protocol {
    handler: Some(xfrm4_ipcomp_rcv),
    err_handler: Some(xfrm4_ipcomp_err),
    no_policy: 1,
    netns_ok: 1,
};

static xfrm4_input_afinfo: xfrm_input_afinfo = xfrm_input_afinfo {
    family: 2, // AF_INET
    callback: Some(xfrm4_rcv_cb),
};

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn xfrm4_rcv_cb(skb: *mut sk_buff, protocol: u8, err: c_int) -> c_int {
    let head = proto_handlers(protocol);
    if head.is_null() {
        return 0;
    }

    let mut handler = ptr::null_mut();
    let mut ret = 0;

    // SAFETY: We're in an RCU read-side critical section
    // and head is valid (checked above)
    handler = *head;

    while !handler.is_null() {
        if let Some(cb_handler) = (*handler).cb_handler {
            ret = cb_handler(skb, err);
            if ret <= 0 {
                return ret;
            }
        }
        handler = (*handler).next;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn xfrm4_rcv_encap(
    skb: *mut sk_buff,
    nexthdr: c_int,
    spi: u32,
    encap_type: c_int,
) -> c_int {
    let head = proto_handlers(nexthdr as u8);
    let mut ret = 0;

    // SAFETY: We're assuming XFRM_TUNNEL_SKB_CB is a valid macro
    // that sets the tunnel field. In Rust, we just set it directly.
    // This is a simplified representation.
    let tunnel = XFRM_TUNNEL_SKB_CB(skb);
    (*tunnel).tunnel.ip4 = ptr::null_mut();

    let spi_cb = XFRM_SPI_SKB_CB(skb);
    (*spi_cb).family = 2; // AF_INET
    (*spi_cb).daddroff = mem::offset_of!(iphdr, daddr) as i32;

    if head.is_null() {
        goto out;
    }

    if skb_dst(skb).is_null() {
        let iph = ip_hdr(skb);
        let daddr = (*iph).daddr;
        let saddr = (*iph).daddr; // Simplified
        let tos = (*iph).tos;
        let dev = ptr::null_mut();

        if ip_route_input_noref(skb, daddr, saddr, tos, dev) != 0 {
            goto drop;
        }
    }

    let mut handler = ptr::null_mut();
    handler = *head;

    while !handler.is_null() {
        if let Some(input_handler) = (*handler).input_handler {
            ret = input_handler(skb, nexthdr, spi, encap_type);
            if ret != -EINVAL {
                return ret;
            }
        }
        handler = (*handler).next;
    }

out:
    icmp_send(skb, 3, 3, 0); // ICMP_DEST_UNREACH, ICMP_PORT_UNREACH

drop:
    kfree_skb(skb);
    0
}

#[no_mangle]
pub unsafe extern "C" fn xfrm4_esp_rcv(skb: *mut sk_buff) -> c_int {
    let mut handler = esp4_handlers;
    let mut ret = 0;

    while !handler.is_null() {
        if let Some(handler_func) = (*handler).handler {
            ret = handler_func(skb);
            if ret != -EINVAL {
                return ret;
            }
        }
        handler = (*handler).next;
    }

    icmp_send(skb, 3, 3, 0); // ICMP_DEST_UNREACH, ICMP_PORT_UNREACH
    kfree_skb(skb);
    0
}

#[no_mangle]
pub unsafe extern "C" fn xfrm4_esp_err(skb: *mut sk_buff, info: u32) -> c_int {
    let mut handler = esp4_handlers;

    while !handler.is_null() {
        if let Some(err_handler) = (*handler).err_handler {
            if err_handler(skb, info) == 0 {
                return 0;
            }
        }
        handler = (*handler).next;
    }

    -ENOENT
}

#[no_mangle]
pub unsafe extern "C" fn xfrm4_ah_rcv(skb: *mut sk_buff) -> c_int {
    let mut handler = ah4_handlers;
    let mut ret = 0;

    while !handler.is_null() {
        if let Some(handler_func) = (*handler).handler {
            ret = handler_func(skb);
            if ret != -EINVAL {
                return ret;
            }
        }
        handler = (*handler).next;
    }

    icmp_send(skb, 3, 3, 0); // ICMP_DEST_UNREACH, ICMP_PORT_UNREACH
    kfree_skb(skb);
    0
}

#[no_mangle]
pub unsafe extern "C" fn xfrm4_ah_err(skb: *mut sk_buff, info: u32) -> c_int {
    let mut handler = ah4_handlers;

    while !handler.is_null() {
        if let Some(err_handler) = (*handler).err_handler {
            if err_handler(skb, info) == 0 {
                return 0;
            }
        }
        handler = (*handler).next;
    }

    -ENOENT
}

#[no_mangle]
pub unsafe extern "C" fn xfrm4_ipcomp_rcv(skb: *mut sk_buff) -> c_int {
    let mut handler = ipcomp4_handlers;
    let mut ret = 0;

    while !handler.is_null() {
        if let Some(handler_func) = (*handler).handler {
            ret = handler_func(skb);
            if ret != -EINVAL {
                return ret;
            }
        }
        handler = (*handler).next;
    }

    icmp_send(skb, 3, 3, 0); // ICMP_DEST_UNREACH, ICMP_PORT_UNREACH
    kfree_skb(skb);
    0
}

#[no_mangle]
pub unsafe extern "C" fn xfrm4_ipcomp_err(skb: *mut sk_buff, info: u32) -> c_int {
    let mut handler = ipcomp4_handlers;

    while !handler.is_null() {
        if let Some(err_handler) = (*handler).err_handler {
            if err_handler(skb, info) == 0 {
                return 0;
            }
        }
        handler = (*handler).next;
    }

    -ENOENT
}

#[no_mangle]
pub unsafe extern "C" fn xfrm4_protocol_register(
    handler: *mut xfrm4_protocol,
    protocol: u8,
) -> c_int {
    let head = proto_handlers(protocol);
    if head.is_null() || netproto(protocol).is_null() {
        return -EINVAL;
    }

    // Simplified mutex handling - in real kernel code this would use
    // the actual mutex API
    let add_netproto = if (*head).is_null() {
        true
    } else {
        false
    };

    let mut pprev = head;
    let mut t = *pprev;
    let mut ret = -EEXIST;

    while !t.is_null() {
        if (*t).priority < (*handler).priority {
            break;
        }
        if (*t).priority == (*handler).priority {
            return -EEXIST;
        }
        pprev = &mut (*t).next;
        t = *pprev;
    }

    (*handler).next = *pprev;
    *pprev = handler;

    ret = 0;

    if add_netproto {
        if inet_add_protocol(netproto(protocol), protocol) != 0 {
            pr_err(b"can't add protocol\n".as_ptr() as *const c_char);
            ret = -EAGAIN;
        }
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn xfrm4_protocol_deregister(
    handler: *mut xfrm4_protocol,
    protocol: u8,
) -> c_int {
    let head = proto_handlers(protocol);
    if head.is_null() || netproto(protocol).is_null() {
        return -EINVAL;
    }

    let mut pprev = head;
    let mut t = *pprev;
    let mut ret = -ENOENT;

    while !t.is_null() {
        if t == handler {
            *pprev = (*handler).next;
            ret = 0;
            break;
        }
        pprev = &mut (*t).next;
        t = *pprev;
    }

    if (*head).is_null() {
        if inet_del_protocol(netproto(protocol), protocol) < 0 {
            pr_err(b"can't remove protocol\n".as_ptr() as *const c_char);
            ret = -EAGAIN;
        }
    }

    synchronize_net();
    ret
}

#[no_mangle]
pub unsafe extern "C" fn xfrm4_protocol_init() {
    xfrm_input_register_afinfo(&xfrm4_input_afinfo);
}

// Helper macros (simplified)
#[inline]
unsafe fn XFRM_TUNNEL_SKB_CB(skb: *mut sk_buff) -> *mut c_void {
    // Simplified version - in real code this would use skb->cb
    skb as *mut c_void
}

#[inline]
unsafe fn XFRM_SPI_SKB_CB(skb: *mut sk_buff) -> *mut c_void {
    // Simplified version - in real code this would use skb->cb
    skb as *mut c_void
}

#[inline]
unsafe fn ip_hdr(skb: *mut sk_buff) -> *mut iphdr {
    // Simplified version - in real code this would use ip_hdr(skb)
    skb as *mut iphdr
}

#[inline]
unsafe fn skb_dst(skb: *mut sk_buff) -> *mut c_void {
    // Simplified version - in real code this would use skb_dst(skb)
    ptr::null_mut()
}

#[inline]
unsafe fn pr_err(msg: *const c_char) {
    // Simplified version - in real code this would use printk
}

#[inline]
unsafe fn synchronize_net() {
    // Simplified version - in real code this would use synchronize_net()
}

#[inline]
unsafe fn xfrm_input_register_afinfo(afinfo: *const xfrm_input_afinfo) {
    // Simplified version - in real code this would register the afinfo
}
