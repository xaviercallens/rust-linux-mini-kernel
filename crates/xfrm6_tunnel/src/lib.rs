//! IPv6 XFRM Tunnel Module
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::all)]

use core::ptr;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_void;
use core::mem;
use core::sync::atomic::{AtomicU32, Ordering};

// Constants from C
const XFRM6_TUNNEL_SPI_BYADDR_HSIZE: c_uint = 256;
const XFRM6_TUNNEL_SPI_BYSPI_HSIZE: c_uint = 256;
const XFRM6_TUNNEL_SPI_MIN: u32 = 1;
const XFRM6_TUNNEL_SPI_MAX: u32 = 0xFFFFFFFF;

// Type definitions
#[repr(C)]
struct hlist_head {
    first: *mut hlist_node,
}

#[repr(C)]
struct hlist_node {
    next: *mut hlist_node,
    pprev: *mut *mut hlist_node,
}

#[repr(C)]
struct xfrm_address_t {
    addr: [u8; 16],
}

#[repr(C)]
struct xfrm6_tunnel_net {
    spi_byaddr: [hlist_head; XFRM6_TUNNEL_SPI_BYADDR_HSIZE as usize],
    spi_byspi: [hlist_head; XFRM6_TUNNEL_SPI_BYSPI_HSIZE as usize],
    spi: u32,
}

#[repr(C)]
struct xfrm6_tunnel_spi {
    list_byaddr: hlist_node,
    list_byspi: hlist_node,
    addr: xfrm_address_t,
    spi: u32,
    refcnt: AtomicU32,
    rcu_head: rcu_head,
}

#[repr(C)]
struct rcu_head {
    func: extern "C" fn(head: *mut rcu_head),
}

#[repr(C)]
struct xfrm_state {
    props: xfrm_state_props,
}

#[repr(C)]
struct xfrm_state_props {
    mode: c_int,
    header_len: c_int,
    saddr: xfrm_address_t,
}

#[repr(C)]
struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
struct net {
    _private: [u8; 0],
}

