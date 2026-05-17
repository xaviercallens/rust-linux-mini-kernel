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

extern "C" {
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

pub fn ip6_icmp_send_echo_reply(
    skb: *mut sk_buff,
    type_: u8,
    code: u8,
    offset: c_int,
    mtu: __be32,
) -> c_int {
    if skb.is_null() {
        return -1;
    }

    let icmp6h = unsafe { (*skb).cb as *mut ip6_icmp };

    if icmp6h.is_null() {
        return -1;
    }

    if unsafe { (*icmp6h).type_ } != type_ || unsafe { (*icmp6h).code } != code {
        return -1;
    }

    let echo = unsafe { &mut (*icmp6h).un.u_echo };
    echo.identifier = unsafe { (*icmp6h).un.u_echo.identifier };
    echo.sequence = unsafe { (*icmp6h).un.u_echo.sequence };

    unsafe { (*icmp6h).type_ = type_ };
    unsafe { (*icmp6h).code = code };

    ip6_icmp_send(skb, type_, code, offset, mtu)
}

pub fn ip6_icmp_send_error(
    skb: *mut sk_buff,
    type_: u8,
    code: u8,
    offset: c_int,
    mtu: __be32,
) -> c_int {
    if skb.is_null() {
        return -1;
    }

    let icmp6h = unsafe { (*skb).cb as *mut ip6_icmp };

    if icmp6h.is_null() {
        return -1;
    }

    if unsafe { (*icmp6h).type_ } != type_ || unsafe { (*icmp6h).code } != code {
        return -1;
    }

    unsafe { (*icmp6h).type_ = type_ };
    unsafe { (*icmp6h).code = code };

    ip6_icmp_error(skb, type_, code, offset, mtu)
}