//! Generic failover infrastructure for Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use core::ffi::c_int;
use core::ffi::c_ulong;
use core::ffi::c_void;
use core::mem::size_of;
use core::mem::offset_of;

// Constants from C
pub const ARPHRD_ETHER: c_int = 1;
pub const IFF_FAILOVER_SLAVE: c_int = 0x00000001;
pub const IFF_LIVE_RENAME_OK: c_int = 0x00000002;
pub const IFF_FAILOVER: c_int = 0x00000004;
pub const NOTIFY_OK: c_int = 0;
pub const NOTIFY_DONE: c_int = 1;
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;

// Type definitions
#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct failover {
    list: list_head,
    ops: *mut failover_ops,
    failover_dev: *mut net_device,
}

#[repr(C)]
struct failover_ops {
    slave_pre_register: Option<unsafe extern "C" fn(*mut net_device, *mut net_device) -> c_int>,
    slave_register: Option<unsafe extern "C" fn(*mut net_device, *mut net_device) -> c_int>,
    slave_pre_unregister: Option<unsafe extern "C" fn(*mut net_device, *mut net_device) -> c_int>,
    slave_unregister: Option<unsafe extern "C" fn(*mut net_device, *mut net_device) -> c_int>,
    slave_link_change: Option<unsafe extern "C" fn(*mut net_device, *mut net_device) -> c_int>,
    slave_name_change: Option<unsafe extern "C" fn(*mut net_device, *mut net_device) -> c_int>,
}

// Opaque types for kernel structures
type net_device = net_device;
type netdev_lag_upper_info = netdev_lag_upper_info;

