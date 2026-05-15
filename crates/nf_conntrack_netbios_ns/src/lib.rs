//! NetBIOS name service broadcast connection tracking helper
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use core::ffi::c_void;
use core::ptr;
use core::mem;

// Constants from C
const NMBD_PORT: u16 = 137;
pub const IPPROTO_UDP: u8 = 17;
pub const NFPROTO_IPV4: u8 = 2;

// Type definitions
#[repr(C)]
struct nfct_tuple {
    src: nfct_tuple_src,
    dst: nfct_tuple_dst,
}

#[repr(C)]
struct nfct_tuple_src {
    l3num: u8,
    u: nfct_tuple_src_u,
}

#[repr(C)]
struct nfct_tuple_src_u {
    udp: nfct_tuple_src_udp,
}

#[repr(C)]
struct nfct_tuple_src_udp {
    port: u16,
}

#[repr(C)]
struct nfct_tuple_dst {
    protonum: u8,
}

#[repr(C)]
struct nf_conntrack_expect_policy {
    max_expected: u32,
    timeout: u32,
}

#[repr(C)]
struct nf_conntrack_helper {
    name: *const u8,
    tuple: nfct_tuple,
    me: *mut c_void,
    help: extern "C" fn(*mut c_void, u32, *mut c_void, u32) -> i32,
    expect_policy: *mut nf_conntrack_expect_policy,
}

// Module parameters
static mut timeout: u32 = 3;

// Helper struct
static mut helper: nf_conntrack_helper = nf_conntrack_helper {
    name: b"netbios-ns\0".as_ptr() as *const u8,
    tuple: nfct_tuple {
        src: nfct_tuple_src {
            l3num: NFPROTO_IPV4,
            u: nfct_tuple_src_u {
                udp: nfct_tuple_src_udp {
                    port: u16::to_be(NMBD_PORT),
                },
            },
        },
        dst: nfct_tuple_dst {
            protonum: IPPROTO_UDP,
        },
    },
    me: ptr::null_mut(),
    help: netbios_ns_help,
    expect_policy: ptr::null_mut(),
};

// Expect policy
static mut exp_policy: nf_conntrack_expect_policy = nf_conntrack_expect_policy {
    max_expected: 1,
    timeout: 0,
};

// Function implementations
extern "C" fn netbios_ns_help(
    skb: *mut c_void,
    protoff: u32,
    ct: *mut c_void,
    ctinfo: u32,
) -> i32 {
    unsafe {
        nf_conntrack_broadcast_help(skb, ct, ctinfo, timeout)
    }
}

// Extern declarations for kernel functions
extern "C" {
    fn nf_conntrack_helper_register(helper: *mut nf_conntrack_helper) -> i32;
    fn nf_conntrack_helper_unregister(helper: *mut nf_conntrack_helper);
    fn nf_conntrack_broadcast_help(
        skb: *mut c_void,
        ct: *mut c_void,
        ctinfo: u32,
        timeout: u32,
    ) -> i32;
}

// Module init/exit
#[no_mangle]
pub extern "C" fn nf_conntrack_netbios_ns_init() -> i32 {
    unsafe {
        // SAFETY: exp_policy is valid and properly initialized
        exp_policy.timeout = timeout;
        
        // Register the helper
        nf_conntrack_helper_register(&mut helper)
    }
}

#[no_mangle]
pub extern "C" fn nf_conntrack_netbios_ns_fini() {
    unsafe {
        nf_conntrack_helper_unregister(&mut helper);
    }
}

// Module parameters (simplified for Rust)
#[no_mangle]
pub static mut module_param_timeout: u32 = 3;