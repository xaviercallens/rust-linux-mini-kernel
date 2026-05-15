//! Linux network device link state notification
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use core::sync::atomic::{AtomicUsize, Ordering};
use libc::{c_int, c_uint, c_ulong, c_void};

// Constants from C
pub const IF_OPER_DOWN: u8 = 0;
pub const IF_OPER_LOWERLAYERDOWN: u8 = 1;
pub const IF_OPER_TESTING: u8 = 3;
pub const IF_OPER_DORMANT: u8 = 5;
pub const IF_OPER_UP: u8 = 6;
pub const IFF_UP: u32 = 1 << 0;

// Error codes
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;

// Type definitions
#[repr(C)]
struct work_struct {
    _private: [u8; 0],
}

#[repr(C)]
struct delayed_work {
    work: work_struct,
    timer: [u8; 0],
}

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct net_device {
    state: c_ulong,
    ifindex: c_int,
    operstate: u8,
    flags: u32,
    link_watch_list: list_head,
    _private: [u8; 0],
}

#[repr(C)]
struct spinlock_t {
    _private: [u8; 0],
}

// Global variables
static mut linkwatch_flags: c_ulong = 0;
static mut linkwatch_nextevent: c_ulong = 0;
static mut lweventlist: list_head = list_head {
    next: &mut lweventlist as *mut _,
    prev: &mut lweventlist as *mut _,
};
static mut lweventlist_lock: spinlock_t = spinlock_t { _private: [0; 0] };
static mut linkwatch_work: delayed_work = unsafe { core::mem::zeroed() };

// Function implementations
/// Handle pre-registration link state changes
fn linkwatch_init_dev(dev: *mut net_device) {
    unsafe {
        if !netif_carrier_ok(dev) || netif_dormant(dev) || netif_testing(dev) {
            rfc2863_policy(dev);
        }
    }
}

/// Determine default operational state of device
fn default_operstate(dev: *const net_device) -> u8 {
    unsafe {
        if netif_testing(dev) != 0 {
            return IF_OPER_TESTING;
        }

        if !netif_carrier_ok(dev) {
            return if (*dev).ifindex != dev_get_iflink(dev) {
                IF_OPER_LOWERLAYERDOWN
            } else {
                IF_OPER_DOWN
            };
        }

        if netif_dormant(dev) != 0 {
            return IF_OPER_DORMANT;
        }

        IF_OPER_UP
    }
}

/// Update device operational state according to RFC2863
fn rfc2863_policy(dev: *mut net_device) {
    unsafe {
        let operstate = default_operstate(dev);
        if operstate == (*dev).operstate {
            return;
        }

        write_lock_bh(&mut dev_base_lock());

        match (*dev).link_mode {
            IF_LINK_MODE_TESTING => {
                if operstate == IF_OPER_UP {
                    (*dev).operstate = IF_OPER_TESTING;
                }
            }
            IF_LINK_MODE_DORMANT => {
                if operstate == IF_OPER_UP {
                    (*dev).operstate = IF_OPER_DORMANT;
                }
            }
            _ => {
                (*dev).operstate = operstate;
            }
        }

        write_unlock_bh(&mut dev_base_lock());
    }
}

/// Check if device has urgent event requirements
fn linkwatch_urgent_event(dev: *mut net_device) -> bool {
    unsafe {
        if !netif_running(dev) {
            return false;
        }

        if (*dev).ifindex != dev_get_iflink(dev) {
            return true;
        }

        if netif_is_lag_port(dev) != 0 || netif_is_lag_master(dev) != 0 {
            return true;
        }

        netif_carrier_ok(dev) != 0 && qdisc_tx_changing(dev) != 0
    }
}

/// Add device to link watch event list
fn linkwatch_add_event(dev: *mut net_device) {
    unsafe {
        let mut flags: c_ulong = 0;
        spin_lock_irqsave(&mut lweventlist_lock, &mut flags);

        if list_empty(&(*dev).link_watch_list) {
            list_add_tail(&mut (*dev).link_watch_list, &mut lweventlist);
            dev_hold(dev);
        }

        spin_unlock_irqrestore(&mut lweventlist_lock, flags);
    }
}

