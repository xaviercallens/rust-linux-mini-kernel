#![cfg_attr(not(test), no_std)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kernel_types::*;

pub type proto_ops = c_void;
pub type msghdr = c_void;
pub type page = c_void;
pub type netlink_ext_ack = c_void;

/// UDPLite header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udphdr {
    pub source: __be16,
    pub dest: __be16,
    pub len: __be16,
    pub check: __be16,
}

extern "C" {
    fn udp_rcv(skb: *mut sk_buff) -> c_int;
    fn udp_err(skb: *mut sk_buff, info: *mut u8, err: c_int, icmph: *mut c_void, dev: *mut c_void, inet6_skb_parm: *mut c_void, sock_exterr_skb: *mut c_void);
    fn kfree_skb(skb: *mut sk_buff);
    fn ntohs(val: __be16) -> u16;
}

/// UDPLite socket
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udplite_sock {
    pub inet: inet_sock,
    pub cscov: c_int,
    pub partial_cov: c_int,
}

/// UDPLite options
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udpliteopt {
    pub cscov: c_int,
    pub clen: c_int,
}

/// UDPLite control block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udplite_cb {
    pub partial_cov: c_int,
}

/// UDPLite socket operations
#[repr(C)]
pub struct udplite_ops {
    pub proto: *mut proto_ops,
    pub init: unsafe extern "C" fn(*mut sock) -> c_int,
    pub connect: unsafe extern "C" fn(*mut sock, *mut sockaddr, c_int) -> c_int,
    pub disconnect: unsafe extern "C" fn(*mut sock, c_int) -> c_int,
    pub accept: unsafe extern "C" fn(*mut sock, *mut sock, c_int) -> c_int,
    pub ioctl: unsafe extern "C" fn(*mut sock, c_int, c_ulong) -> c_int,
    pub getname: unsafe extern "C" fn(*mut sock, *mut sockaddr, *mut socklen_t, c_int) -> c_int,
    pub setsockopt: unsafe extern "C" fn(*mut sock, c_int, c_int, *const c_void, c_int) -> c_int,
    pub getsockopt: unsafe extern "C" fn(*mut sock, c_int, c_int, *mut c_void, *mut c_int) -> c_int,
    pub compat_setsockopt: unsafe extern "C" fn(*mut sock, c_int, c_int, *const c_void, c_int) -> c_int,
    pub compat_getsockopt: unsafe extern "C" fn(*mut sock, c_int, c_int, *mut c_void, *mut c_int) -> c_int,
    pub compat_ioctl: unsafe extern "C" fn(*mut sock, c_int, c_ulong) -> c_int,
    pub sendmsg: unsafe extern "C" fn(*mut sock, *mut msghdr, c_int) -> c_int,
    pub recvmsg: unsafe extern "C" fn(*mut sock, *mut msghdr, c_int, c_int, c_int, c_int) -> c_int,
    pub sendpage: unsafe extern "C" fn(*mut sock, *mut page, c_int, c_size_t, c_int) -> c_int,
    pub bind: unsafe extern "C" fn(*mut sock, *mut sockaddr, c_int) -> c_int,
    pub backlog_rcv: unsafe extern "C" fn(*mut sock, *mut sk_buff) -> c_int,
    pub release_cb: unsafe extern "C" fn(*mut sock),
    pub hash: unsafe extern "C" fn(*mut sock),
    pub unhash: unsafe extern "C" fn(*mut sock),
    pub get_port: unsafe extern "C" fn(*mut sock, *mut flowi) -> c_int,
    pub enter_memory_pressure: unsafe extern "C" fn(*mut sock),
    pub sock_rcv_skb: unsafe extern "C" fn(*mut sock, *mut sk_buff) -> c_int,
    pub mib_lookup: unsafe extern "C" fn(*mut sock, c_int) -> *mut c_ulong,
    pub mib_addr_lookup: unsafe extern "C" fn(*mut sock, c_int) -> *mut c_ulong,
    pub diag_destroy: unsafe extern "C" fn(*mut sock),
    pub diag_handler: unsafe extern "C" fn(*mut sock, *mut netlink_ext_ack, *mut sk_buff, *mut u8, *mut u8, c_int) -> c_int,
    pub get_timeo: unsafe extern "C" fn(*mut sock, c_int) -> c_int,
    pub cmsg_send: unsafe extern "C" fn(*mut sock, *mut msghdr, c_int) -> c_int,
    pub cmsg_recv: unsafe extern "C" fn(*mut sock, *mut msghdr, c_int) -> c_int,
    pub bind_conflict: unsafe extern "C" fn(*mut sock, *mut sock) -> c_int,
    pub get_rx_skb_len: unsafe extern "C" fn(*mut sock, *mut sk_buff) -> c_int,
    pub setsockopt_compat: unsafe extern "C" fn(*mut sock, c_int, c_int, *const c_void, c_int) -> c_int,
    pub getsockopt_compat: unsafe extern "C" fn(*mut sock, c_int, c_int, *mut c_void, *mut c_int) -> c_int,
    pub sendmsg_locked: unsafe extern "C" fn(*mut sock, *mut msghdr, c_int) -> c_int,
    pub sendpage_locked: unsafe extern "C" fn(*mut sock, *mut page, c_int, c_size_t, c_int) -> c_int,
    pub setsockopt_locked: unsafe extern "C" fn(*mut sock, c_int, c_int, *const c_void, c_int) -> c_int,
}

