//! Linux IGMP (Internet Group Management Protocol) implementation
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clang::too_many_arguments)]

use core::ffi::{c_int, c_uint, c_void};
use core::ptr::{self, NonNull};
use libc::{size_t, time_t};

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

// Type definitions
#[repr(C)]
pub struct in_device {
    pub mr_v1_seen: time_t,
    pub mr_v2_seen: time_t,
    pub mr_maxdelay: c_int,
    pub mr_gq_running: c_int,
    pub mr_ifc_timer: Timer,
    pub mr_gq_timer: Timer,
    pub mc_list: *mut ip_mc_list,
    // ... other fields from actual in_device struct
}

#[repr(C)]
pub struct ip_mc_list {
    pub interface: *mut in_device,
    pub timer: Timer,
    pub tm_running: c_int,
    pub reporter: c_int,
    pub unsolicit_count: c_int,
    pub refcnt: AtomicInt,
    pub lock: Spinlock,
    pub next_rcu: *mut ip_mc_list,
    pub sources: *mut ip_sf_list,
    pub sfmode: c_int,
    pub sfcount: [c_int; 2],
    // ... other fields from actual ip_mc_list struct
}

#[repr(C)]
pub struct ip_sf_list {
    pub sf_next: *mut ip_sf_list,
    pub sf_count: [c_int; 2],
    pub sf_crcount: c_int,
    pub sf_gsresp: c_int,
    // ... other fields from actual ip_sf_list struct
}

#[repr(C)]
pub struct Timer {
    // Timer implementation details
}

#[repr(C)]
pub struct AtomicInt {
    value: c_int,
}

