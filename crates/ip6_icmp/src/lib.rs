use core::ffi::c_int;
use core::ptr::NonNull;
use kernel_types::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_icmp {
    pub type_: u8,
    pub code: u8,
    pub checksum: __be16,
    pub un: ip6_icmp_body,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ip6_icmp_body {
    pub u_echo: ip6_icmp_echo,
    pub u_paramprob: ip6_icmp_paramprob,
    pub u_redirect: ip6_icmp_redirect,
    pub u_neighbor: ip6_icmp_neighbor,
    pub u_router: ip6_icmp_router,
    pub u_routersolicit: ip6_icmp_routersolicit,
    pub u_timeexceed: ip6_icmp_timeexceed,
    pub u_unreach: ip6_icmp_unreach,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_icmp_echo {
    pub identifier: __be16,
    pub sequence: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_icmp_paramprob {
    pub pointer: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_icmp_redirect {
    pub target: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_icmp_neighbor {
    pub target: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_icmp_router {
    pub lifetime: __be32,
    pub addr: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_icmp_routersolicit {
    pub reserved: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_icmp_timeexceed {
    pub unused: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_icmp_unreach {
    pub unused: __be32,
}

unsafe extern "C" {
    pub fn ip6_icmp_send(
        skb: *mut sk_buff,
        type_: u8,
        code: u8,
        offset: c_int,
        mtu: __be32,
    ) -> c_int;

    pub fn ip6_icmp_error(
        skb: *mut sk_buff,
        type_: u8,
        code: u8,
        offset: c_int,
        mtu: __be32,
    ) -> c_int;
}

#[inline]
fn skb_cb_as_icmp(skb: NonNull<sk_buff>) -> *mut ip6_icmp {
    let cb_ptr = unsafe { (*skb.as_ptr()).cb.as_mut_ptr() };
    cb_ptr.cast::<ip6_icmp>()
}

pub fn ip6_icmp_send_echo_reply(
    skb: *mut sk_buff,
    type_: u8,
    code: u8,
    offset: c_int,
    mtu: __be32,
) -> c_int {
    let Some(skb_nn) = NonNull::new(skb) else {
        return -1;
    };

    let icmp6h = skb_cb_as_icmp(skb_nn);
    if icmp6h.is_null() {
        return -1;
    }

    let (hdr_type, hdr_code) = unsafe { ((*icmp6h).type_, (*icmp6h).code) };
    if hdr_type != type_ || hdr_code != code {
        return -1;
    }

    unsafe {
        let echo = (*icmp6h).un.u_echo;
        (*icmp6h).un.u_echo = echo;
        (*icmp6h).type_ = type_;
        (*icmp6h).code = code;
        ip6_icmp_send(skb, type_, code, offset, mtu)
    }
}

pub fn ip6_icmp_send_error(
    skb: *mut sk_buff,
    type_: u8,
    code: u8,
    offset: c_int,
    mtu: __be32,
) -> c_int {
    let Some(skb_nn) = NonNull::new(skb) else {
        return -1;
    };

    let icmp6h = skb_cb_as_icmp(skb_nn);
    if icmp6h.is_null() {
        return -1;
    }

    let (hdr_type, hdr_code) = unsafe { ((*icmp6h).type_, (*icmp6h).code) };
    if hdr_type != type_ || hdr_code != code {
        return -1;
    }

    unsafe {
        (*icmp6h).type_ = type_;
        (*icmp6h).code = code;
        ip6_icmp_error(skb, type_, code, offset, mtu)
    }
}