use kernel_types::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_v6_lookup_args {
    pub skb: *mut sk_buff,
    pub flowi: *mut flowi,
    pub oif: c_int,
    pub saddr: *const in6_addr,
    pub daddr: *const in6_addr,
    pub nexthdr: c_int,
    pub hdr_len: c_int,
    pub dev: *mut net_device,
    pub flags: c_uint,
    pub sdif: c_int,
    pub connected: c_int,
    pub flow: *mut flowi,
    pub loc_sk: *mut sock,
    pub loc_addr: *mut in6_addr,
    pub loc_port: c_int,
    pub loc_rcv_saddr: *mut in6_addr,
    pub loc_rcv_port: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn raw_v6_lookup(
    skb: *mut sk_buff,
    flowi: *mut flowi,
    oif: c_int,
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    nexthdr: c_int,
    hdr_len: c_int,
    dev: *mut net_device,
    flags: c_uint,
    sdif: c_int,
    connected: c_int,
    flow: *mut flowi,
    loc_sk: *mut sock,
    loc_addr: *mut in6_addr,
    loc_port: c_int,
    loc_rcv_saddr: *mut in6_addr,
    loc_rcv_port: c_int,
) -> c_int {
    if skb.is_null() || flowi.is_null() || saddr.is_null() || daddr.is_null() || dev.is_null() {
        return -EINVAL;
    }

    let skb = &mut *skb;
    let flowi = &mut *flowi;
    let saddr = &*saddr;
    let daddr = &*daddr;
    let dev = &mut *dev;

    if ipv6_addr_is_multicast(daddr) {
        if !ipv6_chk_mcast_addr(dev, daddr) {
            return -EINVAL;
        }
    }

    if !ipv6_addr_valid(daddr) {
        return -EINVAL;
    }

    if !ipv6_addr_valid(saddr) {
        return -EINVAL;
    }

    if !ipv6_chk_addr(dev, saddr) {
        return -EINVAL;
    }

    if !ipv6_chk_addr(dev, daddr) {
        return -EINVAL;
    }

    0
}