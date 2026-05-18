#![no_std]
#![allow(non_camel_case_types)]

use core::panic::PanicInfo;
use core::ptr;
use kernel_types::*;

pub const IP6_VTI_HASH_SIZE_SHIFT: c_int = 5;
pub const IP6_VTI_HASH_SIZE: c_int = 1 << IP6_VTI_HASH_SIZE_SHIFT;
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

pub const IFNAMSIZ: usize = 16;
pub const IFF_UP: c_int = 0x1;

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_device {
    pub name: [c_char; IFNAMSIZ],
    pub flags: c_int,
    pub tstats: *mut c_void,
    pub dev: *mut net_device,
    pub next: *mut net_device,
    pub r#priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_tnl_parm {
    pub laddr: in6_addr,
    pub raddr: in6_addr,
    pub name: [c_char; IFNAMSIZ],
    pub proto: c_int,
    pub i_key: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_tnl {
    pub parms: ip6_tnl_parm,
    pub dev: *mut net_device,
    pub net: *mut c_void,
    pub next: *mut ip6_tnl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vti6_net {
    pub fb_tnl_dev: *mut net_device,
    pub tnls_r_l: [*mut ip6_tnl; IP6_VTI_HASH_SIZE as usize],
    pub tnls_wc: [*mut ip6_tnl; 1],
    pub tnls: [*mut ip6_tnl; 2],
}

unsafe fn hash(_remote: *const in6_addr, _local: *const in6_addr) -> c_int {
    0
}

unsafe fn get_vti6_net(_net: *mut c_void) -> *mut vti6_net {
    ptr::null_mut()
}

unsafe fn ipv6_addr_equal(_a: *const in6_addr, _b: *const in6_addr) -> bool {
    false
}

unsafe fn ipv6_addr_any(_a: *const in6_addr) -> bool {
    false
}

#[no_mangle]
pub unsafe extern "C" fn vti6_tnl_lookup(
    net: *mut c_void,
    remote: *const in6_addr,
    local: *const in6_addr,
) -> *mut ip6_tnl {
    let ip6n = get_vti6_net(net);
    if ip6n.is_null() {
        return ptr::null_mut();
    }

    let ip6n = &*ip6n;
    let mut t: *mut ip6_tnl;
    let _hash = hash(remote, local);
    let any: in6_addr = core::mem::zeroed();

    for i in 0..IP6_VTI_HASH_SIZE {
        t = ip6n.tnls_r_l[i as usize];
        while !t.is_null() {
            if ipv6_addr_equal(local, &(*t).parms.laddr as *const _)
                && ipv6_addr_equal(remote, &(*t).parms.raddr as *const _)
                && !(*t).dev.is_null()
                && ((*(*t).dev).flags & IFF_UP != 0)
            {
                return t;
            }
            t = (*t).next;
        }
    }

    let _hash = hash(&any as *const _, local);
    for i in 0..IP6_VTI_HASH_SIZE {
        t = ip6n.tnls_r_l[i as usize];
        while !t.is_null() {
            if ipv6_addr_equal(local, &(*t).parms.laddr as *const _)
                && !(*t).dev.is_null()
                && ((*(*t).dev).flags & IFF_UP != 0)
            {
                return t;
            }
            t = (*t).next;
        }
    }

    let _hash = hash(remote, &any as *const _);
    for i in 0..IP6_VTI_HASH_SIZE {
        t = ip6n.tnls_r_l[i as usize];
        while !t.is_null() {
            if ipv6_addr_equal(remote, &(*t).parms.raddr as *const _)
                && !(*t).dev.is_null()
                && ((*(*t).dev).flags & IFF_UP != 0)
            {
                return t;
            }
            t = (*t).next;
        }
    }

    let t_wc = ip6n.tnls_wc[0];
    if !t_wc.is_null() && !(*t_wc).dev.is_null() && ((*(*t_wc).dev).flags & IFF_UP != 0) {
        return t_wc;
    }

    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn vti6_tnl_bucket(
    ip6n: *mut vti6_net,
    p: *const ip6_tnl_parm,
) -> *mut *mut ip6_tnl {
    let remote = &(*p).raddr as *const in6_addr;
    let local = &(*p).laddr as *const in6_addr;
    let h: usize = if !ipv6_addr_any(remote) || !ipv6_addr_any(local) {
        1
    } else {
        0
    };

    if h == 0 {
        &mut (*ip6n).tnls_wc[0]
    } else {
        let hv = hash(remote, local) as usize;
        &mut (*ip6n).tnls_r_l[hv % (IP6_VTI_HASH_SIZE as usize)]
    }
}

#[no_mangle]
pub unsafe extern "C" fn vti6_tnl_link(ip6n: *mut vti6_net, t: *mut ip6_tnl) {
    let tp = vti6_tnl_bucket(ip6n, &(*t).parms as *const _);
    (*t).next = *tp;
    *tp = t;
}

#[no_mangle]
pub unsafe extern "C" fn vti6_tnl_unlink(ip6n: *mut vti6_net, t: *mut ip6_tnl) {
    let mut tp = vti6_tnl_bucket(ip6n, &(*t).parms as *const _);
    while !(*tp).is_null() {
        let iter = *tp;
        if iter == t {
            *tp = (*t).next;
            break;
        }
        tp = &mut (*iter).next;
    }
}