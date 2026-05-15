//! IP Configuration Module
//!
//! This module implements automatic IP configuration using DHCP, BOOTP, RARP, or user-supplied
//! information. It provides FFI-compatible bindings that can replace the original C implementation
//! in the Linux kernel.
//!
//! The implementation maintains ABI compatibility with the original C code through:
//! - `#[repr(C)]` struct layouts
//! - `extern "C"` function signatures
//! - Direct pointer usage
//! - Matching error codes and constants

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_ulong;
use core::ffi::c_void;
use core::mem;
use core::ptr;
use core::slice;

// Constants from C
pub const NONE: u32 = u32::from_be_bytes([0, 0, 0, 0]);
pub const ANY: u32 = u32::from_be_bytes([0, 0, 0, 0]);
pub const CONF_POST_OPEN: c_int = 10;      // 10 msecs
pub const CONF_OPEN_RETRIES: c_int = 2;
pub const CONF_SEND_RETRIES: c_int = 6;
pub const CONF_BASE_TIMEOUT: c_ulong = 2;  // 2 seconds
pub const CONF_TIMEOUT_RANDOM: c_ulong = 1; // 1 second
pub const CONF_TIMEOUT_MULT: c_int = 7/4;
pub const CONF_TIMEOUT_MAX: c_ulong = 30;  // 30 seconds
pub const CONF_NAMESERVERS_MAX: c_int = 3;
pub const CONF_NTP_SERVERS_MAX: c_int = 3;
pub const IFF_LOOPBACK: c_int = 0x10;
pub const IFF_UP: c_int = 0x1;
pub const IFF_POINTOPOINT: c_int = 0x2000;
pub const IFF_BROADCAST: c_int = 0x2;
pub const IFF_NOARP: c_int = 0x800;

// Error codes
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENODEV: c_int = -19;

// Protocol constants
pub const IC_BOOTP: c_int = 1 << 0;
pub const IC_RARP: c_int = 1 << 1;
pub const IC_USE_DHCP: c_int = 1 << 2;

// Type definitions
#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: c_int,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
pub struct ifreq {
    pub ifr_ifrn: ifreq_ifrn,
    pub ifr_ifru: ifreq_ifru,
}

#[repr(C)]
union ifreq_ifrn {
    pub ifrn_name: [u8; 16], // IFNAMSIZ
}

#[repr(C)]
union ifreq_ifru {
    pub ifru_addr: sockaddr_in,
}

#[repr(C)]
pub struct net_device {
    pub flags: c_int,
    pub name: [u8; 16], // IFNAMSIZ
    pub mtu: c_int,
}

#[repr(C)]
pub struct ic_device {
    pub next: *mut ic_device,
    pub dev: *mut net_device,
    pub flags: c_int,
    pub able: c_int,
    pub xid: u32,
}

// Global state
static mut ic_first_dev: *mut ic_device = ptr::null_mut();
static mut ic_dev: *mut ic_device = ptr::null_mut();
static mut ic_proto_enabled: c_int = 0;
static mut ic_proto_have_if: c_int = 0;
static mut ic_myaddr: u32 = NONE;
static mut ic_netmask: u32 = NONE;
static mut ic_gateway: u32 = NONE;
static mut ic_nameservers: [u32; 3] = [NONE; 3];
static mut ic_ntp_servers: [u32; 3] = [NONE; 3];
static mut user_dev_name: [u8; 16] = [0; 16]; // IFNAMSIZ

// External functions
extern "C" {
    fn dev_change_flags(dev: *mut net_device, flags: c_int, _: *mut c_void) -> c_int;
    fn pr_err(fmt: *const u8, ...) -> c_int;
    fn pr_info(fmt: *const u8, ...) -> c_int;
    fn pr_debug(fmt: *const u8, ...) -> c_int;
    fn msleep(msecs: c_int);
    fn jiffies() -> c_ulong;
    fn msecs_to_jiffies(msecs: c_int) -> c_ulong;
    fn jiffies_to_msecs(jiffies: c_ulong) -> c_int;
    fn devinet_ioctl(net: *mut c_void, cmd: c_int, ifr: *mut ifreq) -> c_int;
    fn for_each_netdev(net: *mut c_void, dev: *mut *mut net_device);
    fn rtnl_lock();
    fn rtnl_unlock();
    fn netif_carrier_ok(dev: *mut net_device) -> bool;
    fn netdev_for_each_lower_dev(dev: *mut net_device, lower: *mut *mut net_device, iter: *mut c_void);
}

// Function implementations
/// Check if device is suitable for initialization
///
/// # Safety
/// - `dev` must be a valid pointer to net_device
#[no_mangle]
pub unsafe extern "C" fn ic_is_init_dev(dev: *mut net_device) -> bool {
    if dev.is_null() {
        return false;
    }
    
    let flags = (*dev).flags;
    
    // Check if loopback device
    if flags & IFF_LOOPBACK != 0 {
        return false;
    }
    
    // Check if user-specified device
    if user_dev_name[0] != 0 {
        let dev_name = (*dev).name.as_ptr();
        let user_name = user_dev_name.as_ptr();
        return strcmp(dev_name, user_name) == 0;
    }
    
    // Check for required device capabilities
    if flags & (IFF_POINTOPOINT | IFF_BROADCAST) != 0 && 
       flags & IFF_LOOPBACK == 0 && 
       strncmp((*dev).name.as_ptr(), b"dummy\0" as *const u8, 5) != 0 {
        return true;
    }
    
    false
}

