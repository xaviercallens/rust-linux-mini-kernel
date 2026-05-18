#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::c_void;
use core::mem::size_of;
use core::panic::PanicInfo;
use kernel_types::{c_int, c_uint, size_t};

pub const HZ: c_uint = 100;
pub const CTA_TIMEOUT_GENERIC_TIMEOUT: usize = 1;
pub const CTA_TIMEOUT_GENERIC_MAX: usize = 2;
pub const ENOSPC: c_int = -12;

pub const NLA_U32: c_uint = 1;
pub const IPPROTO_RAW: u8 = 255;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nlattr {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct NfGenericNet {
    timeout: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct NlaPolicy {
    type_: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct NfCtnlTimeout {
    nlattr_to_obj: Option<
        unsafe extern "C" fn(tb: *mut *mut nlattr, net: *mut c_void, data: *mut c_void) -> c_int,
    >,
    obj_to_nlattr: Option<unsafe extern "C" fn(skb: *mut c_void, data: *const c_void) -> c_int>,
    nlattr_max: c_int,
    obj_size: size_t,
    nla_policy: *const NlaPolicy,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NfConntrackL4proto {
    l4proto: u8,
    #[cfg(CONFIG_NF_CONNTRACK_TIMEOUT)]
    ctnl_timeout: NfCtnlTimeout,
}

#[cfg(CONFIG_NF_CONNTRACK_TIMEOUT)]
#[no_mangle]
static generic_timeout_nla_policy: [NlaPolicy; CTA_TIMEOUT_GENERIC_MAX + 1] = {
    let mut arr = [NlaPolicy { type_: 0 }; CTA_TIMEOUT_GENERIC_MAX + 1];
    arr[CTA_TIMEOUT_GENERIC_TIMEOUT] = NlaPolicy { type_: NLA_U32 };
    arr
};

#[no_mangle]
pub static nf_conntrack_l4proto_generic: NfConntrackL4proto = NfConntrackL4proto {
    l4proto: IPPROTO_RAW,
    #[cfg(CONFIG_NF_CONNTRACK_TIMEOUT)]
    ctnl_timeout: NfCtnlTimeout {
        nlattr_to_obj: Some(generic_timeout_nlattr_to_obj),
        obj_to_nlattr: Some(generic_timeout_obj_to_nlattr),
        nlattr_max: CTA_TIMEOUT_GENERIC_MAX as c_int,
        obj_size: size_of::<c_uint>() as size_t,
        nla_policy: generic_timeout_nla_policy.as_ptr(),
    },
};

unsafe extern "C" {
    fn nf_generic_pernet(net: *mut c_void) -> *mut NfGenericNet;
    fn nla_get_be32(attr: *const nlattr) -> u32;
    fn nla_put_be32(skb: *mut c_void, type_: c_int, data: u32) -> c_int;
    fn ntohl(x: u32) -> u32;
    fn htonl(x: u32) -> u32;
}

#[inline(always)]
unsafe extern "C" fn nf_ct_generic_timeout() -> c_uint {
    600 * HZ
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_generic_init_net(net: *mut c_void) {
    let gn = nf_generic_pernet(net);
    (*gn).timeout = nf_ct_generic_timeout();
}

#[no_mangle]
pub unsafe extern "C" fn generic_timeout_nlattr_to_obj(
    tb: *mut *mut nlattr,
    net: *mut c_void,
    data: *mut c_void,
) -> c_int {
    let gn = nf_generic_pernet(net);
    let mut timeout = data as *mut c_uint;
    let gn_timeout = &mut (*gn).timeout;

    if timeout.is_null() {
        timeout = gn_timeout as *mut c_uint;
    }

    let attr = *tb.add(CTA_TIMEOUT_GENERIC_TIMEOUT);

    if !attr.is_null() {
        let value = nla_get_be32(attr);
        *timeout = ntohl(value) * HZ;
    } else {
        *timeout = *gn_timeout;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn generic_timeout_obj_to_nlattr(
    skb: *mut c_void,
    data: *const c_void,
) -> c_int {
    let timeout = data as *const c_uint;
    let timeout_val = *timeout;

    if nla_put_be32(skb, CTA_TIMEOUT_GENERIC_TIMEOUT as c_int, htonl(timeout_val / HZ)) != 0 {
        return ENOSPC;
    }

    0
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}