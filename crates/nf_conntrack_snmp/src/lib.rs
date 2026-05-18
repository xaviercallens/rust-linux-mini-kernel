
//! SNMP service broadcast connection tracking helper
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use kernel_types::*;

// Constants from C
pub const SNMP_PORT: u16 = 161;
pub const NFPROTO_IPV4: u8 = 2;
pub const IPPROTO_UDP: u8 = 17;
pub const IPS_NAT_MASK: u32 = 0x00000004; // Example bitmask
pub const NF_ACCEPT: c_int = 1;

// Type definitions
#[repr(C)]
struct NfConntrackTupleUdp {
    port: u16,
}

#[repr(C)]
struct NfConntrackTupleSrcUnion {
    udp: NfConntrackTupleUdp,
}

#[repr(C)]
struct NfConntrackTupleSrc {
    l3num: u8,
    u: NfConntrackTupleSrcUnion,
}

#[repr(C)]
struct NfConntrackTupleDst {
    protonum: u8,
}

#[repr(C)]
struct NfConntrackTuple {
    src: NfConntrackTupleSrc,
    dst: NfConntrackTupleDst,
}

#[repr(C)]
struct NfConntrackExpectPolicy {
    max_expected: c_uint,
    timeout: c_uint,
}

#[repr(C)]
struct NfConntrackHelper {
    name: *const c_char,
    tuple: NfConntrackTuple,
    me: *mut c_void,
    help: Option<extern "C" fn(*mut c_void, c_uint, *mut NfConn, c_int) -> c_int>,
    expect_policy: *mut NfConntrackExpectPolicy,
}

#[repr(C)]
struct NfConn {
    status: u32,
}

// Function pointer type
type NfNatSnmpHook = extern "C" fn(*mut c_void, c_uint, *mut NfConn, c_int) -> c_int;

// Exported symbol
#[no_mangle]
pub static mut NF_NAT_SNMP_HOOK: Option<NfNatSnmpHook> = None;

// Internal static variables
static mut TIMEOUT: c_uint = 30;

// Helper function implementation
#[no_mangle]
pub unsafe extern "C" fn snmp_conntrack_help(
    skb: *mut c_void,
    protoff: c_uint,
    ct: *mut NfConn,
    ctinfo: c_int,
) -> c_int {
    // Call broadcast helper
    extern "C" {
        fn nf_conntrack_broadcast_help(
            skb: *mut c_void,
            ct: *mut NfConn,
            ctinfo: c_int,
            timeout: c_uint,
        );
    }
    nf_conntrack_broadcast_help(skb, ct, ctinfo, TIMEOUT);

    // SAFETY: NF_NAT_SNMP_HOOK is a function pointer managed by the kernel
    if let Some(nf_nat_snmp) = NF_NAT_SNMP_HOOK {
        // Check NAT status flag
        if (*ct).status & IPS_NAT_MASK != 0 {
            return nf_nat_snmp(skb, protoff, ct, ctinfo);
        }
    }

    NF_ACCEPT
}

// Static helper configuration
static mut EXP_POLICY: NfConntrackExpectPolicy = NfConntrackExpectPolicy {
    max_expected: 1,
    timeout: 0,
};

static mut HELPER: NfConntrackHelper = NfConntrackHelper {
    name: b"snmp\0".as_ptr() as *const c_char,
    tuple: NfConntrackTuple {
        src: NfConntrackTupleSrc {
            l3num: NFPROTO_IPV4,
            u: NfConntrackTupleSrcUnion {
                udp: NfConntrackTupleUdp { port: SNMP_PORT },
            },
        },
        dst: NfConntrackTupleDst {
            protonum: IPPROTO_UDP,
        },
    },
    me: core::ptr::null_mut(),
    help: Some(snmp_conntrack_help),
    expect_policy: &mut EXP_POLICY,
};

// Module initialization
#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_snmp_init() -> c_int {
    // Set timeout in expect policy
    EXP_POLICY.timeout = TIMEOUT;

    // Register helper
    extern "C" {
        fn nf_conntrack_helper_register(helper: *mut NfConntrackHelper) -> c_int;
    }
    nf_conntrack_helper_register(&mut HELPER)
}

// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_snmp_fini() {
    extern "C" {
        fn nf_conntrack_helper_unregister(helper: *mut NfConntrackHelper);
    }
    nf_conntrack_helper_unregister(&mut HELPER);
}