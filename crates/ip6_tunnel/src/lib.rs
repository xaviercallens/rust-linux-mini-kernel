#![no_std]
#![no_main]
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::panic::PanicInfo;
use core::ptr;
use kernel_types::*;

type Net = c_void;
type DstCache = c_void;
type GroCells = c_void;

const IP6_TUNNEL_HASH_SIZE_SHIFT: c_int = 5;
const IP6_TUNNEL_HASH_SIZE: usize = 1usize << (IP6_TUNNEL_HASH_SIZE_SHIFT as usize);
const IFNAMSIZ: usize = 16;
const IFF_UP: c_int = 1 << 0;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENODEV: c_int = -19;
pub const EEXIST: c_int = -17;
pub const E2BIG: c_int = -75;

#[repr(C)]
pub struct net_device {
    pub name: [c_char; IFNAMSIZ],
    pub flags: c_int,
    pub priv_: *mut c_void,
}

#[repr(C)]
pub struct __ip6_tnl_parm {
    pub name: [c_char; IFNAMSIZ],
    pub link: c_int,
    pub mode: c_int,
    pub collect_md: c_int,
    pub raddr: in6_addr,
    pub laddr: in6_addr,
}

#[repr(C)]
pub struct ip6_tnl {
    pub dev: *mut net_device,
    pub net: *mut Net,
    pub dst_cache: *mut DstCache,
    pub gro_cells: *mut GroCells,
    pub next: *mut ip6_tnl,
    pub parms: __ip6_tnl_parm,
}

#[repr(C)]
pub struct ip6_tnl_net {
    pub fb_tnl_dev: *mut net_device,
    pub tnls_r_l: [*mut ip6_tnl; IP6_TUNNEL_HASH_SIZE],
    pub tnls_wc: [*mut ip6_tnl; 1],
    pub tnls: [[*mut ip6_tnl; IP6_TUNNEL_HASH_SIZE]; 2],
    pub collect_md_tun: *mut ip6_tnl,
}

unsafe extern "C" {
    static mut ip6_tnl_net_id: c_int;
    fn net_generic(net: *mut Net, id: c_int) -> *mut ip6_tnl_net;
    fn ipv6_addr_equal(a1: *const in6_addr, a2: *const in6_addr) -> bool;
    fn ipv6_addr_hash(a: *const in6_addr) -> c_uint;
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

unsafe fn in6_addr_any() -> in6_addr {
    core::mem::zeroed()
}

unsafe fn in6_addr_is_any(a: *const in6_addr) -> bool {
    let any = in6_addr_any();
    ipv6_addr_equal(a, &any as *const in6_addr)
}

unsafe fn get_list(mut head: *mut ip6_tnl, mut f: impl FnMut(*mut ip6_tnl)) {
    while !head.is_null() {
        f(head);
        head = (*head).next;
    }
}

#[no_mangle]
pub unsafe extern "C" fn hash_32(val: c_uint, bits: u32) -> c_uint {
    if bits == 0 {
        0
    } else {
        val & ((1u32 << bits) - 1u32)
    }
}

#[no_mangle]
pub unsafe extern "C" fn HASH(addr1: *const in6_addr, addr2: *const in6_addr) -> c_uint {
    let h1 = ipv6_addr_hash(addr1);
    let h2 = ipv6_addr_hash(addr2);
    hash_32(h1 ^ h2, IP6_TUNNEL_HASH_SIZE_SHIFT as u32)
}

#[no_mangle]
pub unsafe extern "C" fn ip6_tnl_lookup(
    net: *mut Net,
    link: c_int,
    remote: *const in6_addr,
    local: *const in6_addr,
) -> *mut ip6_tnl {
    if net.is_null() || remote.is_null() || local.is_null() {
        return ptr::null_mut();
    }

    let ip6n = net_generic(net, ip6_tnl_net_id);
    if ip6n.is_null() {
        return ptr::null_mut();
    }

    let any = in6_addr_any();
    let mut cand: *mut ip6_tnl = ptr::null_mut();

    let h1 = HASH(remote, local) as usize;
    get_list((*ip6n).tnls_r_l[h1], |t| {
        if !ipv6_addr_equal(local, &(*t).parms.laddr as *const in6_addr)
            || !ipv6_addr_equal(remote, &(*t).parms.raddr as *const in6_addr)
            || (*(*t).dev).flags & IFF_UP == 0
        {
            return;
        }
        if link == (*t).parms.link {
            cand = t;
        } else if cand.is_null() {
            cand = t;
        }
    });
    if !cand.is_null() && (*cand).parms.link == link {
        return cand;
    }

    let h2 = HASH(&any as *const in6_addr, local) as usize;
    get_list((*ip6n).tnls_r_l[h2], |t| {
        if !ipv6_addr_equal(local, &(*t).parms.laddr as *const in6_addr)
            || !in6_addr_is_any(&(*t).parms.raddr as *const in6_addr)
            || (*(*t).dev).flags & IFF_UP == 0
        {
            return;
        }
        if link == (*t).parms.link {
            cand = t;
        } else if cand.is_null() {
            cand = t;
        }
    });
    if !cand.is_null() && (*cand).parms.link == link {
        return cand;
    }

    let h3 = HASH(remote, &any as *const in6_addr) as usize;
    get_list((*ip6n).tnls_r_l[h3], |t| {
        if !ipv6_addr_equal(remote, &(*t).parms.raddr as *const in6_addr)
            || !in6_addr_is_any(&(*t).parms.laddr as *const in6_addr)
            || (*(*t).dev).flags & IFF_UP == 0
        {
            return;
        }
        if link == (*t).parms.link {
            cand = t;
        } else if cand.is_null() {
            cand = t;
        }
    });

    if !cand.is_null() {
        return cand;
    }

    if !(*ip6n).collect_md_tun.is_null()
        && !(*(*ip6n).collect_md_tun).dev.is_null()
        && ((*(*(*ip6n).collect_md_tun).dev).flags & IFF_UP) != 0
    {
        return (*ip6n).collect_md_tun;
    }

    if !(*ip6n).tnls_wc[0].is_null()
        && !(*(*ip6n).tnls_wc[0]).dev.is_null()
        && ((*(*(*ip6n).tnls_wc[0]).dev).flags & IFF_UP) != 0
    {
        return (*ip6n).tnls_wc[0];
    }

    ptr::null_mut()
}