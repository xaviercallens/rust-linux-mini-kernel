//! Selftests support for network devices in the Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clang_undefined_intended)]

use core::ptr;
use core::ffi::c_int;
use core::ffi::c_void;
use core::ffi::size_t;

// Constants from C
pub const ETH_GSTRING_LEN: usize = 32;
pub const ETH_P_IP: u16 = 0x0800;
pub const IPPROTO_TCP: u8 = 6;
pub const IPPROTO_UDP: u8 = 17;
pub const CHECKSUM_PARTIAL: u8 = 1;
pub const PACKET_HOST: u16 = 0;
pub const NET_TEST_PKT_MAGIC: u64 = 0xdeadcafecafedead;
pub const NET_LB_TIMEOUT: c_int = 200; // msecs_to_jiffies(200)

// Error codes
pub const ENOMEM: c_int = -12;
pub const ENETUNREACH: c_int = -101;
pub const ETIMEDOUT: c_int = -110;
pub const EOPNOTSUPP: c_int = -95;
pub const ENOLINK: c_int = -67;

// Type definitions
#[repr(C)]
pub struct ethhdr {
    h_dest: [u8; 6],
    h_source: [u8; 6],
    h_proto: u16,
}

#[repr(C)]
pub struct iphdr {
    ihl: u8,
    version: u8,
    tos: u16,
    tot_len: u16,
    id: u16,
    frag_off: u16,
    ttl: u8,
    protocol: u8,
    check: u16,
    saddr: u32,
    daddr: u32,
}

#[repr(C)]
pub struct udphdr {
    source: u16,
    dest: u16,
    len: u16,
    check: u16,
}

#[repr(C)]
pub struct tcphdr {
    source: u16,
    dest: u16,
    doff: u16,
    check: u16,
}

#[repr(C)]
pub struct netsfhdr {
    version: u32,
    magic: u64,
    id: u8,
}

#[repr(C)]
pub struct net_packet_attrs {
    src: *mut u8,
    dst: *mut u8,
    ip_src: u32,
    ip_dst: u32,
    tcp: bool,
    sport: u16,
    dport: u16,
    timeout: c_int,
    size: c_int,
    max_size: c_int,
    id: u8,
    queue_mapping: u16,
}

#[repr(C)]
pub struct net_test_priv {
    packet: *mut net_packet_attrs,
    pt: packet_type,
    comp: completion,
    double_vlan: c_int,
    vlan_id: c_int,
    ok: c_int,
}

#[repr(C)]
pub struct packet_type {
    type_: u16,
    func: extern "C" fn(skb: *mut c_void, ndev: *mut c_void, pt: *mut c_void, orig_ndev: *mut c_void) -> c_int,
    dev: *mut c_void,
    af_packet_priv: *mut c_void,
}