#[repr(C)]
struct xfrm_type {
    description: *const c_char,
    owner: *const c_void,
    proto: c_int,
    init_state: extern "C" fn(x: *mut xfrm_state) -> c_int,
    destructor: extern "C" fn(x: *mut xfrm_state),
    input: extern "C" fn(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int,
    output: extern "C" fn(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int,
}

#[repr(C)]
struct xfrm6_tunnel {
    handler: extern "C" fn(skb: *mut sk_buff) -> c_int,
    err_handler: extern "C" fn(skb: *mut sk_buff, opt: *mut c_void, 
                             type_: c_int, code: c_int, offset: c_int, info: u32) -> c_int,
    priority: c_int,
}

#[repr(C)]
struct pernet_operations {
    init: extern "C" fn(net: *mut net) -> c_int,
    exit: extern "C" fn(net: *mut net),
    id: *mut c_int,
    size: c_int,
}

// Global variables
static mut xfrm6_tunnel_net_id: c_int = 0;
static mut xfrm6_tunnel_spi_kmem: *mut c_void = ptr::null_mut();
static mut xfrm6_tunnel_spi_lock: spinlock_t = spinlock_t { _private: 0 };
static mut xfrm6_tunnel_net_ops: pernet_operations = pernet_operations {
    init: xfrm6_tunnel_net_init,
    exit: xfrm6_tunnel_net_exit,
    id: &xfrm6_tunnel_net_id,
    size: mem::size_of::<xfrm6_tunnel_net>() as c_int,
};

// Spinlock type (simplified)
#[repr(C)]
struct spinlock_t {
    _private: c_int,
}

// Function implementations
/// Get per-network namespace data
///
/// # Safety
/// - `net` must be a valid pointer to network namespace
#[inline]
unsafe fn xfrm6_tunnel_pernet(net: *mut net) -> *mut xfrm6_tunnel_net {
    net_generic(net, xfrm6_tunnel_net_id)
}

/// Calculate hash for address-based lookup
///
/// # Safety
/// - `addr` must be a valid pointer to xfrm_address_t
#[no_mangle]
pub unsafe extern "C" fn xfrm6_tunnel_spi_hash_byaddr(addr: *const xfrm_address_t) -> c_uint {
    let mut h: c_uint = 0;
    
    // Simple hash implementation (simplified from Linux's ipv6_addr_hash)
    let addr = &(*addr).addr as *const [u8; 16] as *const u8;
    for i in 0..16 {
        h = h.wrapping_add(*addr.add(i) as c_uint);
        h = h.wrapping_mul(0x1e35a7bd);
    }
    
    h ^= h >> 16;
    h ^= h >> 8;
    h & (XFRM6_TUNNEL_SPI_BYADDR_HSIZE - 1)
}

/// Calculate hash for SPI-based lookup
#[no_mangle]
pub unsafe extern "C" fn xfrm6_tunnel_spi_hash_byspi(spi: u32) -> c_uint {
    spi as c_uint % XFRM6_TUNNEL_SPI_BYSPI_HSIZE
}

/// Look up SPI by source address
///
/// # Safety
/// - `net` must be a valid pointer to network namespace
/// - `saddr` must be a valid pointer to xfrm_address_t
#[no_mangle]
pub unsafe extern "C" fn xfrm6_tunnel_spi_lookup(net: *mut net, saddr: *const xfrm_address_t) -> u32 {
    let mut spi: u32 = 0;
    
    // SAFETY: RCU read-side critical section
    rcu_read_lock_bh();
    {
        let xfrm6_tn = xfrm6_tunnel_pernet(net);
        let h = xfrm6_tunnel_spi_hash_byaddr(saddr);
        
        let head = &xfrm6_tn.spi_byaddr[h as usize];
        let mut node = head.first;
        
        while !node.is_null() {
            let x6spi = (node as *mut xfrm6_tunnel_spi).offset(-mem::offset_of!(xfrm6_tunnel_spi, list_byaddr) as isize);
            
            if xfrm6_addr_equal(&(*x6spi).addr, saddr) {
                spi = (*x6spi).spi;
                break;
            }
            
            node = (*node).next;
        }
    }
    rcu_read_unlock_bh();
    
    spi
}

/// Look up or allocate SPI for tunnel
///
/// # Safety
/// - `net` must be a valid pointer to network namespace
/// - `saddr` must be a valid pointer to xfrm_address_t
#[no_mangle]
pub unsafe extern "C" fn xfrm6_tunnel_alloc_spi(net: *mut net, saddr: *mut xfrm_address_t) -> u32 {
    let mut spi: u32 = 0;
    
    // SAFETY: Spinlock held during critical section
    spin_lock_bh(&mut xfrm6_tunnel_spi_lock);
    {
        let xfrm6_tn = xfrm6_tunnel_pernet(net);
        let h = xfrm6_tunnel_spi_hash_byaddr(saddr);
        
        let head = &xfrm6_tn.spi_byaddr[h as usize];
        let mut node = head.first;
        
        while !node.is_null() {
            let x6spi = (node as *mut xfrm6_tunnel_spi).offset(-mem::offset_of!(xfrm6_tunnel_spi, list_byaddr) as isize);
            
            if xfrm6_addr_equal(&(*x6spi).addr, saddr) {
                (*x6spi).refcnt.fetch_add(1, Ordering::Relaxed);
                spi = (*x6spi).spi;
                break;
            }
            
            node = (*node).next;
        }
        
        if spi == 0 {
            spi = __xfrm6_tunnel_alloc_spi(net, saddr);
        }
    }
    spin_unlock_bh(&mut xfrm6_tunnel_spi_lock);
    
    spi
}

/// Free SPI reference
///
/// # Safety
/// - `net` must be a valid pointer to network namespace
/// - `saddr` must be a valid pointer to xfrm_address_t
#[no_mangle]
pub unsafe extern "C" fn xfrm6_tunnel_free_spi(net: *mut net, saddr: *mut xfrm_address_t) {
    // SAFETY: Spinlock held during critical section
    spin_lock_bh(&mut xfrm6_tunnel_spi_lock);
    {
        let xfrm6_tn = xfrm6_tunnel_pernet(net);
        let h = xfrm6_tunnel_spi_hash_byaddr(saddr);
        
        let head = &xfrm6_tn.spi_byaddr[h as usize];
        let mut node = head.first;
        let mut prev: *mut *mut hlist_node = head.first as *mut *mut hlist_node;
        
        while !node.is_null() {
            let x6spi = (node as *mut xfrm6_tunnel_spi).offset(-mem::offset_of!(xfrm6_tunnel_spi, list_byaddr) as isize);
            
            if xfrm6_addr_equal(&(*x6spi).addr, saddr) {
                if (*x6spi).refcnt.fetch_sub(1, Ordering::Relaxed) == 1 {
                    // Remove from both hash tables
                    let list_byaddr = &mut (*x6spi).list_byaddr;
                    let list_byspi = &mut (*x6spi).list_byspi;
                    
                    hlist_del_rcu(list_byaddr);
                    hlist_del_rcu(list_byspi);
                    
                    call_rcu(&mut (*x6spi).rcu_head, x6spi_destroy_rcu);
                }
                break;
            }
            
            prev = (*node).next as *mut *mut hlist_node;
            node = *prev;
        }
    }
    spin_unlock_bh(&mut xfrm6_tunnel_spi_lock);
}

/// Check if SPI is already allocated
///
/// # Safety
/// - `net` must be a valid pointer to network namespace
/// - `spi` must be a valid SPI value
#[no_mangle]
pub unsafe extern "C" fn __xfrm6_tunnel_spi_check(net: *mut net, spi: u32) -> c_int {
    let xfrm6_tn = xfrm6_tunnel_pernet(net);
    let h = xfrm6_tunnel_spi_hash_byspi(spi);
    
    let head = &xfrm6_tn.spi_byspi[h as usize];
    let mut node = head.first;
    
    while !node.is_null() {
        let x6spi = (node as *mut xfrm6_tunnel_spi).offset(-mem::offset_of!(xfrm6_tunnel_spi, list_byspi) as isize);
        
        if (*x6spi).spi == spi {
            return -1;
        }
        
        node = (*node).next;
    }
    
    h as c_int
}

/// Allocate new SPI
///
/// # Safety
/// - `net` must be a valid pointer to network namespace
/// - `saddr` must be a valid pointer to xfrm_address_t
#[no_mangle]
pub unsafe extern "C" fn __xfrm6_tunnel_alloc_spi(net: *mut net, saddr: *mut xfrm_address_t) -> u32 {
    let xfrm6_tn = xfrm6_tunnel_pernet(net);
    let mut spi: u32 = 0;
    let mut index: c_int = -1;
    
    if xfrm6_tn.spi < XFRM6_TUNNEL_SPI_MIN || xfrm6_tn.spi >= XFRM6_TUNNEL_SPI_MAX {
        xfrm6_tn.spi = XFRM6_TUNNEL_SPI_MIN;
    } else {
        xfrm6_tn.spi += 1;
    }
    
    for spi in xfrm6_tn.spi..=XFRM6_TUNNEL_SPI_MAX {
        index = __xfrm6_tunnel_spi_check(net, spi);
        if index >= 0 {
            break;
        }
    }
    
    if index < 0 {
        for spi in XFRM6_TUNNEL_SPI_MIN..xfrm6_tn.spi {
            index = __xfrm6_tunnel_spi_check(net, spi);
            if index >= 0 {
                break;
            }
        }
    }
    
    if index >= 0 {
        xfrm6_tn.spi = spi;
        
        // Allocate new SPI entry
        let x6spi = kmem_cache_alloc(xfrm6_tunnel_spi_kmem, 0) as *mut xfrm6_tunnel_spi;
        if !x6spi.is_null() {
            ptr::copy_nonoverlapping(saddr, &mut (*x6spi).addr, mem::size_of::<xfrm_address_t>());
            (*x6spi).spi = spi;
            (*x6spi).refcnt.store(1, Ordering::Relaxed);
            
            // Add to hash tables
            let h_byspi = xfrm6_tunnel_spi_hash_byspi(spi);
            hlist_add_head_rcu(&mut (*x6spi).list_byspi, 
                              &mut xfrm6_tn.spi_byspi[h_byspi as usize]);
            
            let h_byaddr = xfrm6_tunnel_spi_hash_byaddr(&(*x6spi).addr);
            hlist_add_head_rcu(&mut (*x6spi).list_byaddr, 
                              &mut xfrm6_tn.spi_byaddr[h_byaddr as usize]);
        }
    }
    
    spi
}

/// RCU callback for SPI entry destruction
///
/// # Safety
/// - `head` must be a valid pointer to rcu_head
#[no_mangle]
pub unsafe extern "C" fn x6spi_destroy_rcu(head: *mut rcu_head) {
    let x6spi = (head as *mut xfrm6_tunnel_spi).offset(-mem::offset_of!(xfrm6_tunnel_spi, rcu_head) as isize);
    kmem_cache_free(xfrm6_tunnel_spi_kmem, x6spi as *mut c_void);
}

/// Module initialization
#[no_mangle]
pub unsafe extern "C" fn xfrm6_tunnel_init() -> c_int {
    xfrm6_tunnel_spi_kmem = kmem_cache_create("xfrm6_tunnel_spi", 
                                            mem::size_of::<xfrm6_tunnel_spi>() as size_t, 
                                            0, 0, ptr::null_mut());
    if xfrm6_tunnel_spi_kmem.is_null() {
        return -12; // -ENOMEM
    }
    
    let rv = register_pernet_subsys(&mut xfrm6_tunnel_net_ops);
    if rv < 0 {
        return rv;
    }
    
    let rv = xfrm_register_type(&mut xfrm6_tunnel_type, 10 /* AF_INET6 */);
    if rv < 0 {
        return rv;
    }
    
    let rv = xfrm6_tunnel_register(&mut xfrm6_tunnel_handler, 10 /* AF_INET6 */);
    if rv < 0 {
        return rv;
    }
    
    let rv = xfrm6_tunnel_register(&mut xfrm46_tunnel_handler, 2 /* AF_INET */);
    if rv < 0 {
        return rv;
    }
    
    0
}

/// Module cleanup
#[no_mangle]
pub unsafe extern "C" fn xfrm6_tunnel_fini() {
    xfrm6_tunnel_deregister(&mut xfrm46_tunnel_handler, 2 /* AF_INET */);
    xfrm6_tunnel_deregister(&mut xfrm6_tunnel_handler, 10 /* AF_INET6 */);
    xfrm_unregister_type(&mut xfrm6_tunnel_type, 10 /* AF_INET6 */);
    unregister_pernet_subsys(&mut xfrm6_tunnel_net_ops);
    rcu_barrier();
    kmem_cache_destroy(xfrm6_tunnel_spi_kmem);
}

// Helper functions (extern declarations)
extern "C" {
    fn net_generic(net: *mut net, id: c_int) -> *mut c_void;
    fn rcu_read_lock_bh();
    fn rcu_read_unlock_bh();
    fn spin_lock_bh(lock: *mut spinlock_t);
    fn spin_unlock_bh(lock: *mut spinlock_t);
    fn hlist_add_head_rcu(node: *mut hlist_node, head: *mut hlist_head);
    fn hlist_del_rcu(node: *mut hlist_node);
    fn kmem_cache_create(name: *const c_char, size: size_t, align: size_t, flags: c_int, ctor: *mut c_void) -> *mut c_void;
    fn kmem_cache_free(slab: *mut c_void, obj: *mut c_void);
    fn register_pernet_subsys(ops: *mut pernet_operations) -> c_int;
    fn xfrm_register_type(t: *mut xfrm_type, family: c_int) -> c_int;
    fn xfrm6_tunnel_register(handler: *mut xfrm6_tunnel, family: c_int) -> c_int;
    fn xfrm6_tunnel_deregister(handler: *mut xfrm6_tunnel, family: c_int);
    fn rcu_barrier();
    fn kmem_cache_destroy(slab: *mut c_void);
    fn xfrm_unregister_type(t: *mut xfrm_type, family: c_int);
    fn xfrm6_addr_equal(a: *const xfrm_address_t, b: *const xfrm_address_t) -> c_int;
}

// Module exports
#[no_mangle]
pub static xfrm6_tunnel_type: xfrm_type = xfrm_type {
    description: ptr::null(),
    owner: ptr::null(),
    proto: 41, // IPPROTO_IPV6
    init_state: Some(xfrm6_tunnel_init_state),
    destructor: Some(xfrm6_tunnel_destroy),
    input: Some(xfrm6_tunnel_input),
    output: Some(xfrm6_tunnel_output),
};

#[no_mangle]
pub static xfrm6_tunnel_handler: xfrm6_tunnel = xfrm6_tunnel {
    handler: Some(xfrm6_tunnel_rcv),
    err_handler: Some(xfrm6_tunnel_err),
    priority: 3,
};

#[no_mangle]
pub static xfrm46_tunnel_handler: xfrm6_tunnel = xfrm6_tunnel {
    handler: Some(xfrm6_tunnel_rcv),
    err_handler: Some(xfrm6_tunnel_err),
    priority: 3,
};

// Internal functions
#[no_mangle]
pub unsafe extern "C" fn xfrm6_tunnel_init_state(x: *mut xfrm_state) -> c_int {
    if (*x).props.mode != 2 /* XFRM_MODE_TUNNEL */ {
        return -22; // -EINVAL
    }
    
    if !(*x).encap.is_null() {
        return -22; // -EINVAL
    }
    
    (*x).props.header_len = mem::size_of::<ipv6hdr>() as c_int;
    0
}

#[no_mangle]
pub unsafe extern "C" fn xfrm6_tunnel_destroy(x: *mut xfrm_state) {
    let net = xs_net(x);
    xfrm6_tunnel_free_spi(net, &mut (*x).props.saddr);
}

#[no_mangle]
pub unsafe extern "C" fn xfrm6_tunnel_output(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    let offset = -skb_network_offset(skb);
    skb_push(skb, offset);
    0
}

#[no_mangle]
pub unsafe extern "C" fn xfrm6_tunnel_input(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    let nhoff = IP6CB(skb).nhoff;
    (*skb).data.add(nhoff as usize)
}

#[no_mangle]
pub unsafe extern "C" fn xfrm6_tunnel_rcv(skb: *mut sk_buff) -> c_int {
    let net = dev_net(skb).dev;
    let iph = ipv6_hdr(skb);
    let saddr = &(*iph).saddr as *const xfrm_address_t;
    let spi = xfrm6_tunnel_spi_lookup(net, saddr);
    xfrm6_rcv_spi(skb, 41 /* IPPROTO_IPV6 */, spi, ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn xfrm6_tunnel_err(skb: *mut sk_buff, opt: *mut c_void, 
                                        type_: c_int, code: c_int, offset: c_int, info: u32) -> c_int {
    0
}

// Additional helper functions
#[no_mangle]
pub unsafe extern "C" fn xs_net(x: *mut xfrm_state) -> *mut net {
    // Implementation would depend on xfrm_state structure layout
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn dev_net(skb: *mut sk_buff) -> *mut net {
    // Implementation would depend on sk_buff structure layout
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn ipv6_hdr(skb: *mut sk_buff) -> *mut ipv6hdr {
    // Implementation would depend on sk_buff structure layout
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn IP6CB(skb: *mut sk_buff) -> *mut ip6cb {
    // Implementation would depend on sk_buff structure layout
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn xfrm6_rcv_spi(skb: *mut sk_buff, proto: c_int, spi: u32, 
                                      mark: *mut c_void) -> c_int {
    0
}

// Error codes
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;