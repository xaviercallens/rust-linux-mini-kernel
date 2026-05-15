//! Socket Diagnostic Interface for Linux Kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::transmutes)]

use core::ptr;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_void;
use core::ffi::size_t;
use core::mem;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;
pub const ENOENT: c_int = -2;
pub const EOPNOTSUPP: c_int = -95;
pub const EPERM: c_int = -1;
pub const ESTALE: c_int = -116;

// Type definitions
#[repr(C)]
pub struct sock_diag_handler {
    pub family: c_int,
    pub dump: extern "C" fn(skb: *mut c_void, nlh: *mut c_void) -> c_int,
    pub destroy: extern "C" fn(skb: *mut c_void, nlh: *mut c_void) -> c_int,
}

#[repr(C)]
pub struct sock_diag_req {
    pub sdiag_family: c_int,
}

#[repr(C)]
pub struct sock_diag_msg {
    pub sdiag_family: c_int,
    pub sdiag_protocol: c_int,
    pub sdiag_type: c_int,
    pub sdiag_state: c_int,
    pub sdiag_ino: u32,
    pub sdiag_cookie: [u32; 2],
}

#[repr(C)]
pub struct broadcast_sk {
    pub sk: *mut c_void,
    pub work: work_struct,
}

#[repr(C)]
pub struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net {
    diag_nlsk: *mut c_void,
    user_ns: *mut c_void,
}

#[repr(C)]
pub struct netlink_kernel_cfg {
    groups: c_int,
    input: extern "C" fn(skb: *mut c_void),
    bind: extern "C" fn(net: *mut net, group: c_int),
    flags: c_int,
}

