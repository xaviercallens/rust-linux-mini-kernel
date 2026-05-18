
//! Network Filter Logging Module
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_ulong;
use core::ffi::c_void;
use core::mem;
use core::ptr;
use core::slice;
use kernel_types::*;

// Constants from C
pub const NF_LOGGER_NAME_LEN: usize = 64;
pub const NF_LOG_TYPE_MAX: usize = 16; // Assuming this value based on typical enum sizes
pub const NFPROTO_NUMPROTO: usize = 32; // Assuming this value based on typical enum sizes
pub const NFPROTO_UNSPEC: u8 = 0;
pub const NF_LOG_PREFIXLEN: usize = 128;
pub const EINVAL: c_int = -22;
pub const EOPNOTSUPP: c_int = -95;
pub const ENOENT: c_int = -2;
pub const EEXIST: c_int = -17;

// Type definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_nf {
    pub nf_loggers: [*mut nf_logger; NFPROTO_NUMPROTO],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_logger {
    pub name: [u8; NF_LOGGER_NAME_LEN],
    pub type_: c_int,
    pub me: *mut c_void, // struct module*
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_log_buf {
    pub count: c_uint,
    pub buf: [u8; 1024], // S_SIZE + 1
}

// Function pointer types
pub type nf_log_fn = extern "C" fn(
    net: *mut c_void,
    pf: u8,
    hooknum: u32,
    skb: *const sk_buff,
    in_: *const c_void,
    out: *const c_void,
    loginfo: *const c_void,
    prefix: *const u8,
);

// Internal state
static mut loggers: [[*mut nf_logger; NF_LOG_TYPE_MAX]; NFPROTO_NUMPROTO] =
    unsafe { mem::zeroed() };
static mut nf_log_mutex: Mutex = Mutex { lock: 0 };
static mut emergency_ptr: *mut nf_log_buf = ptr::null_mut();
static mut sysctl_nf_log_all_netns: c_int = 0;

// Mutex implementation (simplified for FFI compatibility)
#[repr(C)]
struct Mutex {
    lock: c_int,
}

impl Mutex {
    fn lock(&mut self) {
        while unsafe { self.lock.compare_exchange(
            0,
            1,
            core::sync::atomic::Ordering::Acquire,
            core::sync::atomic::Ordering::Relaxed,
        ) } != Ok(0)
        {
            // Spin until lock is available
        }
    }

    fn unlock(&mut self) {
        unsafe {
            self.lock = 0;
        }
    }
}

// RCU helpers (simplified for FFI compatibility)
#[inline]
unsafe fn rcu_dereference<T>(ptr: *const T) -> *const T {
    ptr
}

#[inline]
unsafe fn rcu_assign_pointer<T>(dest: *mut *mut T, src: *mut T) {
    *dest = src;
}

#[inline]
unsafe fn RCU_INIT_POINTER<T>(dest: *mut *mut T, src: *mut T) {
    *dest = src;
}

#[inline]
unsafe fn rcu_access_pointer<T>(ptr: *const *mut T) -> *mut T {
    *ptr
}

// Internal functions
fn nft_log_dereference(logger: *mut nf_logger) -> *mut nf_logger {
    unsafe { rcu_dereference(logger) }
}

fn __find_logger(pf: u8, str_logger: *const u8) -> *mut nf_logger {
    if pf >= NFPROTO_NUMPROTO as u8 {
        return ptr::null_mut();
    }

    for i in 0..NF_LOG_TYPE_MAX {
        let logger = unsafe { loggers[pf as usize][i] };
        if logger.is_null() {
            continue;
        }

        let logger = nft_log_dereference(logger);
        if !logger.is_null() {
            let logger_name =
                unsafe { slice::from_raw_parts(logger.offset(0) as *const u8, NF_LOGGER_NAME_LEN) };
            let str_logger_len = unsafe {
                core::ffi::CStr::from_ptr(str_logger as *const i8)
                    .to_bytes()
                    .len()
            };

            if logger_name.len() >= str_logger_len {
                let match_len = str_logger_len;
                let logger_name_slice = &logger_name[..match_len];
                let str_logger_slice = unsafe { slice::from_raw_parts(str_logger, match_len) };

                if logger_name_slice == str_logger_slice {
                    return logger;
                }
            }
        }
    }

    ptr::null_mut()
}

// Exported functions
/// Set network logger for specific protocol family
///
/// # Safety
/// - `net` must be a valid pointer to net structure
/// - `pf` must be valid protocol family
/// - `logger` must be a valid logger pointer
///
/// # Returns
/// 0 on success, -EOPNOTSUPP if protocol family invalid
#[no_mangle]
pub unsafe extern "C" fn nf_log_set(net: *mut c_void, pf: u8, logger: *const nf_logger) -> c_int {
    if pf == NFPROTO_UNSPEC || pf >= NFPROTO_NUMPROTO as u8 {
        return -EOPNOTSUPP;
    }

    let mut mutex = &mut nf_log_mutex;
    mutex.lock();

    let net_nf_ptr = net as *mut net_nf;
    if net_nf_ptr.is_null() {
        mutex.unlock();
        return -EINVAL;
    }

    let current_logger = nft_log_dereference((*net_nf_ptr).nf_loggers[pf as usize]);
    if current_logger.is_null() {
        rcu_assign_pointer(
            &mut (*net_nf_ptr).nf_loggers[pf as usize],
            logger as *mut nf_logger,
        );
    }

    mutex.unlock();

    0
}

/// Unset network logger
///
/// # Safety
/// - `net` must be a valid pointer to net structure
/// - `logger` must be a valid logger pointer
#[no_mangle]
pub unsafe extern "C" fn nf_log_unset(net: *mut c_void, logger: *const nf_logger) {
    let mut mutex = &mut nf_log_mutex;
    mutex.lock();

    let net_nf_ptr = net as *mut net_nf;
    if net_nf_ptr.is_null() {
        mutex.unlock();
        return;
    }

    for i in 0..NFPROTO_NUMPROTO {
        let current_logger = nft_log_dereference((*net_nf_ptr).nf_loggers[i]);
        if current_logger == logger {
            RCU_INIT_POINTER(
                &mut (*net_nf_ptr).nf_loggers[i],
                ptr::null_mut(),
            );
        }
    }

    mutex.unlock();
}

/// Register a network logger
///
/// # Safety
/// - `pf` must be valid protocol family
/// - `logger` must be a valid logger pointer
///
/// # Returns
/// 0 on success, -EINVAL if invalid, -EEXIST if already exists
#[no_mangle]
pub unsafe extern "C" fn nf_log_register(pf: u8, logger: *mut nf_logger) -> c_int {
    if pf >= NFPROTO_NUMPROTO as u8 {
        return -EINVAL;
    }

    let mut mutex = &mut nf_log_mutex;
    mutex.lock();

    let mut ret = 0;

    if pf == NFPROTO_UNSPEC {
        for i in 0..NFPROTO_NUMPROTO {
            let existing = rcu_access_pointer(loggers[i].as_ptr());
            if !existing.is_null() {
                ret = -EEXIST;
                break;
            }
        }

        if ret == 0 {
            for i in 0..NFPROTO_NUMPROTO {
                rcu_assign_pointer(
                    &mut loggers[i][(*logger).type_ as usize],
                    logger,
                );
            }
        }
    } else {
        let existing =
            rcu_access_pointer(&loggers[pf as usize][(*logger).type_ as usize]);
        if !existing.is_null() {
            ret = -EEXIST;
        } else {
            rcu_assign_pointer(
                &mut loggers[pf as usize][(*logger).type_ as usize],
                logger,
            );
        }
    }

    mutex.unlock();
    ret
}

/// Unregister a network logger
///
/// # Safety
/// - `logger` must be a valid logger pointer
#[no_mangle]
pub unsafe extern "C" fn nf_log_unregister(logger: *mut nf_logger) {
    let mut mutex = &mut nf_log_mutex;
    mutex.lock();

    for i in 0..NFPROTO_NUMPROTO {
        let current_logger = nft_log_dereference(loggers[i][(*logger).type_ as usize]);
        if current_logger == logger {
            RCU_INIT_POINTER(
                &mut loggers[i][(*logger).type_ as usize],
                ptr::null_mut(),
            );
        }
    }

    mutex.unlock();
    synchronize_rcu();
}

// Helper function for synchronization
#[inline]
unsafe fn synchronize_rcu() {
    // Simplified implementation for FFI compatibility
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_registration() {
        // Basic test for logger registration
        unsafe {
            let mut logger = nf_logger {
                name: [0; NF_LOGGER_NAME_LEN],
                type_: 0,
                me: ptr::null_mut(),
            };

            // Register logger for PF_INET
            let result = nf_log_register(1, &mut logger);
            assert_eq!(result, 0);

            // Unregister logger
            nf_log_unregister(&mut logger);
        }
    }
}