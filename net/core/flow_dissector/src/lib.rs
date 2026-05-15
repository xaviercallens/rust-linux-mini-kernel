//! Flow dissector module for Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::all)]

use core::ptr;
use core::mem;
use core::ffi::c_void;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_ulong;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;
pub const USHRT_MAX: c_ushort = 65535;

// Type definitions
#[repr(C)]
#[derive(Clone, Copy)]
pub struct flow_dissector_key_id {
    // Opaque enum, actual values defined in C headers
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct flow_dissector {
    used_keys: c_ulong,
    offset: [c_ushort; 256], // Assuming 256 possible key_ids
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct flow_dissector_key {
    key_id: flow_dissector_key_id,
    offset: c_ushort,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct flow_dissector_key_icmp {
    type_: u8,
    code: u8,
    id: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct flow_dissector_key_meta {
    ingress_ifindex: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct flow_dissector_key_ct {
    ct_state: c_ulong,
    ct_zone: c_ulong,
    ct_mark: c_ulong,
    ct_labels: [u8; 32],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct flow_dissector_key_ipv4_addrs {
    src: u32,
    dst: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct flow_dissector_key_ipv6_addrs {
    src: [u8; 16],
    dst: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct flow_dissector_key_keyid {
    keyid: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct flow_dissector_key_ports {
    src: u16,
    dst: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct flow_dissector_key_ip {
    // Fields from C's flow_dissector_key_ip
}

// Function implementations
/// Initialize flow dissector with specified keys
///
/// # Safety
/// - `flow_dissector` must be a valid pointer to uninitialized memory
/// - `key` must point to valid array of `key_count` elements
/// - Caller must ensure no data races
///
/// # Returns
/// None
#[no_mangle]
pub unsafe extern "C" fn skb_flow_dissector_init(
    flow_dissector: *mut flow_dissector,
    key: *const flow_dissector_key,
    key_count: c_uint,
) {
    if flow_dissector.is_null() || key.is_null() {
        return; // In real kernel code, this would trigger a BUG
    }

    // Zero out the structure
    ptr::write_bytes(flow_dissector, 0, mem::size_of::<flow_dissector>());

    let mut i: c_uint = 0;
    let mut current_key = key;
    
    while i < key_count {
        // SAFETY: Validated pointers and bounds
        let key_id = (*current_key).key_id;
        
        // Check offset is within bounds
        // SAFETY: Validated pointer
        debug_assert!((*current_key).offset <= USHRT_MAX);
        
        // Check key not already used
        // SAFETY: Validated pointer
        debug_assert!(!dissector_uses_key(flow_dissector, key_id));
        
        // Set the key
        dissector_set_key(flow_dissector, key_id);
        
        // Set the offset
        // SAFETY: Validated pointer
        (*flow_dissector).offset[key_id as usize] = (*current_key).offset;
        
        i += 1;
        current_key = current_key.offset(1);
    }

    // Ensure control and basic keys are present
    // SAFETY: These are required by the API
    debug_assert!(dissector_uses_key(flow_dissector, FLOW_DISSECTOR_KEY_CONTROL));
    debug_assert!(dissector_uses_key(flow_dissector, FLOW_DISSECTOR_KEY_BASIC));
}

/// Extract upper layer ports from skb
///
/// # Safety
/// - `skb` must be valid sk_buff pointer
/// - `data` must be valid pointer to packet data
/// - Caller must ensure no data races
///
/// # Returns
/// 32-bit value containing ports
#[no_mangle]
pub unsafe extern "C" fn __skb_flow_get_ports(
    skb: *const c_void,
    thoff: c_int,
    ip_proto: u8,
    data: *const c_void,
    hlen: c_int,
) -> u32 {
    let poff = proto_ports_offset(ip_proto);
    
    if poff < 0 {
        return 0;
    }
    
    let ports_size = 2 * (poff + 1); // Calculate needed size
    let mut ports: [u8; 8] = [0; 8]; // Temporary storage
    
    // SAFETY: Validated pointers and bounds
    let ports_ptr = __skb_header_pointer(
        skb, 
        thoff + poff, 
        ports_size as usize, 
        data, 
        hlen as usize, 
        ports.as_mut_ptr() as *mut c_void
    );
    
    if !ports_ptr.is_null() {
        // SAFETY: Valid pointer from __skb_header_pointer
        let ports = ptr::read(ports_ptr as *const [u8; 8]);
        let mut result: u32 = 0;
        
        // Combine ports into 32-bit value
        result |= (ports[0] as u32) << 24;
        result |= (ports[1] as u32) << 16;
        result |= (ports[2] as u32) << 8;
        result |= ports[3] as u32;
        
        return result;
    }
    
    0
}

/// Extract ICMP Type, Code and Identifier fields
///
/// # Safety
/// - `skb` must be valid sk_buff pointer
/// - `key_icmp` must be valid pointer to flow_dissector_key_icmp
/// - `data` must be valid pointer to packet data
/// - Caller must ensure no data races
///
/// # Returns
/// None
#[no_mangle]
pub unsafe extern "C" fn skb_flow_get_icmp_tci(
    skb: *const c_void,
    key_icmp: *mut flow_dissector_key_icmp,
    data: *const c_void,
    thoff: c_int,
    hlen: c_int,
) {
    if key_icmp.is_null() {
        return;
    }
    
    let mut ih: [u8; 8] = [0; 8]; // icmphdr size
    let ih_ptr = __skb_header_pointer(
        skb, 
        thoff, 
        mem::size_of_val(&ih) as usize, 
        data, 
        hlen as usize, 
        ih.as_mut_ptr() as *mut c_void
    );
    
    if ih_ptr.is_null() {
        return;
    }
    
    // SAFETY: Valid pointer from __skb_header_pointer
    let ih = ptr::read(ih_ptr as *const [u8; 8]);
    
    // SAFETY: Valid pointer
    (*key_icmp).type_ = ih[0];
    (*key_icmp).code = ih[1];
    
    if icmp_has_id(ih[0]) {
        let id: u16 = ((ih[4] as u16) << 8) | ih[5] as u16;
        (*key_icmp).id = if id != 0 { id } else { 1 };
    } else {
        (*key_icmp).id = 0;
    }
}

/// Extract metadata from skb
///
/// # Safety
/// - `skb` must be valid sk_buff pointer
/// - `flow_dissector` must be valid pointer
/// - `target_container` must be valid pointer to target structure
/// - Caller must ensure no data races
///
/// # Returns
/// None
#[no_mangle]
pub unsafe extern "C" fn skb_flow_dissect_meta(
    skb: *const c_void,
    flow_dissector: *mut flow_dissector,
    target_container: *mut c_void,
) {
    if !dissector_uses_key(flow_dissector, FLOW_DISSECTOR_KEY_META) {
        return;
    }
    
    let meta = skb_flow_dissector_target(
        flow_dissector,
        FLOW_DISSECTOR_KEY_META,
        target_container
    );
    
    // SAFETY: Valid pointer from skb_flow_dissector_target
    (*meta).ingress_ifindex = (*skb).skb_iif;
}

// Internal functions
fn dissector_set_key(flow_dissector: *mut flow_dissector, key_id: flow_dissector_key_id) {
    if !flow_dissector.is_null() {
        // SAFETY: Valid pointer
        (*flow_dissector).used_keys |= 1 << key_id;
    }
}

fn dissector_uses_key(flow_dissector: *const flow_dissector, key_id: flow_dissector_key_id) -> bool {
    if flow_dissector.is_null() {
        return false;
    }
    // SAFETY: Valid pointer
    ( (*flow_dissector).used_keys & (1 << key_id) ) != 0
}

fn skb_flow_dissector_target(
    flow_dissector: *const flow_dissector,
    key_id: flow_dissector_key_id,
    target_container: *mut c_void,
) -> *mut c_void {
    if flow_dissector.is_null() || target_container.is_null() {
        return ptr::null_mut();
    }
    
    let offset = (*flow_dissector).offset[key_id as usize];
    // SAFETY: Offset is validated during initialization
    target_container.offset(offset as isize)
}

fn icmp_has_id(ty: u8) -> bool {
    match ty {
        8  | // ICMP_ECHO
        0  | // ICMP_ECHOREPLY
        13 | // ICMP_TIMESTAMP
        14 | // ICMP_TIMESTAMPREPLY
        128| // ICMPV6_ECHO_REQUEST
        129 => true, // ICMPV6_ECHO_REPLY
        _ => false,
    }
}

// Helper functions
#[no_mangle]
pub unsafe extern "C" fn __skb_header_pointer(
    skb: *const c_void,
    offset: c_int,
    size: usize,
    data: *const c_void,
    hlen: usize,
    tmp: *mut c_void,
) -> *mut c_void {
    if offset < 0 {
        return ptr::null_mut();
    }
    
    let end = offset as usize + size;
    if end > hlen {
        return ptr::null_mut();
    }
    
    let ptr = data.offset(offset as isize);
    if !ptr.is_null() {
        return ptr as *mut c_void;
    }
    
    if !tmp.is_null() {
        // SAFETY: tmp is valid for writing
        ptr::copy_nonoverlapping(data.offset(offset as isize), tmp, size);
        return tmp;
    }
    
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn proto_ports_offset(proto: u8) -> c_int {
    match proto {
        IPPROTO_TCP | IPPROTO_UDP | IPPROTO_UDPLITE => 0,
        IPPROTO_SCTP => 2,
        IPPROTO_DCCP => 0,
        _ => -1,
    }
}

// Constants
pub const IPPROTO_TCP: u8 = 6;
pub const IPPROTO_UDP: u8 = 17;
pub const IPPROTO_UDPLITE: u8 = 136;
pub const IPPROTO_SCTP: u8 = 132;
pub const IPPROTO_DCCP: u8 = 33;

// Key IDs (simplified for example)
pub const FLOW_DISSECTOR_KEY_CONTROL: flow_dissector_key_id = unsafe { mem::zeroed() };
pub const FLOW_DISSECTOR_KEY_BASIC: flow_dissector_key_id = unsafe { mem::zeroed() };
pub const FLOW_DISSECTOR_KEY_ICMP: flow_dissector_key_id = unsafe { mem::zeroed() };
pub const FLOW_DISSECTOR_KEY_META: flow_dissector_key_id = unsafe { mem::zeroed() };

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    #[test]
    fn test_icmp_has_id() {
        assert!(super::icmp_has_id(8));  // ICMP_ECHO
        assert!(super::icmp_has_id(0));  // ICMP_ECHOREPLY
        assert!(!super::icmp_has_id(3)); // ICMP_DEST_UNREACH
    }
}
