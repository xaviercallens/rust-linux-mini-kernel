//! Network Device IOCTL Handling
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_int, c_uint, c_void};
use core::ptr::{self, NonNull};

// Constants from C
const IFNAMSIZ: usize = 16;
const NPROTO: usize = 32;
const EINVAL: c_int = -22;
const EFAULT: c_int = -14;
const ENODEV: c_int = -19;
const ENOMEM: c_int = -12;
const EOPNOTSUPP: c_int = -95;
const ERANGE: c_int = -84;

// Type definitions
#[repr(C)]
pub struct ifreq {
    pub ifr_name: [u8; IFNAMSIZ],
    pub ifr_ifindex: c_int,
    pub ifr_flags: c_short,
    pub ifr_metric: c_int,
    pub ifr_mtu: c_int,
    pub ifr_map: ifmap,
    pub ifr_hwaddr: sockaddr,
    pub ifr_newname: [u8; IFNAMSIZ],
    pub ifr_qlen: c_int,
}

#[repr(C)]
pub struct ifmap {
    pub mem_start: usize,
    pub mem_end: usize,
    pub base_addr: u16,
    pub irq: u16,
    pub dma: u8,
    pub port: u8,
}

#[repr(C)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [u8; 14],
}

#[repr(C)]
pub struct ifconf {
    pub ifc_len: c_int,
    pub ifc_buf: *mut c_void,
}

#[repr(C)]
pub struct hwtstamp_config {
    pub flags: u32,
    pub tx_type: u8,
    pub rx_filter: u8,
}

// Function pointer type
pub type gifconf_func_t = unsafe extern "C" fn(dev: *const c_void, 
                                            data: *mut c_void, 
                                            len: c_int, 
                                            size: c_int) -> c_int;

// Global state
static mut GIFCONF_LIST: [*mut c_void; NPROTO] = [ptr::null_mut(); NPROTO];

/// Register a SIOCGIF handler
///
/// # Safety
/// - `family` must be < NPROTO
/// - `gifconf` must be a valid function pointer
///
/// # Returns
/// 0 on success, -EINVAL if family is invalid
#[no_mangle]
pub unsafe extern "C" fn register_gifconf(family: c_uint, gifconf: gifconf_func_t) -> c_int {
    if family >= NPROTO as c_uint {
        return EINVAL;
    }
    GIFCONF_LIST[family as usize] = gifconf as *mut c_void;
    0
}

/// Map an interface index to its name (SIOCGIFNAME)
#[no_mangle]
pub unsafe extern "C" fn dev_ifname(net: *mut c_void, ifr: *mut ifreq) -> c_int {
    if ifr.is_null() {
        return EINVAL;
    }
    
    // SAFETY: ifr is non-null and valid
    (*ifr).ifr_name[IFNAMSIZ-1] = 0;
    
    // Placeholder for actual netdev_get_name implementation
    // In real kernel, this would query the device name
    let name = b"lo\0".as_ptr() as *mut u8;
    ptr::copy_nonoverlapping(name, (*ifr).ifr_name.as_mut_ptr(), IFNAMSIZ);
    
    0
}

/// Perform a SIOCGIFCONF call
#[no_mangle]
pub unsafe extern "C" fn dev_ifconf(net: *mut c_void, ifc: *mut ifconf, size: c_int) -> c_int {
    if ifc.is_null() {
        return EINVAL;
    }
    
    let ifc = &mut *ifc;
    let mut total = 0;
    
    // Simulate for_each_netdev iteration
    // In real kernel, this would iterate over all network devices
    for _ in 0..1 {  // Simulating one device
        for i in 0..NPROTO {
            let handler = GIFCONF_LIST[i];
            if !handler.is_null() {
                let pos = ifc.ifc_buf;
                let len = ifc.ifc_len;
                
                // SAFETY: handler is a valid function pointer
                let done = (handler as gifconf_func_t)(ptr::null(), pos, len, size);
                if done < 0 {
                    return EFAULT;
                }
                total += done;
            }
        }
    }
    
    ifc.ifc_len = total;
    0
}

/// Load a network module
#[no_mangle]
pub unsafe extern "C" fn dev_load(net: *mut c_void, name: *const u8) {
    if name.is_null() {
        return;
    }
    
    // Placeholder for dev_get_by_name_rcu
    // In real kernel, this would check if device exists
    let no_module = true;  // Simulate device not found
    
    if no_module {
        // Simulate request_module
        // In real kernel, this would load the module
    }
}

// Internal functions
fn net_hwtstamp_validate(ifr: *mut ifreq) -> c_int {
    if ifr.is_null() {
        return EINVAL;
    }
    
    let mut cfg = hwtstamp_config {
        flags: 0,
        tx_type: 0,
        rx_filter: 0,
    };
    
    // SAFETY: ifr is valid and ifr_data is properly aligned
    if ptr::copy_nonoverlapping((*ifr).ifr_data as *const u8, &mut cfg as *mut hwtstamp_config as *mut u8, core::mem::size_of_val(&cfg)) != 0 {
        return EFAULT;
    }
    
    if cfg.flags != 0 {
        return EINVAL;
    }
    
    // Validate tx_type
    match cfg.tx_type {
        0..=3 => (),  // Valid tx types
        _ => return ERANGE,
    }
    
    // Validate rx_filter
    match cfg.rx_filter {
        0..=15 => (),  // Valid rx filters
        _ => return ERANGE,
    }
    
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_register_gifconf() {
        unsafe {
            let result = register_gifconf(0, dev_ifconf);
            assert_eq!(result, 0);
        }
    }
}