/// Schedule work execution with appropriate delay
fn linkwatch_schedule_work(urgent: c_int) {
    unsafe {
        let delay = linkwatch_nextevent - jiffies();
        if test_bit(LW_URGENT, &linkwatch_flags) != 0 {
            return;
        }

        if urgent != 0 {
            if test_and_set_bit(LW_URGENT, &mut linkwatch_flags) != 0 {
                return;
            }
            delay = 0;
        }

        if delay > HZ() {
            delay = 0;
        }

        if test_bit(LW_URGENT, &linkwatch_flags) != 0 {
            mod_delayed_work(system_wq(), &mut linkwatch_work, 0);
        } else {
            schedule_delayed_work(&mut linkwatch_work, delay);
        }
    }
}

/// Process a single device's link state change
fn linkwatch_do_dev(dev: *mut net_device) {
    unsafe {
        smp_mb__before_atomic();

        clear_bit(__LINK_STATE_LINKWATCH_PENDING, &mut (*dev).state);

        rfc2863_policy(dev);
        if (*dev).flags & IFF_UP != 0 {
            if netif_carrier_ok(dev) != 0 {
                dev_activate(dev);
            } else {
                dev_deactivate(dev);
            }

            netdev_state_change(dev);
        }
        dev_put(dev);
    }
}

/// Process link watch event queue
fn __linkwatch_run_queue(urgent_only: c_int) {
    unsafe {
        const MAX_DO_DEV_PER_LOOP: c_int = 100;
        let mut do_dev = MAX_DO_DEV_PER_LOOP;
        let mut wrk: list_head = list_head {
            next: &mut wrk as *mut _,
            prev: &mut wrk as *mut _,
        };
        let mut dev: *mut net_device = ptr::null_mut();

        if urgent_only != 0 {
            do_dev += MAX_DO_DEV_PER_LOOP;
        }

        if urgent_only == 0 {
            linkwatch_nextevent = jiffies() + HZ();
        } else if time_after(linkwatch_nextevent, jiffies() + HZ()) != 0 {
            linkwatch_nextevent = jiffies();
        }

        clear_bit(LW_URGENT, &mut linkwatch_flags);

        spin_lock_irq(&mut lweventlist_lock);
        list_splice_init(&mut lweventlist, &mut wrk);

        while !list_empty(&mut wrk) && do_dev > 0 {
            dev = list_first_entry(&mut wrk, net_device, link_watch_list);
            list_del_init(&mut (*dev).link_watch_list);

            if !netif_device_present(dev) || (urgent_only != 0 && !linkwatch_urgent_event(dev)) {
                list_add_tail(&mut (*dev).link_watch_list, &mut lweventlist);
                continue;
            }

            spin_unlock_irq(&mut lweventlist_lock);
            linkwatch_do_dev(dev);
            do_dev -= 1;
            spin_lock_irq(&mut lweventlist_lock);
        }

        list_splice_init(&mut wrk, &mut lweventlist);

        if !list_empty(&mut lweventlist) {
            linkwatch_schedule_work(0);
        }
        spin_unlock_irq(&mut lweventlist_lock);
    }
}

/// Remove device from link watch queue
fn linkwatch_forget_dev(dev: *mut net_device) {
    unsafe {
        let mut flags: c_ulong = 0;
        let mut clean = 0;

        spin_lock_irqsave(&mut lweventlist_lock, &mut flags);
        if !list_empty(&(*dev).link_watch_list) {
            list_del_init(&mut (*dev).link_watch_list);
            clean = 1;
        }
        spin_unlock_irqrestore(&mut lweventlist_lock, flags);

        if clean != 0 {
            linkwatch_do_dev(dev);
        }
    }
}

