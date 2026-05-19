
//! NetBIOS name service broadcast connection tracking helper
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use core::ffi::c_void;
use core::ptr;
use kernel_types::*;

const NMBD_PORT: u16 = 137;
const IPPROTO_UDP: u8 = 17;
const NFPROTO_IPV4: u8 = 2;

#[repr(C)]
#[derive(Copy, Clone)]
struct nfct_tuple_src_udp {
    port: u16,
}

#[repr(C)]
union nfct_tuple_src_u {
    udp: nfct_tuple_src_udp,
}

#[repr(C)]
struct nfct_tuple_src {
    u3: [u32; 4],
    u: nfct_tuple_src_u,
    l3num: u16,
}

#[repr(C)]
struct nfct_tuple_dst {
    u3: [u32; 4],
    protonum: u8,
    dir: u8,
}

#[repr(C)]
struct nfct_tuple {
    src: nfct_tuple_src,
    dst: nfct_tuple_dst,
}

#[repr(C)]
struct nf_conntrack_expect_policy {
    max_expected: u32,
    timeout: u32,
}

#[repr(C)]
struct nf_conntrack_helper {
    name: *const c_char,
    tuple: nfct_tuple,
    expect_policy: *mut nf_conntrack_expect_policy,
    me: *mut c_void,
    help: extern "C" fn(*mut c_void, u32, *mut c_void, u32) -> c_int,
}

// Module parameters
static mut TIMEOUT: u32 = 3;

// Helper struct
static mut HELPER: nf_conntrack_helper = nf_conntrack_helper {
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
static mut EXP_POLICY: nf_conntrack_expect_policy = nf_conntrack_expect_policy {
    max_expected: 1,
    timeout: 3,
};

// Function implementations
#[no_mangle]
extern "C" fn netbios_ns_help(skb: *mut c_void, protoff: u32, ct: *mut c_void, ctinfo: u32) -> i32 {
    unsafe { nf_conntrack_broadcast_help(skb, ct, ctinfo, TIMEOUT) }
}

static mut HELPER: nf_conntrack_helper = nf_conntrack_helper {
    name: HELPER_NAME.as_ptr() as *const c_char,
    tuple: nfct_tuple {
        src: nfct_tuple_src {
            u3: [0; 4],
            u: nfct_tuple_src_u {
                udp: nfct_tuple_src_udp {
                    port: NMBD_PORT.to_be(),
                },
            },
            l3num: NFPROTO_IPV4 as u16,
        },
        dst: nfct_tuple_dst {
            u3: [0; 4],
            protonum: IPPROTO_UDP,
            dir: 0,
        },
    },
    expect_policy: ptr::null_mut(),
    me: ptr::null_mut(),
    help: netbios_ns_help,
};

unsafe extern "C" {
    fn nf_conntrack_helper_register(helper: *mut nf_conntrack_helper) -> c_int;
    fn nf_conntrack_helper_unregister(helper: *mut nf_conntrack_helper);
    fn nf_conntrack_broadcast_help(
        skb: *mut c_void,
        ct: *mut c_void,
        ctinfo: u32,
        timeout: u32,
    ) -> c_int;
}

#[unsafe(no_mangle)]
pub extern "C" fn nf_conntrack_netbios_ns_init() -> c_int {
    unsafe {
        // SAFETY: EXP_POLICY is valid and properly initialized
        EXP_POLICY.timeout = TIMEOUT;

        // Register the helper
        nf_conntrack_helper_register(&mut HELPER)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn nf_conntrack_netbios_ns_fini() {
    unsafe {
        nf_conntrack_helper_unregister(&mut HELPER);
    }
}

// Module parameters (simplified for Rust)
#[no_mangle]
pub static mut module_param_timeout: u32 = 3;
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
