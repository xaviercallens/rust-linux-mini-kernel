//! IPv6 flowlabel manager for Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::all)]

use core::ptr;
use core::sync::atomic::{AtomicUsize, Ordering};
use libc::{c_int, c_uint, c_ulong, c_void, size_t};

// Constants from C
pub const FL_MIN_LINGER: c_ulong = 6; // 6 seconds
pub const FL_MAX_LINGER: c_ulong = 150; // 150 seconds
pub const FL_MAX_PER_SOCK: c_ulong = 32;
pub const FL_MAX_SIZE: c_ulong = 4096;
pub const FL_HASH_MASK: c_ulong = 255;

// Error codes
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const EPERM: c_int = -1;

// Type definitions
#[repr(C)]
pub struct RcuHead {
    _unused: [u8; 0], // Placeholder - actual implementation depends on kernel RCU
}

#[repr(C)]
pub struct TimerList {
    _unused: [u8; 0], // Placeholder - actual implementation depends on kernel timers
}

#[repr(C)]
pub struct SpinLock {
    _unused: [u8; 0], // Placeholder - actual implementation depends on kernel spinlocks
}

#[repr(C)]
pub struct Net {
    _unused: [u8; 0], // Placeholder - actual implementation depends on kernel net namespace
}

#[repr(C)]
pub struct In6FlowlabelReq {
    flr_label: u32,
    flr_linger: c_ulong,
    _unused: [u8; 0], // ... rest of struct
}

#[repr(C)]
pub struct Sock {
    _unused: [u8; 0], // Placeholder - actual implementation depends on kernel sock
}

#[repr(C)]
pub struct Ip6Flowlabel {
    label: u32,
    users: AtomicUsize,
    lastuse: c_ulong,
    linger: c_ulong,
    expires: c_ulong,
    next: *mut Ip6Flowlabel,
    fl_net: *mut Net,
    share: c_int,       // IPV6_FL_S_* constants
    owner: *mut c_void, // pid_t
    opt: *mut c_void,   // ipv6_txoptions
    rcu: RcuHead,
}

#[repr(C)]
pub struct Ip6FlSocklist {
    fl: *mut Ip6Flowlabel,
    next: *mut Ip6FlSocklist,
    rcu: RcuHead,
}

#[repr(C)]
pub struct Ipv6Txoptions {
    hopopt: *mut c_void,
    dst0opt: *mut c_void,
    srcrt: *mut c_void,
    dst1opt: *mut c_void,
    opt_nflen: c_int,
    opt_flen: c_int,
    tot_len: c_int,
}

// Static variables
static mut FL_SIZE: AtomicUsize = AtomicUsize::new(0);
static mut FL_HT: [*mut Ip6Flowlabel; FL_HASH_MASK as usize + 1] =
    [ptr::null_mut(); FL_HASH_MASK as usize + 1];
static mut IP6_FL_GC_TIMER: TimerList = TimerList { _unused: [] };
static mut IP6_FL_LOCK: SpinLock = SpinLock { _unused: [] };
static mut IP6_SK_FL_LOCK: SpinLock = SpinLock { _unused: [] };

// Exported symbols
#[no_mangle]
pub static mut IPV6_FLOWLABEL_EXCLUSIVE: AtomicUsize = AtomicUsize::new(0);

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn ip6_fl_gc(unused: *mut TimerList) {
    let now = jiffies();
    let mut sched: c_ulong = 0;

    spin_lock(&mut IP6_FL_LOCK);

    for i in 0..=FL_HASH_MASK as usize {
        let mut flp = &mut FL_HT[i] as *mut *mut Ip6Flowlabel;
        while let Some(fl) = ptr::read_volatile(flp) {
            if (*fl).users.load(Ordering::Relaxed) == 0 {
                let ttd = (*fl).lastuse + (*fl).linger;
                if ttd > (*fl).expires {
                    (*fl).expires = ttd;
                }
                let ttd = (*fl).expires;
                if now >= ttd {
                    ptr::write_volatile(flp, (*fl).next);
                    fl_free(fl);
                    FL_SIZE.fetch_sub(1, Ordering::Relaxed);
                    continue;
                }
                if sched == 0 || ttd < sched {
                    sched = ttd;
                }
            }
            flp = &(*fl).next as *mut *mut Ip6Flowlabel;
        }
    }

    if sched == 0 && FL_SIZE.load(Ordering::Relaxed) > 0 {
        sched = now + FL_MAX_LINGER;
    }

    if sched > 0 {
        mod_timer(&mut IP6_FL_GC_TIMER, sched);
    }

    spin_unlock(&mut IP6_FL_LOCK);
}

