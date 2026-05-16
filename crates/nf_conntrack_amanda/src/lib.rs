//! Amanda connection tracking module for Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::all)]

use core::ffi::c_void;
use core::mem;
use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

// Constants from C
pub const IPPROTO_UDP: u8 = 17;
pub const AF_INET: u8 = 2;
pub const AF_INET6: u8 = 10;
pub const IP_CT_DIR_ORIGINAL: u8 = 0;
pub const NF_ACCEPT: c_int = 0;
pub const NF_DROP: c_int = 1;
pub const NF_CT_EXPECT_CLASS_DEFAULT: u8 = 0;
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;

// Type definitions
#[repr(C)]
pub struct in6_addr {
    __in6_u: [u16; 8],
}

#[repr(C)]
pub struct nf_conntrack_tuple {
    src: nf_conntrack_tuple_ip,
    dst: nf_conntrack_tuple_ip,
}

#[repr(C)]
pub struct nf_conntrack_tuple_ip {
    u3: nf_conntrack_tuple_ip_u3,
}

#[repr(C)]
pub struct nf_conntrack_tuple_ip_u3 {
    _addr: [u8; 16], // Flexible based on address family
}

#[repr(C)]
pub struct nf_conn {
    tuplehash: [nf_conn_tuplehash; 2],
    status: u32,
}

#[repr(C)]
pub struct nf_conn_tuplehash {
    tuple: nf_conntrack_tuple,
}

#[repr(C)]
pub struct nf_conntrack_expect {
    _data: [u8; 1], // Opaque data
}

#[repr(C)]
pub struct ts_config {
    _data: [u8; 1], // Opaque textsearch config
}

#[repr(C)]
pub struct nf_conntrack_expect_policy {
    max_expected: c_uint,
    timeout: c_uint,
}

#[repr(C)]
pub struct nf_conntrack_helper {
    name: *const u8,
    me: *const c_void,
    help: extern "C" fn(*mut c_void, c_uint, *mut nf_conn, c_int) -> c_int,
    tuple: nf_conntrack_tuple,
    expect_policy: *const nf_conntrack_expect_policy,
    nat_mod_name: *const u8,
}

// Global variables
static mut master_timeout: c_uint = 300;
static ts_algo: &str = "kmp";
static HELPER_NAME: &str = "amanda";

// Function pointer
type nf_nat_amanda_hook_t = extern "C" fn(
    *mut c_void, // skb
    c_int,       // ctinfo
    c_uint,      // protoff
    c_uint,      // matchoff
    c_uint,      // matchlen
    *mut nf_conntrack_expect,
) -> c_int;

static mut nf_nat_amanda_hook: AtomicPtr<nf_nat_amanda_hook_t> = AtomicPtr::new(ptr::null_mut());

// Search patterns
#[repr(C)]
struct SearchPattern {
    string: *const u8,
    len: size_t,
    ts: *mut ts_config,
}

static mut search: [SearchPattern; 6] = [
    SearchPattern {
        string: b"CONNECT \0".as_ptr() as *const u8,
        len: 8,
        ts: ptr::null_mut(),
    },
    SearchPattern {
        string: b"\n\0".as_ptr() as *const u8,
        len: 1,
        ts: ptr::null_mut(),
    },
    SearchPattern {
        string: b"DATA \0".as_ptr() as *const u8,
        len: 5,
        ts: ptr::null_mut(),
    },
    SearchPattern {
        string: b"MESG \0".as_ptr() as *const u8,
        len: 5,
        ts: ptr::null_mut(),
    },
    SearchPattern {
        string: b"INDEX \0".as_ptr() as *const u8,
        len: 6,
        ts: ptr::null_mut(),
    },
    SearchPattern {
        string: b"STATE \0".as_ptr() as *const u8,
        len: 6,
        ts: ptr::null_mut(),
    },
];

