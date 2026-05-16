//! GRE over IPv4 demultiplexer driver
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};
use libc::{c_int, c_uint, c_void, size_t};

// Constants from C
pub const EINVAL: c_int = -22;
pub const EBUSY: c_int = -16;
pub const ENOSYS: c_int = -38;

// Type definitions
#[repr(C)]
pub struct gre_protocol {
    handler: unsafe extern "C" fn(*mut c_void) -> c_int,
    err_handler: unsafe extern "C" fn(*mut c_void, u32) -> c_int,
    keyerr_handler: unsafe extern "C" fn(*mut c_void, u32) -> c_int,
}

#[repr(C)]
pub struct gre_base_hdr {
    flags: u16,
    protocol: u16,
}

#[repr(C)]
pub struct tnl_ptk_info {
    flags: u16,
    key: u32,
    seq: u32,
    proto: u16,
    hdr_len: u16,
}

#[repr(C)]
pub struct erspan_base_hdr {
    version: u8,
    type_: u8,
    session_id: u32,
}

// Global state
static GREPROTO_MAX: usize = 256; // Assuming this is defined in C
static gre_proto: [AtomicPtr<gre_protocol>; GREPROTO_MAX] =
    unsafe { [AtomicPtr::new(ptr::null_mut()); GREPROTO_MAX] };

// Function implementations
/// Add a GRE protocol handler
///
/// # Safety
/// - `proto` must be a valid pointer to gre_protocol
/// - `version` must be less than GREPROTO_MAX
///
/// # Returns
/// 0 on success, -EINVAL if version invalid, -EBUSY if already registered
#[no_mangle]
pub unsafe extern "C" fn gre_add_protocol(proto: *const gre_protocol, version: u8) -> c_int {
    if version as usize >= GREPROTO_MAX {
        return EINVAL;
    }

    let target = &gre_proto[version as usize];
    let current = target.compare_exchange(
        ptr::null_mut(),
        proto as *const _ as *mut _,
        Ordering::AcqRel,
        Ordering::Relaxed,
    );

    match current {
        Ok(_) => 0,
        Err(_) => EBUSY,
    }
}

/// Remove a GRE protocol handler
///
/// # Safety
/// - `proto` must be a valid pointer to gre_protocol
/// - `version` must be less than GREPROTO_MAX
///
/// # Returns
/// 0 on success, -EINVAL if version invalid, -EBUSY if not registered
#[no_mangle]
pub unsafe extern "C" fn gre_del_protocol(proto: *const gre_protocol, version: u8) -> c_int {
    if version as usize >= GREPROTO_MAX {
        return EINVAL;
    }

    let target = &gre_proto[version as usize];
    let current = target.compare_exchange(
        proto as *const _ as *mut _,
        ptr::null_mut(),
        Ordering::AcqRel,
        Ordering::Relaxed,
    );

    match current {
        Ok(_) => {
            // SAFETY: synchronize_rcu is required after protocol removal
            synchronize_rcu();
            0
        }
        Err(_) => EBUSY,
    }
}

/// Parse GRE header and populate tnl_ptk_info
///
/// # Safety
/// - `skb` must be a valid pointer to socket buffer
/// - `tpi` must be a valid mutable pointer to tnl_ptk_info
/// - `csum_err` must be a valid mutable pointer to bool
///
/// # Returns
/// Header length on success, error code on failure
#[no_mangle]
pub unsafe extern "C" fn gre_parse_header(
    skb: *mut c_void,
    tpi: *mut tnl_ptk_info,
    csum_err: *mut bool,
    proto: u16,
    nhs: c_int,
) -> c_int {
    let skb = skb as *mut u8;
    let nhs = nhs as usize;

    // Check minimum header size
    if !pskb_may_pull(skb, (nhs + core::mem::size_of::<gre_base_hdr>()) as usize) {
        return EINVAL;
    }

    let greh = (skb.add(nhs)) as *const gre_base_hdr;
    if (greh as *const u8).read_volatile() & (GRE_VERSION | GRE_ROUTING) != 0 {
        return EINVAL;
    }

    // SAFETY: tpi is valid pointer
    (*tpi).flags = gre_flags_to_tnl_flags((*greh).flags);
    let mut hdr_len = gre_calc_hlen((*tpi).flags);

    if !pskb_may_pull(skb, (nhs + hdr_len) as usize) {
        return EINVAL;
    }

    let greh = (skb.add(nhs)) as *const gre_base_hdr;
    (*tpi).proto = (*greh).protocol;

    let options = (greh as *const u8).add(core::mem::size_of::<gre_base_hdr>()) as *const u32;

    if (*greh).flags & GRE_CSUM != 0 {
        if !skb_checksum_simple_validate(skb) {
            skb_checksum_try_convert(skb, IPPROTO_GRE, null_compute_pseudo);
        } else if !csum_err.is_null() {
            *csum_err.write_volatile(true);
            return EINVAL;
        }
        options.add(1);
    }

    if (*greh).flags & GRE_KEY != 0 {
        (*tpi).key = *options;
        options.add(1);
    } else {
        (*tpi).key = 0;
    }

    if (*greh).flags & GRE_SEQ != 0 {
        (*tpi).seq = *options;
        options.add(1);
    } else {
        (*tpi).seq = 0;
    }

    // WCCP version handling
    if (*greh).flags == 0 && (*tpi).proto == htons(ETH_P_WCCP) {
        let val = skb_header_pointer(skb, nhs + hdr_len, 1, 0 as *mut u8) as *const u8;
        if val.is_null() {
            return EINVAL;
        }
        (*tpi).proto = proto;
        if (*val as u8 & 0xF0) != 0x40 {
            hdr_len += 4;
        }
    }

    (*tpi).hdr_len = hdr_len as u16;

    // ERSPAN handling
    if ((*greh).protocol == htons(ETH_P_ERSPAN) && hdr_len != 4)
        || (*greh).protocol == htons(ETH_P_ERSPAN2)
    {
        if !pskb_may_pull(
            skb,
            (nhs + hdr_len + core::mem::size_of::<erspan_base_hdr>()) as usize,
        ) {
            return EINVAL;
        }

        let ershdr = (skb.add(nhs + hdr_len)) as *const erspan_base_hdr;
        (*tpi).key = cpu_to_be32(get_session_id(ershdr));
    }

    hdr_len as c_int
}