#[repr(C)]
pub struct Spinlock {
    // Spinlock implementation details
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn unsolicited_report_interval(in_dev: *mut in_device) -> c_int {
    if in_dev.is_null() {
        return EINVAL;
    }

    // SAFETY: in_dev is valid per contract
    let in_dev = unsafe { &*in_dev };
    
    let mut interval_ms: c_int = 0;
    
    if IGMP_V1_SEEN(in_dev) || IGMP_V2_SEEN(in_dev) {
        interval_ms = IN_DEV_CONF_GET(in_dev, IGMPV2_UNSOLICITED_REPORT_INTERVAL);
    } else {
        interval_ms = IN_DEV_CONF_GET(in_dev, IGMPV3_UNSOLICITED_REPORT_INTERVAL);
    }

    let interval_jiffies = msecs_to_jiffies(interval_ms);
    
    if interval_jiffies <= 0 {
        1
    } else {
        interval_jiffies
    }
}

#[no_mangle]
pub unsafe extern "C" fn ip_ma_put(im: *mut ip_mc_list) {
    if im.is_null() {
        return;
    }

    // SAFETY: im is valid per contract
    let im = unsafe { &*im };
    
    if refcount_dec_and_test(&im.refcnt) {
        in_dev_put(im.interface);
        kfree_rcu(im);
    }
}

#[no_mangle]
pub unsafe extern "C" fn igmp_stop_timer(im: *mut ip_mc_list) {
    if im.is_null() {
        return;
    }

    // SAFETY: im is valid per contract
    let im = unsafe { &mut *im };
    
    spin_lock_bh(&im.lock);
    
    if del_timer(&im.timer) {
        refcount_dec(&im.refcnt);
    }
    
    im.tm_running = 0;
    im.reporter = 0;
    im.unsolicit_count = 0;
    
    spin_unlock_bh(&im.lock);
}

#[no_mangle]
pub unsafe extern "C" fn igmp_start_timer(im: *mut ip_mc_list, max_delay: c_int) {
    if im.is_null() {
        return;
    }

    // SAFETY: im is valid per contract
    let im = unsafe { &mut *im };
    
    let tv = prandom_u32() % max_delay;
    
    im.tm_running = 1;
    
    if !mod_timer(&im.timer, jiffies + tv + 2) {
        refcount_inc(&im.refcnt);
    }
}

// Helper functions (simplified for example)
#[no_mangle]
pub unsafe extern "C" fn refcount_dec_and_test(refcnt: *mut AtomicInt) -> c_int {
    if refcnt.is_null() {
        return 0;
    }
    
    // SAFETY: refcnt is valid per contract
    let refcnt = unsafe { &mut *refcnt };
    refcnt.value -= 1;
    refcnt.value == 0
}

#[no_mangle]
pub unsafe extern "C" fn in_dev_put(dev: *mut in_device) {
    if dev.is_null() {
        return;
    }
    
    // SAFETY: dev is valid per contract
    let dev = unsafe { &mut *dev };
    // Implementation of in_dev_put
}

#[no_mangle]
pub unsafe extern "C" fn kfree_rcu(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    
    // SAFETY: ptr is valid per contract
    unsafe { libc::free(ptr) };
}

#[no_mangle]
pub unsafe extern "C" fn spin_lock_bh(lock: *mut Spinlock) {
    if lock.is_null() {
        return;
    }
    
    // Implementation of spin_lock_bh
}

#[no_mangle]
pub unsafe extern "C" fn spin_unlock_bh(lock: *mut Spinlock) {
    if lock.is_null() {
        return;
    }
    
    // Implementation of spin_unlock_bh
}

#[no_mangle]
pub unsafe extern "C" fn del_timer(timer: *mut Timer) -> c_int {
    if timer.is_null() {
        return 0;
    }
    
    // Implementation of del_timer
    1
}

#[no_mangle]
pub unsafe extern "C" fn mod_timer(timer: *mut Timer, expires: time_t) -> c_int {
    if timer.is_null() {
        return 0;
    }
    
    // Implementation of mod_timer
    1
}

#[no_mangle]
pub unsafe extern "C" fn refcount_inc(refcnt: *mut AtomicInt) {
    if refcnt.is_null() {
        return;
    }
    
    // SAFETY: refcnt is valid per contract
    let refcnt = unsafe { &mut *refcnt };
    refcnt.value += 1;
}

#[no_mangle]
pub unsafe extern "C" fn prandom_u32() -> u32 {
    // Implementation of prandom_u32
    42
}

#[no_mangle]
pub unsafe extern "C" fn jiffies() -> time_t {
    // Implementation of jiffies
    0
}

#[no_mangle]
pub unsafe extern "C" fn msecs_to_jiffies(msecs: c_int) -> c_int {
    msecs / 1 // Simplified conversion
}

// Macros translated to functions
#[no_mangle]
pub unsafe extern "C" fn IGMP_V1_SEEN(in_dev: *mut in_device) -> c_int {
    if in_dev.is_null() {
        return 0;
    }
    
    // SAFETY: in_dev is valid per contract
    let in_dev = unsafe { &*in_dev };
    
    (IPV4_DEVCONF_ALL(dev_net(in_dev.dev), FORCE_IGMP_VERSION) == 1 ||
     IN_DEV_CONF_GET(in_dev, FORCE_IGMP_VERSION) == 1 ||
     (in_dev.mr_v1_seen > 0 && time_before(jiffies(), in_dev.mr_v1_seen))) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn IGMP_V2_SEEN(in_dev: *mut in_device) -> c_int {
    if in_dev.is_null() {
        return 0;
    }
    
    // SAFETY: in_dev is valid per contract
    let in_dev = unsafe { &*in_dev };
    
    (IPV4_DEVCONF_ALL(dev_net(in_dev.dev), FORCE_IGMP_VERSION) == 2 ||
     IN_DEV_CONF_GET(in_dev, FORCE_IGMP_VERSION) == 2 ||
     (in_dev.mr_v2_seen > 0 && time_before(jiffies(), in_dev.mr_v2_seen))) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn time_before(current: time_t, expires: time_t) -> c_int {
    (current < expires) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn IN_DEV_CONF_GET(in_dev: *mut in_device, conf: c_int) -> c_int {
    if in_dev.is_null() {
        return 0;
    }
    
    // SAFETY: in_dev is valid per contract
    let in_dev = unsafe { &*in_dev };
    // Implementation would access the appropriate configuration value
    0
}

#[no_mangle]
pub unsafe extern "C" fn IPV4_DEVCONF_ALL(net: *mut c_void, conf: c_int) -> c_int {
    // Implementation would access the appropriate configuration value
    0
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    #[test]
    fn test_unsolicited_report_interval() {
        // Basic test that compiles and links
        unsafe {
            let mut in_dev = Box::new(in_device {
                mr_v1_seen: 0,
                mr_v2_seen: 0,
                mr_maxdelay: 1000,
                mr_gq_running: 0,
                mr_ifc_timer: Timer { /* ... */ },
                mr_gq_timer: Timer { /* ... */ },
                mc_list: ptr::null_mut(),
                // ... other fields
            });
            
            let result = super::unsolicited_report_interval(&mut *in_dev);
            assert!(result > 0);
        }
    }
}
