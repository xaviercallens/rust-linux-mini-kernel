//! Amanda NAT helper for TCP NAT alteration in Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kernel_types::*;

// Constants from C
pub const IP_CT_DIR_ORIGINAL: c_int = 0;
pub const NF_DROP: c_int = 0xFFFFFFFF;
pub const NF_ACCEPT: c_int = 0x00000001;
pub const EBUSY: c_int = 16;
pub const EINVAL: c_int = 22;

// Type definitions
#[repr(C)]
struct nf_conntrack_proto {
    tcp: nf_conntrack_proto_tcp,
}

#[repr(C)]
struct nf_conntrack_proto_tcp {
    port: u16,
}

#[repr(C)]
struct nf_conntrack_tuple_dst {
    u: nf_conntrack_tuple_u,
}

#[repr(C)]
struct nf_conntrack_tuple_u {
    tcp: nf_conntrack_tuple_tcp,
}

#[repr(C)]
struct nf_conntrack_tuple_tcp {
    port: u16,
}

#[repr(C)]
struct nf_conntrack_tuple {
    dst: nf_conntrack_tuple_dst,
}

#[repr(C)]
struct nf_conntrack_expect {
    saved_proto: nf_conntrack_proto,
    tuple: nf_conntrack_tuple,
    dir: c_int,
    expectfn: *mut c_void,
}

// Function declarations for kernel functions
extern "C" {
    fn nf_ct_expect_related(exp: *mut nf_conntrack_expect, flags: c_int) -> c_int;
    fn nf_ct_unexpect_related(exp: *mut nf_conntrack_expect);
    fn nf_nat_mangle_udp_packet(
        skb: *mut sk_buff,
        master: *mut c_void,
        ctinfo: c_uint,
        protoff: c_uint,
        matchoff: c_uint,
        matchlen: c_uint,
        buffer: *const u8,
        buflen: c_uint,
    ) -> c_int;
    fn nf_ct_helper_log(skb: *mut sk_buff, master: *mut c_void, msg: *const u8);
    fn nf_nat_helper_unregister(helper: *mut nf_conntrack_nat_helper);
    fn nf_nat_helper_register(helper: *mut nf_conntrack_nat_helper);
    fn synchronize_rcu();
}

#[repr(C)]
struct nf_conntrack_nat_helper {
    name: *const u8,
}

// Helper macro for initializing nf_conntrack_nat_helper
const fn nf_ct_nat_helper_init(name: *const u8) -> nf_conntrack_nat_helper {
    nf_conntrack_nat_helper { name }
}

// Global static variables
static mut nat_helper_amanda: nf_conntrack_nat_helper = nf_ct_nat_helper_init(b"amanda\0".as_ptr() as *const u8);
static mut nf_nat_amanda_hook: Option<unsafe extern "C" fn(
    skb: *mut sk_buff,
    ctinfo: c_uint,
    protoff: c_uint,
    matchoff: c_uint,
    matchlen: c_uint,
    exp: *mut nf_conntrack_expect,
) -> c_uint> = None;

// Main helper function
#[no_mangle]
pub unsafe extern "C" fn help(
    skb: *mut sk_buff,
    ctinfo: c_uint,
    protoff: c_uint,
    matchoff: c_uint,
    matchlen: c_uint,
    exp: *mut nf_conntrack_expect,
) -> c_uint {
    // SAFETY: Caller guarantees exp is valid
    let exp = exp;

    // Save original port and set direction
    (*exp).saved_proto.tcp.port = (*exp).tuple.dst.u.tcp.port;
    (*exp).dir = IP_CT_DIR_ORIGINAL;

    // Set expectation function
    (*exp).expectfn = core::ptr::null_mut();

    // Try to allocate port
    let mut port = ntohs((*exp).saved_proto.tcp.port);
    let mut res: c_int;

    while port != 0 {
        // Set port in network byte order
        (*exp).tuple.dst.u.tcp.port = htons(port as u16);

        res = nf_ct_expect_related(exp, 0);

        if res == 0 {
            break;
        } else if res != -EBUSY {
            port = 0;
            break;
        }

        port -= 1;
    }

    if port == 0 {
        nf_ct_helper_log(skb, core::ptr::null_mut(), b"all ports in use\0".as_ptr() as *const u8);
        return NF_DROP;
    }

    // Convert port to string
    let mut buffer: [u8; 6] = [0; 6];
    let mut buffer_ptr = buffer.as_mut_ptr();

    // Manual conversion from u16 to string
    let mut temp_port = port;
    let mut i = 5;
    buffer[i] = 0;

    while temp_port > 0 && i > 0 {
        i -= 1;
        buffer[i] = (temp_port % 10) as u8 + b'0';
        temp_port /= 10;
    }

    if temp_port > 0 {
        // Port too large (shouldn't happen as port is <= 65535)
        buffer[0] = b'0';
        buffer[1] = 0;
    }

    // Mangle UDP packet
    let success = nf_nat_mangle_udp_packet(
        skb,
        core::ptr::null_mut(),
        ctinfo,
        protoff,
        matchoff,
        matchlen,
        buffer.as_ptr(),
        (6 - i) as c_uint,
    );

    if success != 0 {
        nf_ct_helper_log(skb, core::ptr::null_mut(), b"cannot mangle packet\0".as_ptr() as *const u8);
        nf_ct_unexpect_related(exp);
        return NF_DROP;
    }

    NF_ACCEPT
}

// Module exit handler
#[no_mangle]
pub unsafe extern "C" fn nf_nat_amanda_fini() {
    nf_nat_helper_unregister(&mut nat_helper_amanda);
    nf_nat_amanda_hook = None;
    synchronize_rcu();
}

// Module init handler
#[no_mangle]
pub unsafe extern "C" fn nf_nat_amanda_init() {
    // SAFETY: This is a module init function, called once at load time
    assert!(nf_nat_amanda_hook.is_none(), "Hook already initialized");

    nf_nat_helper_register(&mut nat_helper_amanda);
    nf_nat_amanda_hook = Some(help);
}

// Helper functions for byte order conversion
#[inline]
unsafe fn htons(x: u16) -> u16 {
    x.to_be()
}

#[inline]
unsafe fn ntohs(x: u16) -> u16 {
    u16::from_be(x)
}
