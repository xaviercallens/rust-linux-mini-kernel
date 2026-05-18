#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::all)]

use core::ffi::c_void;
use core::panic::PanicInfo;
use core::ptr;
use core::sync::atomic::{AtomicUsize, Ordering};

mod kernel_types {
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_ulong = u64;
    pub type size_t = usize;
    pub type c_size_t = usize;
    pub type socklen_t = u32;
}

use kernel_types::{c_int, c_ulong};

pub const FL_MIN_LINGER: c_ulong = 6;
pub const FL_MAX_LINGER: c_ulong = 150;
pub const FL_MAX_PER_SOCK: c_ulong = 32;
pub const FL_MAX_SIZE: c_ulong = 4096;
pub const FL_HASH_MASK: c_ulong = 255;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const EPERM: c_int = -1;

#[repr(C)]
pub struct RcuHead {
    _priv: *mut c_void,
}

#[repr(C)]
pub struct TimerList {
    _priv: *mut c_void,
}

#[repr(C)]
pub struct SpinLock {
    _priv: *mut c_void,
}

#[repr(C)]
pub struct Net {
    _priv: *mut c_void,
}

#[repr(C)]
pub struct In6FlowlabelReq {
    pub flr_label: u32,
    pub flr_linger: c_ulong,
}

#[repr(C)]
pub struct Sock {
    _priv: *mut c_void,
}

#[repr(C)]
pub struct Ip6Flowlabel {
    pub label: u32,
    pub users: AtomicUsize,
    pub lastuse: c_ulong,
    pub linger: c_ulong,
    pub expires: c_ulong,
    pub next: *mut Ip6Flowlabel,
    pub fl_net: *mut Net,
    pub share: c_int,
    pub owner: *mut c_void,
    pub opt: *mut c_void,
    pub rcu: RcuHead,
}

#[repr(C)]
pub struct Ip6FlSocklist {
    pub fl: *mut Ip6Flowlabel,
    pub next: *mut Ip6FlSocklist,
    pub rcu: RcuHead,
}

#[repr(C)]
pub struct Ipv6Txoptions {
    pub hopopt: *mut c_void,
    pub dst0opt: *mut c_void,
    pub srcrt: *mut c_void,
    pub dst1opt: *mut c_void,
    pub opt_nflen: c_int,
    pub opt_flen: c_int,
    pub tot_len: c_int,
}

static FL_SIZE: AtomicUsize = AtomicUsize::new(0);
static mut FL_HT: [*mut Ip6Flowlabel; FL_HASH_MASK as usize + 1] =
    [ptr::null_mut(); FL_HASH_MASK as usize + 1];
static mut IP6_FL_GC_TIMER: TimerList = TimerList { _priv: ptr::null_mut() };
static mut IP6_FL_LOCK: SpinLock = SpinLock { _priv: ptr::null_mut() };
static mut IP6_SK_FL_LOCK: SpinLock = SpinLock { _priv: ptr::null_mut() };

#[no_mangle]
pub static IPV6_FLOWLABEL_EXCLUSIVE: AtomicUsize = AtomicUsize::new(0);

#[inline]
fn fl_hash(label: u32) -> usize {
    (label as usize) & (FL_HASH_MASK as usize)
}

#[no_mangle]
pub unsafe extern "C" fn ip6_fl_gc(_unused: *mut TimerList) {
    let now = jiffies();
    let mut sched: c_ulong = 0;

    spin_lock(core::ptr::addr_of_mut!(IP6_FL_LOCK));

    for i in 0..=FL_HASH_MASK as usize {
        let mut flp: *mut *mut Ip6Flowlabel = core::ptr::addr_of_mut!(FL_HT[i]);
        while !(*flp).is_null() {
            let fl = *flp;
            if (*fl).users.load(Ordering::Relaxed) == 0 {
                let mut ttd = (*fl).lastuse + (*fl).linger;
                if ttd > (*fl).expires {
                    (*fl).expires = ttd;
                }
                ttd = (*fl).expires;
                if now >= ttd {
                    *flp = (*fl).next;
                    fl_free(fl);
                    FL_SIZE.fetch_sub(1, Ordering::Relaxed);
                    continue;
                }
                if sched == 0 || ttd < sched {
                    sched = ttd;
                }
            }
            flp = core::ptr::addr_of_mut!((*fl).next);
        }
    }

    if sched == 0 && FL_SIZE.load(Ordering::Relaxed) > 0 {
        sched = now + FL_MAX_LINGER;
    }

    if sched > 0 {
        mod_timer(core::ptr::addr_of_mut!(IP6_FL_GC_TIMER), sched);
    }

    spin_unlock(core::ptr::addr_of_mut!(IP6_FL_LOCK));
}

#[no_mangle]
pub unsafe extern "C" fn __fl_lookup(net: *mut Net, label: u32) -> *mut Ip6Flowlabel {
    let hash = fl_hash(label);
    let mut fl = FL_HT[hash];

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
    let mut fl = __fl_lookup(net, label);
    if !fl.is_null() && !atomic_inc_not_zero(core::ptr::addr_of_mut!((*fl).users)) {
        fl = ptr::null_mut();
    }
    rcu_read_unlock_bh();
    fl
}

#[no_mangle]
pub unsafe extern "C" fn fl_shared_exclusive(fl: *mut Ip6Flowlabel) -> bool {
    let share = (*fl).share;
    share == 1 || share == 2 || share == 3
}

extern "C" {
    fn jiffies() -> c_ulong;
    fn spin_lock(lock: *mut SpinLock);
    fn spin_unlock(lock: *mut SpinLock);
    fn mod_timer(timer: *mut TimerList, expires: c_ulong) -> c_int;
    fn fl_free(fl: *mut Ip6Flowlabel);
    fn net_eq(a: *mut Net, b: *mut Net) -> bool;
    fn rcu_read_lock_bh();
    fn rcu_read_unlock_bh();
    fn atomic_inc_not_zero(v: *mut AtomicUsize) -> bool;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}