/// Run link watch queue (must be called with rtnl lock held)
fn linkwatch_run_queue() {
    unsafe {
        __linkwatch_run_queue(0);
    }
}

/// Work handler for link watch events
#[no_mangle]
pub unsafe extern "C" fn linkwatch_event(dummy: *mut work_struct) {
    rtnl_lock();
    __linkwatch_run_queue(time_after(linkwatch_nextevent, jiffies()) as c_int);
    rtnl_unlock();
}

/// Trigger link state change event
#[no_mangle]
pub unsafe extern "C" fn linkwatch_fire_event(dev: *mut net_device) {
    let urgent = linkwatch_urgent_event(dev) as c_int;

    if !test_and_set_bit(__LINK_STATE_LINKWATCH_PENDING, &mut (*dev).state) {
        linkwatch_add_event(dev);
    } else if urgent == 0 {
        return;
    }

    linkwatch_schedule_work(urgent);
}

// External functions (assumed to be defined elsewhere in the kernel)
#[link(name = "kernel")]
extern "C" {
    fn netif_carrier_ok(dev: *mut net_device) -> c_int;
    fn netif_dormant(dev: *mut net_device) -> c_int;
    fn netif_testing(dev: *mut net_device) -> c_int;
    fn netif_running(dev: *mut net_device) -> c_int;
    fn netif_is_lag_port(dev: *mut net_device) -> c_int;
    fn netif_is_lag_master(dev: *mut net_device) -> c_int;
    fn qdisc_tx_changing(dev: *mut net_device) -> c_int;
    fn netif_device_present(dev: *mut net_device) -> c_int;
    fn dev_get_iflink(dev: *mut net_device) -> c_int;
    fn dev_activate(dev: *mut net_device);
    fn dev_deactivate(dev: *mut net_device);
    fn netdev_state_change(dev: *mut net_device);
    fn dev_hold(dev: *mut net_device);
    fn dev_put(dev: *mut net_device);
    fn jiffies() -> c_ulong;
    fn HZ() -> c_ulong;
    fn time_after(a: c_ulong, b: c_ulong) -> c_int;
    fn test_bit(nr: c_int, addr: *const c_ulong) -> c_int;
    fn test_and_set_bit(nr: c_int, addr: *mut c_ulong) -> c_int;
    fn clear_bit(nr: c_int, addr: *mut c_ulong);
    fn smp_mb__before_atomic();
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_lock_irq(lock: *mut spinlock_t);
    fn spin_unlock_irq(lock: *mut spinlock_t);
    fn list_empty(head: *mut list_head) -> c_int;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del_init(entry: *mut list_head);
    fn list_splice_init(list: *mut list_head, head: *mut list_head);
    fn list_first_entry(head: *mut list_head, type_: *mut net_device, member: *mut list_head) -> *mut net_device;
    fn list_add_tail(&mut self, new: *mut list_head, head: *mut list_head);
    fn mod_delayed_work(wq: *mut c_void, work: *mut delayed_work, delay: c_ulong);
    fn schedule_delayed_work(work: *mut delayed_work, delay: c_ulong);
    fn rtnl_lock();
    fn rtnl_unlock();
}

// Helper macros translated to functions
const LW_URGENT: c_int = 0;
const __LINK_STATE_LINKWATCH_PENDING: c_int = 1;
const IF_LINK_MODE_TESTING: c_int = 1;
const IF_LINK_MODE_DORMANT: c_int = 2;
const IF_LINK_MODE_DEFAULT: c_int = 3;

// Dummy dev_base_lock implementation
fn dev_base_lock() -> *mut spinlock_t {
    unsafe { &mut lweventlist_lock }
}

fn write_lock_bh(lock: *mut spinlock_t) {
    unsafe { spin_lock_irq(lock) }
}

fn write_unlock_bh(lock: *mut spinlock_t) {
    unsafe { spin_unlock_irq(lock) }
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    // No tests for kernel module code
}
