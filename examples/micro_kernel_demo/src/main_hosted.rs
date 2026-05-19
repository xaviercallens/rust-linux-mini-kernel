use kernel_types::*;

/// Helper to create an in6_addr from standard library Ipv6Addr
pub fn from_ipv6(addr: std::net::Ipv6Addr) -> in6_addr {
    let bytes = addr.octets();
    in6_addr {
        in6_u: in6_addr_union { u6_addr8: bytes },
        s6_addr: std::ptr::null_mut()
    }
}

fn main() {
    println!("Micro kernel demo - hosted version");

    // Example usage
    let localhost = std::net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
    let addr = from_ipv6(localhost);

    println!("IPv6 localhost initialized successfully");
}
