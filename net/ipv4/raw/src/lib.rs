//! IPv4 RAW Socket Implementation
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.
//!
//! Handles raw IP sockets, ICMP filtering, and error reporting for the IPv4 protocol stack.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clang::too_many_arguments)]

use core::ptr;
use core::mem;
use core::sync::atomic::{AtomicUsize, Ordering};

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;
pub const EHOSTUNREACH: c_int = -101;
pub const EMSGSIZE: c_int = -92;
pub const EPROTO: c_int = -75;

// Type definitions
#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct iphdr {
    pub ihl: u8,
    pub version: u8,
    pub tos: u8,
    pub tot_len: u16,
    pub id: u16,
    pub frag_off: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub check: u16,
    pub saddr: u32,
    pub daddr: u32,
}

#[repr(C)]
pub struct icmphdr {
    pub type_: u8,
    pub code: u8,
    pub checksum: u16,
    pub unused: u16,
    pub gateway: u32,
    pub __pad: [u8; 4],
    pub mtu: u16,
}

#[repr(C)]
pub struct sock {
    pub sk_prot: *const c_void,
    pub sk_bound_dev_if: c_int,
    pub sk_state: c_int,
    pub sk_drops: AtomicUsize,
    pub sk_priority: u32,
    pub sk_mark: u32,
    pub sk_net: *const c_void,
    pub sk_node: *mut c_void,
}

#[repr(C)]
pub struct inet_sock {
    pub inet_num: u16,
    pub inet_daddr: u32,
    pub inet_rcv_saddr: u32,
}

#[repr(C)]
pub struct raw_hashinfo {
    pub lock: *mut c_void, // Placeholder for rwlock_t
    pub ht: [*mut c_void; RAW_HTABLE_SIZE], // Placeholder for hlist_head
}

#[repr(C)]
pub struct raw_frag_vec {
    pub msg: *mut c_void, // Placeholder for msghdr
    pub hdr: [u8; 1],
    pub hlen: c_int,
}

// Function pointers for socket operations
#[repr(C)]
pub struct sock_ops {
    pub h: *mut c_void, // Placeholder for raw_hashinfo
}

// Exported symbols
#[no_mangle]
pub static mut raw_v4_hashinfo: raw_hashinfo = raw_hashinfo {
    lock: ptr::null_mut(),
    ht: [ptr::null_mut(); RAW_HTABLE_SIZE],
};

// Function implementations
/// Add socket to raw hash table
///
/// # Safety
/// - `sk` must be a valid pointer to a socket
/// - Caller must hold the appropriate locks
#[no_mangle]
pub unsafe extern "C" fn raw_hash_sk(sk: *mut sock) -> c_int {
    let h = (*sk).sk_prot as *mut sock_ops;
    let h_raw_hash = (*h).h as *mut raw_hashinfo;
    let h_raw_hash = h_raw_hash as *mut raw_hashinfo;
    
    let inet_sk = (sk as *mut inet_sock).offset(0);
    let num = (*inet_sk).inet_num;
    let head = &mut (*h_raw_hash).ht[(num & (RAW_HTABLE_SIZE - 1)) as usize];
    
    // SAFETY: Caller is responsible for lock ordering and validity
    write_lock_bh((*h_raw_hash).lock);
    
    sk_add_node(sk, *head);
    sock_prot_inuse_add((*sk).sk_net, (*sk).sk_prot, 1);
    
    write_unlock_bh((*h_raw_hash).lock);
    
    0
}

/// Remove socket from raw hash table
///
/// # Safety
/// - `sk` must be a valid pointer to a socket
/// - Caller must hold the appropriate locks
#[no_mangle]
pub unsafe extern "C" fn raw_unhash_sk(sk: *mut sock) {
    let h = (*sk).sk_prot as *mut sock_ops;
    let h_raw_hash = (*h).h as *mut raw_hashinfo;
    
    // SAFETY: Caller is responsible for lock ordering and validity
    write_lock_bh((*h_raw_hash).lock);
    
    if sk_del_node_init(sk) {
        sock_prot_inuse_add((*sk).sk_net, (*sk).sk_prot, -1);
    }
    
    write_unlock_bh((*h_raw_hash).lock);
}

