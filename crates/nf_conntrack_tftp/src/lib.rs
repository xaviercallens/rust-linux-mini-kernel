
//! TFTP connection tracking helper for Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;
use kernel_types::*;

pub const TFTP_PORT: u16 = 69;
pub const TFTP_OPCODE_READ: u16 = 1;
pub const TFTP_OPCODE_WRITE: u16 = 2;
pub const NF_ACCEPT: c_int = 1;
pub const AF_INET: c_int = 2;
pub const AF_INET6: c_int = 10;
pub const IPPROTO_UDP: c_int = 17;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tftphdr {
    pub opcode: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udphdr {
    _private: [u8; 8],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nf_conn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nf_conntrack_expect {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_udp {
    pub port: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_conntrack_tuple_union {
    pub u3: [u8; 16],
    pub udp: nf_conntrack_tuple_udp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple {
    pub src: nf_conntrack_tuple_union,
    pub dst: nf_conntrack_tuple_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_expect_policy {
    pub max_expected: c_uint,
    pub timeout: c_uint,
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_helper {
    _private: [u8; 0],
}

// Function pointer type
pub type NfNatTftpHookFn = extern "C" fn(
    skb: *mut sk_buff,
    ctinfo: c_int,
    exp: *mut nf_conntrack_expect,
) -> c_uint;

// Module parameters
static mut PORTS: [u16; 8] = [0; 8];
static mut PORTS_C: c_uint = 0;

// Exported symbol
static mut NF_NAT_TFTP_HOOK: Option<NfNatTftpHookFn> = None;

#[no_mangle]
pub extern "C" fn nf_nat_tftp_hook() -> *mut c_void {
    // SAFETY: This is a function pointer cast to void*, which is safe in C
    // but requires unsafe in Rust
    unsafe { ptr::from_ref(&NF_NAT_TFTP_HOOK) as *mut c_void }
}

static HELPER_NAME: &[u8] = b"tftp\0";
static ERR_MSG: &[u8] = b"failed to register helpers\n\0";

#[unsafe(no_mangle)]
pub static mut nf_nat_tftp_hook: Option<nf_nat_tftp_hook_t> = None;

static mut PORTS: [u16; 8] = [0; 8];
static mut PORTS_C: c_uint = 0;

static mut TFTP_HELPERS: [nf_conntrack_helper; 16] = [nf_conntrack_helper { _private: [] }; 16];

static TFTP_EXP_POLICY: nf_conntrack_expect_policy = nf_conntrack_expect_policy {
    max_expected: 1,
    timeout: 5 * 60,
};

const THIS_MODULE: *mut module = ptr::null_mut();

#[unsafe(no_mangle)]
pub extern "C" fn nf_conntrack_tftp_init() -> c_int {
    let mut ret: c_int = 0;

    // Initialize default port if none specified
    if unsafe { PORTS_C } == 0 {
        unsafe {
            PORTS[0] = TFTP_PORT;
            PORTS_C = 1;
        }

    // Register helpers for both IPv4 and IPv6
    for i in 0..unsafe { PORTS_C } {
        // Initialize IPv4 helper
        unsafe {
            nf_ct_helper_init(
                &mut TFTP[2 * i as usize],
                2, // AF_INET
                17, // IPPROTO_UDP
                HELPER_NAME.as_ptr() as *const c_char,
                TFTP_PORT,
                PORTS[i as usize],
                i as c_int,
                &TFTP_EXP_POLICY,
                0,
                Some(tftp_help),
                ptr::null_mut(),
                THIS_MODULE,
            );

            nf_ct_helper_init(
                &mut TFTP[2 * i as usize + 1],
                10, // AF_INET6
                17, // IPPROTO_UDP
                HELPER_NAME.as_ptr() as *const c_char,
                TFTP_PORT,
                PORTS[i as usize],
                i as c_int,
                &TFTP_EXP_POLICY,
                0,
                Some(tftp_help),
                ptr::null_mut(),
                THIS_MODULE,
            );

    // Register helpers
    ret = unsafe { nf_conntrack_helpers_register(TFTP.as_mut_ptr(), 2 * PORTS_C as c_int) };
    if ret < 0 {
        pr_err(b"failed to register helpers\0".as_ptr() as *const c_char);
    }

    ret
}

#[unsafe(no_mangle)]
pub extern "C" fn nf_conntrack_tftp_fini() {
    unsafe { nf_conntrack_helpers_unregister(TFTP.as_mut_ptr(), 2 * PORTS_C as c_int) };
}

#[unsafe(no_mangle)]
pub extern "C" fn tftp_help(
    skb: *mut sk_buff,
    protoff: c_uint,
    _ct: *mut nf_conn,
    _ctinfo: c_int,
) -> c_int {
    let mut ret: c_int = 0;
    let mut tfh: *const tftphdr = ptr::null();
    let mut _tftph: tftphdr = tftphdr { opcode: [0; 2] };
    let mut exp: *mut nf_conntrack_expect = ptr::null_mut();
    let mut tuple: *mut nf_conntrack_tuple = ptr::null_mut();
    let mut nf_nat_tftp: Option<NfNatTftpHookFn> = None;

    let tfh = unsafe {
        skb_header_pointer(
            skb,
            protoff + core::mem::size_of::<udphdr>() as c_uint,
            core::mem::size_of::<tftphdr>() as c_uint,
            (&raw mut local_hdr).cast::<c_void>(),
        ) as *mut tftphdr
    };

    if tfh.is_null() {
        return NF_ACCEPT;
    }

    let opcode = unsafe {
        let raw = [(*tfh).opcode[0], (*tfh).opcode[1]];
        ntohs(u16::from_ne_bytes(raw))
    };

    if opcode != TFTP_OPCODE_READ && opcode != TFTP_OPCODE_WRITE {
        return NF_ACCEPT;
    }

            // Allocate expectation
            exp = unsafe { nf_ct_expect_alloc(ct) };
            if exp.is_null() {
                unsafe {
                    nf_ct_helper_log(
                        skb,
                        ct,
                        b"cannot alloc expectation\0".as_ptr() as *const c_char,
                    );
                }
                return NF_DROP;
            }

            // Initialize expectation
            tuple = unsafe { &(*ct).tuplehash[IP_CT_DIR_REPLY].tuple };
            unsafe {
                nf_ct_expect_init(
                    exp,
                    NF_CT_EXPECT_CLASS_DEFAULT,
                    nf_ct_l3num(ct),
                    &(*tuple).src.u3,
                    &(*tuple).dst.u3,
                    17, // IPPROTO_UDP
                    ptr::null_mut(),
                    &(*tuple).dst.udp.port,
                );
            }

            // Log expectation
            unsafe { pr_debug(b"expect: \0".as_ptr() as *const c_char) };
            unsafe { nf_ct_dump_tuple(&(*exp).tuple) };

            // NAT hook
            nf_nat_tftp = unsafe { rcu_dereference(NF_NAT_TFTP_HOOK) };
            if nf_nat_tftp.is_some() && ((*ct).status & IPS_NAT_MASK) != 0 {
                ret = nf_nat_tftp.unwrap()(
                    skb,
                    ctinfo,
                    exp,
                ) as c_int;
            } else if unsafe { nf_ct_expect_related(exp, 0) } != 0 {
                unsafe {
                    nf_ct_helper_log(
                        skb,
                        ct,
                        b"cannot add expectation\0".as_ptr() as *const c_char,
                    );
                }
                ret = NF_DROP;
            }

            // Release expectation
            unsafe { nf_ct_expect_put(exp) };
        }
        TFTP_OPCODE_DATA | TFTP_OPCODE_ACK => {
            unsafe { pr_debug(b"Data/ACK opcode\n\0".as_ptr() as *const c_char) };
        }
        TFTP_OPCODE_ERROR => {
            unsafe { pr_debug(b"Error opcode\n\0".as_ptr() as *const c_char) };
        }
        _ => {
            unsafe { pr_debug(b"Unknown opcode\n\0".as_ptr() as *const c_char) };
        }
    }

    NF_ACCEPT
}

// Helper functions (declared as extern in C)
extern "C" {
    fn skb_header_pointer(
        skb: *mut sk_buff,
        offset: c_uint,
        len: c_uint,
        data: *mut c_void,
    ) -> *mut c_void;

    fn ntohs(x: u16) -> u16;

    fn nf_ct_dump_tuple(tuple: *mut nf_conntrack_tuple);

    fn nf_ct_expect_alloc(ct: *mut nf_conn) -> *mut nf_conntrack_expect;

    fn nf_ct_expect_init(
        exp: *mut nf_conntrack_expect,
        class: c_int,
        l3num: c_int,
        src: *mut nf_conntrack_tuple_union,
        dst: *mut nf_conntrack_tuple_union,
        protonum: c_int,
        l4src: *mut c_void,
        l4dst: *mut nf_conntrack_tuple_union,
    );

    fn rcu_dereference<T>(ptr: *mut T) -> *mut T;

    fn nf_ct_expect_related(exp: *mut nf_conntrack_expect, timeout: c_int) -> c_int;

    fn nf_ct_expect_put(exp: *mut nf_conntrack_expect);

    fn nf_ct_helper_log(
        skb: *mut sk_buff,
        ct: *mut nf_conn,
        msg: *const c_char,
    );

    fn nf_ct_l3num(ct: *mut nf_conn) -> c_int;

    fn nf_conntrack_helpers_register(
        helpers: *mut nf_conntrack_helper,
        count: c_int,
    ) -> c_int;

    fn nf_conntrack_helpers_unregister(
        helpers: *mut nf_conntrack_helper,
        count: c_int,
    );

    fn pr_debug(msg: *const c_char);

    fn pr_err(msg: *const c_char);

    fn nf_ct_helper_init(
        helper: *mut nf_conntrack_helper,
        family: c_int,
        protocol: c_int,
        name: *const c_char,
        port: u16,
        port2: u16,
        index: c_int,
        policy: *const nf_conntrack_expect_policy,
        flags: c_int,
        help: Option<extern "C" fn(*mut sk_buff, c_uint, *mut nf_conn, c_int) -> c_int>,
        destroy: *mut c_void,
        module: *mut c_void,
    );
}

// Constants for return values
pub const NF_ACCEPT: c_int = 0;
pub const NF_DROP: c_int = 1;

// Constants for IP_CT_DIR
pub const IP_CT_DIR_ORIGINAL: c_int = 0;
pub const IP_CT_DIR_REPLY: c_int = 1;

// Constants for NF_CT_EXPECT_CLASS_DEFAULT
pub const NF_CT_EXPECT_CLASS_DEFAULT: c_int = 0;

// Constants for IPS_NAT_MASK
pub const IPS_NAT_MASK: c_int = 0x0000000F;

// Constants for HELPER_NAME
const HELPER_NAME: &str = "tftp";

// Module parameter
#[no_mangle]
pub static mut THIS_MODULE: *mut c_void = ptr::null_mut();

// Expect policy
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_expect_policy {
    max_expected: c_int,
    timeout: c_int,
}

static TFTP_EXP_POLICY: nf_conntrack_expect_policy = nf_conntrack_expect_policy {
    max_expected: 1,
    timeout: 5 * 60,
};

// Static storage for helpers
static mut TFTP: [nf_conntrack_helper; 16] = unsafe { [core::mem::zeroed(); 16] };