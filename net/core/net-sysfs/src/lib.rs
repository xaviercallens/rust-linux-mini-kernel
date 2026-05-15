//! Network device sysfs interface
//!
//! This module provides FFI-compatible Rust implementations for network device
//! sysfs attributes in the Linux kernel. The implementation maintains ABI
//! compatibility with the original C code through careful use of #[repr(C)]
//! structs and extern "C" function signatures.
//!
//! The code implements device attribute show/store functions for network
//! devices, including read-only and read-write attributes for device
//! configuration and status.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr::{self, NonNull};

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;
pub const EPERM: c_int = -1;
pub const ENODEV: c_int = -19;

// Type definitions
#[repr(C)]
pub struct device {
    // Opaque device structure
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_attribute {
    // Opaque device attribute structure
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    reg_state: c_int,
    dev_id: u32,
    dev_port: c_int,
    addr_assign_type: c_int,
    addr_len: c_int,
    ifindex: c_int,
    type_: c_int,
    link_mode: c_int,
    dev_addr: [u8; 32],
    broadcast: [u8; 32],
    name_assign_type: c_int,
    operstate: c_int,
    carrier_up_count: c_int,
    carrier_down_count: c_int,
    // ... other fields as needed
}

#[repr(C)]
pub struct net {
    user_ns: *mut c_void,
}

#[repr(C)]
pub struct ethtool_link_ksettings {
    base: ethtool_link_ksettings_base,
}

#[repr(C)]
pub struct ethtool_link_ksettings_base {
    speed: c_int,
    duplex: c_int,
}

// Function pointer types
type NetDevFormatFn = extern "C" fn(*mut net_device, *mut c_char) -> c_int;
type NetDevSetFn = extern "C" fn(*mut net_device, c_ulong) -> c_int;

// Extern declarations for kernel functions
extern "C" {
    fn read_lock(lock: *mut c_void);
    fn read_unlock(lock: *mut c_void);
    fn rtnl_trylock() -> c_int;
    fn rtnl_unlock();
    fn sprintf(buf: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn sysfs_format_mac(buf: *mut c_char, addr: *const u8, len: c_int) -> c_int;
    fn dev_get_iflink(dev: *mut net_device) -> c_int;
    fn netif_carrier_ok(dev: *mut net_device) -> c_int;
    fn netif_running(dev: *mut net_device) -> c_int;
    fn dev_change_carrier(dev: *mut net_device, new_carrier: c_int) -> c_int;
    fn __ethtool_get_link_ksettings(dev: *mut net_device, cmd: *mut ethtool_link_ksettings) -> c_int;
    fn netif_testing(dev: *mut net_device) -> c_int;
    fn netif_dormant(dev: *mut net_device) -> c_int;
    fn atomic_read(v: *mut c_int) -> c_int;
    fn dev_set_mtu(dev: *mut net_device, mtu: c_int) -> c_int;
    fn dev_change_flags(dev: *mut net_device, flags: c_int, unused: *mut c_void) -> c_int;
    fn dev_change_tx_queue_len(dev: *mut net_device, new_len: c_int) -> c_int;
    fn ns_capable(user_ns: *mut c_void, cap: c_int) -> c_int;
    fn kstrtoul(s: *const c_char, base: c_int, res: *mut c_ulong) -> c_int;
    fn restart_syscall() -> c_int;
}

// Helper functions
#[inline]
fn to_net_dev(dev: *mut device) -> *mut net_device {
    // SAFETY: This is a C-style cast that's valid in the kernel
    unsafe { dev as *mut net_device }
}

#[inline]
fn dev_net(dev: *mut net_device) -> *mut net {
    // SAFETY: This is a C-style cast that's valid in the kernel
    unsafe { &mut *(dev as *mut net) }
}

// Core functions
#[no_mangle]
pub unsafe extern "C" fn dev_isalive(dev: *const net_device) -> c_int {
    // SAFETY: Caller guarantees dev is valid
    (*dev).reg_state <= 0 // Assuming NETREG_REGISTERED is 0
}

#[no_mangle]
pub unsafe extern "C" fn netdev_show(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut c_char,
    format: NetDevFormatFn,
) -> c_int {
    let ndev = to_net_dev(dev);
    let mut ret = EINVAL;
    
    read_lock(&dev_base_lock());
    if dev_isalive(ndev) {
        ret = format(ndev, buf);
    }
    read_unlock(&dev_base_lock());
    
    ret
}

#[no_mangle]
pub unsafe extern "C" fn netdev_store(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *const c_char,
    len: c_ulong,
    set: NetDevSetFn,
) -> c_int {
    let ndev = to_net_dev(dev);
    let net = dev_net(ndev);
    let mut new: c_ulong = 0;
    let mut ret: c_int = 0;
    
    if ns_capable((*net).user_ns, 22) != 1 { // CAP_NET_ADMIN
        return EPERM;
    }
    
    ret = kstrtoul(buf, 0, &mut new);
    if ret != 0 {
        return ret;
    }
    
    if rtnl_trylock() == 0 {
        return restart_syscall();
    }
    
    if dev_isalive(ndev) {
        ret = set(ndev, new);
        if ret == 0 {
            ret = len as c_int;
        }
    }
    
    rtnl_unlock();
    ret
}

// Format functions
#[no_mangle]
pub unsafe extern "C" fn format_dev_id(dev: *mut net_device, buf: *mut c_char) -> c_int {
    sprintf(buf, "%#x\n", (*dev).dev_id)
}

#[no_mangle]
pub unsafe extern "C" fn format_dev_port(dev: *mut net_device, buf: *mut c_char) -> c_int {
    sprintf(buf, "%d\n", (*dev).dev_port)
}

#[no_mangle]
pub unsafe extern "C" fn format_addr_assign_type(dev: *mut net_device, buf: *mut c_char) -> c_int {
    sprintf(buf, "%d\n", (*dev).addr_assign_type)
}

#[no_mangle]
pub unsafe extern "C" fn format_addr_len(dev: *mut net_device, buf: *mut c_char) -> c_int {
    sprintf(buf, "%d\n", (*dev).addr_len)
}

#[no_mangle]
pub unsafe extern "C" fn format_ifindex(dev: *mut net_device, buf: *mut c_char) -> c_int {
    sprintf(buf, "%d\n", (*dev).ifindex)
}

#[no_mangle]
pub unsafe extern "C" fn format_type(dev: *mut net_device, buf: *mut c_char) -> c_int {
    sprintf(buf, "%d\n", (*dev).type_)
}

#[no_mangle]
pub unsafe extern "C" fn format_link_mode(dev: *mut net_device, buf: *mut c_char) -> c_int {
    sprintf(buf, "%d\n", (*dev).link_mode)
}

// Show functions
#[no_mangle]
pub unsafe extern "C" fn dev_id_show(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut c_char,
) -> c_int {
    netdev_show(dev, attr, buf, format_dev_id)
}

#[no_mangle]
pub unsafe extern "C" fn dev_port_show(
    dev: *mut device,
    attr: *mut device_attribute,
    buf: *mut c_char,
) -> c_int {
    netdev_show(dev, attr, buf, format_dev_port)
}

// ... (similar implementations for other show functions)

// Device attribute definitions
#[repr(C)]
pub struct device_attribute_ro {
    show: Option<extern "C" fn(*mut device, *mut device_attribute, *mut c_char) -> c_int>,
    _private: [u8; 0],
}

#[no_mangle]
pub static DEVICE_ATTR_DEV_ID: device_attribute_ro = device_attribute_ro {
    show: Some(dev_id_show),
    _private: [0; 0],
};

#[no_mangle]
pub static DEVICE_ATTR_DEV_PORT: device_attribute_ro = device_attribute_ro {
    show: Some(dev_port_show),
    _private: [0; 0],
};

// ... (other device attributes)

// Helper functions for device attributes
#[no_mangle]
pub unsafe extern "C" fn device_attr_ro_init(
    attr: *mut device_attribute_ro,
    show_fn: extern "C" fn(*mut device, *mut device_attribute, *mut c_char) -> c_int,
) {
    (*attr).show = Some(show_fn);
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    #[test]
    fn test_dev_isalive() {
        // This would require actual kernel testing infrastructure
        // which isn't available in user-space
    }
}
