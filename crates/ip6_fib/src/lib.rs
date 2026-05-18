#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::c_void;
use core::panic::PanicInfo;
use core::ptr;
use core::sync::atomic::AtomicU32;

pub mod kernel_types {
    pub type c_char = i8;
    pub type c_uchar = u8;
    pub type c_short = i16;
    pub type c_ushort = u16;
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_long = i64;
    pub type c_ulong = u64;
    pub type c_size_t = usize;
    pub type size_t = usize;
    pub type socklen_t = u32;
}

use kernel_types::*;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;
pub const INT_MAX: c_int = 2147483647;
pub const FIB6_TABLE_HASHSZ: usize = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hlist_head {
    pub first: *mut hlist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hlist_node {
    pub next: *mut hlist_node,
}

#[repr(C)]
pub struct net {
    pub ipv6: ipv6_net,
}

#[repr(C)]
pub struct ipv6_net {
    pub fib6_walkers: list_head,
    pub fib6_walker_lock: spinlock_t,
    pub fib6_sernum: AtomicU32,
    pub rt6_stats: *mut rt6_stats,
    pub fib_table_hash: [hlist_head; FIB6_TABLE_HASHSZ],
    pub fib6_main_tbl: *mut fib6_table,
    pub fib6_local_tbl: *mut fib6_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt6_stats {
    pub fib_nodes: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fib6_table {
    pub tb6_hlist: hlist_head,
    pub tb6_id: u32,
    pub tb6_lock: spinlock_t,
    pub tb6_root: fib6_node,
    pub tb6_peers: inetpeer_base,
    pub fib_seq: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inetpeer_base {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib6_node {
    pub fn_sernum: u32,
    pub __child: *mut fib6_node,
    pub __parent: *mut fib6_node,
    pub fn_flags: u32,
    pub tb6_list: list_head,
    pub tb6_list_s: list_head,
    pub tb6_list_l: list_head,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fib6_info {
    pub fib6_node: *mut fib6_node,
    pub fib6_table: *mut fib6_table,
    pub fib6_ref: AtomicU32,
    pub fib6_siblings: list_head,
    pub fib6_metrics: *mut c_void,
    pub fib6_nh: *mut fib6_nh,
    pub nh: *mut nexthop,
    pub fib6_nsiblings: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib6_nh {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nexthop {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib6_walker {
    pub lh: list_head,
    pub net: *mut net,
    pub func: Option<extern "C" fn(*mut fib6_info, *mut c_void) -> c_int>,
    pub sernum: c_int,
    pub arg: *mut c_void,
    pub skip_notify: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib6_cleaner {
    pub w: fib6_walker,
    pub net: *mut net,
    pub func: Option<extern "C" fn(*mut fib6_info, *mut c_void) -> c_int>,
    pub sernum: c_int,
    pub arg: *mut c_void,
    pub skip_notify: bool,
}

#[no_mangle]
pub unsafe extern "C" fn fib6_link_table(_net: *mut net, _tb: *mut fib6_table) {}

#[no_mangle]
pub unsafe extern "C" fn fib6_tables_init(net: *mut net) {
    if !net.is_null() {
        fib6_link_table(net, (*net).ipv6.fib6_main_tbl);
        fib6_link_table(net, (*net).ipv6.fib6_local_tbl);
    }
}

#[no_mangle]
pub unsafe extern "C" fn fib6_alloc_table(_net: *mut net, _id: u32) -> *mut fib6_table {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn fib6_new_table(net: *mut net, mut id: u32) -> *mut fib6_table {
    if id == 0 {
        id = 0x100;
    }

    let mut tb = fib6_get_table(net, id);
    if tb.is_null() {
        tb = fib6_alloc_table(net, id);
        if !tb.is_null() {
            fib6_link_table(net, tb);
        }
    }
    tb
}

#[no_mangle]
pub unsafe extern "C" fn fib6_get_table(net: *mut net, id: u32) -> *mut fib6_table {
    if net.is_null() {
        return ptr::null_mut();
    }

    let hash: usize = (id as usize) & (FIB6_TABLE_HASHSZ - 1);
    let mut node = (*net).ipv6.fib_table_hash[hash].first;

    while !node.is_null() {
        let tb = node as *mut fib6_table;
        if (*tb).tb6_id == id {
            return tb;
        }
        node = (*node).next;
    }

    ptr::null_mut()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}