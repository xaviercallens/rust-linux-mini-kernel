#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]

use core::ffi::c_void;
use core::mem;
use core::ptr;
use kernel_types::*;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOPROTOOPT: c_int = -92;
pub const ENOBUFS: c_int = -105;
pub const EADDRINUSE: c_int = -98;
pub const EFAULT: c_int = -14;

pub const SOCK_RAW: c_int = 3;
pub const IPPROTO_RAW: c_int = 255;
pub const GFP_KERNEL: u32 = 0x20;

pub type socklen_t = u32;

#[repr(C)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: u16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

#[repr(C)]
pub struct ipv6_txoptions {
    pub opt_nflen: u32,
    pub opt_flen: u32,
}

#[repr(C)]
pub struct group_source_req {
    pub gsr_interface: u32,
    pub gsr_group: sockaddr_in6,
    pub gsr_source: sockaddr_in6,
}

#[repr(C)]
pub struct group_filter {
    pub gf_interface: u32,
    pub gf_fmode: u32,
    pub gf_numsrc: u32,
    pub gf_group: sockaddr_in6,
    pub gf_slist: *const sockaddr_in6,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct rwlock_t {
    pub raw_lock: u64,
}

#[repr(C)]
pub struct sock {
    pub sk_type: u16,
    pub _pad: [u8; 6],
}

#[repr(C)]
pub struct inet_sock {
    pub sk: sock,
    pub inet_num: u16,
    pub _pad2: [u8; 6],
}

#[repr(C)]
pub struct ip6_ra_chain {
    pub sk: *mut sock,
    pub sel: c_int,
    pub next: *mut ip6_ra_chain,
}

unsafe extern "C" {
    fn write_lock_bh(lock: *mut c_void);
    fn write_unlock_bh(lock: *mut c_void);
    fn kmalloc(size: size_t, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn sock_hold(sk: *mut sock);
    fn sock_put(sk: *mut sock);
}

static mut IP6_RA_CHAIN_HEAD: *mut ip6_ra_chain = ptr::null_mut();
static mut IP6_RA_LOCK: rwlock_t = rwlock_t { raw_lock: 0 };

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ip6_ra_control(sk: *mut sock, sel: c_int) -> c_int {
    if sk.is_null() {
        return EINVAL;
    }

    let isk = sk as *mut inet_sock;
    let sk_type = (*sk).sk_type as c_int;
    let inet_num = (*isk).inet_num as c_int;

    if sk_type != SOCK_RAW || inet_num != IPPROTO_RAW {
        return ENOPROTOOPT;
    }

    let new_ra: *mut ip6_ra_chain = if sel >= 0 {
        let p = kmalloc(mem::size_of::<ip6_ra_chain>() as size_t, GFP_KERNEL);
        if p.is_null() {
            return ENOMEM;
        }
        let ra = p as *mut ip6_ra_chain;
        ptr::write(
            ra,
            ip6_ra_chain {
                sk,
                sel,
                next: ptr::null_mut(),
            },
        );
        ra
    } else {
        ptr::null_mut()
    };

    write_lock_bh((&raw mut IP6_RA_LOCK).cast::<c_void>());

    let mut rap: *mut *mut ip6_ra_chain = &raw mut IP6_RA_CHAIN_HEAD;

    while !(*rap).is_null() {
        let ra = *rap;
        if (*ra).sk == sk {
            if sel >= 0 {
                write_unlock_bh((&raw mut IP6_RA_LOCK).cast::<c_void>());
                if !new_ra.is_null() {
                    kfree(new_ra.cast::<c_void>());
                }
                return EADDRINUSE;
            }

            *rap = (*ra).next;
            write_unlock_bh((&raw mut IP6_RA_LOCK).cast::<c_void>());
            sock_put(sk);
            kfree(ra.cast::<c_void>());
            return 0;
        }
        rap = &raw mut (*ra).next;
    }

    if new_ra.is_null() {
        write_unlock_bh((&raw mut IP6_RA_LOCK).cast::<c_void>());
        return ENOBUFS;
    }

    (*new_ra).next = ptr::null_mut();
    *rap = new_ra;
    sock_hold(sk);
    write_unlock_bh((&raw mut IP6_RA_LOCK).cast::<c_void>());
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ipv6_update_options(
    _sk: *mut sock,
    opt: *mut ipv6_txoptions,
) -> *mut ipv6_txoptions {
    opt
}

#[cfg(not(test))]
#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

#[cfg(not(test))]
#[unsafe(no_mangle)]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}