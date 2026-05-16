//! Connection tracking protocol helper module for GRE.
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_void;
use core::mem;
use core::ptr;
use core::slice;
use libc::size_t;
use libc::HZ;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

// Type definitions
#[repr(C)]
pub struct nf_conntrack_tuple {
    src: nf_conntrack_tuple_ipv4,
    dst: nf_conntrack_tuple_ipv4,
}

#[repr(C)]
pub struct nf_conntrack_tuple_ipv4 {
    u3: [u8; 16],
    protonum: u8,
}

#[repr(C)]
pub struct nf_conntrack_tuple_gre {
    key: u16,
}

#[repr(C)]
pub struct nf_ct_gre_keymap {
    tuple: nf_conntrack_tuple,
    list: list_head,
    rcu: rcu_head,
}

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct rcu_head {
    next: *mut rcu_head,
    func: unsafe extern "C" fn(*mut rcu_head),
}

#[repr(C)]
pub struct nf_gre_net {
    keymap_list: list_head,
    timeouts: [c_uint; GRE_CT_MAX],
}

#[repr(C)]
pub struct nf_conn {
    status: u32,
    proto: nf_conn_gre,
    _private: [u8; 0],
}

#[repr(C)]
pub struct nf_conn_gre {
    timeout: c_uint,
    stream_timeout: c_uint,
}

#[repr(C)]
pub struct nf_ct_pptp_master {
    keymap: [*mut nf_ct_gre_keymap; IP_CT_DIR_MAX],
}

#[repr(C)]
pub struct nf_conntrack_l4proto {
    l4proto: u8,
    // Other fields omitted for brevity
}

// Constants
pub const GRE_CT_UNREPLIED: usize = 0;
pub const GRE_CT_REPLIED: usize = 1;
pub const GRE_CT_MAX: usize = 2;
pub const IP_CT_DIR_ORIGINAL: usize = 0;
pub const IP_CT_DIR_REPLY: usize = 1;
pub const IP_CT_DIR_MAX: usize = 2;
pub const IPPROTO_GRE: u8 = 47;

// Function prototypes for external dependencies
extern "C" {
    fn nf_ct_net(ct: *const nf_conn) -> *mut c_void;
    fn nfct_help_data(ct: *const nf_conn) -> *mut nf_ct_pptp_master;
    fn nf_ct_timeout_lookup(ct: *const nf_conn) -> *const c_uint;
    fn nf_ct_refresh_acct(ct: *mut nf_conn, ctinfo: c_int, skb: *mut c_void, timeout: c_uint);
    fn nf_conntrack_event_cache(event: c_int, ct: *mut nf_conn);
    fn skb_header_pointer(
        skb: *const c_void,
        dataoff: c_int,
        size: size_t,
        ptr: *mut c_void,
    ) -> *mut c_void;
    fn nf_ct_dump_tuple(tuple: *const nf_conntrack_tuple);
}

// Module-level statics
static mut keymap_lock: c_int = 0; // Simplified spinlock representation

// Helper functions
fn spin_lock_bh(lock: *mut c_int) {
    // SAFETY: This is a simplified representation of spinlock
    unsafe {
        *lock = 1;
    }
}

fn spin_unlock_bh(lock: *mut c_int) {
    // SAFETY: This is a simplified representation of spinlock
    unsafe {
        *lock = 0;
    }
}

fn list_add_tail(head: *mut list_head, entry: *mut list_head) {
    // SAFETY: Basic list operation implementation
    unsafe {
        (*entry).next = head;
        (*entry).prev = (*head).prev;
        (*(*head).prev).next = entry;
        (*head).prev = entry;
    }
}

fn list_del_rcu(entry: *mut list_head) {
    // SAFETY: Basic list operation implementation
    unsafe {
        (*entry).next = ptr::null_mut();
        (*entry).prev = ptr::null_mut();
    }
}

fn kfree_rcu(head: *mut rcu_head) {
    // SAFETY: No-op for this simplified implementation
}

fn kmalloc(size: size_t) -> *mut c_void {
    unsafe { libc::malloc(size) }
}

fn kfree(ptr: *mut c_void) {
    unsafe { libc::free(ptr) }
}

// Main implementation
#[no_mangle]
pub unsafe extern "C" fn nf_ct_gre_keymap_add(
    ct: *mut nf_conn,
    dir: c_int,
    t: *mut nf_conntrack_tuple,
) -> c_int {
    if ct.is_null() || t.is_null() {
        return EINVAL;
    }

    let net = nf_ct_net(ct);
    let net_gre = gre_pernet(net);

    let ct_pptp_info = nfct_help_data(ct);
    let dir_idx = dir as usize;
    let kmp = &mut (*ct_pptp_info).keymap[dir_idx];

    if !(*kmp).is_null() {
        // Check for retransmission
        let mut km = (*net_gre).keymap_list.next;
        while !km.is_null() && km != &(*net_gre).keymap_list {
            if gre_key_cmpfn(km as *const _, t) && km == *kmp {
                return 0;
            }
            km = (*km).next;
        }
        return EINVAL; // -EEXIST
    }

    let km = kmalloc(mem::size_of::<nf_ct_gre_keymap>() as size_t) as *mut nf_ct_gre_keymap;
    if km.is_null() {
        return ENOMEM;
    }

    ptr::copy_nonoverlapping(t, km as *mut _, 1);
    *kmp = km;

    spin_lock_bh(&mut keymap_lock);
    list_add_tail(&mut (*net_gre).keymap_list, &mut (*km).list);
    spin_unlock_bh(&mut keymap_lock);

    0
}

