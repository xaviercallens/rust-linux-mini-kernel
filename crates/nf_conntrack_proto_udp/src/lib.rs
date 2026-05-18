#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::all)]

use core::ffi::{c_char, c_int, c_void};
use core::mem::{self, size_of};
use kernel_types::*;

pub const IPPROTO_UDP: c_int = 17;
pub const IPPROTO_UDPLITE: c_int = 136;
pub const NF_ACCEPT: c_int = 1;
pub const NF_INET_PRE_ROUTING: c_int = 0;

pub const UDP_CT_UNREPLIED: usize = 0;
pub const UDP_CT_REPLIED: usize = 1;
pub const UDP_CT_MAX: usize = 2;

pub const IPS_SEEN_REPLY_BIT: c_int = 1;
pub const IPS_ASSURED_BIT: c_int = 2;
pub const IPS_NAT_CLASH: c_int = 4;
pub const IPCT_ASSURED: c_int = 1;
pub const CTA_TIMEOUT_UDP_UNREPLIED: c_int = 1;
pub const CTA_TIMEOUT_UDP_REPLIED: c_int = 2;
pub const CTA_TIMEOUT_UDP_MAX: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_hook_state {
    pub net: *mut c_void,
    pub pf: c_int,
    pub hook: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn_udp {
    pub stream_ts: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn_proto {
    pub udp: nf_conn_udp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn {
    pub status: c_int,
    pub proto: nf_conn_proto,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_udp_net {
    pub timeouts: [c_int; UDP_CT_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_l4proto {
    pub l4proto: c_int,
    pub allow_clash: bool,
}

#[repr(C)]
pub struct net_t {
    _priv: [u8; 0],
}

static udp_timeouts: [c_int; UDP_CT_MAX] = [30, 120];

unsafe extern "C" {
    fn nf_udp_pernet(net: *mut c_void) -> *mut nf_udp_net;
    fn nf_l4proto_log_invalid(
        skb: *mut sk_buff,
        net: *mut c_void,
        pf: c_int,
        proto: c_int,
        fmt: *const c_char,
        ...
    ) -> c_int;
    fn nf_checksum(skb: *mut sk_buff, hook: c_int, dataoff: c_int, proto: c_int, pf: c_int) -> bool;
    fn nf_checksum_partial(
        skb: *mut sk_buff,
        hook: c_int,
        dataoff: c_int,
        cscov: c_int,
        proto: c_int,
        pf: c_int,
    ) -> bool;
    fn skb_header_pointer(
        skb: *mut sk_buff,
        dataoff: c_int,
        size: c_int,
        hdr: *mut udphdr,
    ) -> *mut udphdr;
    fn nf_ct_timeout_lookup(ct: *mut nf_conn) -> *mut c_int;
    fn nf_ct_refresh_acct(ct: *mut nf_conn, ctinfo: c_int, skb: *mut sk_buff, timeout: c_int) -> c_int;
    fn nf_conntrack_event_cache(event: c_int, ct: *mut nf_conn);
    fn nf_ct_net(ct: *mut nf_conn) -> *mut c_void;
    fn nf_ct_port_tuple_to_nlattr(skb: *mut sk_buff, data: *mut c_void) -> c_int;
    fn nf_ct_port_nlattr_to_tuple(tb: *mut c_void, data: *mut c_void) -> c_int;
    fn nf_ct_port_nlattr_tuple_size() -> c_int;
}

#[inline(always)]
unsafe fn ntohs(v: u16) -> u16 {
    u16::from_be(v)
}

fn udp_error_log(skb: *mut sk_buff, state: *mut nf_hook_state, msg: *const c_char) {
    unsafe {
        nf_l4proto_log_invalid(skb, (*state).net, (*state).pf, IPPROTO_UDP, msg);
    }
}

fn udplite_error_log(skb: *mut sk_buff, state: *mut nf_hook_state, msg: *const c_char) {
    unsafe {
        nf_l4proto_log_invalid(skb, (*state).net, (*state).pf, IPPROTO_UDPLITE, msg);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn udp_error(
    skb: *mut sk_buff,
    dataoff: c_int,
    state: *mut nf_hook_state,
) -> bool {
    let udplen = (*skb).len - dataoff as u32;
    let mut _hdr: udphdr = mem::zeroed();
    let hdr = skb_header_pointer(skb, dataoff, size_of::<udphdr>() as c_int, &mut _hdr);

    if hdr.is_null() {
        udp_error_log(skb, state, c"short packet".as_ptr());
        return true;
    }

    let hdr_len = (*hdr).len;
    if (ntohs(hdr_len) as u32 > udplen) || ((ntohs(hdr_len) as usize) < size_of::<udphdr>()) {
        udp_error_log(skb, state, c"truncated/malformed packet".as_ptr());
        return true;
    }

    if (*hdr).check == 0 {
        return false;
    }

    if (*state).hook == NF_INET_PRE_ROUTING
        && nf_checksum(skb, (*state).hook, dataoff, IPPROTO_UDP, (*state).pf)
    {
        udp_error_log(skb, state, c"bad UDP checksum".as_ptr());
        return true;
    }

    false
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn udp_get_timeouts(net: *mut c_void) -> *const c_int {
    let un = nf_udp_pernet(net);
    if !un.is_null() {
        (*un).timeouts.as_ptr()
    } else {
        udp_timeouts.as_ptr()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn udp_timeout(ct: *mut nf_conn) -> c_int {
    let t = nf_ct_timeout_lookup(ct);
    if !t.is_null() {
        *t
    } else {
        udp_timeouts[UDP_CT_UNREPLIED]
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}