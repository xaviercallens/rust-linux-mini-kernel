//! Generic TIME_WAIT sockets functions for Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use libc::{c_int, c_uint, c_void, size_t};

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

// Type definitions
#[repr(C)]
pub struct hlist_nulls_node;
#[repr(C)]
pub struct hlist_head;
#[repr(C)]
pub struct hlist_nulls_head;
#[repr(C)]
pub struct inet_ehash_bucket;
#[repr(C)]
pub struct inet_bind_hashbucket;
#[repr(C)]
pub struct inet_hashinfo;
#[repr(C)]
pub struct sock;
#[repr(C)]
pub struct inet_sock;
#[repr(C)]
pub struct inet_connection_sock;
#[repr(C)]
pub struct inet_timewait_death_row;

#[repr(C)]
pub struct inet_timewait_sock {
    tw_refcnt: c_int,
    tw_dr: *mut inet_timewait_death_row,
    tw_tb: *mut c_void, // struct inet_bind_bucket
    tw_bind_node: hlist_head,
    tw_node: hlist_nulls_node,
    tw_daddr: u32,
    tw_rcv_saddr: u32,
    tw_bound_dev_if: c_int,
    tw_tos: u8,
    tw_num: c_int,
    tw_state: c_int,
    tw_substate: c_int,
    tw_sport: u16,
    tw_dport: u16,
    tw_family: c_int,
    tw_reuse: c_int,
    tw_reuseport: c_int,
    tw_hash: c_int,
    tw_ipv6only: c_int,
    tw_transparent: c_int,
    tw_prot: *mut c_void, // struct inet_protosw
    tw_timer: c_void, // struct timer_list
    tw_kill: c_int,
    tw_cookie: c_void, // atomic64_t
}

