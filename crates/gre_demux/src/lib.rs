#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::panic::PanicInfo;
use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};
use kernel_types::*;

pub const EINVAL: c_int = -22;
pub const EBUSY: c_int = -16;
pub const ENOSYS: c_int = -38;

pub const GRE_VERSION: u16 = 0x7000;
pub const GRE_ROUTING: u16 = 0x4000;
pub const GRE_CSUM: u16 = 0x8000;
pub const GRE_KEY: u16 = 0x2000;
pub const GRE_SEQ: u16 = 0x1000;

pub const IPPROTO_GRE: c_int = 47;

pub const ETH_P_WCCP: u16 = 0x883E;
pub const ETH_P_ERSPAN: u16 = 0x88BE;
pub const ETH_P_ERSPAN2: u16 = 0x22EB;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gre_protocol {
    pub handler: unsafe extern "C" fn(*mut c_void) -> c_int,
    pub err_handler: unsafe extern "C" fn(*mut c_void, u32) -> c_int,
    pub keyerr_handler: unsafe extern "C" fn(*mut c_void, u32) -> c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gre_base_hdr {
    pub flags: u16,
    pub protocol: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tnl_ptk_info {
    pub flags: u16,
    pub key: u32,
    pub seq: u32,
    pub proto: u16,
    pub hdr_len: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erspan_base_hdr {
    pub version: u8,
    pub type_: u8,
    pub session_id: u32,
}

pub const GREPROTO_MAX: usize = 256;

static mut gre_proto: [AtomicPtr<gre_protocol>; GREPROTO_MAX] =
    [const { AtomicPtr::new(ptr::null_mut()) }; GREPROTO_MAX];

unsafe extern "C" {
    fn synchronize_rcu();

    fn pskb_may_pull(skb: *mut c_void, len: size_t) -> bool;
    fn skb_checksum_simple_validate(skb: *mut c_void) -> bool;
    fn skb_checksum_try_convert(
        skb: *mut c_void,
        proto: c_int,
        compute_pseudo: unsafe extern "C" fn(*mut c_void) -> c_int,
    ) -> c_int;
    fn null_compute_pseudo(skb: *mut c_void) -> c_int;
    fn gre_flags_to_tnl_flags(flags: u16) -> u16;
    fn gre_calc_hlen(flags: u16) -> u16;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn rust_eh_personality() {}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gre_add_protocol(proto: *const gre_protocol, version: u8) -> c_int {
    if (version as usize) >= GREPROTO_MAX {
        return EINVAL;
    }

    let target = unsafe { &gre_proto[version as usize] };
    match target.compare_exchange(
        ptr::null_mut(),
        proto as *mut gre_protocol,
        Ordering::AcqRel,
        Ordering::Relaxed,
    ) {
        Ok(_) => 0,
        Err(_) => EBUSY,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gre_del_protocol(proto: *const gre_protocol, version: u8) -> c_int {
    if (version as usize) >= GREPROTO_MAX {
        return EINVAL;
    }

    let target = unsafe { &gre_proto[version as usize] };
    match target.compare_exchange(
        proto as *mut gre_protocol,
        ptr::null_mut(),
        Ordering::AcqRel,
        Ordering::Relaxed,
    ) {
        Ok(_) => {
            unsafe { synchronize_rcu() };
            0
        }
        Err(_) => EBUSY,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn gre_parse_header(
    skb: *mut c_void,
    tpi: *mut tnl_ptk_info,
    csum_err: *mut bool,
    _proto: u16,
    nhs: c_int,
) -> c_int {
    if skb.is_null() || tpi.is_null() || csum_err.is_null() || nhs < 0 {
        return EINVAL;
    }

    let nhs_usize = nhs as usize;
    if unsafe { !pskb_may_pull(skb, (nhs_usize + core::mem::size_of::<gre_base_hdr>()) as size_t) }
    {
        return EINVAL;
    }

    let base = unsafe { (skb as *mut u8).add(nhs_usize) as *const gre_base_hdr };
    let gre_flags = unsafe { (*base).flags };

    if (gre_flags & (GRE_VERSION | GRE_ROUTING)) != 0 {
        return EINVAL;
    }

    unsafe {
        (*tpi).flags = gre_flags_to_tnl_flags(gre_flags);
        (*tpi).hdr_len = gre_calc_hlen((*tpi).flags);
    }

    let hdr_len = unsafe { (*tpi).hdr_len as usize };
    if unsafe { !pskb_may_pull(skb, (nhs_usize + hdr_len) as size_t) } {
        return EINVAL;
    }

    let greh = unsafe { (skb as *mut u8).add(nhs_usize) as *const gre_base_hdr };

    unsafe {
        (*tpi).proto = (*greh).protocol;
        (*tpi).key = 0;
        (*tpi).seq = 0;
        *csum_err = false;
    }

    if (gre_flags & GRE_CSUM) != 0 {
        let ok = unsafe { skb_checksum_simple_validate(skb) };
        if !ok {
            let r = unsafe { skb_checksum_try_convert(skb, IPPROTO_GRE, null_compute_pseudo) };
            if r != 0 {
                unsafe { *csum_err = true };
            }
        }
    }

    unsafe { (*tpi).hdr_len as c_int }
}