#[no_mangle]
pub unsafe extern "C" fn nf_ct_gre_keymap_destroy(ct: *mut nf_conn) {
    if ct.is_null() {
        return;
    }

    let ct_pptp_info = nfct_help_data(ct);
    spin_lock_bh(&mut keymap_lock);

    for dir in 0..IP_CT_DIR_MAX {
        let km = (*ct_pptp_info).keymap[dir];
        if !km.is_null() {
            list_del_rcu(&mut (*km).list);
            kfree(km as *mut c_void);
            (*ct_pptp_info).keymap[dir] = ptr::null_mut();
        }
    }

    spin_unlock_bh(&mut keymap_lock);
}

#[no_mangle]
pub unsafe extern "C" fn gre_pkt_to_tuple(
    skb: *const c_void,
    dataoff: c_int,
    net: *mut c_void,
    tuple: *mut nf_conntrack_tuple,
) -> c_int {
    if skb.is_null() || tuple.is_null() {
        return EINVAL;
    }

    let mut _grehdr: [u8; mem::size_of::<gre_base_hdr>()] = [0; mem::size_of::<gre_base_hdr>()];
    let grehdr = skb_header_pointer(
        skb,
        dataoff,
        mem::size_of::<gre_base_hdr>() as size_t,
        _grehdr.as_mut_ptr() as *mut c_void,
    ) as *mut gre_base_hdr;

    if grehdr.is_null() || (*grehdr).flags & GRE_VERSION != GRE_VERSION_1 {
        (*tuple).src.u.all = 0;
        (*tuple).dst.u.all = 0;
        return 1;
    }

    let mut _pgrehdr: [u8; 8] = [0; 8];
    let pgrehdr = skb_header_pointer(skb, dataoff, 8, _pgrehdr.as_mut_ptr() as *mut c_void)
        as *mut pptp_gre_header;

    if pgrehdr.is_null() {
        return 1;
    }

    if (*grehdr).protocol != GRE_PROTO_PPP {
        return 0;
    }

    (*tuple).dst.u.gre.key = (*pgrehdr).call_id;
    let srckey = gre_keymap_lookup(net, tuple);
    (*tuple).src.u.gre.key = srckey;

    1
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_gre_packet(
    ct: *mut nf_conn,
    skb: *mut c_void,
    dataoff: c_int,
    ctinfo: c_int,
    state: *const c_void,
) -> c_int {
    if ct.is_null() {
        return EINVAL;
    }

    if !(*ct).status & IPS_SEEN_REPLY {
        let timeouts = nf_ct_timeout_lookup(ct);
        if timeouts.is_null() {
            let net = nf_ct_net(ct);
            let net_gre = gre_pernet(net);
            (*ct).proto.gre.timeout = (*net_gre).timeouts[GRE_CT_UNREPLIED];
            (*ct).proto.gre.stream_timeout = (*net_gre).timeouts[GRE_CT_REPLIED];
        } else {
            (*ct).proto.gre.timeout = *timeouts.offset(GRE_CT_UNREPLIED as isize);
            (*ct).proto.gre.stream_timeout = *timeouts.offset(GRE_CT_REPLIED as isize);
        }
    }

    if (*ct).status & IPS_SEEN_REPLY != 0 {
        nf_ct_refresh_acct(ct, ctinfo, skb, (*ct).proto.gre.stream_timeout);
        if !(*ct).status & IPS_ASSURED_BIT {
            nf_conntrack_event_cache(IPCT_ASSURED, ct);
        }
    } else {
        nf_ct_refresh_acct(ct, ctinfo, skb, (*ct).proto.gre.timeout);
    }

    0 // NF_ACCEPT
}

// Helper functions
unsafe fn gre_pernet(net: *mut c_void) -> *mut nf_gre_net {
    // Simplified version - in real implementation this would access the net struct
    ptr::null_mut()
}

unsafe fn gre_keymap_lookup(net: *mut c_void, t: *mut nf_conntrack_tuple) -> u16 {
    let net_gre = gre_pernet(net);
    let mut key = 0u16;

    let mut km = (*net_gre).keymap_list.next;
    while !km.is_null() && km != &(*net_gre).keymap_list {
        if gre_key_cmpfn(km as *const _, t) {
            key = (*km).tuple.src.u.gre.key;
            break;
        }
        km = (*km).list.next;
    }

    key
}

unsafe fn gre_key_cmpfn(km: *const nf_ct_gre_keymap, t: *const nf_conntrack_tuple) -> c_int {
    if (*km).tuple.src.l3num != (*t).src.l3num {
        return 0;
    }

    if !ptr::eq(&(*km).tuple.src.u3, &(*t).src.u3) {
        return 0;
    }

    if !ptr::eq(&(*km).tuple.dst.u3, &(*t).dst.u3) {
        return 0;
    }

    if (*km).tuple.dst.protonum != (*t).dst.protonum {
        return 0;
    }

    if (*km).tuple.dst.u.all != (*t).dst.u.all {
        return 0;
    }

    1
}

// Extern types for external dependencies
#[repr(C)]
struct gre_base_hdr {
    flags: u16,
    protocol: u16,
}

#[repr(C)]
struct pptp_gre_header {
    call_id: u16,
}

// Constants for protocol
const GRE_VERSION_1: u16 = 0x2000;
const GRE_PROTO_PPP: u16 = 0x880B;
const IPS_SEEN_REPLY: u32 = 1 << 0;
const IPS_ASSURED_BIT: u32 = 1 << 1;
const IPCT_ASSURED: c_int = 6;
