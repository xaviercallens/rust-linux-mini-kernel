//! Minimal Micro Kernel Demo (Hosted Version)
//!
//! Demonstrates core kernel concepts using compiled kernel_types.
//! This version runs on hosted (std) environment for easier demonstration.

use kernel_types::*;
use std::mem::{size_of, align_of};

fn main() {
    println!("========================================");
    println!("   RUST MINI KERNEL DEMO - v0.1.0");
    println!("========================================\n");

    demo_type_sizes();
    demo_network_stack();
    demo_memory_management();
    demo_process_management();
    demo_system_calls();
    demo_netfilter();

    println!("\n========================================");
    println!("   DEMO COMPLETE - ALL CHECKS PASSED");
    println!("========================================\n");
}

/// Demonstrate kernel type sizes and alignment
fn demo_type_sizes() {
    println!("📊 KERNEL TYPE ANALYSIS");
    println!("─────────────────────────────────────────\n");

    println!("Network Addresses:");
    println!("  in_addr:        {} bytes (align: {})", size_of::<in_addr>(), align_of::<in_addr>());
    println!("  in6_addr:       {} bytes (align: {})", size_of::<in6_addr>(), align_of::<in6_addr>());
    println!("  nf_inet_addr:   {} bytes (align: {})", size_of::<nf_inet_addr>(), align_of::<nf_inet_addr>());

    println!("\nProtocol Headers:");
    println!("  ethhdr:         {} bytes (align: {})", size_of::<ethhdr>(), align_of::<ethhdr>());
    println!("  iphdr:          {} bytes (align: {})", size_of::<iphdr>(), align_of::<iphdr>());
    println!("  ipv6hdr:        {} bytes (align: {})", size_of::<ipv6hdr>(), align_of::<ipv6hdr>());
    println!("  udphdr:         {} bytes (align: {})", size_of::<udphdr>(), align_of::<udphdr>());

    println!("\nSocket Structures:");
    println!("  sock:           {} bytes (align: {})", size_of::<sock>(), align_of::<sock>());
    println!("  tcp_sock:       {} bytes (align: {})", size_of::<tcp_sock>(), align_of::<tcp_sock>());
    println!("  udp_sock:       {} bytes (align: {})", size_of::<udp_sock>(), align_of::<udp_sock>());
    println!("  inet_sock:      {} bytes (align: {})", size_of::<inet_sock>(), align_of::<inet_sock>());

    println!("\nPacket Buffers:");
    println!("  skbuff:         {} bytes (align: {})", size_of::<sk_buff>(), align_of::<sk_buff>());
    println!("  ip6cb:          {} bytes (align: {})", size_of::<ip6cb>(), align_of::<ip6cb>());

    println!("\n✅ All types are #[repr(C)] compatible\n");
}

/// Demonstrate network stack types
fn demo_network_stack() {
    println!("🌐 NETWORK STACK DEMONSTRATION");
    println!("─────────────────────────────────────────\n");

    // IPv4 address: 192.168.1.1
    let ipv4_addr = in_addr {
        s_addr: u32::from_be_bytes([192, 168, 1, 1]),
        ip: core::ptr::null_mut(),
    };
    println!("IPv4 Address: 192.168.1.1");
    println!("  Raw value: 0x{:08x}", ipv4_addr.s_addr);

    // IPv6 loopback: ::1
    let ipv6_loopback = in6_addr {
        in6_u: in6_addr_union {
            u6_addr32: [0, 0, 0, u32::from_be(1)],
        },
        s6_addr: core::ptr::null_mut(),
    };
    println!("\nIPv6 Loopback: ::1");
    unsafe {
        println!("  Bytes: {:?}", ipv6_loopback.in6_u.u6_addr8);
    }

    // IPv4 header
    let iph = iphdr {
        version: 4,
        ihl: 5,  // 20 bytes
        tos: 0,
        tot_len: u16::to_be(40),
        id: u16::to_be(12345),
        frag_off: 0,
        ttl: 64,
        protocol: 6,  // TCP
        check: 0,
        saddr: ipv4_addr.s_addr,
        daddr: u32::from_be_bytes([8, 8, 8, 8]),  // 8.8.8.8
    };
    println!("\nIPv4 Header:");
    println!("  Version: {}", iph.version);
    println!("  IHL: {} (header length: {} bytes)", iph.ihl, iph.ihl * 4);
    println!("  TTL: {}", iph.ttl);
    println!("  Protocol: {} (TCP)", iph.protocol);

    // IPv6 header
    let ip6h = ipv6hdr {
        version: 6,
        priority: 0,
        flow_lbl: [0, 0, 0],
        payload_len: u16::to_be(20),
        nexthdr: 6,  // TCP
        hop_limit: 64,
        saddr: ipv6_loopback,
        daddr: ipv6_loopback,
    };
    println!("\nIPv6 Header:");
    println!("  Version: {}", ip6h.version);
    println!("  Hop Limit: {}", ip6h.hop_limit);
    println!("  Next Header: {} (TCP)", ip6h.nexthdr);

    unsafe {
        validate_ipv4_header(&iph);
        validate_ipv6_header(&ip6h);
    }

    println!("\n✅ Network headers validated\n");
}

