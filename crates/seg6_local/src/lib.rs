#![no_std]
#![allow(non_camel_case_types)]

use core::{mem::size_of, ptr};
use kernel_types::*;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[repr(C)]
struct seg6_local_lwtunnel_ops {
    build_state:
        Option<unsafe extern "C" fn(*mut seg6_local_lwt, *const c_void, *mut c_void) -> c_int>,
    destroy_state: Option<unsafe extern "C" fn(*mut seg6_local_lwt)>,
}

#[repr(C)]
struct seg6_action_desc {
    action: c_int,
    attrs: c_ulong,
    optattrs: c_ulong,
    input: Option<unsafe extern "C" fn(*mut c_void, *mut seg6_local_lwt) -> c_int>,
    static_headroom: c_int,
    slwt_ops: seg6_local_lwtunnel_ops,
}

#[repr(C)]
struct bpf_lwt_prog {
    prog: *mut c_void,
    name: *mut c_char,
}

#[repr(C)]
enum seg6_end_dt_mode {
    DT_INVALID_MODE = -1,
    DT_LEGACY_MODE = 0,
    DT_VRF_MODE = 1,
}

#[repr(C)]
struct seg6_end_dt_info {
    mode: seg6_end_dt_mode,
    net: *mut c_void,
    vrf_ifindex: c_int,
    vrf_table: c_int,
    proto: u16,
    family: u16,
    hdrlen: c_int,
}

#[repr(C)]
struct u64_stats_sync {
    _priv: [u8; 0],
}

#[repr(C)]
struct in_addr {
    s_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct in6_addr {
    s6_addr: [u8; 16],
}

#[repr(C)]
struct pcpu_seg6_local_counters {
    packets: u64,
    bytes: u64,
    errors: u64,
    syncp: u64_stats_sync,
}

#[repr(C)]
struct seg6_local_counters {
    packets: u64,
    bytes: u64,
    errors: u64,
}

#[repr(C)]
struct seg6_local_lwt {
    action: c_int,
    srh: *mut ipv6_sr_hdr,
    table: c_int,
    nh4: in_addr,
    nh6: in6_addr,
    iif: c_int,
    oif: c_int,
    bpf: bpf_lwt_prog,
    pcpu_counters: *mut pcpu_seg6_local_counters,
    headroom: c_int,
    desc: *mut seg6_action_desc,
    parsed_optattrs: c_ulong,
}

#[repr(C)]
struct lwtunnel_state {
    data: *mut c_void,
}

#[repr(C)]
struct sk_buff {
    dev: *mut c_void,
    data: *mut u8,
    len: c_int,
    mark: c_int,
}

#[repr(C)]
struct ipv6hdr {
    saddr: in6_addr,
    daddr: in6_addr,
    nexthdr: u8,
}

#[repr(C)]
struct ipv6_sr_hdr {
    hdrlen: u8,
    segments_left: u8,
    segments: [in6_addr; 0],
}

const IPPROTO_ROUTING: c_int = 43;
const IP6_FH_F_SKIP_RH: c_int = 1;

unsafe extern "C" {
    fn ipv6_find_hdr(
        skb: *mut sk_buff,
        offset: *mut c_int,
        target: c_int,
        fragoff: *mut c_void,
        flags: *const c_int,
    ) -> c_int;
    fn pskb_may_pull(skb: *mut sk_buff, len: c_int) -> bool;
    fn skb_data(skb: *mut sk_buff) -> *mut c_void;
    fn seg6_validate_srh(srh: *mut ipv6_sr_hdr, len: c_int, strict: bool) -> bool;
    fn pskb_pull(skb: *mut sk_buff, len: usize) -> bool;
    fn skb_postpull_rcsum(skb: *mut sk_buff, start: *mut c_void, len: usize);
    fn skb_network_header(skb: *mut sk_buff) -> *mut c_void;
    fn skb_reset_network_header(skb: *mut sk_buff);
    fn skb_reset_transport_header(skb: *mut sk_buff);
    fn iptunnel_pull_offloads(skb: *mut sk_buff) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn seg6_local_lwtunnel(lwt: *mut lwtunnel_state) -> *mut seg6_local_lwt {
    (*lwt).data as *mut seg6_local_lwt
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_srh(skb: *mut sk_buff, flags: c_int) -> *mut ipv6_sr_hdr {
    let mut srhoff: c_int = 0;

    if ipv6_find_hdr(
        skb,
        &mut srhoff as *mut c_int,
        IPPROTO_ROUTING,
        ptr::null_mut(),
        &flags as *const c_int,
    ) < 0
    {
        return ptr::null_mut();
    }

    if !pskb_may_pull(skb, srhoff + size_of::<ipv6_sr_hdr>() as c_int) {
        return ptr::null_mut();
    }

    let srh = (skb_data(skb) as *mut u8).offset(srhoff as isize) as *mut ipv6_sr_hdr;

    let len = (((*srh).hdrlen as c_int) + 1) << 3;
    if !pskb_may_pull(skb, srhoff + len) {
        return ptr::null_mut();
    }

    let srh = (skb_data(skb) as *mut u8).offset(srhoff as isize) as *mut ipv6_sr_hdr;

    if !seg6_validate_srh(srh, len, true) {
        return ptr::null_mut();
    }

    srh
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_and_validate_srh(skb: *mut sk_buff) -> *mut ipv6_sr_hdr {
    get_srh(skb, IP6_FH_F_SKIP_RH)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn decap_and_validate(skb: *mut sk_buff, proto: c_int) -> bool {
    let srh = get_srh(skb, 0);
    if !srh.is_null() && (*srh).segments_left > 0 {
        return false;
    }

    let mut off: c_int = 0;
    if ipv6_find_hdr(skb, &mut off as *mut c_int, proto, ptr::null_mut(), ptr::null()) < 0 {
        return false;
    }

    if !pskb_pull(skb, off as usize) {
        return false;
    }

    skb_postpull_rcsum(skb, skb_network_header(skb), off as usize);
    skb_reset_network_header(skb);
    skb_reset_transport_header(skb);

    iptunnel_pull_offloads(skb) >= 0
}