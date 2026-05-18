#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;
use core::sync::atomic::{AtomicI32, Ordering};
use kernel_types::*;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub const NF_LOGGER_NAME_LEN: usize = 64;
pub const NF_LOG_TYPE_MAX: usize = 16;
pub const NFPROTO_NUMPROTO: usize = 32;
pub const NFPROTO_UNSPEC: u8 = 0;
pub const NF_LOG_PREFIXLEN: usize = 128;
pub const EINVAL: c_int = 22;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOENT: c_int = 2;
pub const EEXIST: c_int = 17;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_nf {
    pub nf_loggers: [*mut nf_logger; NFPROTO_NUMPROTO],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net {
    pub nf: net_nf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_logger {
    pub name: [u8; NF_LOGGER_NAME_LEN],
    pub type_: c_int,
    pub me: *mut c_void,
    pub logfn: Option<
        extern "C" fn(
            net: *mut net,
            pf: u8,
            hooknum: c_uint,
            skb: *const c_void,
            in_: *const c_void,
            out: *const c_void,
            loginfo: *const c_void,
            prefix: *const c_char,
        ),
    >,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_log_buf {
    pub count: c_uint,
    pub buf: [u8; 1024],
}

#[repr(C)]
pub struct Mutex {
    lock: AtomicI32,
}

impl Mutex {
    pub const fn new() -> Self {
        Self {
            lock: AtomicI32::new(0),
        }
    }

    fn lock(&self) {
        while self
            .lock
            .compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {}
    }

    fn unlock(&self) {
        self.lock.store(0, Ordering::Release);
    }
}

static mut loggers: [[*mut nf_logger; NF_LOG_TYPE_MAX]; NFPROTO_NUMPROTO] =
    [[ptr::null_mut(); NF_LOG_TYPE_MAX]; NFPROTO_NUMPROTO];
static nf_log_mutex: Mutex = Mutex::new();
static mut emergency_ptr: *mut nf_log_buf = ptr::null_mut();
static mut sysctl_nf_log_all_netns: c_int = 0;

#[inline]
unsafe fn rcu_dereference<T>(p: *mut T) -> *mut T {
    p
}

#[inline]
unsafe fn rcu_assign_pointer<T>(dst: *mut *mut T, src: *mut T) {
    *dst = src;
}

fn __find_logger(pf: u8, str_logger: *const c_char) -> *mut nf_logger {
    if pf as usize >= NFPROTO_NUMPROTO || str_logger.is_null() {
        return ptr::null_mut();
    }

    let mut i = 0usize;
    while i < NF_LOG_TYPE_MAX {
        let logger = unsafe { rcu_dereference(loggers[pf as usize][i]) };
        if !logger.is_null() {
            let mut matched = true;
            let mut j = 0usize;
            while j < NF_LOGGER_NAME_LEN {
                let a = unsafe { (*logger).name[j] };
                let b = unsafe { *str_logger.add(j) as u8 };
                if a != b {
                    matched = false;
                    break;
                }
                if b == 0 {
                    break;
                }
                j += 1;
            }
            if matched {
                return logger;
            }
        }
        i += 1;
    }

    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nf_log_set(netns: *mut net, pf: u8, logger: *const nf_logger) -> c_int {
    if netns.is_null() || logger.is_null() {
        return -EINVAL;
    }
    if pf == NFPROTO_UNSPEC || (pf as usize) >= NFPROTO_NUMPROTO {
        return -EOPNOTSUPP;
    }

    nf_log_mutex.lock();
    rcu_assign_pointer(
        (*netns).nf.nf_loggers.as_mut_ptr().add(pf as usize),
        logger as *mut nf_logger,
    );
    nf_log_mutex.unlock();
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nf_log_unset(netns: *mut net, pf: u8) {
    if netns.is_null() || pf as usize >= NFPROTO_NUMPROTO {
        return;
    }

    nf_log_mutex.lock();
    rcu_assign_pointer((*netns).nf.nf_loggers.as_mut_ptr().add(pf as usize), ptr::null_mut());
    nf_log_mutex.unlock();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nf_log_register(pf: u8, logger: *mut nf_logger) -> c_int {
    if logger.is_null() {
        return -EINVAL;
    }
    if pf as usize >= NFPROTO_NUMPROTO {
        return -EOPNOTSUPP;
    }

    nf_log_mutex.lock();

    let t = (*logger).type_;
    if t < 0 || (t as usize) >= NF_LOG_TYPE_MAX {
        nf_log_mutex.unlock();
        return -EINVAL;
    }

    if !loggers[pf as usize][t as usize].is_null() {
        nf_log_mutex.unlock();
        return -EEXIST;
    }

    loggers[pf as usize][t as usize] = logger;
    nf_log_mutex.unlock();
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nf_log_unregister(pf: u8, logger: *mut nf_logger) {
    if logger.is_null() || (pf as usize) >= NFPROTO_NUMPROTO {
        return;
    }

    nf_log_mutex.lock();
    let t = (*logger).type_;
    if t >= 0 && (t as usize) < NF_LOG_TYPE_MAX && loggers[pf as usize][t as usize] == logger {
        loggers[pf as usize][t as usize] = ptr::null_mut();
    }
    nf_log_mutex.unlock();
}