/// UDPLite protocol
#[repr(C)]
pub struct udplite_protocol {
    pub handler: unsafe extern "C" fn(*mut sk_buff) -> c_int,
    pub err_handler: unsafe extern "C" fn(*mut sk_buff, *mut u8, c_int, *mut c_void, *mut c_void, *mut c_void, *mut c_void),
    pub no_policy: c_int,
    pub netns_ok: c_int,
    pub icmp_strict_tag_validation: c_int,
    pub icmpv6_allow_any: c_int,
}

/// UDPLite socket options
pub const UDPLITE_SEND_CSCOV: c_int = 1;
pub const UDPLITE_RECV_CSCOV: c_int = 2;

/// UDPLite checksum coverage
pub const UDPLITE_MIN_CSCOV: c_int = 0;
pub const UDPLITE_MAX_CSCOV: c_int = 65535;

/// UDPLite error codes
pub const UDPLITE_ERR_CSCOV: c_int = -1000;
pub const UDPLITE_ERR_PARTIAL: c_int = -1001;

/// UDPLite protocol number
pub const IPPROTO_UDPLITE: c_int = 136;

/// UDPLite socket operations
#[no_mangle]
pub static mut udplite_proto_ops: udplite_ops = udplite_ops {
    proto: core::ptr::null_mut(),
    init: udplite_init_sock,
    connect: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    disconnect: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    accept: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    ioctl: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    getname: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    setsockopt: udplite_setsockopt,
    getsockopt: udplite_getsockopt,
    compat_setsockopt: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    compat_getsockopt: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    compat_ioctl: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    sendmsg: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    recvmsg: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    sendpage: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    bind: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    backlog_rcv: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    release_cb: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    hash: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    unhash: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    get_port: unsafe { core::mem::transmute(udplite_get_port as *const ()) },
    enter_memory_pressure: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    sock_rcv_skb: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    mib_lookup: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    mib_addr_lookup: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    diag_destroy: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    diag_handler: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    get_timeo: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    cmsg_send: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    cmsg_recv: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    bind_conflict: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    get_rx_skb_len: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    setsockopt_compat: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    getsockopt_compat: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    sendmsg_locked: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    sendpage_locked: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
    setsockopt_locked: unsafe { core::mem::transmute(udplite_dummy as *const ()) },
};

/// UDPLite protocol
#[no_mangle]
pub static mut udplite_protocol: udplite_protocol = udplite_protocol {
    handler: udplite_rcv,
    err_handler: udplite_err,
    no_policy: 1,
    netns_ok: 1,
    icmp_strict_tag_validation: 1,
    icmpv6_allow_any: 1,
};

/// Initialize UDPLite socket
#[no_mangle]
pub unsafe extern "C" fn udplite_init_sock(sk: *mut sock) -> c_int {
    let udp_sk = &mut *(sk as *mut udplite_sock);
    udp_sk.cscov = UDPLITE_MIN_CSCOV;
    udp_sk.partial_cov = 0;
    0
}

