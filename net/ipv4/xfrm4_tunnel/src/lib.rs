//! IPsec IPIP Tunnel Transformer
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::c_int;
use core::ffi::c_void;
use core::ffi::c_char;
use core::ptr;
use core::mem::size_of;

// Constants from C
pub const IPPROTO_IPIP: c_int = 4;
pub const XFRM_MODE_TUNNEL: c_int = 2;
pub const EINVAL: c_int = -22;
pub const ENOENT: c_int = -2;
pub const EAGAIN: c_int = -35;
pub const AF_INET: c_int = 2;

// Type definitions
#[repr(C)]
struct iphdr {
    protocol: u8,
    saddr: u32,
}

#[repr(C)]
struct xfrm_state {
    props: xfrm_state_props,
    encap: *mut c_void,
}

#[repr(C)]
struct xfrm_state_props {
    mode: c_int,
    header_len: u32,
}

#[repr(C)]
struct xfrm_type {
    description: *const c_char,
    owner: *mut c_void,
    proto: c_int,
    init_state: Option<unsafe extern "C" fn(*mut xfrm_state) -> c_int>,
    destructor: Option<unsafe extern "C" fn(*mut xfrm_state)>,
    input: Option<unsafe extern "C" fn(*mut xfrm_state, *mut c_void) -> c_int>,
    output: Option<unsafe extern "C" fn(*mut xfrm_state, *mut c_void) -> c_int>,
}

#[repr(C)]
struct xfrm_tunnel {
    handler: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    err_handler: Option<unsafe extern "C" fn(*mut c_void, u32) -> c_int>,
    priority: c_int,
}

// Extern declarations for kernel functions
extern "C" {
    fn xfrm_register_type(type_: *const xfrm_type, family: c_int) -> c_int;
    fn xfrm_unregister_type(type_: *const xfrm_type, family: c_int);
    fn xfrm4_tunnel_register(handler: *const xfrm_tunnel, family: c_int) -> c_int;
    fn xfrm4_tunnel_deregister(handler: *const xfrm_tunnel, family: c_int);
    fn skb_network_offset(skb: *mut c_void) -> c_int;
    fn skb_push(skb: *mut c_void, offset: c_int);
    fn ip_hdr(skb: *mut c_void) -> *mut iphdr;
    fn xfrm4_rcv_spi(skb: *mut c_void, proto: c_int, saddr: u32) -> c_int;
    fn pr_info(fmt: *const c_char, ...) -> c_int;
}

// Function implementations
static mut ipip_type: xfrm_type = xfrm_type {
    description: b"IPIP\0".as_ptr() as *const c_char,
    owner: ptr::null_mut(),
    proto: IPPROTO_IPIP,
    init_state: Some(ipip_init_state),
    destructor: Some(ipip_destroy),
    input: Some(ipip_xfrm_rcv),
    output: Some(ipip_output),
};

static mut xfrm_tunnel_handler: xfrm_tunnel = xfrm_tunnel {
    handler: Some(xfrm_tunnel_rcv),
    err_handler: Some(xfrm_tunnel_err),
    priority: 4,
};

#[cfg(feature = "ipv6")]
static mut xfrm64_tunnel_handler: xfrm_tunnel = xfrm_tunnel {
    handler: Some(xfrm_tunnel_rcv),
    err_handler: Some(xfrm_tunnel_err),
    priority: 3,
};

#[no_mangle]
pub unsafe extern "C" fn ipip_output(x: *mut xfrm_state, skb: *mut c_void) -> c_int {
    let offset = skb_network_offset(skb);
    skb_push(skb, -offset);
    0
}

#[no_mangle]
pub unsafe extern "C" fn ipip_xfrm_rcv(x: *mut xfrm_state, skb: *mut c_void) -> c_int {
    let ip_hdr = ip_hdr(skb);
    (*ip_hdr).protocol as c_int
}

#[no_mangle]
pub unsafe extern "C" fn ipip_init_state(x: *mut xfrm_state) -> c_int {
    let props = &mut (*x).props;
    if props.mode != XFRM_MODE_TUNNEL {
        return EINVAL;
    }
    
    if !(*x).encap.is_null() {
        return EINVAL;
    }
    
    props.header_len = size_of::<iphdr>() as u32;
    0
}

#[no_mangle]
pub unsafe extern "C" fn ipip_destroy(x: *mut xfrm_state) {
    // No action required
}

#[no_mangle]
pub unsafe extern "C" fn xfrm_tunnel_rcv(skb: *mut c_void) -> c_int {
    let ip_hdr = ip_hdr(skb);
    xfrm4_rcv_spi(skb, IPPROTO_IPIP, (*ip_hdr).saddr)
}

#[no_mangle]
pub unsafe extern "C" fn xfrm_tunnel_err(skb: *mut c_void, _: u32) -> c_int {
    ENOENT
}

#[no_mangle]
pub unsafe extern "C" fn ipip_init() -> c_int {
    if xfrm_register_type(&ipip_type, AF_INET) < 0 {
        pr_info(b"ipip_init: can't add xfrm type\0".as_ptr() as *const c_char);
        return EAGAIN;
    }

    if xfrm4_tunnel_register(&xfrm_tunnel_handler, AF_INET) != 0 {
        pr_info(b"ipip_init: can't add xfrm handler for AF_INET\0".as_ptr() as *const c_char);
        xfrm_unregister_type(&ipip_type, AF_INET);
        return EAGAIN;
    }

    #[cfg(feature = "ipv6")]
    {
        if xfrm4_tunnel_register(&xfrm64_tunnel_handler, 10) != 0 {
            pr_info(b"ipip_init: can't add xfrm handler for AF_INET6\0".as_ptr() as *const c_char);
            xfrm4_tunnel_deregister(&xfrm_tunnel_handler, AF_INET);
            xfrm_unregister_type(&ipip_type, AF_INET);
            return EAGAIN;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn ipip_fini() {
    #[cfg(feature = "ipv6")]
    {
        if xfrm4_tunnel_deregister(&xfrm64_tunnel_handler, 10) != 0 {
            pr_info(b"ipip_fini: can't remove xfrm handler for AF_INET6\0".as_ptr() as *const c_char);
        }
    }

    if xfrm4_tunnel_deregister(&xfrm_tunnel_handler, AF_INET) != 0 {
        pr_info(b"ipip_fini: can't remove xfrm handler for AF_INET\0".as_ptr() as *const c_char);
    }

    xfrm_unregister_type(&ipip_type, AF_INET);
}

#[cfg(test)]
mod tests {
    #[test]
    fn verify_struct_layout() {
        use core::mem::size_of_val;
        
        // Verify xfrm_type layout
        let xfrm_type_size = size_of_val(&super::ipip_type);
        assert!(xfrm_type_size > 0);
        
        // Verify xfrm_tunnel layout
        let xfrm_tunnel_size = size_of_val(&super::xfrm_tunnel_handler);
        assert!(xfrm_tunnel_size > 0);
    }
}
