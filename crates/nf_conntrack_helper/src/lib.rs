#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::all)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;
use kernel_types::*;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;
pub const ENOENT: c_int = -2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_address {
    pub all: u16,
    pub protonum: u8,
    pub _pad: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple {
    pub src: nf_conntrack_tuple_address,
    pub dst: nf_conntrack_tuple_address,
    pub src_l3num: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hlist_node {
    pub next: *mut hlist_node,
    pub pprev: *mut *mut hlist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hlist_head {
    pub first: *mut hlist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_helper {
    pub name: *const c_char,
    pub tuple: nf_conntrack_tuple,
    pub nat_mod_name: *const c_char,
    pub help: *const c_void,
    pub destroy: Option<unsafe extern "C" fn(*mut nf_conn)>,
    pub me: *mut c_void,
    pub refcnt: u32,
    pub hnode: hlist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn_tuple_hash {
    pub tuple: nf_conntrack_tuple,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn {
    pub status: u32,
    pub tuplehash: [nf_conn_tuple_hash; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn_help {
    pub helper: *mut nf_conntrack_helper,
    pub expectations: hlist_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_net {
    pub sysctl_auto_assign_helper: u8,
    pub auto_assign_helper_warned: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_helper_expectfn {
    pub name: *const c_char,
    pub expectfn: *const c_void,
    pub head: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Mutex {
    _priv: u8,
}

static mut nf_ct_helper_hash: *mut hlist_head = ptr::null_mut();
static mut nf_ct_helper_hsize: c_uint = 0;
static mut nf_ct_helper_count: c_uint = 0;
static mut nf_ct_auto_assign_helper: u8 = 0;
static mut nf_ct_nat_helpers: list_head = list_head {
    next: ptr::null_mut(),
    prev: ptr::null_mut(),
};
static mut nf_ct_helper_mutex: Mutex = Mutex { _priv: 0 };
static mut nf_ct_nat_helpers_mutex: Mutex = Mutex { _priv: 0 };

#[inline(always)]
unsafe fn helper_from_hnode(node: *mut hlist_node) -> *mut nf_conntrack_helper {
    let off = core::mem::offset_of!(nf_conntrack_helper, hnode);
    (node as *mut u8).sub(off) as *mut nf_conntrack_helper
}

#[inline(always)]
unsafe fn nf_ct_tuple_src_mask_cmp(
    t1: *const nf_conntrack_tuple,
    t2: *const nf_conntrack_tuple,
    _mask: *const nf_conntrack_tuple,
) -> bool {
    if t1.is_null() || t2.is_null() {
        return false;
    }
    (*t1).src_l3num == (*t2).src_l3num
        && (*t1).src.all == (*t2).src.all
        && (*t1).dst.protonum == (*t2).dst.protonum
}

#[inline(always)]
unsafe fn strcmp(a: *const c_char, b: *const c_char) -> c_int {
    if a.is_null() || b.is_null() {
        return -1;
    }
    let mut i = 0usize;
    loop {
        let ca = *a.add(i);
        let cb = *b.add(i);
        if ca != cb {
            return (ca as c_int) - (cb as c_int);
        }
        if ca == 0 {
            return 0;
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn helper_hash(tuple: *const nf_conntrack_tuple) -> c_uint {
    if tuple.is_null() || nf_ct_helper_hsize == 0 {
        return 0;
    }

    let l3num = (*tuple).src_l3num as c_uint;
    let protonum = (*tuple).dst.protonum as c_uint;
    let src_all = (*tuple).src.all as c_uint;

    (((l3num << 8) | protonum) ^ src_all) % nf_ct_helper_hsize
}

#[no_mangle]
pub unsafe extern "C" fn __nf_ct_helper_find(
    tuple: *const nf_conntrack_tuple,
) -> *mut nf_conntrack_helper {
    if tuple.is_null() || nf_ct_helper_count == 0 || nf_ct_helper_hash.is_null() {
        return ptr::null_mut();
    }

    let h = helper_hash(tuple);
    let head = &mut *nf_ct_helper_hash.add(h as usize);
    let mut node = head.first;

    let mask = nf_conntrack_tuple {
        src: nf_conntrack_tuple_address {
            all: u16::MAX,
            protonum: u8::MAX,
            _pad: 0,
        },
        dst: nf_conntrack_tuple_address {
            all: u16::MAX,
            protonum: u8::MAX,
            _pad: 0,
        },
        src_l3num: u16::MAX,
    };

    while !node.is_null() {
        let helper = helper_from_hnode(node);
        if nf_ct_tuple_src_mask_cmp(tuple, &(*helper).tuple, &mask) {
            return helper;
        }
        node = (*node).next;
    }

    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn __nf_conntrack_helper_find(
    name: *const c_char,
    l3num: u16,
    protonum: u8,
) -> *mut nf_conntrack_helper {
    if name.is_null() || nf_ct_helper_count == 0 || nf_ct_helper_hash.is_null() {
        return ptr::null_mut();
    }

    let mut i: c_uint = 0;
    while i < nf_ct_helper_hsize {
        let head = &mut *nf_ct_helper_hash.add(i as usize);
        let mut node = head.first;

        while !node.is_null() {
            let helper = helper_from_hnode(node);
            if !helper.is_null()
                && (*helper).tuple.src_l3num == l3num
                && (*helper).tuple.dst.protonum == protonum
                && strcmp((*helper).name, name) == 0
            {
                return helper;
            }
            node = (*node).next;
        }

        i += 1;
    }

    ptr::null_mut()
}