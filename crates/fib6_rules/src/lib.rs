//! IPv6 Routing Policy Rules
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ffi::c_void;
use core::mem;
use core::ptr;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENETUNREACH: c_int = -101;
pub const EACCES: c_int = -13;
pub const EAGAIN: c_int = -11;

// Type definitions
#[repr(C)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
pub struct rt6key {
    pub addr: in6_addr,
    pub plen: u8,
}

#[repr(C)]
pub struct fib_rule {
    pub action: u8,
    pub table: u32,
    pub l3mdev: u8,
    pub flags: u32,
    pub suppress_prefixlen: u8,
    pub suppress_ifgroup: i32,
    pub ip_proto: u8,
    pub sport_range: [u16; 2],
    pub dport_range: [u16; 2],
    pub fr_net: *mut c_void, // struct net*
}

#[repr(C)]
pub struct fib6_rule {
    pub common: fib_rule,
    pub src: rt6key,
    pub dst: rt6key,
    pub tclass: u8,
}

#[repr(C)]
pub struct flowi6 {
    pub daddr: in6_addr,
    pub saddr: in6_addr,
    pub flowlabel: u32,
    pub fl6_sport: u16,
    pub fl6_dport: u16,
    pub flowi6_proto: u8,
}

#[repr(C)]
pub struct fib6_result {
    pub f6i: *mut c_void, // struct rt6_info*
    pub nh: *mut c_void,  // struct fib_info*
    pub rt6: *mut c_void, // struct rt6_info*
}

#[repr(C)]
pub struct fib_lookup_arg {
    pub lookup_ptr: extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> c_int,
    pub lookup_data: *mut c_void,
    pub result: *mut fib6_result,
    pub flags: u32,
}

#[repr(C)]
pub struct net {
    pub ipv6: ipv6_net,
}

#[repr(C)]
pub struct ipv6_net {
    pub fib6_has_custom_rules: u8,
    pub fib6_rules_require_fldissect: u32,
    pub fib6_rules_ops: *mut c_void, // struct fib_rules_ops*
    pub fib6_local_tbl: *mut c_void, // struct fib6_table*
    pub fib6_main_tbl: *mut c_void,  // struct fib6_table*
    pub fib6_null_entry: *mut c_void, // struct rt6_info*
    pub ip6_null_entry: *mut c_void,  // struct rt6_info*
    pub ip6_blk_hole_entry: *mut c_void, // struct rt6_info*
    pub ip6_prohibit_entry: *mut c_void, // struct rt6_info*
}

// Function implementations
/// Check if rule matches all addresses
///
/// # Safety
/// - `rule` must be a valid pointer to fib_rule
///
/// # Returns
/// true if rule matches all addresses
#[no_mangle]
pub unsafe extern "C" fn fib6_rule_matchall(rule: *const fib_rule) -> bool {
    if rule.is_null() {
        return false;
    }
    
    // SAFETY: Rule is non-null and valid (caller guarantees)
    let r = (rule as *const fib6_rule).offset(-mem::offset_of!(fib6_rule, common) as isize);
    let r = &*(r as *const fib6_rule);
    
    if r.src.plen != 0 || r.dst.plen != 0 || r.tclass != 0 {
        return false;
    }
    
    // Call base implementation
    fib_rule_matchall(rule)
}

/// Check if rule is default
///
/// # Safety
/// - `rule` must be a valid pointer to fib_rule
///
/// # Returns
/// true if rule is default
#[no_mangle]
pub unsafe extern "C" fn fib6_rule_default(rule: *const fib_rule) -> bool {
    if rule.is_null() {
        return false;
    }
    
    // SAFETY: Rule is non-null and valid (caller guarantees)
    let rule = &*rule;
    
    if rule.action != 0 || rule.l3mdev != 0 {
        return false;
    }
    
    if rule.table != 254 && rule.table != 253 {
        return false;
    }
    
    true
}

#[no_mangle]
pub extern "C" fn fib6_rules_dump(net: *mut net, nb: *mut c_void, extack: *mut c_void) -> c_int {
    if net.is_null() || nb.is_null() {
        return EINVAL;
    }
    
    // SAFETY: net and nb are valid (caller guarantees)
    fib_rules_dump(net, nb, 10 /* AF_INET6 */, extack)
}

#[no_mangle]
pub extern "C" fn fib6_rules_seq_read(net: *mut net) -> u32 {
    if net.is_null() {
        return 0;
    }
    
    // SAFETY: net is valid (caller guarantees)
    fib_rules_seq_read(net, 10 /* AF_INET6 */)
}

#[no_mangle]
pub extern "C" fn fib6_lookup(
    net: *mut net,
    oif: c_int,
    fl6: *mut flowi6,
    res: *mut fib6_result,
    flags: c_int
) -> c_int {
    if net.is_null() || fl6.is_null() || res.is_null() {
        return EINVAL;
    }
    
    // SAFETY: net and fl6 are valid (caller guarantees)
    let net = &*net;
    let mut arg = fib_lookup_arg {
        lookup_ptr: fib6_table_lookup,
        lookup_data: &oif as *const _ as *mut c_void,
        result: res,
        flags: FIB_LOOKUP_NOREF,
    };
    
    if net.ipv6.fib6_has_custom_rules != 0 {
        l3mdev_update_flow(net, fl6 as *mut c_void);
        return fib_rules_lookup(
            net.ipv6.fib6_rules_ops,
            fl6 as *mut c_void,
            flags,
            &mut arg as *mut _ as *mut c_void
        );
    }
    
    let err = fib6_table_lookup(
        net,
        net.ipv6.fib6_local_tbl,
        oif,
        fl6,
        res,
        flags
    );
    
    if err == 0 && (*res).f6i != net.ipv6.fib6_null_entry {
        return 0;
    }
    
    fib6_table_lookup(
        net,
        net.ipv6.fib6_main_tbl,
        oif,
        fl6,
        res,
        flags
    )
}

// ... (remaining functions would follow the same pattern)

// Helper functions (simplified for example)
#[no_mangle]
pub extern "C" fn fib6_table_lookup(
    net: *mut net,
    table: *mut c_void,
    oif: c_int,
    fl6: *mut flowi6,
    res: *mut fib6_result,
    flags: c_int
) -> c_int {
    // Implementation would follow C code logic
    0
}

#[no_mangle]
pub extern "C" fn fib_rules_lookup(
    ops: *mut c_void,
    fl: *mut c_void,
    flags: c_int,
    arg: *mut c_void
) -> c_int {
    // Implementation would follow C code logic
    0
}

#[no_mangle]
pub extern "C" fn fib_rules_dump(
    net: *mut net,
    nb: *mut c_void,
    af: c_int,
    extack: *mut c_void
) -> c_int {
    // Implementation would follow C code logic
    0
}

#[no_mangle]
pub extern "C" fn fib_rules_seq_read(net: *mut net, af: c_int) -> u32 {
    // Implementation would follow C code logic
    0
}

#[no_mangle]
pub extern "C" fn l3mdev_update_flow(net: *mut net, fl: *mut c_void) {
    // Implementation would follow C code logic
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    #[test]
    fn test_fib6_rule_default() {
        let rule = fib_rule {
            action: 0,
            table: 254,
            l3mdev: 0,
            ..Default::default()
        };
        
        unsafe {
            assert!(super::fib6_rule_default(&rule as *const _));
        }
    }
}
```

Note: This is a simplified and partial implementation focusing on the core concepts. A complete translation would require implementing all the functions and helper routines from the original C code, handling all the complex logic, and ensuring proper memory management. The actual implementation would also need to handle the various kernel-specific functions and data structures that are referenced in the original code.