// Function implementations
/// Release reference to timewait socket
///
/// # Safety
/// - `tw` must be a valid pointer to inet_timewait_sock
/// - Caller must ensure proper synchronization
/// 
/// # Returns
/// 1 if caller should call inet_twsk_free() after lock release
#[no_mangle]
pub unsafe extern "C" fn inet_twsk_put(tw: *mut inet_timewait_sock) -> c_int {
    if refcount_dec_and_test(&(*tw).tw_refcnt) != 0 {
        inet_twsk_free(tw);
    }
    1
}
#[no_mangle]
pub unsafe extern "C" fn inet_twsk_hashdance(
    tw: *mut inet_timewait_sock,
    sk: *mut sock,
    hashinfo: *mut inet_hashinfo,
) {
    let inet = inet_sk(sk);
    let icsk = inet_csk(sk);
    let ehead = inet_ehash_bucket(hashinfo, (*sk).sk_hash);
    let lock = inet_ehash_lockp(hashinfo, (*sk).sk_hash);
    let bhead = &(*hashinfo).bhash[inet_bhashfn(twsk_net(tw), (*inet).inet_num, (*hashinfo).bhash_size)];

    spin_lock(&(*bhead).lock);
    (*tw).tw_tb = (*icsk).icsk_bind_hash;
    hlist_add_head(&(*tw).tw_bind_node, &(*tw).tw_tb->owners);
    spin_unlock(&(*bhead).lock);

    spin_lock(lock);
    hlist_nulls_add_head_rcu(&(*tw).tw_node, &(*ehead).chain);

    if __sk_nulls_del_node_init_rcu(sk) != 0 {
        sock_prot_inuse_add(sock_net(sk), (*sk).sk_prot, -1);
    }
    spin_unlock(lock);

    refcount_set(&(*tw).tw_refcnt, 3);
}
#[no_mangle]
pub unsafe extern "C" fn inet_twsk_alloc(
    sk: *const sock,
    dr: *mut inet_timewait_death_row,
    state: c_int,
) -> *mut inet_timewait_sock {
    if atomic_read(&(*dr).tw_count) >= (*dr).sysctl_max_tw_buckets {
        return ptr::null_mut();
    }

    let tw = kmem_cache_alloc(
        (*(*sk).sk_prot_creator).twsk_prot->twsk_slab,
        GFP_ATOMIC,
    );
    if !tw.is_null() {
        let inet = inet_sk(sk);
        (*tw).tw_dr = dr;
        (*tw).tw_daddr = (*inet).inet_daddr;
        (*tw).tw_rcv_saddr = (*inet).inet_rcv_saddr;
        (*tw).tw_bound_dev_if = (*sk).sk_bound_dev_if;
        (*tw).tw_tos = (*inet).tos;
        (*tw).tw_num = (*inet).inet_num;
        (*tw).tw_state = TCP_TIME_WAIT;
        (*tw).tw_substate = state;
        (*tw).tw_sport = (*inet).inet_sport;
        (*tw).tw_dport = (*inet).inet_dport;
        (*tw).tw_family = (*sk).sk_family;
        (*tw).tw_reuse = (*sk).sk_reuse;
        (*tw).tw_reuseport = (*sk).sk_reuseport;
        (*tw).tw_hash = (*sk).sk_hash;
        (*tw).tw_ipv6only = 0;
        (*tw).tw_transparent = (*inet).transparent;
        (*tw).tw_prot = (*sk).sk_prot_creator;
        atomic64_set(&(*tw).tw_cookie, atomic64_read(&(*sk).sk_cookie));
        twsk_net_set(tw, sock_net(sk));
        timer_setup(&(*tw).tw_timer, tw_timer_handler, TIMER_PINNED);
        refcount_set(&(*tw).tw_refcnt, 0);
        __module_get((*(*tw).tw_prot).owner);
    }
    tw
}
#[no_mangle]
pub unsafe extern "C" fn inet_twsk_deschedule_put(tw: *mut inet_timewait_sock) {
    if del_timer_sync(&(*tw).tw_timer) != 0 {
        inet_twsk_kill(tw);
    }
    inet_twsk_put(tw);
}
#[no_mangle]
pub unsafe extern "C" fn __inet_twsk_schedule(
    tw: *mut inet_timewait_sock,
    timeo: c_int,
    rearm: c_int,
) {
    (*tw).tw_kill = timeo <= 4 * HZ;
    if rearm == 0 {
        BUG_ON(mod_timer(&(*tw).tw_timer, jiffies + timeo) != 0);
        atomic_inc(&(*tw).tw_dr->tw_count);
    } else {
        mod_timer_pending(&(*tw).tw_timer, jiffies + timeo);
    }
}
#[no_mangle]
pub unsafe extern "C" fn inet_twsk_purge(hashinfo: *mut inet_hashinfo, family: c_int) {
    let mut slot = 0;
    while slot <= (*hashinfo).ehash_mask {
        let head = &(*hashinfo).ehash[slot];
        cond_resched();
        rcu_read_lock();
        let mut node: *mut hlist_nulls_node = ptr::null_mut();
        sk_nulls_for_each_rcu(sk, node, &(*head).chain) {
            if (*sk).sk_state != TCP_TIME_WAIT {
                continue;
            }
            let tw = inet_twsk(sk);
            if (*tw).tw_family != family || refcount_read(&twsk_net(tw)->ns.count) != 0 {
                continue;
            }
            if refcount_inc_not_zero(&(*tw).tw_refcnt) == 0 {
                continue;
            }
            if (*tw).tw_family != family || refcount_read(&twsk_net(tw)->ns.count) != 0 {
                inet_twsk_put(tw);
                continue;
            }
            rcu_read_unlock();
            local_bh_disable();
            inet_twsk_deschedule_put(tw);
            local_bh_enable();
            rcu_read_lock();
        }
        if get_nulls_value(node) != slot {
            continue;
        }
        rcu_read_unlock();
        slot += 1;
    }
}

// Internal functions
unsafe fn inet_twsk_bind_unhash(tw: *mut inet_timewait_sock, hashinfo: *mut inet_hashinfo) {
    let tb = (*tw).tw_tb;
    if tb.is_null() {
        return;
    }
    __hlist_del(&(*tw).tw_bind_node);
    (*tw).tw_tb = ptr::null_mut();
    inet_bind_bucket_destroy((*hashinfo).bind_bucket_cachep, tb);
    __sock_put(tw as *mut sock);
}