// Helper functions
extern "C" {
    fn skb_find_text(skb: *mut c_void, from: c_uint, to: c_uint, ts: *mut ts_config) -> c_uint;

    fn nf_ct_refresh(ct: *mut nf_conn, skb: *mut c_void, timeout: c_uint);

    fn nf_ct_expect_alloc(ct: *mut nf_conn) -> *mut nf_conntrack_expect;

    fn nf_ct_expect_init(
        exp: *mut nf_conntrack_expect,
        class: u8,
        l3num: u8,
        src: *const nf_conntrack_tuple_ip_u3,
        dst: *const nf_conntrack_tuple_ip_u3,
        protonum: u8,
        _l4num: *const c_void,
        port: *const u16,
    );

    fn nf_ct_expect_related(exp: *mut nf_conntrack_expect, timeout: c_int) -> c_int;

    fn nf_ct_expect_put(exp: *mut nf_conntrack_expect);

    fn nf_conntrack_helpers_register(helpers: *mut nf_conntrack_helper, nhelpers: c_int) -> c_int;

    fn nf_conntrack_helpers_unregister(helpers: *mut nf_conntrack_helper, nhelpers: c_int);

    fn textsearch_prepare(
        algo: *const u8,
        string: *const u8,
        len: size_t,
        gfp: c_int,
        flags: c_int,
    ) -> *mut ts_config;

    fn textsearch_destroy(ts: *mut ts_config);

    fn nf_ct_helper_log(skb: *mut c_void, ct: *mut nf_conn, msg: *const u8);
}

