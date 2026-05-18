
//! SNMP service broadcast connection tracking helper
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::ManuallyDrop;
use core::panic::PanicInfo;
use kernel_types::*;

pub const SNMP_PORT: u16 = 161;
pub const NFPROTO_IPV4: u8 = 2;
pub const IPPROTO_UDP: u8 = 17;
pub const IPS_NAT_MASK: u32 = 0x00000004;
pub const NF_ACCEPT: c_int = 1;

#[repr(C)]
pub struct NfConn {
    _priv: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NfConntrackTupleUdp {
    pub port: u16,
}

#[repr(C)]
pub union NfConntrackTupleSrcUnion {
    pub udp: ManuallyDrop<NfConntrackTupleUdp>,
}

#[repr(C)]
pub struct NfConntrackTupleSrc {
    pub l3num: u8,
    pub u: NfConntrackTupleSrcUnion,
}

#[repr(C)]
pub struct NfConntrackTupleDst {
    pub protonum: u8,
}

#[repr(C)]
pub struct NfConntrackTuple {
    pub src: NfConntrackTupleSrc,
    pub dst: NfConntrackTupleDst,
}

#[repr(C)]
pub struct NfConntrackExpectPolicy {
    pub max_expected: c_uint,
    pub timeout: c_uint,
}

pub type NfNatSnmpHook = extern "C" fn(*mut c_void, c_uint, *mut NfConn, c_int) -> c_int;

#[repr(C)]
pub struct NfConntrackHelper {
    pub name: *const c_char,
    pub tuple: NfConntrackTuple,
    pub me: *mut c_void,
    pub help: Option<extern "C" fn(*mut c_void, c_uint, *mut NfConn, c_int) -> c_int>,
    pub expect_policy: *mut NfConntrackExpectPolicy,
}

unsafe extern "C" {
    fn nf_conntrack_broadcast_help(
        skb: *mut c_void,
        ct: *mut NfConn,
        ctinfo: c_int,
        timeout: c_uint,
    );
    fn nf_conntrack_helper_register(helper: *mut NfConntrackHelper) -> c_int;
    fn nf_conntrack_helper_unregister(helper: *mut NfConntrackHelper);
    fn nf_ct_is_nat(ct: *mut NfConn) -> c_int;
}

// Exported symbol
#[no_mangle]
pub static mut NF_NAT_SNMP_HOOK: Option<NfNatSnmpHook> = None;

// Internal static variables
static mut TIMEOUT: c_uint = 30;

#[unsafe(no_mangle)]
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
                udp: ManuallyDrop::new(NfConntrackTupleUdp { port: SNMP_PORT }),
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nf_conntrack_snmp_init() -> c_int {
    // Set timeout in expect policy
    EXP_POLICY.timeout = TIMEOUT;

    // Register helper
    extern "C" {
        fn nf_conntrack_helper_register(helper: *mut NfConntrackHelper) -> c_int;
    }
    nf_conntrack_helper_register(&mut HELPER)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nf_conntrack_snmp_fini() {
    unsafe {
        nf_conntrack_helper_unregister(core::ptr::addr_of_mut!(helper));
    }
    nf_conntrack_helper_unregister(&mut HELPER);
}