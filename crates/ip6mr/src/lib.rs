#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_int, c_void};
use core::mem::size_of;
use core::ptr;
use kernel_types::*;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

pub type size_t = usize;
pub type c_size_t = usize;
pub type socklen_t = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_rules_ops {
    _priv: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timer_list {
    _priv: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rhltable {
    _priv: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rhashtable_compare_arg {
    pub key: *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rhashtable_params {
    pub head_offset: usize,
    pub key_offset: usize,
    pub key_len: usize,
    pub nelem_hint: usize,
    pub obj_cmpfn:
        Option<unsafe extern "C" fn(arg: *const rhashtable_compare_arg, ptr: *const c_void) -> c_int>,
    pub automatic_shrinking: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mfc6_cache_cmp_arg {
    pub mf6c_origin: in6_addr,
    pub mf6c_mcastgrp: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mfc6_cache {
    pub mf6c_origin: in6_addr,
    pub mf6c_mcastgrp: in6_addr,
    pub cmparg: mfc6_cache_cmp_arg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mr_table_ops {
    pub rht_params: *const rhashtable_params,
    pub cmparg_any: *const mfc6_cache_cmp_arg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mr_table {
    pub list: list_head,
    pub id: u32,
    pub mfc_hash: rhltable,
    pub ipmr_expire_timer: timer_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_net {
    #[cfg(CONFIG_IPV6_MROUTE_MULTIPLE_TABLES)]
    pub mr6_tables: list_head,
    #[cfg(not(CONFIG_IPV6_MROUTE_MULTIPLE_TABLES))]
    pub mrt6: *mut mr_table,
    pub mr6_rules_ops: *mut fib_rules_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net {
    pub ipv6: ipv6_net,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flowi6 {
    pub daddr: in6_addr,
    pub saddr: in6_addr,
}

unsafe extern "C" {
    fn ip6mr_get_table(net: *const net, id: u32) -> *mut mr_table;
    fn mr_table_alloc(
        net: *const net,
        id: u32,
        ops: *const mr_table_ops,
        expire_cb: Option<unsafe extern "C" fn(*mut timer_list)>,
        new_table_set: Option<unsafe extern "C" fn(*mut net, *mut mr_table)>,
    ) -> *mut mr_table;
    fn ipmr_expire_process(t: *mut timer_list);
    fn ip6mr_new_table_set(net: *mut net, mrt: *mut mr_table);

    fn del_timer_sync(timer: *mut timer_list) -> c_int;
    fn mroute_clean_tables(mrt: *mut mr_table, flags: u32);
    fn rhltable_destroy(ht: *mut rhltable);
    fn free(p: *mut c_void);
}

pub const MRT6_FLUSH_MIFS: u32 = 0x0001;
pub const MRT6_FLUSH_MIFS_STATIC: u32 = 0x0002;
pub const MRT6_FLUSH_MFC: u32 = 0x0004;
pub const MRT6_FLUSH_MFC_STATIC: u32 = 0x0008;

static mut mrt_cachep: *mut c_void = ptr::null_mut();

#[cfg(CONFIG_IPV6_MROUTE_MULTIPLE_TABLES)]
static mut IP6MR_CMPARG_ANY: mfc6_cache_cmp_arg = mfc6_cache_cmp_arg {
    mf6c_origin: in6_addr {
        in6_u: in6_addr_union { u6_addr8: [0; 16] },
    },
    mf6c_mcastgrp: in6_addr {
        in6_u: in6_addr_union { u6_addr8: [0; 16] },
    },
};

static mut IP6MR_RHT_PARAMS: rhashtable_params = rhashtable_params {
    head_offset: 0,
    key_offset: 0,
    key_len: size_of::<mfc6_cache_cmp_arg>(),
    nelem_hint: 3,
    obj_cmpfn: Some(ip6mr_hash_cmp),
    automatic_shrinking: true,
};

#[cfg(CONFIG_IPV6_MROUTE_MULTIPLE_TABLES)]
static mut IP6MR_TABLE_OPS: mr_table_ops = mr_table_ops {
    rht_params: core::ptr::addr_of!(IP6MR_RHT_PARAMS),
    cmparg_any: core::ptr::addr_of!(IP6MR_CMPARG_ANY),
};

#[cfg(not(CONFIG_IPV6_MROUTE_MULTIPLE_TABLES))]
static mut IP6MR_TABLE_OPS: mr_table_ops = mr_table_ops {
    rht_params: core::ptr::addr_of!(IP6MR_RHT_PARAMS),
    cmparg_any: ptr::null(),
};

#[no_mangle]
pub unsafe extern "C" fn ipv6_addr_equal(a: *const in6_addr, b: *const in6_addr) -> bool {
    let aa = (*a).in6_u.u6_addr8;
    let bb = (*b).in6_u.u6_addr8;
    aa == bb
}

#[no_mangle]
pub unsafe extern "C" fn ip6mr_hash_cmp(
    arg: *const rhashtable_compare_arg,
    ptr_obj: *const c_void,
) -> c_int {
    let cmparg = (*arg).key as *const mfc6_cache_cmp_arg;
    let c = ptr_obj as *const mfc6_cache;

    if !ipv6_addr_equal(
        core::ptr::addr_of!((*c).mf6c_origin),
        core::ptr::addr_of!((*cmparg).mf6c_origin),
    ) || !ipv6_addr_equal(
        core::ptr::addr_of!((*c).mf6c_mcastgrp),
        core::ptr::addr_of!((*cmparg).mf6c_mcastgrp),
    ) {
        return 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn ip6mr_new_table(netp: *mut net, id: u32) -> *mut mr_table {
    let existing = ip6mr_get_table(netp as *const net, id);
    if !existing.is_null() {
        return existing;
    }

    mr_table_alloc(
        netp as *const net,
        id,
        core::ptr::addr_of!(IP6MR_TABLE_OPS),
        Some(ipmr_expire_process),
        Some(ip6mr_new_table_set),
    )
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}