use kernel_types::*;

const IPPROTO_TCP: u8 = 6;
const IPPROTO_UDP: u8 = 17;
const NF_NAT_RANGE_MAP_IPS: u32 = 1;
const AF_INET: u16 = 2;
const AF_INET6: u16 = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NF_NAT_MASQUERADE {
    pub masq: NF_NAT_RANGE,
    pub timeout: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NF_NAT_RANGE {
    pub flags: u32,
    pub min_addr: NF_INET_ADDR,
    pub max_addr: NF_INET_ADDR,
    pub min_proto: NF_NAT_MULTI_RANGE_COMPAT,
    pub max_proto: NF_NAT_MULTI_RANGE_COMPAT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NF_NAT_MULTI_RANGE_COMPAT {
    pub range: [u32; 2],
}

#[no_mangle]
pub extern "C" fn NF_NAT_MASQUERADE_IPV4(
    ct: *mut NF_CONN,
    min: *mut NF_NAT_RANGE,
    max: *mut NF_NAT_RANGE,
) -> c_int {
    unsafe {
        let ct = match ct.as_ref() {
            Some(ct) => ct,
            None => return -EINVAL,
        };
        let min = match min.as_ref() {
            Some(min) => min,
            None => return -EINVAL,
        };
        let max = match max.as_ref() {
            Some(max) => max,
            None => return -EINVAL,
        };

        if ct.proto.protocol != IPPROTO_TCP && ct.proto.protocol != IPPROTO_UDP {
            return -EINVAL;
        }

        let mut masq = NF_NAT_MASQUERADE {
            masq: NF_NAT_RANGE {
                flags: NF_NAT_RANGE_MAP_IPS,
                min_addr: NF_INET_ADDR {
                    ip: ct.src.u3.ip,
                },
                max_addr: NF_INET_ADDR {
                    ip: ct.src.u3.ip,
                },
                min_proto: NF_NAT_MULTI_RANGE_COMPAT {
                    range: [0, 0],
                },
                max_proto: NF_NAT_MULTI_RANGE_COMPAT {
                    range: [0, 0],
                },
            },
            timeout: 0,
            flags: 0,
        };

        if ct.proto.protocol == IPPROTO_TCP {
            let tcp = ct as *const NF_CONN as *const TCP_SOCK;
            let tcp = match tcp.as_ref() {
                Some(tcp) => tcp,
                None => return -EINVAL,
            };
            masq.masq.min_proto.range[0] = tcp.inet.inet_sport;
            masq.masq.max_proto.range[0] = tcp.inet.inet_sport;
        } else if ct.proto.protocol == IPPROTO_UDP {
            let udp = ct as *const NF_CONN as *const UDP_SOCK;
            let udp = match udp.as_ref() {
                Some(udp) => udp,
                None => return -EINVAL,
            };
            masq.masq.min_proto.range[0] = udp.inet.inet_sport;
            masq.masq.max_proto.range[0] = udp.inet.inet_sport;
        }

        if NF_NAT_SETUP_INFO(ct, &mut masq.masq, min, max) != 0 {
            return -EINVAL;
        }

        if NF_CT_TIMEOUT_SET(ct, &mut masq.timeout, masq.flags) != 0 {
            return -EINVAL;
        }

        0
    }
}

#[no_mangle]
pub extern "C" fn NF_NAT_MASQUERADE_IPV6(
    ct: *mut NF_CONN,
    min: *mut NF_NAT_RANGE,
    max: *mut NF_NAT_RANGE,
) -> c_int {
    unsafe {
        let ct = match ct.as_ref() {
            Some(ct) => ct,
            None => return -EINVAL,
        };
        let min = match min.as_ref() {
            Some(min) => min,
            None => return -EINVAL,
        };
        let max = match max.as_ref() {
            Some(max) => max,
            None => return -EINVAL,
        };

        if ct.proto.protocol != IPPROTO_TCP && ct.proto.protocol != IPPROTO_UDP {
            return -EINVAL;
        }

        let mut masq = NF_NAT_MASQUERADE {
            masq: NF_NAT_RANGE {
                flags: NF_NAT_RANGE_MAP_IPS,
                min_addr: NF_INET_ADDR {
                    ip6: ct.src.u3.ip6,
                },
                max_addr: NF_INET_ADDR {
                    ip6: ct.src.u3.ip6,
                },
                min_proto: NF_NAT_MULTI_RANGE_COMPAT {
                    range: [0, 0],
                },
                max_proto: NF_NAT_MULTI_RANGE_COMPAT {
                    range: [0, 0],
                },
            },
            timeout: 0,
            flags: 0,
        };

        if ct.proto.protocol == IPPROTO_TCP {
            let tcp = ct as *const NF_CONN as *const TCP_SOCK;
            let tcp = match tcp.as_ref() {
                Some(tcp) => tcp,
                None => return -EINVAL,
            };
            masq.masq.min_proto.range[0] = tcp.inet.inet_sport;
            masq.masq.max_proto.range[0] = tcp.inet.inet_sport;
        } else if ct.proto.protocol == IPPROTO_UDP {
            let udp = ct as *const NF_CONN as *const UDP_SOCK;
            let udp = match udp.as_ref() {
                Some(udp) => udp,
                None => return -EINVAL,
            };
            masq.masq.min_proto.range[0] = udp.inet.inet_sport;
            masq.masq.max_proto.range[0] = udp.inet.inet_sport;
        }

        if NF_NAT_SETUP_INFO(ct, &mut masq.masq, min, max) != 0 {
            return -EINVAL;
        }

        if NF_CT_TIMEOUT_SET(ct, &mut masq.timeout, masq.flags) != 0 {
            return -EINVAL;
        }

        0
    }
}

#[no_mangle]
pub extern "C" fn NF_NAT_MASQUERADE_INET(
    ct: *mut NF_CONN,
    min: *mut NF_NAT_RANGE,
    max: *mut NF_NAT_RANGE,
) -> c_int {
    unsafe {
        let ct = match ct.as_ref() {
            Some(ct) => ct,
            None => return -EINVAL,
        };

        if ct.src.l3num == AF_INET {
            NF_NAT_MASQUERADE_IPV4(ct, min, max)
        } else if ct.src.l3num == AF_INET6 {
            NF_NAT_MASQUERADE_IPV6(ct, min, max)
        } else {
            -EINVAL
        }
    }
}