// Module functions
#[no_mangle]
pub unsafe extern "C" fn amanda_help(
    skb: *mut c_void,
    protoff: c_uint,
    ct: *mut nf_conn,
    ctinfo: c_int,
) -> c_int {
    let mut ret = NF_ACCEPT;
    let nf_nat_amanda: nf_nat_amanda_hook_t;

    // Only look at packets from the Amanda server
    if CTINFO2DIR(ctinfo) == IP_CT_DIR_ORIGINAL {
        return NF_ACCEPT;
    }

    // Increase the UDP timeout of the master connection
    nf_ct_refresh(ct, skb, master_timeout * 1);

    let dataoff = protoff + mem::size_of::<udphdr>() as c_uint;
    if dataoff >= (*skb).len {
        nf_ct_helper_log(skb, ct, b"amanda_help: skblen = \0".as_ptr() as *const u8);
        return NF_ACCEPT;
    }

    let start = skb_find_text(skb, dataoff, (*skb).len, search[0].ts);
    if start == c_uint::MAX {
        return NF_ACCEPT;
    }
    let mut start = start + dataoff + search[0].len;

    let stop = skb_find_text(skb, start, (*skb).len, search[1].ts);
    if stop == c_uint::MAX {
        return NF_ACCEPT;
    }
    let stop = stop + start;

    for i in 2..=5 {
        let off = skb_find_text(skb, start, stop, search[i].ts);
        if off == c_uint::MAX {
            continue;
        }
        let mut off = off + start + search[i].len;

        let mut pbuf: [u8; 6] = [0; 6];
        let len = (stop - off).min(pbuf.len() - 1) as usize;
        if skb_copy_bits(skb, off, pbuf.as_mut_ptr(), len) != 0 {
            break;
        }
        pbuf[len] = 0;

        let port = u16::from_str_radix(core::str::from_utf8_unchecked(&pbuf[..len]), 10)
            .map_or(0, |n| n as u16);
        if port == 0 || len > 5 {
            break;
        }

        let exp = nf_ct_expect_alloc(ct);
        if exp.is_null() {
            nf_ct_helper_log(skb, ct, b"cannot alloc expectation\0".as_ptr() as *const u8);
            ret = NF_DROP;
            continue;
        }

        let tuple = &ct.tuplehash[IP_CT_DIR_ORIGINAL].tuple;
        nf_ct_expect_init(
            exp,
            NF_CT_EXPECT_CLASS_DEFAULT,
            nf_ct_l3num(ct),
            &tuple.src.u3,
            &tuple.dst.u3,
            IPPROTO_UDP,
            ptr::null(),
            &port,
        );

        nf_nat_amanda = *nf_nat_amanda_hook.load(Ordering::Relaxed);
        if !nf_nat_amanda.is_null() && (ct.status & IPS_NAT_MASK) != 0 {
            ret = nf_nat_amanda(skb, ctinfo, protoff, off - dataoff, len as c_uint, exp);
        } else if nf_ct_expect_related(exp, 0) != 0 {
            nf_ct_helper_log(skb, ct, b"cannot add expectation\0".as_ptr() as *const u8);
            ret = NF_DROP;
        }
        nf_ct_expect_put(exp);
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_amanda_init() -> c_int {
    let mut ret = 0;
    let mut i: c_int;

    for i in 0..6 {
        let algo = ts_algo.as_ptr() as *const u8;
        let string = search[i].string;
        let len = search[i].len;
        search[i].ts = textsearch_prepare(algo, string, len, 0, 0);
        if search[i].ts.is_null() {
            ret = -ENOMEM;
            while i > 0 {
                i -= 1;
                textsearch_destroy(search[i].ts);
            }
            return ret;
        }
    }

    let helpers = &mut search[0] as *mut _ as *mut nf_conntrack_helper;
    ret = nf_conntrack_helpers_register(helpers, 2);
    if ret < 0 {
        for i in (0..6).rev() {
            textsearch_destroy(search[i].ts);
        }
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_amanda_fini() {
    let mut i: c_int;

    nf_conntrack_helpers_unregister(&mut search[0] as *mut _ as *mut nf_conntrack_helper, 2);
    for i in (0..6).rev() {
        textsearch_destroy(search[i].ts);
    }
}

// Module exports
#[no_mangle]
pub unsafe extern "C" fn nf_nat_amanda_hook(
    skb: *mut c_void,
    ctinfo: c_int,
    protoff: c_uint,
    matchoff: c_uint,
    matchlen: c_uint,
    exp: *mut nf_conntrack_expect,
) -> c_int {
    // Implementation would be provided by NAT module
    0
}

// Helper macros
#[inline]
unsafe fn CTINFO2DIR(ctinfo: c_int) -> u8 {
    (ctinfo & 0x01) as u8
}

#[inline]
unsafe fn nf_ct_l3num(ct: *mut nf_conn) -> u8 {
    (*ct).tuplehash[0].tuple.src.l3num
}

#[inline]
unsafe fn skb_copy_bits(skb: *mut c_void, offset: c_uint, to: *mut u8, len: c_int) -> c_int {
    // Simplified implementation - actual implementation would copy from skb
    0
}

// Module metadata
#[no_mangle]
pub static HELPER_NAME_BYTES: [u8; 7] = *b"amanda\0";

#[no_mangle]
pub static amanda_exp_policy: nf_conntrack_expect_policy = nf_conntrack_expect_policy {
    max_expected: 4,
    timeout: 180,
};

#[no_mangle]
pub static amanda_helper: [nf_conntrack_helper; 2] = [
    nf_conntrack_helper {
        name: HELPER_NAME_BYTES.as_ptr(),
        me: ptr::null_mut(),
        help: amanda_help,
        tuple: nf_conntrack_tuple {
            src: nf_conntrack_tuple_ip {
                u3: nf_conntrack_tuple_ip_u3 { _addr: [0; 16] },
            },
            dst: nf_conntrack_tuple_ip {
                u3: nf_conntrack_tuple_ip_u3 { _addr: [0; 16] },
            },
        },
        expect_policy: &amanda_exp_policy,
        nat_mod_name: ptr::null_mut(),
    },
    nf_conntrack_helper {
        name: HELPER_NAME_BYTES.as_ptr(),
        me: ptr::null_mut(),
        help: amanda_help,
        tuple: nf_conntrack_tuple {
            src: nf_conntrack_tuple_ip {
                u3: nf_conntrack_tuple_ip_u3 { _addr: [0; 16] },
            },
            dst: nf_conntrack_tuple_ip {
                u3: nf_conntrack_tuple_ip_u3 { _addr: [0; 16] },
            },
        },
        expect_policy: &amanda_exp_policy,
        nat_mod_name: ptr::null_mut(),
    },
];