#[repr(C)]
pub struct pernet_operations {
    init: extern "C" fn(net: *mut net) -> c_int,
    exit: extern "C" fn(net: *mut net),
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn sock_diag_check_cookie(
    sk: *mut c_void,
    cookie: *const u32,
) -> c_int {
    if cookie.is_null() {
        return EINVAL;
    }

    // SAFETY: Caller guarantees cookie is valid
    let cookie0 = *cookie;
    let cookie1 = *cookie.offset(1);
    
    if cookie0 == INET_DIAG_NOCOOKIE && cookie1 == INET_DIAG_NOCOOKIE {
        return 0;
    }

    // SAFETY: Caller guarantees sk is valid
    let res = sock_gen_cookie(sk);
    
    if (res as u32) != cookie0 || (res >> 32) as u32 != cookie1 {
        return ESTALE;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn sock_diag_save_cookie(
    sk: *mut c_void,
    cookie: *mut u32,
) {
    if cookie.is_null() {
        return;
    }

    // SAFETY: Caller guarantees sk is valid
    let res = sock_gen_cookie(sk);
    
    // SAFETY: Caller guarantees cookie is valid
    *cookie = res as u32;
    *cookie.offset(1) = (res >> 32) as u32;
}

#[no_mangle]
pub unsafe extern "C" fn sock_diag_put_meminfo(
    sk: *mut c_void,
    skb: *mut c_void,
    attrtype: c_int,
) -> c_int {
    let mut mem: [u32; SK_MEMINFO_VARS] = [0; SK_MEMINFO_VARS];
    
    // SAFETY: Caller guarantees sk is valid
    sk_get_meminfo(sk, mem.as_mut_ptr());
    
    // SAFETY: Caller guarantees skb is valid
    nla_put(skb, attrtype, mem.len() as size_t, mem.as_ptr() as *const c_void)
}

#[no_mangle]
pub unsafe extern "C" fn sock_diag_put_filterinfo(
    may_report_filterinfo: c_int,
    sk: *mut c_void,
    skb: *mut c_void,
    attrtype: c_int,
) -> c_int {
    if may_report_filterinfo == 0 {
        nla_reserve(skb, attrtype, 0);
        return 0;
    }

    let mut err = 0;
    
    rcu_read_lock();
    
    // SAFETY: Caller guarantees sk is valid
    let filter = rcu_dereference(sk, sk_filter_offset);
    
    if filter.is_null() {
        rcu_read_unlock();
        return 0;
    }
    
    let fprog = (*filter).prog.orig_prog;
    
    if fprog.is_null() {
        rcu_read_unlock();
        return 0;
    }
    
    let flen = bpf_classic_proglen(fprog);
    
    let attr = nla_reserve(skb, attrtype, flen);
    
    if attr.is_null() {
        err = ENOMEM;
        rcu_read_unlock();
        return err;
    }
    
    // SAFETY: Caller guarantees fprog is valid
    memcpy(nla_data(attr), fprog.filter, flen);
    
    rcu_read_unlock();
    err
}

#[no_mangle]
pub unsafe extern "C" fn sock_diag_register_inet_compat(
    fn_ptr: extern "C" fn(skb: *mut c_void, nlh: *mut c_void) -> c_int,
) {
    mutex_lock(&sock_diag_table_mutex);
    inet_rcv_compat = fn_ptr;
    mutex_unlock(&sock_diag_table_mutex);
}

#[no_mangle]
pub unsafe extern "C" fn sock_diag_unregister_inet_compat(
    fn_ptr: extern "C" fn(skb: *mut c_void, nlh: *mut c_void) -> c_int,
) {
    mutex_lock(&sock_diag_table_mutex);
    inet_rcv_compat = None;
    mutex_unlock(&sock_diag_table_mutex);
}

#[no_mangle]
pub unsafe extern "C" fn sock_diag_register(
    hndl: *const sock_diag_handler,
) -> c_int {
    if hndl.is_null() {
        return EINVAL;
    }
    
    let family = (*hndl).family;
    
    if family >= AF_MAX {
        return EINVAL;
    }
    
    mutex_lock(&sock_diag_table_mutex);
    
    if !sock_diag_handlers[family as usize].is_null() {
        mutex_unlock(&sock_diag_table_mutex);
        return EBUSY;
    }
    
    sock_diag_handlers[family as usize] = hndl;
    mutex_unlock(&sock_diag_table_mutex);
    
    0
}

#[no_mangle]
pub unsafe extern "C" fn sock_diag_unregister(
    hnld: *const sock_diag_handler,
) {
    if hnld.is_null() {
        return;
    }
    
    let family = (*hnld).family;
    
    if family >= AF_MAX {
        return;
    }
    
    mutex_lock(&sock_diag_table_mutex);
    
    // SAFETY: Caller guarantees handler is registered
    assert!(!sock_diag_handlers[family as usize].is_null());
    
    sock_diag_handlers[family as usize] = ptr::null();
    mutex_unlock(&sock_diag_table_mutex);
}

#[no_mangle]
pub unsafe extern "C" fn sock_diag_broadcast_destroy(
    sk: *mut c_void,
) {
    let bsk = kmalloc(mem::size_of::<broadcast_sk>() as size_t, GFP_ATOMIC);
    
    if bsk.is_null() {
        sk_destruct(sk);
        return;
    }
    
    // SAFETY: Caller guarantees sk is valid
    (*bsk).sk = sk;
    INIT_WORK(&(*bsk).work, sock_diag_broadcast_destroy_work);
    queue_work(broadcast_wq, &(*bsk).work);
}

#[no_mangle]
pub unsafe extern "C" fn sock_diag_destroy(
    sk: *mut c_void,
    err: c_int,
) -> c_int {
    if !ns_capable((*sk).net, CAP_NET_ADMIN) {
        return EPERM;
    }
    
    if (*sk).sk_prot.diag_destroy.is_null() {
        return EOPNOTSUPP;
    }
    
    // SAFETY: Caller guarantees sk_prot is valid
    (*(*sk).sk_prot).diag_destroy(sk, err)
}

#[no_mangle]
pub unsafe extern "C" fn sock_diag_rcv(
    skb: *mut c_void,
) {
    mutex_lock(&sock_diag_mutex);
    netlink_rcv_skb(skb, sock_diag_rcv_msg);
    mutex_unlock(&sock_diag_mutex);
}

#[no_mangle]
pub unsafe extern "C" fn diag_net_init(
    net: *mut net,
) -> c_int {
    let mut cfg: netlink_kernel_cfg = mem::zeroed();
    cfg.groups = SKNLGRP_MAX;
    cfg.input = sock_diag_rcv;
    cfg.bind = sock_diag_bind;
    cfg.flags = NL_CFG_F_NONROOT_RECV;
    
    (*net).diag_nlsk = netlink_kernel_create(net, NETLINK_SOCK_DIAG, &cfg);
    
    if (*net).diag_nlsk.is_null() {
        -ENOMEM
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn diag_net_exit(
    net: *mut net,
) {
    netlink_kernel_release((*net).diag_nlsk);
    (*net).diag_nlsk = ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn sock_diag_init() -> c_int {
    broadcast_wq = alloc_workqueue("sock_diag_events", 0, 0);
    
    if broadcast_wq.is_null() {
        -ENOMEM
    } else {
        register_pernet_subsys(&diag_net_ops)
    }
}

// Internal functions
unsafe fn sock_diag_rcv_msg(
    skb: *mut c_void,
    nlh: *mut c_void,
    extack: *mut c_void,
) -> c_int {
    let msg_type = (*nlh).nlmsg_type;
    
    match msg_type {
        TCPDIAG_GETSOCK | DCCPDIAG_GETSOCK => {
            if inet_rcv_compat.is_null() {
                sock_load_diag_module(AF_INET, 0);
            }
            
            mutex_lock(&sock_diag_table_mutex);
            
            if !inet_rcv_compat.is_null() {
                let ret = (*inet_rcv_compat)(skb, nlh);
                mutex_unlock(&sock_diag_table_mutex);
                ret
            } else {
                mutex_unlock(&sock_diag_table_mutex);
                -EOPNOTSUPP
            }
        },
        SOCK_DIAG_BY_FAMILY | SOCK_DESTROY => {
            __sock_diag_cmd(skb, nlh)
        },
        _ => -EINVAL,
    }
}

unsafe fn __sock_diag_cmd(
    skb: *mut c_void,
    nlh: *mut c_void,
) -> c_int {
    let req = nlmsg_data(nlh);
    
    if nlmsg_len(nlh) < mem::size_of::<sock_diag_req>() as c_int {
        return -EINVAL;
    }
    
    let family = (*req).sdiag_family;
    
    if family >= AF_MAX {
        return -EINVAL;
    }
    
    // SAFETY: family is within bounds
    let family = array_index_nospec(family, AF_MAX);
    
    if sock_diag_handlers[family as usize].is_null() {
        sock_load_diag_module(family, 0);
    }
    
    mutex_lock(&sock_diag_table_mutex);
    
    let hndl = sock_diag_handlers[family as usize];
    
    if hndl.is_null() {
        mutex_unlock(&sock_diag_table_mutex);
        return -ENOENT;
    }
    
    let msg_type = (*nlh).nlmsg_type;
    
    if msg_type == SOCK_DIAG_BY_FAMILY {
        // SAFETY: hndl is valid
        let ret = (*(*hndl).dump)(skb, nlh);
        mutex_unlock(&sock_diag_table_mutex);
        ret
    } else if msg_type == SOCK_DESTROY && !(*hndl).destroy.is_null() {
        // SAFETY: hndl is valid
        let ret = (*(*hndl).destroy)(skb, nlh);
        mutex_unlock(&sock_diag_table_mutex);
        ret
    } else {
        mutex_unlock(&sock_diag_table_mutex);
        -EOPNOTSUPP
    }
}

// Static variables
static mut sock_diag_handlers: [*const sock_diag_handler; AF_MAX as usize] = [ptr::null(); AF_MAX as usize];
static mut inet_rcv_compat: Option<extern "C" fn(skb: *mut c_void, nlh: *mut c_void) -> c_int> = None;
static mut sock_diag_table_mutex: mutex_t = mutex_init();
static mut broadcast_wq: *mut workqueue_struct = ptr::null_mut();
static mut sock_cookie: cookie_t = cookie_init();
static mut sock_diag_mutex: mutex_t = mutex_init();
static mut diag_net_ops: pernet_operations = pernet_operations {
    init: diag_net_init,
    exit: diag_net_exit,
};

// Extern functions
extern "C" {
    fn kmalloc(size: size_t, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn mutex_lock(mutex: *mut mutex_t);
    fn mutex_unlock(mutex: *mut mutex_t);
    fn atomic64_read(ptr: *mut atomic64_t) -> u64;
    fn atomic64_cmpxchg(ptr: *mut atomic64_t, old: u64, new: u64) -> u64;
    fn gen_cookie_next(cookie: *mut cookie_t) -> u64;
    fn sk_get_meminfo(sk: *mut c_void, mem: *mut u32);
    fn nla_put(skb: *mut c_void, attrtype: c_int, attrlen: size_t, data: *const c_void) -> c_int;
    fn nla_reserve(skb: *mut c_void, attrtype: c_int, attrlen: size_t);
    fn nla_data(attr: *mut c_void) -> *mut c_void;
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn rcu_dereference(sk: *mut c_void, offset: size_t) -> *mut c_void;
    fn bpf_classic_proglen(prog: *mut c_void) -> size_t;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t);
    fn sk_destruct(sk: *mut c_void);
    fn INIT_WORK(work: *mut work_struct, fn_ptr: extern "C" fn(work: *mut work_struct));
    fn queue_work(wq: *mut workqueue_struct, work: *mut work_struct);
    fn alloc_workqueue(name: *const c_char, flags: c_int, max_active: c_int) -> *mut workqueue_struct;
    fn netlink_kernel_create(net: *mut net, protocol: c_int, cfg: *const netlink_kernel_cfg) -> *mut c_void;
    fn netlink_kernel_release(sk: *mut c_void);
    fn ns_capable(ns: *mut c_void, cap: c_int) -> c_int;
    fn register_pernet_subsys(ops: *const pernet_operations) -> c_int;
    fn sock_load_diag_module(family: c_int, flags: c_int);
    fn array_index_nospec(index: c_int, max: c_int) -> c_int;
}

// Constants
const AF_MAX: c_int = 40;
const INET_DIAG_NOCOOKIE: u32 = 0xFFFFFFFF;
const SK_MEMINFO_VARS: usize = 4;
const SKNLGRP_MAX: c_int = 16;
const NETLINK_SOCK_DIAG: c_int = 17;
const NL_CFG_F_NONROOT_RECV: c_int = 1;
const TCPDIAG_GETSOCK: c_int = 1;
const DCCPDIAG_GETSOCK: c_int = 2;
const SOCK_DIAG_BY_FAMILY: c_int = 3;
const SOCK_DESTROY: c_int = 4;
const SKNLGRP_INET_TCP_DESTROY: c_int = 1;
const SKNLGRP_INET_UDP_DESTROY: c_int = 2;
const SKNLGRP_INET6_TCP_DESTROY: c_int = 3;
const SKNLGRP_INET6_UDP_DESTROY: c_int = 4;
const CAP_NET_ADMIN: c_int = 12;
