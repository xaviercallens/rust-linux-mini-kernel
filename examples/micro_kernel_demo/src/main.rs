//! Minimal Micro Kernel Demo
//!
//! Demonstrates core kernel concepts using compiled kernel_types:
//! - Memory management primitives
//! - Process/task structures
//! - Network stack types (IPv4/IPv6)
//! - System call interface
//! - FFI boundaries

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use kernel_types::*;

/// Kernel panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Kernel entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize kernel
    kernel_init();

    // Run demo
    demo_network_stack();
    demo_memory_management();
    demo_process_management();

    // Shutdown
    kernel_shutdown();

    loop {}
}

/// Initialize kernel subsystems
fn kernel_init() {
    // In a real kernel: initialize memory allocator, set up page tables,
    // initialize interrupt handlers, etc.
}

/// Demonstrate network stack types
fn demo_network_stack() {
    // IPv4 address example: 192.168.1.1
    let ipv4_addr = in_addr {
        s_addr: u32::from_be_bytes([192, 168, 1, 1]),
        ip: core::ptr::null_mut(),
    };

    // IPv6 loopback: ::1
    let ipv6_loopback = in6_addr {
        in6_u: in6_addr_union {
            u6_addr32: [0, 0, 0, u32::from_be(1)],
        },
        s6_addr: core::ptr::null_mut(),
    };

    // Unified address (netfilter)
    let unified = nf_inet_addr {
        all: [0, 0, 0, 0],
    };

    // IPv4 header
    let iph = iphdr {
        version: 4,
        ihl: 5,  // 5 * 4 = 20 bytes
        tos: 0,
        tot_len: u16::from_be(40),
        id: u16::from_be(12345),
        frag_off: 0,
        ttl: 64,
        protocol: 6,  // TCP
        check: 0,
        saddr: ipv4_addr.s_addr,
        daddr: u32::from_be_bytes([8, 8, 8, 8]),  // 8.8.8.8
    };

    // IPv6 header
    let ip6h = ipv6hdr {
        version: 6,
        priority: 0,
        flow_lbl: [0, 0, 0],
        payload_len: u16::from_be(20),
        nexthdr: 6,  // TCP
        hop_limit: 64,
        saddr: ipv6_loopback,
        daddr: ipv6_loopback,
    };

    // Validate headers
    unsafe {
        validate_ipv4_header(&iph);
        validate_ipv6_header(&ip6h);
    }
}

/// Demonstrate memory management
fn demo_memory_management() {
    // Socket buffer (packet buffer)
    let skb: *mut sk_buff = core::ptr::null_mut();

    // Destination cache entry
    let dst: *mut dst_entry = core::ptr::null_mut();

    // In real kernel: allocate skb, attach dst, route packet
    unsafe {
        if !skb.is_null() {
            // Would manipulate packet buffer
        }
        if !dst.is_null() {
            // Would perform routing
        }
    }
}

/// Demonstrate process/task management
fn demo_process_management() {
    // Socket structure (represents a network endpoint)
    let sock_ptr: *mut sock = core::ptr::null_mut();

    // TCP socket
    let tcp_sock_ptr: *mut tcp_sock = core::ptr::null_mut();

    // UDP socket
    let udp_sock_ptr: *mut udp_sock = core::ptr::null_mut();

    unsafe {
        if !sock_ptr.is_null() {
            // Would manage socket state
        }
        if !tcp_sock_ptr.is_null() {
            // Would handle TCP connection
        }
        if !udp_sock_ptr.is_null() {
            // Would handle UDP datagram
        }
    }
}

/// Validate IPv4 header
unsafe fn validate_ipv4_header(iph: *const iphdr) {
    if iph.is_null() {
        return;
    }

    let header = &*iph;
    let version = header.version;
    let ihl = header.ihl;

    // Version must be 4
    assert!(version == 4, "Invalid IPv4 version");

    // IHL must be at least 5 (20 bytes)
    assert!(ihl >= 5, "Invalid IPv4 IHL");

    // Total length must be at least IHL * 4
    let tot_len = u16::from_be(header.tot_len);
    let min_len = (ihl as u16) * 4;
    assert!(tot_len >= min_len, "Invalid IPv4 total length");
}

/// Validate IPv6 header
unsafe fn validate_ipv6_header(ip6h: *const ipv6hdr) {
    if ip6h.is_null() {
        return;
    }

    let header = &*ip6h;
    let version = header.version;

    // Version must be 6
    assert!(version == 6, "Invalid IPv6 version");
}

/// Shutdown kernel
fn kernel_shutdown() {
    // Cleanup resources
}

// System call interface (extern "C" for FFI)

/// System call: socket creation
#[no_mangle]
pub unsafe extern "C" fn sys_socket(
    family: c_int,
    sock_type: c_int,
    protocol: c_int,
) -> c_int {
    // Validate parameters
    if family < 0 || sock_type < 0 {
        return -EINVAL;
    }

    // In real kernel: allocate socket structure, initialize, return fd
    0  // Success
}

/// System call: bind socket to address
#[no_mangle]
pub unsafe extern "C" fn sys_bind(
    sockfd: c_int,
    addr: *const sockaddr,
    addrlen: socklen_t,
) -> c_int {
    // Validate parameters
    if sockfd < 0 || addr.is_null() || addrlen == 0 {
        return -EINVAL;
    }

    // In real kernel: find socket by fd, bind to address
    0  // Success
}

/// System call: send data
#[no_mangle]
pub unsafe extern "C" fn sys_sendto(
    sockfd: c_int,
    buf: *const c_void,
    len: size_t,
    flags: c_int,
    dest_addr: *const sockaddr,
    addrlen: socklen_t,
) -> ssize_t {
    // Validate parameters
    if sockfd < 0 || buf.is_null() || len == 0 {
        return -EINVAL as ssize_t;
    }

    // In real kernel: find socket, create skb, route, transmit
    len as ssize_t  // Return bytes sent
}

/// System call: receive data
#[no_mangle]
pub unsafe extern "C" fn sys_recvfrom(
    sockfd: c_int,
    buf: *mut c_void,
    len: size_t,
    flags: c_int,
    src_addr: *mut sockaddr,
    addrlen: *mut socklen_t,
) -> ssize_t {
    // Validate parameters
    if sockfd < 0 || buf.is_null() || len == 0 {
        return -EINVAL as ssize_t;
    }

    // In real kernel: find socket, dequeue skb, copy to user buffer
    0  // Return bytes received
}

/// System call: close socket
#[no_mangle]
pub unsafe extern "C" fn sys_close(sockfd: c_int) -> c_int {
    if sockfd < 0 {
        return -EINVAL;
    }

    // In real kernel: release socket resources
    0  // Success
}

// Netfilter hook example

/// Netfilter hook: packet filtering
#[no_mangle]
pub unsafe extern "C" fn nf_hook_filter(
    priv_data: *mut c_void,
    skb: *mut sk_buff,
    state: *const nf_hook_state,
) -> c_uint {
    // Validate parameters
    if skb.is_null() || state.is_null() {
        return NF_DROP as c_uint;
    }

    // In real kernel: inspect packet, apply filtering rules
    NF_ACCEPT as c_uint  // Accept packet
}

// Netfilter verdicts
const NF_DROP: c_int = 0;
const NF_ACCEPT: c_int = 1;
const NF_STOLEN: c_int = 2;
const NF_QUEUE: c_int = 3;
const NF_REPEAT: c_int = 4;

// Placeholder for nf_hook_state (simplified)
#[repr(C)]
struct nf_hook_state {
    hook: u8,
    pf: u8,
}
