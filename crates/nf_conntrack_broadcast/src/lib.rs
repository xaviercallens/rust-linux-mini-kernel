#![no_std]

use core::ffi::{c_int, c_uint, c_void};
use kernel_types::*;

pub type size_t = usize;
pub type c_size_t = usize;
pub type socklen_t = u32;

#[repr(C)]
pub struct nf_conn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nf_hook_state {
    pub hook: c_uint,
    pub pf: u8,
    pub in_dev: *mut c_void,
    pub out_dev: *mut c_void,
    pub sk: *mut c_void,
    pub net: *mut net,
    pub okfn: Option<unsafe extern "C" fn(*mut net, *mut c_void, *mut sk_buff) -> c_int>,
}

unsafe extern "C" {
    fn nf_ct_get(skb: *mut sk_buff, ctinfo: *mut c_uint) -> *mut nf_conn;
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_broadcast(
    skb: *mut sk_buff,
    state: *const nf_hook_state,
) -> c_uint {
    if skb.is_null() || state.is_null() {
        return 0;
    }

    let mut ctinfo: c_uint = 0;
    let ct = unsafe { nf_ct_get(skb, &mut ctinfo as *mut c_uint) };

    if ct.is_null() {
        return 0;
    }

    let _ = state;
    let _ = ctinfo;
    1
}