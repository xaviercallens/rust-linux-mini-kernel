
#![no_std]
#![no_main]
#![allow(non_camel_case_types)]

use core::ptr;
use core::ffi::{c_int, c_char, c_void};
use kernel_types::*;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;
pub const NF_CT_EXT_ACCT: u32 = 1;

#[repr(C)]
struct nf_conn_acct {
    _priv: [u8; 0],
}

#[repr(C)]
struct nf_ct_ext_type {
    len: usize,
    align: usize,
    id: u32,
}

#[repr(C)]
struct net {
    ct: net_ct,
}

#[repr(C)]
struct net_ct {
    sysctl_acct: u8,
}

static mut NF_CT_ACCT: u8 = 0;

// FFI-compatible static variables
pub static mut __UDP_DISCONNECT: extern "C" fn(*mut c_void) -> c_int = unsafe { core::mem::zeroed() };
pub static mut ICMPV6_ERR_CONVERT: extern "C" fn(*mut c_void) -> c_int = unsafe { core::mem::zeroed() };
pub static mut INET6_SOCKRAW_OPS: *mut core::ffi::c_void = core::ptr::null_mut();
pub static mut IP6_DATAGRAM_CONNECT_V6_ONLY: extern "C" fn(*mut c_void) -> c_int = unsafe { core::mem::zeroed() };
pub static mut IP6_DATAGRAM_RECV_COMMON_CTL: extern "C" fn(*mut c_void) -> c_int = unsafe { core::mem::zeroed() };

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_acct_pernet_init(net: *mut net) {
    if !net.is_null() {
        (*net).ct.sysctl_acct = NF_CT_ACCT;
    }
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_acct_init() -> c_int {
    let acct_extend = nf_ct_ext_type {
        len: core::mem::size_of::<nf_conn_acct>(),
        align: core::mem::align_of::<nf_conn_acct>(),
        id: NF_CT_EXT_ACCT,
    };

    let ret = nf_ct_extend_register(&acct_extend as *const nf_ct_ext_type);

    if ret < 0 {
        pr_err(b"Unable to register extension\n\0".as_ptr() as *const c_char);
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_acct_fini() {
    let acct_extend = nf_ct_ext_type {
        len: core::mem::size_of::<nf_conn_acct>(),
        align: core::mem::align_of::<nf_conn_acct>(),
        id: NF_CT_EXT_ACCT,
    };

    nf_ct_extend_unregister(&acct_extend as *const nf_ct_ext_type);
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}