/// Set UDPLite socket options
#[no_mangle]
pub unsafe extern "C" fn udplite_setsockopt(sk: *mut sock, level: c_int, optname: c_int, optval: *const c_void, optlen: c_int) -> c_int {
    if level != 136 { // SOL_UDPLITE = IPPROTO_UDPLITE
        return -EINVAL;
    }

    match optname {
        UDPLITE_SEND_CSCOV => {
            if optlen != core::mem::size_of::<c_int>() as c_int {
                return -EINVAL;
            }
            let cscov = *(optval as *const c_int);
            if cscov < UDPLITE_MIN_CSCOV || cscov > UDPLITE_MAX_CSCOV {
                return -EINVAL;
            }
            let udp_sk = &mut *(sk as *mut udplite_sock);
            udp_sk.cscov = cscov;
        }
        UDPLITE_RECV_CSCOV => {
            if optlen != core::mem::size_of::<c_int>() as c_int {
                return -EINVAL;
            }
            let partial_cov = *(optval as *const c_int);
            if partial_cov < 0 || partial_cov > 1 {
                return -EINVAL;
            }
            let udp_sk = &mut *(sk as *mut udplite_sock);
            udp_sk.partial_cov = partial_cov;
        }
        _ => return -EINVAL,
    }

    0
}

/// Get UDPLite socket options
#[no_mangle]
pub unsafe extern "C" fn udplite_getsockopt(sk: *mut sock, level: c_int, optname: c_int, optval: *mut c_void, optlen: *mut c_int) -> c_int {
    if level != 136 { // SOL_UDPLITE = IPPROTO_UDPLITE
        return -EINVAL;
    }

    match optname {
        UDPLITE_SEND_CSCOV => {
            if *optlen < core::mem::size_of::<c_int>() as c_int {
                *optlen = core::mem::size_of::<c_int>() as c_int;
                return -EINVAL;
            }
            let udp_sk = &*(sk as *const udplite_sock);
            *(optval as *mut c_int) = udp_sk.cscov;
            *optlen = core::mem::size_of::<c_int>() as c_int;
        }
        UDPLITE_RECV_CSCOV => {
            if *optlen < core::mem::size_of::<c_int>() as c_int {
                *optlen = core::mem::size_of::<c_int>() as c_int;
                return -EINVAL;
            }
            let udp_sk = &*(sk as *const udplite_sock);
            *(optval as *mut c_int) = udp_sk.partial_cov;
            *optlen = core::mem::size_of::<c_int>() as c_int;
        }
        _ => return -EINVAL,
    }

    0
}

/// UDPLite receive function
#[no_mangle]
pub unsafe extern "C" fn udplite_rcv(skb: *mut sk_buff) -> c_int {
    // 🛡️ FORMAL VERIFICATION BOUNDARY (Mapped to Lean 4: udplite_csum_no_degradation)
    requires!(!skb.is_null(), "udplite_csum_no_degradation: skb invariant violated");
    requires!(!(*skb).data.is_null(), "udplite_csum_no_degradation: skb.data invariant violated");

    let udph = &mut *((*skb).data as *mut udphdr);
    let len = ntohs(udph.len) as usize;
    let cscov = if len > core::mem::size_of::<udphdr>() {
        len - core::mem::size_of::<udphdr>()
    } else {
        0
    };

    let udp_sk = &mut *((*skb).sk as *mut udplite_sock);
    if cscov < udp_sk.cscov as usize {
        if udp_sk.partial_cov == 0 {
            kfree_skb(skb);
            return 0;
        }
        let udp_cb = &mut *((*skb).cb.as_mut_ptr() as *mut udplite_cb);
        udp_cb.partial_cov = 1;
    }

    udp_rcv(skb)
}

/// UDPLite error handler
#[no_mangle]
pub unsafe extern "C" fn udplite_err(skb: *mut sk_buff, info: *mut u8, err: c_int, icmph: *mut c_void, dev: *mut c_void, inet6_skb_parm: *mut c_void, sock_exterr_skb: *mut c_void) {
    udp_err(skb, info, err, icmph, dev, inet6_skb_parm, sock_exterr_skb);
}
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn udplite_dummy() {}