#[no_mangle]
pub unsafe extern "C" fn gre_rcv(skb: *mut c_void) -> c_int {
    if !pskb_may_pull(skb as *mut u8, 12) {
        goto_drop(skb);
        return -1;
    }

    let ver = (*skb as *mut u8).add(1).read_volatile() & 0x7f;
    if ver as usize >= GREPROTO_MAX {
        goto_drop(skb);
        return -1;
    }

    rcu_read_lock();
    let proto = rcu_dereference(&gre_proto[ver as usize]);
    if proto.is_null() || (*proto).handler.is_null() {
        rcu_read_unlock();
        goto_drop(skb);
        return -1;
    }

    let ret = ((*proto).handler)(skb);
    rcu_read_unlock();
    ret
}

#[no_mangle]
pub unsafe extern "C" fn gre_err(skb: *mut c_void, info: u32) -> c_int {
    let iph = skb as *const u8;
    let ver = (*iph.add((*iph as *const u16 as *const u16) << 2) + 1).read_volatile() & 0x7f;
    if ver as usize >= GREPROTO_MAX {
        return EINVAL;
    }

    rcu_read_lock();
    let proto = rcu_dereference(&gre_proto[ver as usize]);
    if !proto.is_null() && !(*proto).err_handler.is_null() {
        ((*proto).err_handler)(skb, info);
    } else {
        rcu_read_unlock();
        return ENOSYS;
    }
    rcu_read_unlock();
    0
}

#[no_mangle]
pub unsafe extern "C" fn gre_init() -> c_int {
    if inet_add_protocol(&net_gre_protocol, IPPROTO_GRE) < 0 {
        return ENOSYS;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn gre_exit() {
    inet_del_protocol(&net_gre_protocol, IPPROTO_GRE);
}

// Helper functions (assumed to exist in kernel)
#[repr(C)]
struct net_protocol {
    handler: unsafe extern "C" fn(*mut c_void) -> c_int,
    err_handler: unsafe extern "C" fn(*mut c_void, u32) -> c_int,
    netns_ok: c_int,
}

static net_gre_protocol: net_protocol = net_protocol {
    handler: gre_rcv,
    err_handler: gre_err,
    netns_ok: 1,
};

// Dummy implementations for kernel functions
unsafe fn synchronize_rcu() {}
unsafe fn rcu_read_lock() {}
unsafe fn rcu_read_unlock() {}
unsafe fn rcu_dereference<T>(ptr: *const AtomicPtr<T>) -> *const T {
    ptr::null()
}
unsafe fn pskb_may_pull(skb: *mut u8, len: usize) -> bool {
    true
}
unsafe fn skb_checksum_simple_validate(skb: *mut c_void) -> bool {
    true
}
unsafe fn skb_checksum_try_convert(skb: *mut c_void, proto: c_int, cb: unsafe extern "C" fn()) {}
unsafe fn null_compute_pseudo() {}
unsafe fn skb_header_pointer(
    skb: *mut c_void,
    offset: usize,
    size: usize,
    data: *mut c_void,
) -> *mut c_void {
    ptr::null_mut()
}
unsafe fn gre_flags_to_tnl_flags(flags: u16) -> u16 {
    flags
}
unsafe fn gre_calc_hlen(flags: u16) -> usize {
    4
}
unsafe fn get_session_id(ershdr: *const erspan_base_hdr) -> u32 {
    (*ershdr).session_id
}
unsafe fn cpu_to_be32(x: u32) -> u32 {
    x
}
unsafe fn htons(x: u16) -> u16 {
    x
}
unsafe fn inet_add_protocol(proto: *const net_protocol, num: c_int) -> c_int {
    0
}
unsafe fn inet_del_protocol(proto: *const net_protocol, num: c_int) {}
unsafe fn goto_drop(skb: *mut c_void) {
    // SAFETY: Kernel function to free skb
    unsafe { ptr::null_mut() };
}

// Module metadata
#[cfg(feature = "kernel_module")]
mod module {
    use super::*;

    #[no_mangle]
    pub static gre_init: unsafe extern "C" fn() -> c_int = super::gre_init;
    #[no_mangle]
    pub static gre_exit: unsafe extern "C" fn() = super::gre_exit;
}
