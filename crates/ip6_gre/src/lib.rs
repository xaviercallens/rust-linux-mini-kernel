#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]

use core::ffi::{c_int, c_void};
use core::panic::PanicInfo;
use core::ptr;
use kernel_types::*;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

pub const IFNAMSIZ: usize = 16;
pub const IP6_GRE_HASH_SIZE_SHIFT: u32 = 5;
pub const IP6_GRE_HASH_SIZE: usize = 1 << IP6_GRE_HASH_SIZE_SHIFT;
pub const IFF_UP: u32 = 1 << 0;
pub const ARPHRD_IP6GRE: c_int = 1;
pub const ARPHRD_ETHER: c_int = 6;
pub const ETH_P_TEB: u16 = 0x6558;
pub const ETH_P_ERSPAN: u16 = 0x22f3;
pub const ETH_P_ERSPAN2: u16 = 0x22f4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
pub struct net {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct dst_cache {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    pub ifindex: c_int,
    pub flags: u32,
    pub type_: c_int,
    pub dev_private: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_tnl_parm {
    pub raddr: in6_addr,
    pub laddr: in6_addr,
    pub i_key: u32,
    pub o_key: u32,
    pub link: c_int,
    pub flags: u16,
    pub proto: u16,
    pub encap_type: u16,
    pub encap_limit: u8,
    pub hop_lmt: u8,
    pub flowinfo: u32,
    pub name: [u8; IFNAMSIZ],
    pub collect_md: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_tnl {
    pub parms: ip6_tnl_parm,
    pub dev: *mut net_device,
    pub net: *mut net,
    pub next: *mut ip6_tnl,
    pub dst_cache: *mut dst_cache,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6gre_net {
    pub tunnels: [*mut ip6_tnl; 4 * IP6_GRE_HASH_SIZE],
    pub collect_md_tun: *mut ip6_tnl,
    pub collect_md_tun_erspan: *mut ip6_tnl,
    pub fb_tunnel_dev: *mut net_device,
}

#[no_mangle]
pub static mut ip6gre_net_id: c_int = 0;

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[no_mangle]
pub unsafe extern "C" fn dev_net(_dev: *mut net_device) -> *mut net {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn net_generic(_net: *mut net, _id: c_int) -> *mut ip6gre_net {
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn htons(v: u16) -> u16 {
    v.to_be()
}

#[no_mangle]
pub unsafe extern "C" fn hash_32(val: u32, bits: u32) -> u32 {
    if bits >= 32 {
        val
    } else {
        val & ((1u32 << bits) - 1)
    }
}

#[no_mangle]
pub unsafe extern "C" fn ipv6_addr_hash(addr: *const in6_addr) -> u32 {
    if addr.is_null() {
        return 0;
    }
    let a = &*addr;
    let mut h = 0u32;
    let mut i = 0usize;
    while i < 16 {
        h = h.wrapping_mul(33).wrapping_add(a.s6_addr[i] as u32);
        i += 1;
    }
    h
}

#[no_mangle]
pub unsafe extern "C" fn ipv6_addr_any(a: *const in6_addr) -> bool {
    if a.is_null() {
        return true;
    }
    let p = &*a;
    let mut i = 0usize;
    while i < 16 {
        if p.s6_addr[i] != 0 {
            return false;
        }
        i += 1;
    }
    true
}

#[no_mangle]
pub unsafe extern "C" fn ipv6_addr_is_multicast(a: *const in6_addr) -> bool {
    if a.is_null() {
        return false;
    }
    (*a).s6_addr[0] == 0xff
}

#[no_mangle]
pub unsafe extern "C" fn ipv6_addr_equal(a: *const in6_addr, b: *const in6_addr) -> bool {
    if a.is_null() || b.is_null() {
        return false;
    }
    (*a).s6_addr == (*b).s6_addr
}

#[no_mangle]
pub unsafe extern "C" fn HASH_ADDR(addr: *const in6_addr) -> u32 {
    if addr.is_null() {
        return 0;
    }
    let hash = ipv6_addr_hash(addr);
    hash_32(hash, IP6_GRE_HASH_SIZE_SHIFT)
}

#[no_mangle]
pub unsafe extern "C" fn HASH_KEY(key: u32) -> u32 {
    (key ^ (key >> 4)) & ((IP6_GRE_HASH_SIZE as u32) - 1)
}

#[no_mangle]
pub unsafe extern "C" fn ip6gre_tunnel_lookup(
    dev: *mut net_device,
    remote: *const in6_addr,
    local: *const in6_addr,
    key: u32,
    gre_proto: u16,
) -> *mut ip6_tnl {
    if dev.is_null() || remote.is_null() || local.is_null() {
        return ptr::null_mut();
    }

    let netp = dev_net(dev);
    if netp.is_null() {
        return ptr::null_mut();
    }

    let h0 = HASH_ADDR(remote) as usize;
    let h1 = HASH_KEY(key) as usize;
    let ign = net_generic(netp, ip6gre_net_id);
    if ign.is_null() {
        return ptr::null_mut();
    }

    let dev_type = if gre_proto == htons(ETH_P_TEB)
        || gre_proto == htons(ETH_P_ERSPAN)
        || gre_proto == htons(ETH_P_ERSPAN2)
    {
        ARPHRD_ETHER
    } else {
        ARPHRD_IP6GRE
    };

    let mut t = (*ign).tunnels[(3 * IP6_GRE_HASH_SIZE) + (h0 ^ h1)];
    while !t.is_null() {
        let t_ref = &*t;
        if ipv6_addr_equal(local, &t_ref.parms.laddr)
            && ipv6_addr_equal(remote, &t_ref.parms.raddr)
            && key == t_ref.parms.i_key
            && !t_ref.dev.is_null()
            && ((*t_ref.dev).flags & IFF_UP != 0)
            && ((*t_ref.dev).type_ == ARPHRD_IP6GRE || (*t_ref.dev).type_ == dev_type)
        {
            return t;
        }
        t = t_ref.next;
    }

    ptr::null_mut()
}