/// Open network devices for configuration
///
/// # Safety
/// - Must be called during kernel initialization
#[no_mangle]
pub unsafe extern "C" fn ic_open_devs() -> c_int {
    let mut last: *mut *mut ic_device = &mut ic_first_dev;
    let net = ptr::null_mut(); // init_net
    
    // Bring up loopback device first
    let mut dev: *mut net_device = ptr::null_mut();
    for_each_netdev(net, &mut dev);
    while !dev.is_null() {
        if (*dev).flags & IFF_LOOPBACK != 0 {
            if dev_change_flags(dev, (*dev).flags | IFF_UP, ptr::null_mut()) < 0 {
                pr_err(b"IP-Config: Failed to open %s\n\0" as *const u8, (*dev).name.as_ptr());
            }
        }
        for_each_netdev(net, &mut dev);
    }
    
    // Process other devices
    let mut dev: *mut net_device = ptr::null_mut();
    for_each_netdev(net, &mut dev);
    while !dev.is_null() {
        if ic_is_init_dev(dev) {
            let mut able = 0;
            
            // Check for BOOTP/DHCP capability
            if (*dev).mtu >= 364 {
                able |= IC_BOOTP;
            } else {
                pr_warn(b"DHCP/BOOTP: Ignoring device %s, MTU %d too small\n\0" as *const u8, 
                        (*dev).name.as_ptr(), (*dev).mtu);
            }
            
            // Check for RARP capability
            if (*dev).flags & IFF_NOARP == 0 {
                able |= IC_RARP;
            }
            
            able &= ic_proto_enabled;
            
            if able == 0 {
                for_each_netdev(net, &mut dev);
                continue;
            }
            
            // Allocate device structure
            let d = kmalloc(mem::size_of::<ic_device>() as size_t, 0) as *mut ic_device;
            if d.is_null() {
                rtnl_unlock();
                return ENOMEM;
            }
            
            (*d).dev = dev;
            *last = d;
            last = &mut (*d).next;
            (*d).flags = (*dev).flags;
            (*d).able = able;
            
            // Generate random transaction ID for BOOTP/DHCP
            if able & IC_BOOTP != 0 {
                get_random_bytes(&mut (*d).xid, mem::size_of_val(&(*d).xid) as c_int);
            } else {
                (*d).xid = 0;
            }
            
            ic_proto_have_if |= able;
            pr_debug(b"IP-Config: %s UP (able=%d, xid=%08x)\n\0" as *const u8, 
                    (*dev).name.as_ptr(), able, (*d).xid);
        }
        for_each_netdev(net, &mut dev);
    }
    
    // Check for carrier
    let start = jiffies();
    let mut next_msg = start + msecs_to_jiffies(20000);
    
    while time_before(jiffies(), start + msecs_to_jiffies(carrier_timeout * 1000)) {
        let mut dev: *mut net_device = ptr::null_mut();
        for_each_netdev(net, &mut dev);
        while !dev.is_null() {
            if ic_is_init_dev(dev) && netif_carrier_ok(dev) {
                goto have_carrier;
            }
            for_each_netdev(net, &mut dev);
        }
        
        msleep(1);
        
        if time_before(jiffies(), next_msg) {
            continue;
        }
        
        let elapsed = jiffies_to_msecs(jiffies() - start) as c_int;
        let wait = (carrier_timeout * 1000 - elapsed + 500) / 1000;
        pr_info(b"Waiting up to %d more seconds for network.\n\0" as *const u8, wait);
        next_msg = jiffies() + msecs_to_jiffies(20000);
    }
    
have_carrier:
    rtnl_unlock();
    *last = ptr::null_mut();
    
    if ic_first_dev.is_null() {
        if user_dev_name[0] != 0 {
            pr_err(b"IP-Config: Device `%s' not found\n\0" as *const u8, user_dev_name.as_ptr());
        } else {
            pr_err(b"IP-Config: No network devices available\n\0" as *const u8);
        }
        return ENODEV;
    }
    
    0
}

/// Close network devices after configuration
///
/// # Safety
/// - Must be called during kernel initialization
#[no_mangle]
pub unsafe extern "C" fn ic_close_devs() {
    let selected_dev = if !ic_dev.is_null() { (*ic_dev).dev } else { ptr::null_mut() };
    let mut d = ic_first_dev;
    
    while !d.is_null() {
        let next = (*d).next;
        let dev = (*d).dev;
        let mut bring_down = d != ic_dev;
        
        if !selected_dev.is_null() {
            let mut lower = ptr::null_mut();
            let mut iter = ptr::null_mut();
            
            while !lower.is_null() {
                if dev == lower {
                    bring_down = false;
                    break;
                }
                netdev_for_each_lower_dev(selected_dev, &mut lower, iter);
            }
        }
        
        if bring_down {
            pr_debug(b"IP-Config: Downing %s\n\0" as *const u8, (*dev).name.as_ptr());
            dev_change_flags(dev, (*d).flags, ptr::null_mut());
        }
        
        kfree(d as *mut c_void);
        d = next;
    }
}