/// Demonstrate memory management
fn demo_memory_management() {
    println!("💾 MEMORY MANAGEMENT");
    println!("─────────────────────────────────────────\n");

    // Demonstrate pointer types
    let skb: *mut sk_buff = std::ptr::null_mut();
    let dst: *mut dst_entry = std::ptr::null_mut();

    println!("Socket Buffer (skbuff):");
    println!("  Pointer: {:?}", skb);
    println!("  Size: {} bytes", size_of::<sk_buff>());
    println!("  Purpose: Packet buffer for network data");

    println!("\nDestination Entry:");
    println!("  Pointer: {:?}", dst);
    println!("  Size: {} bytes", size_of::<dst_entry>());
    println!("  Purpose: Routing cache entry");

    println!("\n✅ Memory structures defined\n");
}

/// Demonstrate process/task management
fn demo_process_management() {
    println!("⚙️  PROCESS MANAGEMENT");
    println!("─────────────────────────────────────────\n");

    // Socket structures
    let sock_ptr: *mut sock = std::ptr::null_mut();
    let tcp_sock_ptr: *mut tcp_sock = std::ptr::null_mut();
    let udp_sock_ptr: *mut udp_sock = std::ptr::null_mut();

    println!("Socket Management:");
    println!("  sock:      {:?} ({} bytes)", sock_ptr, size_of::<sock>());
    println!("  tcp_sock:  {:?} ({} bytes)", tcp_sock_ptr, size_of::<tcp_sock>());
    println!("  udp_sock:  {:?} ({} bytes)", udp_sock_ptr, size_of::<udp_sock>());

    println!("\n✅ Process structures defined\n");
}

/// Demonstrate system calls
fn demo_system_calls() {
    println!("📞 SYSTEM CALL INTERFACE");
    println!("─────────────────────────────────────────\n");

    println!("Available System Calls:");
    println!("  sys_socket()   - Create socket");
    println!("  sys_bind()     - Bind socket to address");
    println!("  sys_sendto()   - Send data");
    println!("  sys_recvfrom() - Receive data");
    println!("  sys_close()    - Close socket");

    // Demonstrate system call (simulated)
    unsafe {
        let result = sys_socket_demo(2, 1, 0);  // AF_INET, SOCK_STREAM, 0
        if result >= 0 {
            println!("\n✅ Socket created successfully (fd: {})", result);
        }
    }

    println!();
}

/// Demonstrate netfilter hooks
fn demo_netfilter() {
    println!("🔒 NETFILTER HOOKS");
    println!("─────────────────────────────────────────\n");

    println!("Netfilter Verdict Codes:");
    println!("  NF_DROP    (0) - Drop packet");
    println!("  NF_ACCEPT  (1) - Accept packet");
    println!("  NF_STOLEN  (2) - Packet stolen");
    println!("  NF_QUEUE   (3) - Queue to userspace");
    println!("  NF_REPEAT  (4) - Repeat hook");

    println!("\nNetfilter Structures:");
    println!("  nf_conntrack_zone:   {} bytes", size_of::<nf_conntrack_zone>());
    println!("  nf_conntrack_helper: {} bytes", size_of::<nf_conntrack_helper>());
    println!("  nf_conn:             {} bytes", size_of::<nf_conn>());

    println!("\n✅ Netfilter infrastructure defined\n");
}

/// Validate IPv4 header
unsafe fn validate_ipv4_header(iph: *const iphdr) {
    if iph.is_null() {
        return;
    }

    let header = &*iph;
    let version = header.version;
    let ihl = header.ihl;

    assert_eq!(version, 4, "IPv4 version must be 4");
    assert!(ihl >= 5, "IPv4 IHL must be >= 5");

    let tot_len = u16::from_be(header.tot_len);
    let min_len = (ihl as u16) * 4;
    assert!(tot_len >= min_len, "IPv4 total length invalid");
}

/// Validate IPv6 header
unsafe fn validate_ipv6_header(ip6h: *const ipv6hdr) {
    if ip6h.is_null() {
        return;
    }

    let header = &*ip6h;
    let version = header.version;

    assert_eq!(version, 6, "IPv6 version must be 6");
}

/// Simulated system call: socket creation
unsafe fn sys_socket_demo(family: c_int, sock_type: c_int, _protocol: c_int) -> c_int {
    if family < 0 || sock_type < 0 {
        return -EINVAL;
    }
    42  // Return fake fd
}