// Function declarations for kernel functions
extern "C" {
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn rtnl_dereference<T>(ptr: *const T) -> *const T;
    fn ether_addr_equal(a: *const u8, b: *const u8) -> bool;
    fn netdev_rx_handler_register(dev: *mut net_device, handler: extern "C" fn(*mut net_device, *mut c_void) -> *mut c_void, data: *mut c_void) -> c_int;
    fn netdev_master_upper_dev_link(slave: *mut net_device, master: *mut net_device, extack: *mut c_void, info: *mut netdev_lag_upper_info, notify: *mut c_void) -> c_int;
    fn netdev_upper_dev_unlink(slave: *mut net_device, master: *mut net_device);
    fn netdev_rx_handler_unregister(dev: *mut net_device);
    fn netdev_err(dev: *mut net_device, fmt: *const c_char, ...);
    fn dev_hold(dev: *mut net_device);
    fn dev_put(dev: *mut net_device);
    fn list_add_tail(entry: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn kfree(ptr: *mut c_void);
    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn register_netdevice_notifier(nb: *mut notifier_block) -> c_int;
    fn unregister_netdevice_notifier(nb: *mut notifier_block);
    fn rtnl_lock();
    fn rtnl_unlock();
    fn for_each_netdev(net: *mut net, dev: *mut net_device);
    fn dev_net(dev: *mut net_device) -> *mut net;
}

// Notifier block struct
#[repr(C)]
struct notifier_block {
    notifier_call: Option<unsafe extern "C" fn(nb: *mut notifier_block, event: c_ulong, ptr: *mut c_void) -> c_int>,
}

// Global variables
static mut failover_list: list_head = list_head {
    next: &mut failover_list as *mut _ as *mut list_head,
    prev: &mut failover_list as *mut _ as *mut list_head,
};

static mut failover_lock: spinlock_t = spinlock_t {};

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn failover_slave_unregister(slave_dev: *mut net_device) -> c_int {
    if !((*slave_dev).priv_flags & IFF_FAILOVER_SLAVE) != 0 {
        return NOTIFY_DONE;
    }

    let mut failover_dev = failover_get_bymac((*slave_dev).perm_addr, &mut ptr::null_mut());
    if failover_dev.is_null() {
        return NOTIFY_DONE;
    }

    let fops = *ops;
    if !fops.is_null() && (*fops).slave_pre_unregister.is_some() {
        if (*fops).slave_pre_unregister.unwrap()(slave_dev, failover_dev) != 0 {
            return NOTIFY_DONE;
        }
    }

    netdev_rx_handler_unregister(slave_dev);
    netdev_upper_dev_unlink(slave_dev, failover_dev);
    (*slave_dev).priv_flags &= !(IFF_FAILOVER_SLAVE | IFF_LIVE_RENAME_OK);

    if !fops.is_null() && (*fops).slave_unregister.is_some() {
        if (*fops).slave_unregister.unwrap()(slave_dev, failover_dev) == 0 {
            return NOTIFY_OK;
        }
    }

    NOTIFY_DONE
}

#[no_mangle]
pub unsafe extern "C" fn failover_register(dev: *mut net_device, ops: *mut failover_ops) -> *mut failover {
    if (*dev).type_ != ARPHRD_ETHER {
        return ptr::invalid_mut(EINVAL as usize);
    }

    let ptr = kzalloc(size_of::<failover>() as usize, 0);
    if ptr.is_null() {
        return ptr::invalid_mut(ENOMEM as usize);
    }

    // SAFETY: ptr is valid and properly aligned
    (*ptr).ops = ops;
    dev_hold(dev);
    (*dev).priv_flags |= IFF_FAILOVER;
    (*ptr).failover_dev = dev;

    spin_lock(&mut failover_lock);
    list_add_tail(&mut (*ptr).list, &mut failover_list);
    spin_unlock(&mut failover_lock);

    // SAFETY: dev is valid and properly referenced
    netdev_info(dev, b"failover master:%s registered\n\0".as_ptr() as *const c_char, (*dev).name);

    failover_existing_slave_register(dev);

    ptr
}

#[no_mangle]
pub unsafe extern "C" fn failover_unregister(failover: *mut failover) {
    let failover_dev = (*failover).failover_dev;

    // SAFETY: failover_dev is valid and properly referenced
    netdev_info(failover_dev, b"failover master:%s unregistered\n\0".as_ptr() as *const c_char, (*failover_dev).name);

    (*failover_dev).priv_flags &= !IFF_FAILOVER;
    dev_put(failover_dev);

    spin_lock(&mut failover_lock);
    list_del(&mut (*failover).list);
    spin_unlock(&mut failover_lock);

    kfree(failover as *mut c_void);
}

// Internal functions
unsafe fn failover_get_bymac(mac: *mut u8, ops: *mut *mut failover_ops) -> *mut net_device {
    spin_lock(&mut failover_lock);
    let mut entry = failover_list.next;
    while entry != &mut failover_list as *mut _ as *mut list_head {
        let failover_ptr = (entry as *mut u8).offset(-offset_of!(failover, list)) as *mut failover;
        let failover_dev = rtnl_dereference((*failover_ptr).failover_dev);
        if ether_addr_equal((*failover_dev).perm_addr, mac) {
            *ops = rtnl_dereference((*failover_ptr).ops);
            spin_unlock(&mut failover_lock);
            return failover_dev;
        }
        entry = (*entry).next;
    }
    spin_unlock(&mut failover_lock);
    ptr::null_mut()
}

unsafe fn failover_existing_slave_register(dev: *mut net_device) {
    let net = dev_net(dev);
    rtnl_lock();
    for_each_netdev(net, dev) {
        if netif_is_failover(dev) {
            continue;
        }
        if ether_addr_equal((*dev).perm_addr, (*dev).perm_addr) {
            failover_slave_register(dev);
        }
    }
    rtnl_unlock();
}

// Module initialization
#[no_mangle]
pub unsafe extern "C" fn failover_init() -> c_int {
    register_netdevice_notifier(&mut failover_notifier);
    0
}

#[no_mangle]
pub unsafe extern "C" fn failover_exit() {
    unregister_netdevice_notifier(&mut failover_notifier);
}

// Notifier block
static mut failover_notifier: notifier_block = notifier_block {
    notifier_call: Some(notify_handler),
};

unsafe extern "C" fn notify_handler(nb: *mut notifier_block, event: c_ulong, ptr: *mut c_void) -> c_int {
    let event_dev = netdev_notifier_info_to_dev(ptr);
    if netif_is_failover(event_dev) {
        return NOTIFY_DONE;
    }

    match event {
        NETDEV_REGISTER => failover_slave_register(event_dev),
        NETDEV_UNREGISTER => failover_slave_unregister(event_dev),
        NETDEV_UP | NETDEV_DOWN | NETDEV_CHANGE => failover_slave_link_change(event_dev),
        NETDEV_CHANGENAME => failover_slave_name_change(event_dev),
        _ => NOTIFY_DONE,
    }
}

// Helper functions
fn netdev_notifier_info_to_dev(ptr: *mut c_void) -> *mut net_device {
    // Implementation depends on actual struct layout
    ptr as *mut net_device
}

fn netif_is_failover(dev: *mut net_device) -> bool {
    (*dev).priv_flags & IFF_FAILOVER != 0
}

fn netif_is_failover_slave(dev: *mut net_device) -> bool {
    (*dev).priv_flags & IFF_FAILOVER_SLAVE != 0
}

fn netdev_info(dev: *mut net_device, fmt: *const c_char, ...) {
    // Implementation depends on actual logging system
}

// SAFETY: This function assumes the lock is held and the list is properly initialized
unsafe fn list_for_each_entry(head: *mut list_head, entry: *mut list_head) -> *mut failover {
    let offset = offset_of!(failover, list);
    (entry as *mut u8).offset(-offset) as *mut failover
}
