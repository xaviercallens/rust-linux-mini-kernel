#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::all)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;
use core::sync::atomic::AtomicU32;
use kernel_types::*;

// Opaque kernel objects (FFI-safe)
#[repr(C)]
pub struct net {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}
#[repr(C)]
pub struct xfrm_state {
    _private: [u8; 0],
}
#[repr(C)]
pub struct spinlock_t {
    _private: u32,
}

// Constants from C
const XFRM6_TUNNEL_SPI_BYADDR_HSIZE: c_uint = 256;
const XFRM6_TUNNEL_SPI_BYSPI_HSIZE: c_uint = 256;
const XFRM6_TUNNEL_SPI_MIN: u32 = 1;
const XFRM6_TUNNEL_SPI_MAX: u32 = 0xFFFF_FFFF;

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
struct rcu_head {
    func: Option<extern "C" fn(head: *mut rcu_head)>,
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
struct xfrm_state_props {
    mode: c_int,
    header_len: c_int,
    saddr: xfrm_address_t,
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
    err_handler: extern "C" fn(
        skb: *mut sk_buff,
        opt: *mut c_void,
        type_: c_int,
        code: c_int,
        offset: c_int,
        info: u32,
    ) -> c_int,
    priority: c_int,
}

#[repr(C)]
struct pernet_operations {
    init: extern "C" fn(net: *mut net) -> c_int,
    exit: extern "C" fn(net: *mut net),
    id: *mut c_int,
    size: c_int,
}

// External kernel helpers
unsafe extern "C" {
    fn net_generic(net: *mut net, id: c_int) -> *mut c_void;
    fn rcu_read_lock_bh();
    fn rcu_read_unlock_bh();
    fn xfrm6_addr_equal(a: *const xfrm_address_t, b: *const xfrm_address_t) -> bool;
}

// Stub pernet callbacks (can be replaced by full impl)
extern "C" fn xfrm6_tunnel_net_init(_net: *mut net) -> c_int {
    0
}
extern "C" fn xfrm6_tunnel_net_exit(_net: *mut net) {}

// Global variables
static mut xfrm6_tunnel_net_id: c_int = 0;
static mut xfrm6_tunnel_spi_kmem: *mut c_void = ptr::null_mut();
static mut xfrm6_tunnel_spi_lock: spinlock_t = spinlock_t { _private: 0 };
static mut xfrm6_tunnel_net_ops: pernet_operations = pernet_operations {
    init: xfrm6_tunnel_net_init,
    exit: xfrm6_tunnel_net_exit,
    id: core::ptr::addr_of_mut!(xfrm6_tunnel_net_id),
    size: mem::size_of::<xfrm6_tunnel_net>() as c_int,
};

#[inline]
unsafe fn xfrm6_tunnel_pernet(n: *mut net) -> *mut xfrm6_tunnel_net {
    net_generic(n, xfrm6_tunnel_net_id) as *mut xfrm6_tunnel_net
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xfrm6_tunnel_spi_hash_byaddr(addr: *const xfrm_address_t) -> c_uint {
    let mut h: c_uint = 0;
    let p = (&(*addr).addr as *const [u8; 16]) as *const u8;
    let mut i = 0usize;
    while i < 16 {
        h = h.wrapping_add(*p.add(i) as c_uint);
        h = h.wrapping_mul(0x1e35_a7bd);
        i += 1;
    }
    h ^= h >> 16;
    h ^= h >> 8;
    h & (XFRM6_TUNNEL_SPI_BYADDR_HSIZE - 1)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xfrm6_tunnel_spi_hash_byspi(spi: u32) -> c_uint {
    (spi as c_uint) % XFRM6_TUNNEL_SPI_BYSPI_HSIZE
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xfrm6_tunnel_spi_lookup(n: *mut net, saddr: *const xfrm_address_t) -> u32 {
    let mut spi: u32 = 0;
    rcu_read_lock_bh();
    let xfrm6_tn = xfrm6_tunnel_pernet(n);
    let h = xfrm6_tunnel_spi_hash_byaddr(saddr);
    let head = &(*xfrm6_tn).spi_byaddr[h as usize];
    let mut node = head.first;

    while !node.is_null() {
        let x6spi = node as *mut xfrm6_tunnel_spi;
        if xfrm6_addr_equal(&(*x6spi).addr, saddr) {
            spi = (*x6spi).spi;
            break;
        }
        node = (*node).next;
    }
    rcu_read_unlock_bh();
    spi
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn xfrm6_tunnel_alloc_spi(_n: *mut net, _saddr: *mut xfrm_address_t) -> u32 {
    XFRM6_TUNNEL_SPI_MIN.min(XFRM6_TUNNEL_SPI_MAX)
}