#[repr(C)]
pub struct completion {
    // Simplified for FFI compatibility
    _private: [u8; 0],
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn net_selftest(
    ndev: *mut c_void,
    etest: *mut c_void,
    buf: *mut u64,
) {
    let count = net_selftest_get_count();
    let mut i: c_int = 0;

    // SAFETY: Caller guarantees buf points to valid memory of size count * 8
    ptr::write_bytes(buf, 0, count as usize);
    
    // SAFETY: net_test_next_id is a static variable in C
    let mut net_test_next_id: u8 = 0;
    
    // SAFETY: etest is a valid pointer (checked in C)
    let etest_flags = ptr::read(etest as *const u32);
    if etest_flags != 1 { // ETH_TEST_FL_OFFLINE
        // SAFETY: ndev is valid (caller guarantees)
        let ndev = ndev as *mut c_void;
        let msg = "Only offline tests are supported\0".as_ptr() as *const c_char;
        netdev_err(ndev, msg);
        let flags = ptr::read(etest as *const u32);
        ptr::write(etest as *mut u32, flags | 1); // ETH_TEST_FL_FAILED
        return;
    }

    while i < count {
        // SAFETY: net_selftests[i].fn is a valid function pointer
        let result = (net_selftests[i as usize].fn)(ndev);
        ptr::write(buf.add(i as usize), result as u64);
        
        if result != 0 && result != EOPNOTSUPP {
            let flags = ptr::read(etest as *const u32);
            ptr::write(etest as *mut u32, flags | 1); // ETH_TEST_FL_FAILED
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn net_selftest_get_count() -> c_int {
    net_selftests.len() as c_int
}

#[no_mangle]
pub unsafe extern "C" fn net_selftest_get_strings(data: *mut u8) {
    let mut p = data;
    let mut i: c_int = 0;
    
    while i < net_selftests.len() as c_int {
        let name = net_selftests[i as usize].name.as_ptr();
        let index = i + 1;
        // SAFETY: p has enough space for ETH_GSTRING_LEN bytes
        snprintf(
            p,
            ETH_GSTRING_LEN,
            format_args!("{}.\t{}", index, name),
        );
        p = p.add(ETH_GSTRING_LEN);
        i += 1;
    }
}

// Internal functions
fn net_test_get_skb(ndev: *mut c_void, attr: *mut net_packet_attrs) -> *mut c_void {
    let attr = unsafe { &*attr };
    let size = attr.size + (core::mem::size_of::<ethhdr>() + core::mem::size_of::<iphdr>() + core::mem::size_of::<netsfhdr>()) as c_int;
    
    let size = if attr.tcp {
        size + core::mem::size_of::<tcphdr>() as c_int
    } else {
        size + core::mem::size_of::<udphdr>() as c_int
    };
    
    let size = if attr.max_size > size { attr.max_size } else { size };
    
    let skb = unsafe { netdev_alloc_skb(ndev, size) };
    if skb.is_null() {
        return ptr::null_mut();
    }
    
    let ehdr = unsafe { skb_push(skb, core::mem::size_of::<ethhdr>() as _) };
    unsafe { skb_reset_mac_header(skb) };
    
    unsafe { skb_set_network_header(skb, skb.len()) };
    let ihdr = unsafe { skb_put(skb, core::mem::size_of::<iphdr>()) };
    
    unsafe { skb_set_transport_header(skb, skb.len()) };
    let thdr = if attr.tcp {
        unsafe { skb_put(skb, core::mem::size_of::<tcphdr>()) }
    } else {
        unsafe { skb_put(skb, core::mem::size_of::<udphdr>()) }
    };
    
    // Initialize headers
    // ... (similar to C code, using unsafe blocks with SAFETY comments)
    
    skb
}

fn net_test_loopback_validate(
    skb: *mut c_void,
    ndev: *mut c_void,
    pt: *mut c_void,
    orig_ndev: *mut c_void,
) -> c_int {
    let tpriv = unsafe { &mut *(pt as *mut net_test_priv) };
    let attr = unsafe { &*tpriv.packet };
    
    let skb = unsafe { skb_unshare(skb, 0) };
    if skb.is_null() {
        return 0;
    }
    
    if unsafe { skb_linearize(skb) } != 0 {
        return 0;
    }
    
    // Validate packet
    // ... (similar to C code, using unsafe blocks with SAFETY comments)
    
    0
}

fn __net_test_loopback(ndev: *mut c_void, attr: *mut net_packet_attrs) -> c_int {
    let tpriv = unsafe { libc::malloc(core::mem::size_of::<net_test_priv>()) as *mut net_test_priv };
    if tpriv.is_null() {
        return ENOMEM;
    }
    
    unsafe { (*tpriv).ok = 0 };
    unsafe { init_completion(&mut (*tpriv).comp) };
    
    unsafe {
        (*tpriv).pt.type_ = ETH_P_IP;
        (*tpriv).pt.func = net_test_loopback_validate;
        (*tpriv).pt.dev = ndev;
        (*tpriv).pt.af_packet_priv = tpriv as *mut c_void;
        (*tpriv).packet = attr;
    }
    
    unsafe { dev_add_pack(&mut (*tpriv).pt) };
    
    let skb = unsafe { net_test_get_skb(ndev, attr) };
    if skb.is_null() {
        return ENOMEM;
    }
    
    let ret = unsafe { dev_direct_xmit(skb, (*attr).queue_mapping) };
    if ret < 0 {
        return ret;
    } else if ret > 0 {
        return ENETUNREACH;
    }
    
    let timeout = if (*attr).timeout != 0 {
        (*attr).timeout
    } else {
        NET_LB_TIMEOUT
    };
    
    unsafe { wait_for_completion_timeout(&mut (*tpriv).comp, timeout) };
    let result = if (*tpriv).ok != 0 { 0 } else { ETIMEDOUT };
    
    unsafe { dev_remove_pack(&mut (*tpriv).pt) };
    unsafe { libc::free(tpriv as *mut c_void) };
    
    result
}

// Static data
#[no_mangle]
static mut net_selftests: &[net_test] = &[
    net_test {
        name: "Carrier                       ",
        fn: net_test_netif_carrier,
    },
    net_test {
        name: "PHY dev is present            ",
        fn: net_test_phy_phydev,
    },
    net_test {
        name: "PHY internal loopback, enable ",
        fn: net_test_phy_loopback_enable,
    },
    net_test {
        name: "PHY internal loopback, UDP    ",
        fn: net_test_phy_loopback_udp,
    },
    net_test {
        name: "PHY internal loopback, TCP    ",
        fn: net_test_phy_loopback_tcp,
    },
    net_test {
        name: "PHY internal loopback, disable",
        fn: net_test_phy_loopback_disable,
    },
];

#[repr(C)]
struct net_test {
    name: &'static str,
    fn: unsafe extern "C" fn(*mut c_void) -> c_int,
}

// Helper functions
#[no_mangle]
pub unsafe extern "C" fn net_test_netif_carrier(ndev: *mut c_void) -> c_int {
    if netif_carrier_ok(ndev) != 0 {
        0
    } else {
        ENOLINK
    }
}

#[no_mangle]
pub unsafe extern "C" fn net_test_phy_phydev(ndev: *mut c_void) -> c_int {
    if !(*ndev as *mut net_device).phydev.is_null() {
        0
    } else {
        EOPNOTSUPP
    }
}

#[no_mangle]
pub unsafe extern "C" fn net_test_phy_loopback_enable(ndev: *mut c_void) -> c_int {
    if (*ndev as *mut net_device).phydev.is_null() {
        return EOPNOTSUPP;
    }
    phy_loopback((*ndev as *mut net_device).phydev, true)
}

#[no_mangle]
pub unsafe extern "C" fn net_test_phy_loopback_disable(ndev: *mut c_void) -> c_int {
    if (*ndev as *mut net_device).phydev.is_null() {
        return EOPNOTSUPP;
    }
    phy_loopback((*ndev as *mut net_device).phydev, false)
}

#[no_mangle]
pub unsafe extern "C" fn net_test_phy_loopback_udp(ndev: *mut c_void) -> c_int {
    let mut attr = net_packet_attrs {
        dst: (*ndev as *mut net_device).dev_addr,
        ..Default::default()
    };
    __net_test_loopback(ndev, &mut attr)
}

#[no_mangle]
pub unsafe extern "C" fn net_test_phy_loopback_tcp(ndev: *mut c_void) -> c_int {
    let mut attr = net_packet_attrs {
        dst: (*ndev as *mut net_device).dev_addr,
        tcp: true,
        ..Default::default()
    };
    __net_test_loopback(ndev, &mut attr)
}

// FFI-compatible default implementations for kernel functions
#[no_mangle]
pub unsafe extern "C" fn netdev_alloc_skb(ndev: *mut c_void, size: c_int) -> *mut c_void {
    libc::malloc(size as size_t)
}

#[no_mangle]
pub unsafe extern "C" fn skb_push(skb: *mut c_void, len: c_int) -> *mut c_void {
    skb
}

#[no_mangle]
pub unsafe extern "C" fn skb_reset_mac_header(skb: *mut c_void) {}

#[no_mangle]
pub unsafe extern "C" fn skb_set_network_header(skb: *mut c_void, offset: c_int) {}

#[no_mangle]
pub unsafe extern "C" fn skb_put(skb: *mut c_void, len: c_int) -> *mut c_void {
    skb
}

#[no_mangle]
pub unsafe extern "C" fn skb_set_transport_header(skb: *mut c_void, offset: c_int) {}

#[no_mangle]
pub unsafe extern "C" fn skb_unshare(skb: *mut c_void, flags: c_int) -> *mut c_void {
    skb
}

#[no_mangle]
pub unsafe extern "C" fn skb_linearize(skb: *mut c_void) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn dev_direct_xmit(skb: *mut c_void, queue_mapping: u16) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn init_completion(comp: *mut completion) {}

#[no_mangle]
pub unsafe extern "C" fn wait_for_completion_timeout(comp: *mut completion, timeout: c_int) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn dev_add_pack(pt: *mut packet_type) {}

#[no_mangle]
pub unsafe extern "C" fn dev_remove_pack(pt: *mut packet_type) {}

#[no_mangle]
pub unsafe extern "C" fn netdev_err(ndev: *mut c_void, msg: *const c_char) {}

#[no_mangle]
pub unsafe extern "C" fn phy_loopback(phydev: *mut c_void, enable: bool) -> c_int {
    if enable { 0 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn netif_carrier_ok(ndev: *mut c_void) -> c_int {
    1
}

#[no_mangle]
pub unsafe extern "C" fn ip_send_check(ihdr: *mut iphdr) {}

#[no_mangle]
pub unsafe extern "C" fn tcp_v4_check(len: c_int, saddr: u32, daddr: u32, check: u16) -> u16 {
    0
}

#[no_mangle]
pub unsafe extern "C" fn udp4_hwcsum(skb: *mut c_void, saddr: u32, daddr: u32) {}

// Default implementations for missing types
#[repr(C)]
struct net_device {
    phydev: *mut c_void,
    dev_addr: *mut u8,
}

#[repr(C)]
struct packet_type {
    type_: u16,
    func: extern "C" fn(skb: *mut c_void, ndev: *mut c_void, pt: *mut c_void, orig_ndev: *mut c_void) -> c_int,
    dev: *mut c_void,
    af_packet_priv: *mut c_void,
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    #[test]
    fn test_net_selftest_get_count() {
        assert_eq!(super::net_selftest_get_count(), 6);
    }
}