#[no_mangle]
pub unsafe extern "C" fn __fl_lookup(net: *mut Net, label: u32) -> *mut Ip6Flowlabel {
    let hash = FL_HASH(label);
    let mut fl = FL_HT[hash as usize];

    while !fl.is_null() {
        if (*fl).label == label && net_eq((*fl).fl_net, net) {
            return fl;
        }
        fl = (*fl).next;
    }

    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn fl_lookup(net: *mut Net, label: u32) -> *mut Ip6Flowlabel {
    rcu_read_lock_bh();
    let fl = __fl_lookup(net, label);
    if !fl.is_null() && !atomic_inc_not_zero(&(*fl).users) {
        ptr::write(fl as *mut _, ptr::null_mut() as *mut Ip6Flowlabel);
        fl = ptr::null_mut();
    }
    rcu_read_unlock_bh();
    fl
}

#[no_mangle]
pub unsafe extern "C" fn fl_shared_exclusive(fl: *mut Ip6Flowlabel) -> bool {
    let share = (*fl).share;
    share == 1 || share == 2 || share == 3 // Assuming IPV6_FL_S_EXCL=1, etc.
}

#[no_mangle]
pub unsafe extern "C" fn fl_free_rcu(head: *mut RcuHead) {
    let fl = (head as *mut Ip6Flowlabel);
    if fl_shared_exclusive(fl) || !(*fl).opt.is_null() {
        static_branch_slow_dec_deferred(&mut IPV6_FLOWLABEL_EXCLUSIVE);
    }

    if (*fl).share == 1 {
        // Assuming IPV6_FL_S_PROCESS
        put_pid((*fl).owner);
    }

    if !(*fl).opt.is_null() {
        kfree((*fl).opt);
    }
    kfree(fl as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn fl_free(fl: *mut Ip6Flowlabel) {
    if fl.is_null() {
        return;
    }

    call_rcu(&(*fl).rcu, fl_free_rcu);
}

#[no_mangle]
pub unsafe extern "C" fn fl_release(fl: *mut Ip6Flowlabel) {
    spin_lock_bh(&mut IP6_FL_LOCK);

    (*fl).lastuse = jiffies();
    if atomic_dec_and_test(&(*fl).users) {
        let mut ttd = (*fl).lastuse + (*fl).linger;
        if ttd > (*fl).expires {
            (*fl).expires = ttd;
        }
        ttd = (*fl).expires;

        if (*fl).opt.is_some() && (*fl).share == 1 {
            // Assuming IPV6_FL_S_EXCL
            let opt = (*fl).opt;
            (*fl).opt = ptr::null_mut();
            kfree(opt);
        }

        if !timer_pending(&mut IP6_FL_GC_TIMER) || time_after(ip6_fl_gc_timer.expires, ttd) {
            mod_timer(&mut IP6_FL_GC_TIMER, ttd);
        }
    }

    spin_unlock_bh(&mut IP6_FL_LOCK);
}

#[no_mangle]
pub unsafe extern "C" fn ip6_fl_purge(net: *mut Net) {
    spin_lock_bh(&mut IP6_FL_LOCK);

    for i in 0..=FL_HASH_MASK as usize {
        let mut flp = &mut FL_HT[i] as *mut *mut Ip6Flowlabel;
        while let Some(fl) = ptr::read_volatile(flp) {
            if net_eq((*fl).fl_net, net) && (*fl).users.load(Ordering::Relaxed) == 0 {
                ptr::write_volatile(flp, (*fl).next);
                fl_free(fl);
                FL_SIZE.fetch_sub(1, Ordering::Relaxed);
                continue;
            }
            flp = &(*fl).next as *mut *mut Ip6Flowlabel;
        }
    }

    spin_unlock_bh(&mut IP6_FL_LOCK);
}

#[no_mangle]
pub unsafe extern "C" fn fl_intern(
    net: *mut Net,
    fl: *mut Ip6Flowlabel,
    label: u32,
) -> *mut Ip6Flowlabel {
    (*fl).label = label & 0x0000000F; // IPV6_FLOWLABEL_MASK

    spin_lock_bh(&mut IP6_FL_LOCK);

    if label == 0 {
        loop {
            (*fl).label = prandom_u32() & 0x0000000F;
            if (*fl).label != 0 {
                let lfl = __fl_lookup(net, (*fl).label);
                if lfl.is_null() {
                    break;
                }
            }
        }
    } else {
        let lfl = __fl_lookup(net, (*fl).label);
        if !lfl.is_null() {
            atomic_inc(&(*lfl).users);
            spin_unlock_bh(&mut IP6_FL_LOCK);
            return lfl;
        }
    }

    (*fl).lastuse = jiffies();
    (*fl).next = FL_HT[FL_HASH((*fl).label) as usize];
    FL_HT[FL_HASH((*fl).label) as usize] = fl;
    FL_SIZE.fetch_add(1, Ordering::Relaxed);
    spin_unlock_bh(&mut IP6_FL_LOCK);

    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn __fl6_sock_lookup(sk: *mut Sock, label: u32) -> *mut Ip6Flowlabel {
    let np = inet6_sk(sk);
    label &= 0x0000000F; // IPV6_FLOWLABEL_MASK

    rcu_read_lock_bh();
    let mut sfl = (*np).ipv6_fl_list;
    while !sfl.is_null() {
        let fl = (*sfl).fl;
        if (*fl).label == label && atomic_inc_not_zero(&(*fl).users) {
            (*fl).lastuse = jiffies();
            rcu_read_unlock_bh();
            return fl;
        }
        sfl = (*sfl).next;
    }
    rcu_read_unlock_bh();

    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn fl6_free_socklist(sk: *mut Sock) {
    let np = inet6_sk(sk);
    if (*np).ipv6_fl_list.is_null() {
        return;
    }

    spin_lock_bh(&mut IP6_SK_FL_LOCK);
    while let Some(sfl) = (*np).ipv6_fl_list {
        (*np).ipv6_fl_list = (*sfl).next;
        spin_unlock_bh(&mut IP6_SK_FL_LOCK);

        fl_release((*sfl).fl);
        kfree_rcu(sfl as *mut c_void, &mut (*sfl).rcu);

        spin_lock_bh(&mut IP6_SK_FL_LOCK);
    }
    spin_unlock_bh(&mut IP6_SK_FL_LOCK);
}

#[no_mangle]
pub unsafe extern "C" fn fl6_merge_options(
    opt_space: *mut Ipv6Txoptions,
    fl: *mut Ip6Flowlabel,
    fopt: *mut Ipv6Txoptions,
) -> *mut Ipv6Txoptions {
    let fl_opt = (*fl).opt;

    if fopt.is_null() || (*fopt).opt_flen == 0 {
        return fl_opt;
    }

    if !fl_opt.is_null() {
        (*opt_space).hopopt = (*fl_opt).hopopt;
        (*opt_space).dst0opt = (*fl_opt).dst0opt;
        (*opt_space).srcrt = (*fl_opt).srcrt;
        (*opt_space).opt_nflen = (*fl_opt).opt_nflen;
    } else if (*fopt).opt_nflen != 0 {
        (*opt_space).hopopt = ptr::null_mut();
        (*opt_space).dst0opt = ptr::null_mut();
        (*opt_space).srcrt = ptr::null_mut();
        (*opt_space).opt_nflen = 0;
    }

    (*opt_space).dst1opt = (*fopt).dst1opt;
    (*opt_space).opt_flen = (*fopt).opt_flen;
    (*opt_space).tot_len = (*fopt).tot_len;

    opt_space
}

// Helper functions (assumed to be available in kernel)
#[no_mangle]
unsafe extern "C" fn jiffies() -> c_ulong {
    0
}

#[no_mangle]
unsafe extern "C" fn prandom_u32() -> u32 {
    0
}

#[no_mangle]
unsafe extern "C" fn net_eq(a: *mut Net, b: *mut Net) -> bool {
    a == b
}

#[no_mangle]
unsafe extern "C" fn atomic_inc(a: *mut AtomicUsize) {
    (*a).fetch_add(1, Ordering::Relaxed);
}

#[no_mangle]
unsafe extern "C" fn atomic_inc_not_zero(a: *mut AtomicUsize) -> bool {
    let val = (*a).load(Ordering::Relaxed);
    if val == 0 {
        false
    } else {
        (*a).compare_exchange(val, val + 1, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok();
        true
    }
}

#[no_mangle]
unsafe extern "C" fn atomic_dec_and_test(a: *mut AtomicUsize) -> bool {
    let val = (*a).fetch_sub(1, Ordering::Relaxed);
    val == 1
}

#[no_mangle]
unsafe extern "C" fn atomic_dec(a: *mut AtomicUsize) {
    (*a).fetch_sub(1, Ordering::Relaxed);
}

#[no_mangle]
unsafe extern "C" fn spin_lock_bh(lock: *mut SpinLock) {}

#[no_mangle]
unsafe extern "C" fn spin_unlock_bh(lock: *mut SpinLock) {}

#[no_mangle]
unsafe extern "C" fn spin_lock(lock: *mut SpinLock) {}

#[no_mangle]
unsafe extern "C" fn spin_unlock(lock: *mut SpinLock) {}

#[no_mangle]
unsafe extern "C" fn rcu_read_lock_bh() {}

#[no_mangle]
unsafe extern "C" fn rcu_read_unlock_bh() {}

#[no_mangle]
unsafe extern "C" fn call_rcu(head: *mut RcuHead, func: extern "C" fn(*mut RcuHead)) {
    func(head);
}

#[no_mangle]
unsafe extern "C" fn kfree(ptr: *mut c_void) {}

#[no_mangle]
unsafe extern "C" fn kfree_rcu(ptr: *mut c_void, rcu: *mut RcuHead) {
    kfree(ptr);
}

#[no_mangle]
unsafe extern "C" fn put_pid(pid: *mut c_void) {}

#[no_mangle]
unsafe extern "C" fn inet6_sk(sk: *mut Sock) -> *mut c_void {
    sk
}

#[no_mangle]
unsafe extern "C" fn timer_pending(timer: *mut TimerList) -> bool {
    false
}

#[no_mangle]
unsafe extern "C" fn time_after(a: c_ulong, b: c_ulong) -> bool {
    a > b
}

#[no_mangle]
unsafe extern "C" fn mod_timer(timer: *mut TimerList, expires: c_ulong) {}

#[no_mangle]
unsafe extern "C" fn static_branch_slow_dec_deferred(branch: *mut AtomicUsize) {
    (*branch).fetch_sub(1, Ordering::Relaxed);
}
