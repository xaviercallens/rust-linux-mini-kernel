#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::{mem, ptr};
use kernel_types::*;

pub type size_t = usize;
pub type c_size_t = usize;
pub type socklen_t = u32;

// Constants from C
pub const IPPROTO_TCP: c_int = 6;
pub const NF_ACCEPT: c_int = 1;
pub const NF_DROP: c_int = 2;
pub const IP_CT_DIR_ORIGINAL: c_int = 0;
pub const IP_CT_DIR_REPLY: c_int = 1;
pub const IP_CT_ESTABLISHED: c_int = 1 << 1;
pub const IP_CT_ESTABLISHED_REPLY: c_int = IP_CT_ESTABLISHED | (1 << 2);
pub const fn CTINFO2DIR(ctinfo: c_int) -> c_int {
    (ctinfo >> 1) & 1
}
pub const SANE_PORT: u16 = 6566;

// SANE protocol constants
pub const SANE_NET_START: u32 = 7;
pub const SANE_STATUS_SUCCESS: u32 = 0;

// Error codes
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;

// Missing kernel FFI opaque types
#[repr(C)]
pub struct nf_conntrack_expect_policy {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct nf_conntrack_expect {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct nf_conntrack_tuple {
    _priv: [u8; 0],
}

// C struct translations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcphdr {
    pub source: u16,
    pub dest: u16,
    pub doff: u8,
    pub _pad: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sane_request {
    pub RPC_code: u32,
    pub handle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sane_reply_net_start {
    pub status: u32,
    pub zero: u16,
    pub port: u16,
}

// Helper data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_sane_master {
    pub state: c_int,
}

// Opaque helper type
#[repr(C)]
pub struct nf_conntrack_helper {
    _priv: [u8; 0],
}

// Extern declarations for kernel functions
unsafe extern "C" {
    fn skb_header_pointer(
        skb: *mut sk_buff,
        offset: c_int,
        size: c_int,
        buffer: *mut c_void,
    ) -> *mut c_void;
    fn nf_ct_expect_alloc(ct: *mut nf_conn) -> *mut nf_conntrack_expect;
    fn nf_ct_expect_init(
        exp: *mut nf_conntrack_expect,
        class: c_int,
        l3num: c_int,
        src: *mut c_void,
        dst: *mut c_void,
        protonum: c_int,
        mask: *mut c_void,
        port: *mut u16,
    );
    fn nf_ct_expect_related(exp: *mut nf_conntrack_expect, strict: c_int) -> c_int;
    fn nf_ct_expect_put(exp: *mut nf_conntrack_expect);
    fn nf_conntrack_helpers_register(helpers: *mut nf_conntrack_helper, count: c_int) -> c_int;
    fn nf_conntrack_helpers_unregister(helpers: *mut nf_conntrack_helper, count: c_int);
    fn nf_ct_l3num(ct: *mut nf_conn) -> c_int;
    fn nf_ct_help_data(ct: *mut nf_conn) -> *mut nf_ct_sane_master;
    fn nf_ct_dump_tuple(tuple: *mut nf_conntrack_tuple);
    fn nf_ct_helper_log(skb: *mut sk_buff, ct: *mut nf_conn, msg: *const c_char);
    fn spin_lock_bh(lock: *mut c_void);
    fn spin_unlock_bh(lock: *mut c_void);
}

// Module parameters
static mut ports: [u16; 8] = [0; 8];
static mut ports_c: c_int = 0;

// Spinlock and buffer
static mut nf_sane_lock: *mut c_void = ptr::null_mut();
static mut sane_buffer: *mut c_void = ptr::null_mut();

// Helper array (opaque pointers are sufficient for this unit)
static mut sane: [*mut nf_conntrack_helper; 16] = [ptr::null_mut(); 16];

// Helper function
#[unsafe(no_mangle)]
pub unsafe extern "C" fn help(
    skb: *mut sk_buff,
    protoff: c_uint,
    _ct: *mut nf_conn,
    ctinfo: c_int,
) -> c_int {
    let dir: c_int = CTINFO2DIR(ctinfo);

    if ctinfo != IP_CT_ESTABLISHED && ctinfo != IP_CT_ESTABLISHED_REPLY {
        return NF_ACCEPT;
    }

    let mut th: tcphdr = unsafe { mem::zeroed() };
    let th_ptr: *mut c_void = &mut th as *mut _ as *mut c_void;
    let th_result = unsafe {
        skb_header_pointer(
            skb,
            protoff as c_int,
            mem::size_of::<tcphdr>() as c_int,
            th_ptr,
        )
    };
    if th_result.is_null() {
        return NF_ACCEPT;
    }

    let dataoff: c_int = protoff as c_int + (th.doff as c_int) * 4;
    let skb_len: c_int = unsafe { (*skb).len as c_int };
    if dataoff >= skb_len {
        return NF_ACCEPT;
    }

    let datalen: c_int = skb_len - dataoff;

    unsafe { spin_lock_bh(nf_sane_lock) };
    let sb_ptr = unsafe { skb_header_pointer(skb, dataoff, datalen, sane_buffer) };
    if sb_ptr.is_null() {
        unsafe { spin_unlock_bh(nf_sane_lock) };
        return NF_ACCEPT;
    }

    if dir == IP_CT_DIR_ORIGINAL {
        if datalen != mem::size_of::<sane_request>() as c_int {
            unsafe { spin_unlock_bh(nf_sane_lock) };
            return NF_ACCEPT;
        }
    } else if datalen != mem::size_of::<sane_reply_net_start>() as c_int {
        unsafe { spin_unlock_bh(nf_sane_lock) };
        return NF_ACCEPT;
    }

    unsafe { spin_unlock_bh(nf_sane_lock) };
    NF_ACCEPT
}