#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ptr;
use kernel_types::{c_int, c_uint, c_void};

pub const MAX_STAT_DEPTH: c_int = 32;
pub const KEYLENGTH: c_int = 8 * 32;
pub const KEY_MAX: c_uint = !0;
pub const halve_threshold: c_int = 25;
pub const inflate_threshold: c_int = 50;
pub const halve_threshold_root: c_int = 15;
pub const inflate_threshold_root: c_int = 30;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_head {
    pub next: *mut rcu_head,
    pub func: Option<unsafe extern "C" fn(*mut rcu_head)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hlist_head {
    pub first: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_alias {
    pub rcu: rcu_head,
    pub fa_tos: c_uint,
    pub fa_type: c_uint,
    pub tb_id: c_uint,
    pub fa_info: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_vector {
    pub key: c_uint,
    pub pos: u8,
    pub bits: u8,
    pub slen: u8,
    pub pad: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tnode {
    pub rcu: rcu_head,
    pub empty_children: c_uint,
    pub full_children: c_uint,
    pub parent: *mut key_vector,
    pub kv: key_vector,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trie {
    pub kv: key_vector,
}

#[no_mangle]
pub static mut tnode_free_size: usize = 0;

#[inline(always)]
unsafe fn tnode_from_kv(kv: *mut key_vector) -> *mut tnode {
    kv.cast::<u8>()
        .sub(core::mem::offset_of!(tnode, kv))
        .cast::<tnode>()
}

#[no_mangle]
pub unsafe extern "C" fn get_index(key: c_uint, kv: *mut key_vector) -> c_uint {
    if kv.is_null() {
        return 0;
    }
    let index = key ^ (*kv).key;
    if (core::mem::size_of::<c_uint>() * 8 <= KEYLENGTH as usize) && (KEYLENGTH == (*kv).pos as c_int)
    {
        0
    } else {
        index >> ((*kv).pos as c_uint)
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_cindex(key: c_uint, kv: *mut key_vector) -> c_uint {
    if kv.is_null() {
        return 0;
    }
    (key ^ (*kv).key) >> ((*kv).pos as c_uint)
}

#[no_mangle]
pub unsafe extern "C" fn node_set_parent(n: *mut key_vector, tp: *mut key_vector) {
    if n.is_null() {
        return;
    }
    let n_info = tnode_from_kv(n);
    (*n_info).parent = tp;
}

#[no_mangle]
pub unsafe extern "C" fn node_parent_rcu(tn: *mut key_vector) -> *mut key_vector {
    if tn.is_null() {
        return ptr::null_mut();
    }
    let tn_info = tnode_from_kv(tn);
    (*tn_info).parent
}

#[no_mangle]
pub unsafe extern "C" fn get_child_rcu(_tn: *mut key_vector, _i: c_int) -> *mut key_vector {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn resize(_t: *mut trie, _tn: *mut key_vector) -> *mut key_vector {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn __node_free_rcu(_head: *mut rcu_head) {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}