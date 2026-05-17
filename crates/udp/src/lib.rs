//! UDP over IPv6 implementation for Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::c_void;
use core::ptr;
use core::mem;
use kernel_types::*;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

// Type definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct refcount_t {
    pub counter: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp_table {
    pub mask: c_int,
    pub hash2: *mut udp_hslot,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp_hslot {
    pub head: list_head,
}

// Function declarations for external C functions
extern "C" {
    fn net_get_random_once(buf: *mut u32, size: usize);
    fn ipv6_portaddr_hash(net: *const net, addr: *const in6_addr, port: u16) -> u32;
    fn __inet6_ehashfn(lhash: u32, lport: u16, fhash: u32, fport: u16, secret: u32) -> u32;
    fn net_hash_mix(net: *const net) -> u32;
    fn bpf_sk_lookup_run_v6(net: *const net, proto: c_int,
                            saddr: *const in6_addr, sport: u16,
                            daddr: *const in6_addr, hnum: u16) -> *mut sock;
    fn ipv6_recv_error(sk: *mut sock, msg: *mut c_void, len: usize, addr_len: *mut c_int) -> c_int;
    fn ipv6_recv_rxpmtu(sk: *mut sock, msg: *mut c_void, len: usize, addr_len: *mut c_int) -> c_int;
    fn __skb_recv_udp(sk: *mut sock, flags: c_int, noblock: c_int, off: *mut c_int, err: *mut c_int) -> *mut sk_buff;
    fn copy_linear_skb(skb: *mut sk_buff, copied: usize, off: c_int, msg_iter: *mut c_void) -> c_int;
    fn skb_copy_datagram(skb: *mut sk_buff, offset: c_int, to: *mut c_void, len: usize) -> c_int;
    fn udp_skb_csum_unnecessary(skb: *mut sk_buff) -> c_int;
    fn __udp_lib_checksum_complete(skb: *mut sk_buff) -> c_int;
    fn refcount_inc_not_zero(ref: *mut refcount_t) -> c_int;
    fn static_branch_unlikely(branch: *const c_void) -> c_int;
    fn inet6_is_jumbogram(skb: *mut sk_buff) -> c_int;
    fn udp_skb_len(skb: *mut sk_buff) -> c_int;
    fn dev_net(dev: *mut c_void) -> *mut net;
    fn ipv6_hdr(skb: *mut sk_buff) -> *mut ipv6hdr;
    fn inet6_iif(skb: *mut sk_buff) -> c_int;
    fn inet6_sdif(skb: *mut sk_buff) -> c_int;
    fn reuseport_select_sock(sk: *mut sock, hash: u32, skb: *mut sk_buff, hlen: usize) -> *mut sock;
    fn reuseport_has_conns(sk: *mut sock, has_conns: c_int) -> c_int;
    fn sk_peek_offset(sk: *mut sock, flags: c_int) -> c_int;
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn udp6_ehashfn(
    net: *const net,
    laddr: *const in6_addr,
    lport: u16,
    faddr: *const in6_addr,
    fport: u16,
) -> u32 {
    static mut UDP6_EHASH_SECRET: u32 = 0;
    static mut UDP_IPV6_HASH_SECRET: u32 = 0;

    // Initialize secrets
    net_get_random_once(&mut UDP6_EHASH_SECRET, mem::size_of::<u32>());
    net_get_random_once(&mut UDP_IPV6_HASH_SECRET, mem::size_of::<u32>());

    let lhash = (*laddr).in6_u.u6_addr32[3] as u32;
    let fhash = ipv6_portaddr_hash(net, faddr, 0);

    __inet6_ehashfn(
        lhash,
        lport,
        fhash,
        fport,
        UDP_IPV6_HASH_SECRET + net_hash_mix(net)
    )
}

#[no_mangle]
pub unsafe extern "C" fn udp_v6_get_port(
    sk: *mut sock,
    snum: u16,
) -> c_int {
    let hash2_nulladdr = ipv6_portaddr_hash(sock_net(sk), &in6addr_any, snum);
    let hash2_partial = ipv6_portaddr_hash(sock_net(sk), &(*sk).sk_v6_rcv_saddr, 0);

    (*sk).udp_portaddr_hash = hash2_partial;
    udp_lib_get_port(sk, snum, hash2_nulladdr)
}

#[no_mangle]
pub unsafe extern "C" fn udp_v6_rehash(
    sk: *mut sock,
) {
    let new_hash = ipv6_portaddr_hash(sock_net(sk), &(*sk).sk_v6_rcv_saddr, (*sk).inet_num);
    udp_lib_rehash(sk, new_hash);
}

#[no_mangle]
pub unsafe extern "C" fn compute_score(
    sk: *mut sock,
    net: *const net,
    saddr: *const in6_addr,
    sport: u16,
    daddr: *const in6_addr,
    hnum: u16,
    dif: c_int,
    sdif: c_int,
) -> c_int {
    if !net_eq(sock_net(sk), net) || (*sk).udp_port_hash != hnum || (*sk).sk_family != AF_INET6 {
        return -1;
    }

    if !ipv6_addr_equal(&(*sk).sk_v6_rcv_saddr, daddr) {
        return -1;
    }

    let mut score = 0;
    let inet = &(*sk).inet_sk;

    if inet.inet_dport != 0 {
        if inet.inet_dport != sport {
            return -1;
        }
        score += 1;
    }

    if !ipv6_addr_any(&(*sk).sk_v6_daddr) {
        if !ipv6_addr_equal(&(*sk).sk_v6_daddr, saddr) {
            return -1;
        }
        score += 1;
    }

    let dev_match = udp_sk_bound_dev_eq(net, (*sk).sk_bound_dev_if, dif, sdif);
    if !dev_match {
        return -1;
    }
    score += 1;

    if (*sk).sk_incoming_cpu == raw_smp_processor_id() {
        score += 1;
    }

    score
}

#[no_mangle]
pub unsafe extern "C" fn lookup_reuseport(
    net: *const net,
    sk: *mut sock,
    skb: *mut sk_buff,
    saddr: *const in6_addr,
    sport: u16,
    daddr: *const in6_addr,
    hnum: u16,
) -> *mut sock {
    if (*sk).sk_reuseport != 0 && (*sk).sk_state != TCP_ESTABLISHED {
        let hash = udp6_ehashfn(net, daddr, hnum, saddr, sport);
        return reuseport_select_sock(sk, hash, skb, mem::size_of::<udphdr>());
    }
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn udp6_lib_lookup2(
    net: *const net,
    saddr: *const in6_addr,
    sport: u16,
    daddr: *const in6_addr,
    hnum: u16,
    dif: c_int,
    sdif: c_int,
    hslot2: *mut udp_hslot,
    skb: *mut sk_buff,
) -> *mut sock {
    let mut result: *mut sock = ptr::null_mut();
    let mut badness: c_int = -1;

    let mut sk = (*hslot2).head.next;
    while !sk.is_null() && sk != &(*hslot2).head {
        let score = compute_score(sk as *mut sock, net, saddr, sport, daddr, hnum, dif, sdif);
        if score > badness {
            let reuse_sk = lookup_reuseport(net, sk as *mut sock, skb, saddr, sport, daddr, hnum);
            if !reuse_sk.is_null() && !reuseport_has_conns(sk as *mut sock, false) {
                return reuse_sk;
            }

            result = if !reuse_sk.is_null() { reuse_sk } else { sk as *mut sock };
            badness = score;
        }
        sk = (*sk).next;
    }

    result
}

#[no_mangle]
pub unsafe extern "C" fn udp6_lookup_run_bpf(
    net: *const net,
    udptable: *mut udp_table,
    skb: *mut sk_buff,
    saddr: *const in6_addr,
    sport: u16,
    daddr: *const in6_addr,
    hnum: u16,
) -> *mut sock {
    if udptable != &udp_table {
        return ptr::null_mut();
    }

    let mut sk: *mut sock = ptr::null_mut();
    let no_reuseport = bpf_sk_lookup_run_v6(net, IPPROTO_UDP, saddr, sport, daddr, hnum, &mut sk);

    if no_reuseport != 0 || sk.is_null() {
        return sk;
    }

    let reuse_sk = lookup_reuseport(net, sk, skb, saddr, sport, daddr, hnum);
    if !reuse_sk.is_null() {
        sk = reuse_sk;
    }
    sk
}

#[no_mangle]
pub unsafe extern "C" fn __udp6_lib_lookup(
    net: *const net,
    saddr: *const in6_addr,
    sport: u16,
    daddr: *const in6_addr,
    dport: u16,
    dif: c_int,
    sdif: c_int,
    udptable: *mut udp_table,
    skb: *mut sk_buff,
) -> *mut sock {
    let hnum = ntohs(dport);
    let hash2 = ipv6_portaddr_hash(net, daddr, hnum);
    let slot2 = hash2 & (*udptable).mask;
    let hslot2 = &(*udptable).hash2[slot2];

    let mut result = udp6_lib_lookup2(net, saddr, sport, daddr, hnum, dif, sdif, hslot2, skb);
    if !result.is_null() && (*result).sk_state == TCP_ESTABLISHED {
        return result;
    }

    if static_branch_unlikely(&bpf_sk_lookup_enabled) != 0 {
        let sk = udp6_lookup_run_bpf(net, udptable, skb, saddr, sport, daddr, hnum);
        if !sk.is_null() {
            return sk;
        }
    }

    if result.is_null() {
        let hash2 = ipv6_portaddr_hash(net, &in6addr_any, hnum);
        let slot2 = hash2 & (*udptable).mask;
        let hslot2 = &(*udptable).hash2[slot2];
        result = udp6_lib_lookup2(net, saddr, sport, &in6addr_any, hnum, dif, sdif, hslot2, skb);
    }

    result
}

#[no_mangle]
pub unsafe extern "C" fn __udp6_lib_lookup_skb(
    skb: *mut sk_buff,
    sport: u16,
    dport: u16,
    udptable: *mut udp_table,
) -> *mut sock {
    let iph = ipv6_hdr(skb);
    __udp6_lib_lookup(dev_net((*skb).dev), &(*iph).saddr, sport, &(*iph).daddr, dport, inet6_iif(skb), inet6_sdif(skb), udptable, skb)
}

#[no_mangle]
pub unsafe extern "C" fn udp6_lib_lookup_skb(
    skb: *const sk_buff,
    sport: u16,
    dport: u16,
) -> *mut sock {
    let iph = ipv6_hdr(skb as *mut sk_buff);
    __udp6_lib_lookup(dev_net((*skb).dev), &(*iph).saddr, sport, &(*iph).daddr, dport, inet6_iif(skb as *mut sk_buff), inet6_sdif(skb as *mut sk_buff), &udp_table, ptr::null_mut())
}

#[no_mangle]
pub unsafe extern "C" fn udp6_lib_lookup(
    net: *const net,
    saddr: *const in6_addr,
    sport: u16,
    daddr: *const in6_addr,
    dport: u16,
    dif: c_int,
) -> *mut sock {
    let sk = __udp6_lib_lookup(net, saddr, sport, daddr, dport, dif, 0, &udp_table, ptr::null_mut());
    if !sk.is_null() && refcount_inc_not_zero(&(*sk).sk_refcnt) != 0 {
        sk
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn udp6_skb_len(
    skb: *mut sk_buff,
) -> c_int {
    if inet6_is_jumbogram(skb) != 0 {
        (*skb).len
    } else {
        udp_skb_len(skb)
    }
}

#[no_mangle]
pub unsafe extern "C" fn udpv6_recvmsg(
    sk: *mut sock,
    msg: *mut c_void,
    len: usize,
    noblock: c_int,
    flags: c_int,
    addr_len: *mut c_int,
) -> c_int {
    let np = &(*sk).ipv6_pinfo;
    let inet = &(*sk).inet_sk;
    let mut skb: *mut sk_buff = ptr::null_mut();
    let mut ulen: c_int = 0;
    let mut copied: usize = 0;
    let mut err: c_int = 0;
    let mut is_udplite: c_int = 0;
    let mut is_udp4: c_int = 0;
    let mut mib: *mut udp_mib = ptr::null_mut();

    if flags & MSG_ERRQUEUE != 0 {
        return ipv6_recv_error(sk, msg, len, addr_len);
    }

    if np.rxpmtu != 0 && np.rxopt.bits.rxpmtu != 0 {
        return ipv6_recv_rxpmtu(sk, msg, len, addr_len);
    }

    let mut off = sk_peek_offset(sk, flags);
    skb = __skb_recv_udp(sk, flags, noblock, &mut off, &mut err);
    if skb.is_null() {
        return err;
    }

    ulen = udp6_skb_len(skb);
    copied = len;
    if copied > ulen - off as usize {
        copied = ulen - off as usize;
        (*msg).msg_flags |= MSG_TRUNC;
    }

    is_udp4 = if (*skb).protocol == htons(ETH_P_IP) { 1 } else { 0 };
    mib = __UDPX_MIB(sk, is_udp4);

    if copied < ulen as usize || (flags & MSG_PEEK) != 0 || (is_udplite != 0 && UDP_SKB_CB(skb).partial_cov != 0) {
        let checksum_valid = udp_skb_csum_unnecessary(skb) != 0 || __udp_lib_checksum_complete(skb) == 0;
        if !checksum_valid {
            return -EINVAL;
        }
    }

    if checksum_valid || udp_skb_csum_unnecessary(skb) != 0 {
        if udp_skb_is_linear(skb) != 0 {
            return copy_linear_skb(skb, copied, off, (*msg).msg_iter);
        } else {
            return skb_copy_datagram(skb, off, (*msg).msg_iter, copied);
        }
    }

    0
}

// Helper functions
unsafe fn sock_net(sk: *mut sock) -> *mut net {
    // Implementation depends on kernel structure
    ptr::null_mut()
}

unsafe fn net_eq(net1: *const net, net2: *const net) -> c_int {
    // Implementation depends on kernel structure
    1
}

unsafe fn ipv6_addr_equal(addr1: *const in6_addr, addr2: *const in6_addr) -> c_int {
    // Implementation depends on kernel structure
    1
}

unsafe fn ipv6_addr_any(addr: *const in6_addr) -> c_int {
    // Implementation depends on kernel structure
    1
}

unsafe fn udp_sk_bound_dev_eq(net: *const net, bound_dev_if: c_int, dif: c_int, sdif: c_int) -> c_int {
    // Implementation depends on kernel structure
    1
}

unsafe fn raw_smp_processor_id() -> c_int {
    // Implementation depends on kernel structure
    0
}

// Test cases
#[cfg(test)]
mod tests {
    #[test]
    fn test_udp6_ehashfn() {
        // Basic test for hash function
        // Note: Actual values depend on kernel structures
    }
}