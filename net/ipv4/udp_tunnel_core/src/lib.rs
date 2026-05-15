//! UDP Tunnel Core Module
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation for UDP tunneling.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::c_int;
use core::ffi::c_void;
use core::mem;
use core::ptr;
use core::slice;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

// Type definitions
#[repr(C)]
pub struct in_addr {
    pub s_addr: u32,
}

#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: in_addr,
    #[cfg(any())]
    #[doc(hidden)]
    __pad: [u8; 8],
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct socket {
    pub sk: *mut sock,
}

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct inet_sock {
    pub mc_loop: u8,
    _private: [u8; 0],
}

#[repr(C)]
pub struct udp_sock {
    pub encap_type: u16,
    pub encap_rcv: extern "C" fn(*mut sock, *mut sk_buff) -> c_int,
    pub encap_err_lookup: extern "C" fn(*mut sock, *mut sk_buff, u16, u16) -> *mut sock,
    pub encap_destroy: extern "C" fn(*mut sock),
    pub gro_receive: extern "C" fn(*mut sk_buff, *mut sk_buff) -> c_int,
    pub gro_complete: extern "C" fn(*mut sk_buff, *mut sk_buff) -> c_int,
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rtable {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ip_tunnel_info {
    pub key: ip_tunnel_key,
}

#[repr(C)]
pub struct ip_tunnel_key {
    pub tp_src: u16,
    pub tp_dst: u16,
    pub tun_flags: u16,
}

#[repr(C)]
pub struct metadata_dst {
    pub u: metadata_dst_u,
}

#[repr(C)]
pub union metadata_dst_u {
    pub tun_info: ip_tunnel_info,
}

#[repr(C)]
pub struct udp_port_cfg {
    pub bind_ifindex: u32,
    pub local_ip: in_addr,
    pub local_udp_port: u16,
    pub peer_ip: in_addr,
    pub peer_udp_port: u16,
    pub use_udp_checksums: u8,
}

#[repr(C)]
pub struct udp_tunnel_sock_cfg {
    pub sk_user_data: *mut c_void,
    pub encap_type: u16,
    pub encap_rcv: extern "C" fn(*mut sock, *mut sk_buff) -> c_int,
    pub encap_err_lookup: extern "C" fn(*mut sock, *mut sk_buff, u16, u16) -> *mut sock,
    pub encap_destroy: extern "C" fn(*mut sock),
    pub gro_receive: extern "C" fn(*mut sk_buff, *mut sk_buff) -> c_int,
    pub gro_complete: extern "C" fn(*mut sk_buff, *mut sk_buff) -> c_int,
}

// Function declarations for kernel APIs
extern "C" {
    fn sock_create_kern(net: *mut net, family: c_int, type_: c_int, protocol: c_int, sock: *mut *mut socket) -> c_int;
    fn sock_bindtoindex(sk: *mut sock, ifindex: u32, force: bool) -> c_int;
    fn kernel_bind(sock: *mut socket, addr: *const c_void, addrlen: socklen_t) -> c_int;
    fn kernel_connect(sock: *mut socket, addr: *const c_void, addrlen: socklen_t, flags: c_int) -> c_int;
    fn kernel_sock_shutdown(sock: *mut socket, how: c_int);
    fn sock_release(sock: *mut socket);
    fn inet_sk(sk: *mut sock) -> *mut inet_sock;
    fn inet_inc_convert_csum(sk: *mut sock);
    fn rcu_assign_sk_user_data(sk: *mut sock, data: *mut c_void);
    fn udp_sk(sk: *mut sock) -> *mut udp_sock;
    fn udp_tunnel_encap_enable(sock: *mut socket);
    fn udp_tunnel_nic_add_port(dev: *mut net_device, ti: *const udp_tunnel_info);
    fn udp_tunnel_nic_del_port(dev: *mut net_device, ti: *const udp_tunnel_info);
    fn iptunnel_xmit(sk: *mut sock, rt: *mut rtable, skb: *mut sk_buff, src: u32, dst: u32, proto: u8, tos: u8, ttl: u8, df: u16, xnet: bool);
    fn ip_tun_rx_dst(skb: *mut sk_buff, flags: u16, tunnel_id: u64, md_size: c_int) -> *mut metadata_dst;
    fn ipv6_tun_rx_dst(skb: *mut sk_buff, flags: u16, tunnel_id: u64, md_size: c_int) -> *mut metadata_dst;
    fn udp_hdr(skb: *mut sk_buff) -> *mut udphdr;
}

type socklen_t = u32;

#[repr(C)]
struct udphdr {
    source: u16,
    dest: u16,
    len: u16,
    check: u16,
}

// Function implementations
/// Create a UDP socket for IPv4 tunnel
///
/// # Safety
/// - `net` must be a valid pointer to network namespace
/// - `cfg` must be a valid configuration struct
/// - `sockp` must point to a valid socket pointer
///
/// # Returns
/// 0 on success, negative errno on failure
#[no_mangle]
pub unsafe extern "C" fn udp_sock_create4(
    net: *mut net,
    cfg: *const udp_port_cfg,
    sockp: *mut *mut socket,
) -> c_int {
    let mut sock: *mut socket = ptr::null_mut();
    let mut err: c_int = 0;

    // Create socket
    err = sock_create_kern(net, AF_INET as c_int, SOCK_DGRAM as c_int, 0, &mut sock);
    if err < 0 {
        goto error;
    }

    // Bind to interface if specified
    if (*cfg).bind_ifindex != 0 {
        err = sock_bindtoindex((*sock).sk, (*cfg).bind_ifindex, true);
        if err < 0 {
            goto error;
        }
    }

    // Bind to local address
    let mut udp_addr = sockaddr_in {
        sin_family: AF_INET as u16,
        sin_port: (*cfg).local_udp_port,
        sin_addr: (*cfg).local_ip,
        __pad: [0; 8],
    };
    err = kernel_bind(
        sock,
        &udp_addr as *const _ as *const c_void,
        mem::size_of::<sockaddr_in>() as socklen_t,
    );
    if err < 0 {
        goto error;
    }

    // Connect to peer if specified
    if (*cfg).peer_udp_port != 0 {
        udp_addr.sin_family = AF_INET as u16;
        udp_addr.sin_addr = (*cfg).peer_ip;
        udp_addr.sin_port = (*cfg).peer_udp_port;
        err = kernel_connect(
            sock,
            &udp_addr as *const _ as *const c_void,
            mem::size_of::<sockaddr_in>() as socklen_t,
            0,
        );
        if err < 0 {
            goto error;
        }
    }

    // Configure checksum
    (*(*sock).sk).sk_no_check_tx = !(*cfg).use_udp_checksums;

    *sockp = sock;
    return 0;

    error:
    if !sock.is_null() {
        kernel_sock_shutdown(sock, SHUT_RDWR as c_int);
        sock_release(sock);
    }
    *sockp = ptr::null_mut();
    return err;
}

/// Configure UDP tunnel socket
///
/// # Safety
/// - `sock` must be a valid socket pointer
/// - `cfg` must be a valid configuration struct
#[no_mangle]
pub unsafe extern "C" fn setup_udp_tunnel_sock(
    net: *mut net,
    sock: *mut socket,
    cfg: *const udp_tunnel_sock_cfg,
) {
    let sk = (*sock).sk;

    // Disable multicast loopback
    (*inet_sk(sk)).mc_loop = 0;

    // Enable checksum conversion
    inet_inc_convert_csum(sk);

    // Assign user data
    rcu_assign_sk_user_data(sk, (*cfg).sk_user_data);

    // Configure encapsulation
    (*udp_sk(sk)).encap_type = (*cfg).encap_type;
    (*udp_sk(sk)).encap_rcv = (*cfg).encap_rcv;
    (*udp_sk(sk)).encap_err_lookup = (*cfg).encap_err_lookup;
    (*udp_sk(sk)).encap_destroy = (*cfg).encap_destroy;
    (*udp_sk(sk)).gro_receive = (*cfg).gro_receive;
    (*udp_sk(sk)).gro_complete = (*cfg).gro_complete;

    udp_tunnel_encap_enable(sock);
}

/// Notify network devices about new tunnel port
///
/// # Safety
/// - `dev` must be a valid network device pointer
/// - `sock` must be a valid socket pointer
#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_notify_add_rx_port(
    sock: *mut socket,
    type_: u16,
) {
    let sk = (*sock).sk;
    let net = sock_net(sk);
    let mut dev: *mut net_device = ptr::null_mut();
    let ti = udp_tunnel_info {
        type: type_,
        sa_family: (*sk).sk_family,
        port: (*inet_sk(sk)).inet_sport,
    };

    // SAFETY: RCU read lock is required to iterate network devices safely
    extern "C" {
        fn rcu_read_lock();
        fn rcu_read_unlock();
        fn for_each_netdev_rcu(net: *mut net, dev: *mut *mut net_device);
    }

    rcu_read_lock();
    for_each_netdev_rcu(net, &mut dev);
    while !dev.is_null() {
        udp_tunnel_nic_add_port(dev, &ti);
        for_each_netdev_rcu(net, &mut dev);
    }
    rcu_read_unlock();
}

/// Notify network devices about removed tunnel port
///
/// # Safety
/// - `dev` must be a valid network device pointer
/// - `sock` must be a valid socket pointer
#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_notify_del_rx_port(
    sock: *mut socket,
    type_: u16,
) {
    let sk = (*sock).sk;
    let net = sock_net(sk);
    let mut dev: *mut net_device = ptr::null_mut();
    let ti = udp_tunnel_info {
        type: type_,
        sa_family: (*sk).sk_family,
        port: (*inet_sk(sk)).inet_sport,
    };

    // SAFETY: RCU read lock is required to iterate network devices safely
    extern "C" {
        fn rcu_read_lock();
        fn rcu_read_unlock();
        fn for_each_netdev_rcu(net: *mut net, dev: *mut *mut net_device);
    }

    rcu_read_lock();
    for_each_netdev_rcu(net, &mut dev);
    while !dev.is_null() {
        udp_tunnel_nic_del_port(dev, &ti);
        for_each_netdev_rcu(net, &mut dev);
    }
    rcu_read_unlock();
}