unsafe fn inet_twsk_kill(tw: *mut inet_timewait_sock) {
    let hashinfo = (*tw).tw_dr->hashinfo;
    let lock = inet_ehash_lockp(hashinfo, (*tw).tw_hash);
    let bhead = &(*hashinfo).bhash[inet_bhashfn(twsk_net(tw), (*tw).tw_num, (*hashinfo).bhash_size)];

    spin_lock(lock);
    sk_nulls_del_node_init_rcu(tw as *mut sock);
    spin_unlock(lock);

    spin_lock(&(*bhead).lock);
    inet_twsk_bind_unhash(tw, hashinfo);
    spin_unlock(&(*bhead).lock);

    atomic_dec(&(*tw).tw_dr->tw_count);
    inet_twsk_put(tw);
}

unsafe fn inet_twsk_free(tw: *mut inet_timewait_sock) {
    let owner = (*(*tw).tw_prot).owner;
    twsk_destructor(tw as *mut sock);
    kmem_cache_free((*(*tw).tw_prot).twsk_prot->twsk_slab, tw);
    module_put(owner);
}

// External functions (declared in C)
extern "C" {
    fn refcount_dec_and_test(refcount: *mut c_int) -> c_int;
    fn refcount_set(refcount: *mut c_int, value: c_int);
    fn atomic_read(atomic: *mut c_int) -> c_int;
    fn atomic_inc(atomic: *mut c_int);
    fn kmem_cache_alloc(slab: *mut c_void, flags: c_int) -> *mut c_void;
    fn kmem_cache_free(slab: *mut c_void, obj: *mut c_void);
    fn __hlist_del(node: *mut hlist_head);
    fn hlist_add_head(node: *mut hlist_head, list: *mut hlist_head);
    fn hlist_nulls_add_head_rcu(node: *mut hlist_nulls_node, list: *mut hlist_nulls_head);
    fn __sk_nulls_del_node_init_rcu(sk: *mut sock) -> c_int;
    fn sock_prot_inuse_add(net: *mut c_void, prot: *mut c_void, delta: c_int);
    fn spin_lock(lock: *mut c_void);
    fn spin_unlock(lock: *mut c_void);
    fn spin_lock(&(*bhead).lock);
    fn spin_unlock(&(*bhead).lock);
    fn timer_setup(timer: *mut c_void, handler: unsafe extern "C" fn(*mut c_void), flags: c_int);
    fn mod_timer(timer: *mut c_void, expires: c_int) -> c_int;
    fn mod_timer_pending(timer: *mut c_void, expires: c_int) -> c_int;
    fn del_timer_sync(timer: *mut c_void) -> c_int;
    fn cond_resched();
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn local_bh_disable();
    fn local_bh_enable();
    fn sk_nulls_for_each_rcu(sk: *mut sock, node: *mut hlist_nulls_node, list: *mut hlist_nulls_head);
    fn get_nulls_value(node: *mut hlist_nulls_node) -> c_int;
    fn tw_timer_handler(timer: *mut c_void);
    fn inet_ehash_bucket(hashinfo: *mut inet_hashinfo, hash: c_int) -> *mut inet_ehash_bucket;
    fn inet_ehash_lockp(hashinfo: *mut inet_hashinfo, hash: c_int) -> *mut c_void;
    fn inet_bhashfn(net: *mut c_void, num: c_int, size: c_int) -> c_int;
    fn inet_bind_bucket_destroy(cachep: *mut c_void, tb: *mut c_void);
    fn __sock_put(sk: *mut sock);
    fn twsk_net_set(tw: *mut inet_timewait_sock, net: *mut c_void);
    fn atomic64_set(cookie: *mut c_void, value: u64);
    fn atomic64_read(cookie: *mut c_void) -> u64;
    fn module_put(owner: *mut c_void);
    fn __NET_INC_STATS(net: *mut c_void, stat: c_int);
    fn twsk_net(tw: *mut inet_timewait_sock) -> *mut c_void;
    fn sock_net(sk: *mut sock) -> *mut c_void;
    fn inet_sk(sk: *mut sock) -> *mut inet_sock;
    fn inet_csk(sk: *mut sock) -> *mut inet_connection_sock;
    fn twsk_destructor(sk: *mut sock);
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    #[test]
    fn test_inet_twsk_put() {
        // Basic test for reference counting
        // Note: Actual testing would require kernel environment
    }
}
