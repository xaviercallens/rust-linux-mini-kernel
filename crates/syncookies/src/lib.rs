#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::mem;
use core::panic::PanicInfo;
use kernel_types::*;

const COOKIEBITS: u32 = 24;
const COOKIEMASK: u32 = (1 << COOKIEBITS) - 1;
const MAX_SYNCOOKIE_AGE: u32 = 3;

#[repr(C)]
struct Combined {
    saddr: in6_addr,
    daddr: in6_addr,
    count: u32,
    sport: u16,
    dport: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct siphash_key_t {
    key: [u64; 2],
}

#[repr(C)]
pub struct tcphdr {
    pub source: u16,
    pub dest: u16,
    pub seq: u32,
}

static MSSTAB: [u16; 4] = [1280 - 60, 1480 - 60, 1500 - 60, 9000 - 60];
static mut SYNCOKIE6_SECRET: [siphash_key_t; 2] = [siphash_key_t { key: [0; 2] }; 2];

#[inline]
fn cookie_hash(
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    sport: u16,
    dport: u16,
    count: u32,
    c: c_int,
) -> u32 {
    let combined = unsafe {
        Combined {
            saddr: *saddr,
            daddr: *daddr,
            count,
            sport,
            dport,
        }
    };

    unsafe {
        net_get_random_once(
            core::ptr::addr_of_mut!(SYNCOKIE6_SECRET) as *mut c_void,
            mem::size_of::<[siphash_key_t; 2]>() as size_t,
        );
    }

    let size = mem::size_of::<Combined>() - mem::size_of::<u16>();

    unsafe {
        siphash(
            &combined as *const _ as *const c_void,
            size as size_t,
            core::ptr::addr_of!(SYNCOKIE6_SECRET[c as usize]),
        )
    }
}

#[inline]
fn secure_tcp_syn_cookie(
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    sport: u16,
    dport: u16,
    sseq: u32,
    data: u32,
) -> u32 {
    let count = tcp_cookie_time();
    let hash1 = cookie_hash(saddr, daddr, sport, dport, 0, 0);
    let hash2 = cookie_hash(saddr, daddr, sport, dport, count, 1);

    hash1
        .wrapping_add(sseq)
        .wrapping_add(count << COOKIEBITS)
        .wrapping_add((hash2.wrapping_add(data)) & COOKIEMASK)
}

#[inline]
fn check_tcp_syn_cookie(
    cookie: u32,
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    sport: u16,
    dport: u16,
    sseq: u32,
) -> u32 {
    let count = tcp_cookie_time();
    let mut val = cookie;

    val = val.wrapping_sub(cookie_hash(saddr, daddr, sport, dport, 0, 0).wrapping_add(sseq));

    let diff = count.wrapping_sub(val >> COOKIEBITS);
    if diff >= MAX_SYNCOOKIE_AGE {
        return u32::MAX;
    }

    val.wrapping_sub(cookie_hash(
        saddr,
        daddr,
        sport,
        dport,
        count.wrapping_sub(diff),
        1,
    )) & COOKIEMASK
}

#[no_mangle]
pub unsafe extern "C" fn __cookie_v6_init_sequence(
    iph: *const ipv6hdr,
    th: *const tcphdr,
    mssp: *mut u16,
) -> u32 {
    let mut mssind: c_int = MSSTAB.len() as c_int - 1;
    let mss = unsafe { *mssp };

    while mssind > 0 {
        if mss >= MSSTAB[mssind as usize] {
            break;
        }
        mssind -= 1;
    }

    unsafe {
        *mssp = MSSTAB[mssind as usize];
    }

    secure_tcp_syn_cookie(
        unsafe { core::ptr::addr_of!((*iph).saddr) },
        unsafe { core::ptr::addr_of!((*iph).daddr) },
        unsafe { (*th).source },
        unsafe { (*th).dest },
        ntohl(unsafe { (*th).seq }),
        mssind as u32,
    )
}

#[no_mangle]
pub unsafe extern "C" fn __cookie_v6_check(iph: *const ipv6hdr, th: *const tcphdr, cookie: u32) -> c_int {
    let seq = ntohl(unsafe { (*th).seq }).wrapping_sub(1);
    let mssind = check_tcp_syn_cookie(
        cookie,
        unsafe { core::ptr::addr_of!((*iph).saddr) },
        unsafe { core::ptr::addr_of!((*iph).daddr) },
        unsafe { (*th).source },
        unsafe { (*th).dest },
        seq,
    );

    if mssind < MSSTAB.len() as u32 {
        return MSSTAB[mssind as usize] as c_int;
    }
    0
}

unsafe fn net_get_random_once(_ptr: *mut c_void, _len: size_t) {}

unsafe fn siphash(_data: *const c_void, _len: size_t, _key: *const siphash_key_t) -> u32 {
    0
}

fn tcp_cookie_time() -> u32 {
    0
}

fn ntohl(n: u32) -> u32 {
    u32::from_be(n)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}