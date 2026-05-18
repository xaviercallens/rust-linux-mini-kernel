#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clashing_extern_declarations)]

use core::ffi::{c_int, c_uint, c_ulong, c_void};
use core::panic::PanicInfo;
use kernel_types::*;

pub const CONNCOUNT_SLOTS: usize = 256;
pub const CONNCOUNT_GC_MAX_NODES: c_uint = 8;
pub const MAX_KEYLEN: usize = 5;
pub const IPPROTO_TCP: c_int = 6;
pub const TCP_CONNTRACK_TIME_WAIT: c_int = 12;
pub const TCP_CONNTRACK_CLOSE: c_int = 13;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const EAGAIN: c_int = -11;
pub const ENOENT: c_int = -2;
pub const EOVERFLOW: c_int = -75;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_node {
    pub rb_parent_color: c_ulong,
    pub rb_left: *mut rb_node,
    pub rb_right: *mut rb_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_zone {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_hash {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conncount_tuple {
    pub node: list_head,
    pub tuple: nf_conntrack_tuple,
    pub zone: nf_conntrack_zone,
    pub cpu: c_int,
    pub jiffies32: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conncount_list {
    pub list_lock: *mut c_void,
    pub head: list_head,
    pub count: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conncount_rb {
    pub node: rb_node,
    pub list: nf_conncount_list,
    pub key: [u32; MAX_KEYLEN],
    pub rcu_head: *mut c_void,
}

const BITS_PER_CULONG: usize = core::mem::size_of::<c_ulong>() * 8;
const PENDING_TREES_LEN: usize = CONNCOUNT_SLOTS.div_ceil(BITS_PER_CULONG);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conncount_data {
    pub keylen: c_uint,
    pub root: [rb_root; CONNCOUNT_SLOTS],
    pub net: *mut c_void,
    pub gc_work: *mut c_void,
    pub pending_trees: [c_ulong; PENDING_TREES_LEN],
    pub gc_tree: c_uint,
}

unsafe extern "C" {
    fn nf_ct_protonum(conn: *const nf_conn) -> c_int;
    fn nf_ct_tcp_state(conn: *const nf_conn) -> c_int;
    fn nf_conntrack_find_get(
        net: *mut c_void,
        zone: *const nf_conntrack_zone,
        tuple: *const nf_conntrack_tuple,
    ) -> *const nf_conntrack_tuple_hash;
    fn nf_ct_tuple_equal(a: *const nf_conntrack_tuple, b: *const nf_conntrack_tuple) -> c_int;
    fn nf_ct_zone_id(zone: *const nf_conntrack_zone, dir: c_int) -> c_int;
    fn nf_ct_zone_equal(a: *const nf_conn, zone: *const nf_conntrack_zone, dir: c_int) -> c_int;
    fn nf_ct_tuplehash_to_ctrack(h: *const nf_conntrack_tuple_hash) -> *mut nf_conn;
    fn nf_ct_put(ct: *mut nf_conn);
    fn kmem_cache_alloc(cachep: *mut c_void, flags: c_uint) -> *mut c_void;
    fn kmem_cache_free(cachep: *mut c_void, objp: *mut c_void);
    fn jiffies() -> c_ulong;
    fn raw_smp_processor_id() -> c_int;
    fn spin_lock_bh(lock: *mut c_void);
    fn spin_unlock_bh(lock: *mut c_void);
    fn spin_trylock(lock: *mut c_void) -> c_int;
    fn spin_unlock(lock: *mut c_void);
    fn list_del(pos: *mut list_head);
    fn list_add_tail(pos: *mut list_head, head: *mut list_head);
    fn list_add(pos: *mut list_head, head: *mut list_head);
    fn call_rcu(head: *mut c_void, func: *mut c_void);
    fn schedule_work(work: *mut c_void);
    fn set_bit(nr: c_ulong, addr: *mut c_ulong);
}

#[unsafe(no_mangle)]
pub static mut conncount_rb_cachep: *mut c_void = core::ptr::null_mut();
#[unsafe(no_mangle)]
pub static mut conncount_conn_cachep: *mut c_void = core::ptr::null_mut();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn already_closed(conn: *const nf_conn) -> c_int {
    if unsafe { nf_ct_protonum(conn) } == IPPROTO_TCP {
        let state = unsafe { nf_ct_tcp_state(conn) };
        ((state == TCP_CONNTRACK_TIME_WAIT) || (state == TCP_CONNTRACK_CLOSE)) as c_int
    } else {
        0
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}