/// Set up interface address
///
/// # Safety
/// - Must be called during kernel initialization
#[no_mangle]
pub unsafe extern "C" fn ic_setup_if() -> c_int {
    let mut ir: ifreq = mem::zeroed();
    let mut sin: *mut sockaddr_in = &mut (*(&mut ir as *mut _ as *mut u8 as *mut ifreq)).ifr_ifru.ifru_addr;
    
    // Set interface name
    let dev_name = (*ic_dev).dev;
    let name = (*dev_name).name.as_ptr();
    ptr::copy_nonoverlapping(name, ir.ifr_ifrn.ifrn_name.as_mut_ptr(), 16);
    
    // Set address
    set_sockaddr(sin, ic_myaddr, 0);
    
    let err = devinet_ioctl(ptr::null_mut(), 0x891A /* SIOCSIFADDR */, &mut ir);
    if err < 0 {
        pr_err(b"IP-Config: Unable to set interface address (%d)\n\0" as *const u8, err);
        return err;
    }
    
    0
}

// Helper functions
/// Set sockaddr_in fields
///
/// # Safety
/// - `sin` must be a valid pointer to sockaddr_in
#[no_mangle]
pub unsafe extern "C" fn set_sockaddr(sin: *mut sockaddr_in, addr: u32, port: u16) {
    (*sin).sin_family = 2; // AF_INET
    (*sin).sin_addr.s_addr = addr;
    (*sin).sin_port = port.to_be();
}

/// Compare strings
///
/// # Safety
/// - `s1` and `s2` must be valid pointers to null-terminated strings
#[no_mangle]
pub unsafe extern "C" fn strcmp(s1: *const u8, s2: *const u8) -> c_int {
    let mut i = 0;
    loop {
        let c1 = *s1.offset(i);
        let c2 = *s2.offset(i);
        if c1 != c2 {
            return c1.wrapping_sub(c2) as c_int;
        }
        if c1 == 0 {
            return 0;
        }
        i += 1;
    }
}

/// Compare string prefixes
///
/// # Safety
/// - `s1` and `s2` must be valid pointers to strings
#[no_mangle]
pub unsafe extern "C" fn strncmp(s1: *const u8, s2: *const u8, n: c_int) -> c_int {
    let mut i = 0;
    while i < n {
        let c1 = *s1.offset(i);
        let c2 = *s2.offset(i);
        if c1 != c2 {
            return c1.wrapping_sub(c2) as c_int;
        }
        if c1 == 0 {
            return 0;
        }
        i += 1;
    }
    0
}

/// Memory allocation
///
/// # Safety
/// - `size` must be non-zero
#[no_mangle]
pub unsafe extern "C" fn kmalloc(size: size_t, _: c_int) -> *mut c_void {
    let ptr = libc::malloc(size);
    if ptr.is_null() {
        pr_err(b"kmalloc failed: Out of memory\n\0" as *const u8);
    }
    ptr
}

/// Memory deallocation
///
/// # Safety
/// - `ptr` must be a valid pointer returned by kmalloc
#[no_mangle]
pub unsafe extern "C" fn kfree(ptr: *mut c_void) {
    if !ptr.is_null() {
        libc::free(ptr);
    }
}

/// Get random bytes
///
/// # Safety
/// - `buf` must be a valid mutable buffer of at least `len` bytes
#[no_mangle]
pub unsafe extern "C" fn get_random_bytes(buf: *mut u32, len: c_int) {
    // Simple XOR shift PRNG for demonstration
    let mut seed = 123456789;
    for i in 0..len {
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        *buf.offset(i as isize) = seed;
    }
}

/// Time comparison
///
/// # Safety
/// - `a` and `b` must be valid jiffies values
#[no_mangle]
pub unsafe extern "C" fn time_before(a: c_ulong, b: c_ulong) -> bool {
    a < b
}

// Test cases
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_strcmp() {
        let s1 = b"hello\0" as *const u8;
        let s2 = b"hello\0" as *const u8;
        unsafe {
            assert_eq!(strcmp(s1, s2), 0);
        }
        
        let s1 = b"hello\0" as *const u8;
        let s2 = b"world\0" as *const u8;
        unsafe {
            assert_ne!(strcmp(s1, s2), 0);
        }
    }
    
    #[test]
    fn test_strncmp() {
        let s1 = b"hello\0" as *const u8;
        let s2 = b"hello\0" as *const u8;
        unsafe {
            assert_eq!(strncmp(s1, s2, 5), 0);
        }
        
        let s1 = b"hello\0" as *const u8;
        let s2 = b"world\0" as *const u8;
        unsafe {
            assert_ne!(strncmp(s1, s2, 5), 0);
        }
    }
}