/// Transmit packet through UDP tunnel
///
/// # Safety
/// - `rt` must be a valid routing table entry
/// - `sk` must be a valid socket pointer
/// - `skb` must be a valid socket buffer
#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_xmit_skb(
    rt: *mut rtable,
    sk: *mut sock,
    skb: *mut sk_buff,
    src: u32,
    dst: u32,
    tos: u8,
    ttl: u8,
    df: u16,
    src_port: u16,
    dst_port: u16,
    xnet: bool,
    nocheck: bool,
) {
    let uh = udp_hdr(skb);
    
    // Push UDP header
    __skb_push(skb, mem::size_of::<udphdr>() as usize);
    skb_reset_transport_header(skb);
    
    // Configure UDP header
    (*uh).dest = dst_port;
    (*uh).source = src_port;
    (*uh).len = (skb_len(skb) as u16).to_be();
    
    // Clear options
    memset(&(IPCB(skb).opt), 0, mem::size_of_val(&IPCB(skb).opt));
    
    // Set checksum
    udp_set_csum(nocheck, skb, src, dst, skb_len(skb));
    
    // Transmit packet
    iptunnel_xmit(sk, rt, skb, src, dst, IPPROTO_UDP as u8, tos, ttl, df, xnet);
}

/// Release UDP tunnel socket
///
/// # Safety
/// - `sock` must be a valid socket pointer
#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_sock_release(
    sock: *mut socket,
) {
    let sk = (*sock).sk;
    
    rcu_assign_sk_user_data(sk, ptr::null_mut());
    kernel_sock_shutdown(sock, SHUT_RDWR as c_int);
    sock_release(sock);
}

