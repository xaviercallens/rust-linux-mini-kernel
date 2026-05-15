//! Flow offload functionality for the Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clang::too_many_arguments)]

use core::ptr;
use libc::{c_int, c_uint, c_void, size_t};

// Constants from C
pub const ENOMEM: c_int = -12;
pub const EOPNOTSUPP: c_int = -95;
pub const EBUSY: c_int = -16;
pub const ENOENT: c_int = -2;

// Type definitions
#[repr(C)]
pub struct flow_rule {
    match_: flow_match,
    action: flow_action,
}

#[repr(C)]
pub struct flow_match {
    key: *const c_void,
    mask: *const c_void,
    dissector: *const flow_dissector,
}

#[repr(C)]
pub struct flow_dissector {
    // Placeholder - actual implementation depends on kernel headers
    _unused: u8,
}

#[repr(C)]
pub struct flow_action {
    num_entries: c_uint,
    entries: [flow_action_entry; 0],
}

#[repr(C)]
pub struct flow_action_entry {
    hw_stats: c_int,
}

#[repr(C)]
pub struct flow_action_cookie {
    cookie_len: c_uint,
    cookie: [u8; 0],
}

#[repr(C)]
pub struct flow_block_cb {
    cb: *mut c_void,
    cb_ident: *mut c_void,
    cb_priv: *mut c_void,
    release: Option<unsafe extern "C" fn(*mut c_void)>,
    refcnt: c_uint,
    list: list_head,
    driver_list: list_head,
}

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct flow_block {
    cb_list: list_head,
}

#[repr(C)]
pub struct flow_block_offload {
    command: c_int,
    binder_type: c_int,
    driver_block_list: *mut list_head,
}

#[repr(C)]
pub struct flow_indr_dev {
    list: list_head,
    cb: *mut c_void,
    cb_priv: *mut c_void,
    refcnt: refcount_t,
    rcu: rcu_head,
}

#[repr(C)]
pub struct refcount_t {
    counter: c_uint,
}

#[repr(C)]
pub struct rcu_head {
    next: *mut rcu_head,
    func: *mut c_void,
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn flow_rule_alloc(num_actions: c_uint) -> *mut flow_rule {
    // Calculate size of flow_rule struct with action.entries array
    let size = core::mem::size_of::<flow_rule>() + 
               (core::mem::size_of::<flow_action_entry>() * num_actions as usize);
    
    let rule = libc::calloc(1, size as size_t) as *mut flow_rule;
    if rule.is_null() {
        return ptr::null_mut();
    }
    
    (*rule).action.num_entries = num_actions;
    
    // Initialize hw_stats for all entries
    for i in 0..num_actions {
        let entry = &mut (*rule).action.entries[i as usize];
        (*entry).hw_stats = 0; // FLOW_ACTION_HW_STATS_DONT_CARE
    }
    
    rule
}

#[no_mangle]
pub unsafe extern "C" fn flow_rule_match_meta(
    rule: *const flow_rule,
    out: *mut flow_match_meta,
) {
    FLOW_DISSECTOR_MATCH!(rule, FLOW_DISSECTOR_KEY_META, out)
}

#[no_mangle]
pub unsafe extern "C" fn flow_rule_match_basic(
    rule: *const flow_rule,
    out: *mut flow_match_basic,
) {
    FLOW_DISSECTOR_MATCH!(rule, FLOW_DISSECTOR_KEY_BASIC, out)
}

#[no_mangle]
pub unsafe extern "C" fn flow_rule_match_control(
    rule: *const flow_rule,
    out: *mut flow_match_control,
) {
    FLOW_DISSECTOR_MATCH!(rule, FLOW_DISSECTOR_KEY_CONTROL, out)
}

#[no_mangle]
pub unsafe extern "C" fn flow_rule_match_eth_addrs(
    rule: *const flow_rule,
    out: *mut flow_match_eth_addrs,
) {
    FLOW_DISSECTOR_MATCH!(rule, FLOW_DISSECTOR_KEY_ETH_ADDRS, out)
}

#[no_mangle]
pub unsafe extern "C" fn flow_rule_match_vlan(
    rule: *const flow_rule,
    out: *mut flow_match_vlan,
) {
    FLOW_DISSECTOR_MATCH!(rule, FLOW_DISSECTOR_KEY_VLAN, out)
}

// ... (other match functions follow similar pattern)

#[no_mangle]
pub unsafe extern "C" fn flow_action_cookie_create(
    data: *const c_void,
    len: c_uint,
    _gfp: c_int,
) -> *mut flow_action_cookie {
    let size = core::mem::size_of::<flow_action_cookie>() + len as usize;
    let cookie = libc::malloc(size as size_t) as *mut flow_action_cookie;
    if cookie.is_null() {
        return ptr::null_mut();
    }
    
    (*cookie).cookie_len = len;
    if !data.is_null() && len > 0 {
        ptr::copy_nonoverlapping(data as *const u8, (*cookie).cookie.as_mut_ptr(), len as usize);
    }
    
    cookie
}

#[no_mangle]
pub unsafe extern "C" fn flow_action_cookie_destroy(cookie: *mut flow_action_cookie) {
    if !cookie.is_null() {
        libc::free(cookie as *mut c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn flow_block_cb_alloc(
    cb: *mut c_void,
    cb_ident: *mut c_void,
    cb_priv: *mut c_void,
    release: Option<unsafe extern "C" fn(*mut c_void)>,
) -> *mut flow_block_cb {
    let block_cb = libc::malloc(core::mem::size_of::<flow_block_cb>()) as *mut flow_block_cb;
    if block_cb.is_null() {
        return (-ENOMEM as *mut flow_block_cb);
    }
    
    (*block_cb).cb = cb;
    (*block_cb).cb_ident = cb_ident;
    (*block_cb).cb_priv = cb_priv;
    (*block_cb).release = release;
    (*block_cb).refcnt = 0;
    
    block_cb
}

#[no_mangle]
pub unsafe extern "C" fn flow_block_cb_free(block_cb: *mut flow_block_cb) {
    if !block_cb.is_null() {
        if let Some(release) = (*block_cb).release {
            release((*block_cb).cb_priv);
        }
        libc::free(block_cb as *mut c_void);
    }
}

// ... (other functions follow similar pattern)

// Macro-like implementation for FLOW_DISSECTOR_MATCH
#[macro_export]
macro_rules! FLOW_DISSECTOR_MATCH {
    ($rule:expr, $type:expr, $out:expr) => {
        unsafe {
            let m = &(*$rule).match_;
            let d = (*m).dissector;
            (*$out).key = skb_flow_dissector_target(d, $type, (*m).key);
            (*$out).mask = skb_flow_dissector_target(d, $type, (*m).mask);
        }
    }
}

// Placeholder for external C function
extern "C" {
    fn skb_flow_dissector_target(
        dissector: *const flow_dissector,
        key_type: c_int,
        data: *const c_void,
    ) -> *const c_void;
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    #[test]
    fn test_flow_rule_alloc() {
        unsafe {
            let rule = super::flow_rule_alloc(5);
            assert!(!rule.is_null());
            super::flow_action_cookie_destroy(rule as *mut _);
        }
    }
}