/// Lookup raw socket in hash table
///
/// # Safety
/// - `net` must be a valid network namespace
/// - `sk` must be a valid pointer to a socket
/// - `dif` and `sdif` must be valid interface indices
#[no_mangle]
pub unsafe extern "C" fn __raw_v4_lookup(
    net: *mut c_void,
    sk: *mut sock,
    num: u16,
    raddr: u32,
    laddr: u32,
    dif: c_int,
    sdif: c_int
) -> *mut sock {
    let mut sk = sk;
    
    while !sk.is_null() {
        let inet_sk = sk as *mut inet_sock;
        
        if net_eq((*sk).sk_net, net) && 
           (*inet_sk).inet_num == num &&
           (!((*inet_sk).inet_daddr != 0 && (*inet_sk).inet_daddr != raddr) &&
            !((*inet_sk).inet_rcv_saddr != 0 && (*inet_sk).inet_rcv_saddr != laddr) &&
            raw_sk_bound_dev_eq(net, (*sk).sk_bound_dev_if, dif, sdif)) {
            break;
        }
        
        sk = sk_next(sk);
    }
    
    sk
}

/// Filter ICMP messages based on socket options
///
/// # Safety
/// - `sk` must be a valid pointer to a socket
/// - `skb` must be a valid pointer to a sk_buff
#[no_mangle]
pub unsafe extern "C" fn icmp_filter(
    sk: *const sock,
    skb: *const c_void
) -> c_int {
    let mut _hdr: icmphdr = mem::zeroed();
    let mut hdr: *const icmphdr = ptr::null();
    
    // SAFETY: Caller guarantees skb is valid
    let transport_offset = skb_transport_offset(skb);
    let result = skb_header_pointer(skb, transport_offset, &mut _hdr as *mut _ as *mut c_void, &mut _hdr as *mut _ as *mut c_void);
    
    if result.is_null() {
        return 1;
    }
    
    hdr = result;
    
    if (*hdr).type_ < 32 {
        let data = (*raw_sk(sk)).filter.data;
        return if (1u32 << (*hdr).type_) & data != 0 { 1 } else { 0 };
    }
    
    0
}

/// Deliver raw IP packet to appropriate sockets
///
/// # Safety
/// - `skb` must be a valid pointer to a sk_buff
/// - `iph` must be a valid pointer to an iphdr
/// - `hash` must be a valid hash index
#[no_mangle]
pub unsafe extern "C" fn raw_v4_input(
    skb: *mut c_void,
    iph: *const iphdr,
    hash: c_int
) -> c_int {
    let sdif = inet_sdif(skb);
    let dif = inet_iif(skb);
    let head = &(*raw_v4_hashinfo.ht[hash as usize]);
    let net = dev_net(skb);
    
    if hlist_empty(head) {
        return 0;
    }
    
    let sk = __raw_v4_lookup(net, __sk_head(head), (*iph).protocol, (*iph).saddr, (*iph).daddr, dif, sdif);
    let mut delivered = 0;
    
    while !sk.is_null() {
        delivered = 1;
        
        if ((*iph).protocol != IPPROTO_ICMP || icmp_filter(sk, skb) == 0) &&
           ip_mc_sf_allow(sk, (*iph).daddr, (*iph).saddr, skb_dev(skb)->ifindex, sdif) {
            
            let clone = skb_clone(skb, GFP_ATOMIC);
            
            if !clone.is_null() {
                raw_rcv(sk, clone);
            }
        }
        
        sk = __raw_v4_lookup(net, sk_next(sk), (*iph).protocol, (*iph).saddr, (*iph).daddr, dif, sdif);
    }
    
    delivered
}

// Helper functions (declared as extern "C" for FFI compatibility)
extern "C" {
    fn write_lock_bh(lock: *mut c_void);
    fn write_unlock_bh(lock: *mut c_void);
    fn sk_add_node(sk: *mut sock, head: *mut c_void);
    fn sk_del_node_init(sk: *mut sock) -> c_int;
    fn sock_prot_inuse_add(net: *mut c_void, prot: *mut c_void, delta: c_int);
    fn skb_transport_offset(skb: *mut c_void) -> c_int;
    fn skb_header_pointer(skb: *mut c_void, offset: c_int, size: *mut c_void, data: *mut c_void) -> *mut c_void;
    fn skb_clone(skb: *mut c_void, gfp_mask: c_int) -> *mut c_void;
    fn raw_sk(sk: *mut sock) -> *mut c_void;
    fn ip_mc_sf_allow(sk: *mut sock, daddr: u32, saddr: u32, ifindex: c_int, sdif: c_int) -> c_int;
    fn skb_dev(skb: *mut c_void) -> *mut c_void;
    fn dev_net(skb: *mut c_void) -> *mut c_void;
    fn __sk_head(head: *mut c_void) -> *mut sock;
    fn sk_next(sk: *mut sock) -> *mut sock;
    fn raw_rcv(sk: *mut sock, skb: *mut c_void);
}

// Constants
const RAW_HTABLE_SIZE: usize = 128;
const IPPROTO_ICMP: u8 = 1;
const GFP_ATOMIC: c_int = 1;