/// Create tunnel destination metadata
///
/// # Safety
/// - `skb` must be a valid socket buffer
#[no_mangle]
pub unsafe extern "C" fn udp_tun_rx_dst(
    skb: *mut sk_buff,
    family: u16,
    flags: u16,
    tunnel_id: u64,
    md_size: c_int,
) -> *mut metadata_dst {
    let mut tun_dst: *mut metadata_dst = ptr::null_mut();
    
    if family == AF_INET as u16 {
        tun_dst = ip_tun_rx_dst(skb, flags, tunnel_id, md_size);
    } else {
        tun_dst = ipv6_tun_rx_dst(skb, flags, tunnel_id, md_size);
    }
    
    if tun_dst.is_null() {
        return ptr::null_mut();
    }
    
    let info = &mut (*tun_dst).u.tun_info;
    let uh = udp_hdr(skb);
    
    info.key.tp_src = (*uh).source;
    info.key.tp_dst = (*uh).dest;
    if (*uh).check != 0 {
        info.key.tun_flags |= TUNNEL_CSUM as u16;
    }
    
    tun_dst
}

// Helper functions (extern declarations)
extern "C" {
    fn AF_INET() -> c_int;
    fn SOCK_DGRAM() -> c_int;
    fn SHUT_RDWR() -> c_int;
    fn IPPROTO_UDP() -> c_int;
    fn TUNNEL_CSUM() -> u16;
    fn __skb_push(skb: *mut sk_buff, len: usize) -> *mut u8;
    fn skb_reset_transport_header(skb: *mut sk_buff);
    fn skb_len(skb: *mut sk_buff) -> usize;
    fn IPCB(skb: *mut sk_buff) -> *mut c_void;
    fn memset(dest: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn udp_set_csum(nocheck: bool, skb: *mut sk_buff, saddr: u32, daddr: u32, len: usize);
    fn sock_net(sk: *mut sock) -> *mut net;
}

// Exported symbols
#[no_mangle]
pub static udp_sock_create4: unsafe extern "C" fn(*mut net, *const udp_port_cfg, *mut *mut socket) -> c_int = udp_sock_create4;
#[no_mangle]
pub static setup_udp_tunnel_sock: unsafe extern "C" fn(*mut net, *mut socket, *const udp_tunnel_sock_cfg) = setup_udp_tunnel_sock;
#[no_mangle]
pub static udp_tunnel_notify_add_rx_port: unsafe extern "C" fn(*mut socket, u16) = udp_tunnel_notify_add_rx_port;
#[no_mangle]
pub static udp_tunnel_notify_del_rx_port: unsafe extern "C" fn(*mut socket, u16) = udp_tunnel_notify_del_rx_port;
#[no_mangle]
pub static udp_tunnel_xmit_skb: unsafe extern "C" fn(*mut rtable, *mut sock, *mut sk_buff, u32, u32, u8, u8, u16, u16, u16, bool, bool) = udp_tunnel_xmit_skb;
#[no_mangle]
pub static udp_tunnel_sock_release: unsafe extern "C" fn(*mut socket) = udp_tunnel_sock_release;
#[no_mangle]
pub static udp_tun_rx_dst: unsafe extern "C" fn(*mut sk_buff, u16, u16, u64, c_int) -> *mut metadata_dst = udp_tun_rx_dst;

// Module license
#[no_mangle]
pub static GPL_LICENSE: [u8; 4] = *b"GPL\0";
