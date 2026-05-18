// SPDX-License-Identifier: GPL-2.0-or-later
#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use core::ffi::{c_char, c_int};
use core::panic::PanicInfo;
use kernel_types::*;

const NF_CT_EXT_TSTAMP: u32 = 0;

static mut nf_ct_tstamp: bool = false;

#[repr(C)]
struct nf_conn_tstamp {
    start: u64,
    stop: u64,
}

#[repr(C)]
struct nf_ct_ext_type {
    len: u32,
    align: u32,
    id: u32,
}

static TSTAMP_EXTEND: nf_ct_ext_type = nf_ct_ext_type {
    len: core::mem::size_of::<nf_conn_tstamp>() as u32,
    align: core::mem::align_of::<nf_conn_tstamp>() as u32,
    id: NF_CT_EXT_TSTAMP,
};

unsafe extern "C" {
    fn nf_ct_extend_register(ext: *const nf_ct_ext_type) -> c_int;
    fn nf_ct_extend_unregister(ext: *const nf_ct_ext_type);
    fn pr_err(fmt: *const c_char);
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_tstamp_pernet_init(netns: *mut net) -> c_int {
    (*netns).ct.sysctl_tstamp = nf_ct_tstamp;
    0
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_tstamp_init() -> c_int {
    let ret = nf_ct_extend_register(&TSTAMP_EXTEND as *const nf_ct_ext_type);
    if ret < 0 {
        pr_err(b"Unable to register extension\n\0".as_ptr() as *const c_char);
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_tstamp_fini() {
    nf_ct_extend_unregister(&TSTAMP_EXTEND as *const nf_ct_ext_type);
}