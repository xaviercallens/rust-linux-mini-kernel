use kernel_types::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arp_tbl {
    pub family: c_int,
    pub key_len: c_int,
    pub hash: *mut c_void,
    pub key_ctor: *mut c_void,
    pub destructor: *mut c_void,
    pub seq_ops: *mut c_void,
    pub arp_parms: *mut c_void,
    pub gc: *mut c_void,
    pub gc_interval: c_int,
    pub gc_thresh1: c_int,
    pub gc_thresh2: c_int,
    pub gc_thresh3: c_int,
    pub last_flush: c_long,
    pub last_rand: c_long,
    pub last_seq: c_long,
    pub entries: c_int,
    pub last_walk: c_long,
    pub rtnl: *mut c_void,
    pub dev: *mut c_void,
    pub stats: *mut c_void,
    pub id: c_char,
    pub parms: *mut c_void,
    pub tb_id: c_char,
    pub owner: *mut c_void,
}

pub const ARPHRD_ETHER: c_int = 1;
pub const ETH_P_IP: c_int = 0x0800;
pub const ETH_HLEN: usize = 14;
pub const ARPOP_REQUEST: c_int = 1;

#[repr(C)]
pub struct net_device {
    pub type_: c_int,
}

#[repr(C)]
pub struct arphdr {
    pub ar_hrd: u16,
    pub ar_pro: u16,
    pub ar_hln: u8,
    pub ar_pln: u8,
    pub ar_op: u16,
    pub ar_sip: *mut in_addr,
    pub ar_tip: *mut in_addr,
}

#[repr(C)]
pub struct neighbour {
    pub dev: *mut net_device,
    pub ops: *mut ndisc_ops,
}

#[repr(C)]
pub struct ndisc_ops {
    pub output: Option<unsafe extern "C" fn(*mut sk_buff, *mut neighbour) -> c_int>,
}

#[inline(always)]
pub fn htons(x: c_int) -> u16 {
    x.to_be() as u16
}

#[no_mangle]
pub extern "C" fn arp_send(
    skb: *mut sk_buff,
    ip: *mut c_void,
) -> c_int {
    unsafe {
        if skb.is_null() || ip.is_null() {
            return -EINVAL;
        }

        if (*skb).dev.is_null() {
            return -EINVAL;
        }

        let dev = (*skb).dev as *mut net_device;
        if (*dev).type_ != ARPHRD_ETHER {
            return -EINVAL;
        }

        let eth = (*skb).data as *mut ethhdr;
        if (*eth).h_proto != htons(ETH_P_IP) {
            return -EINVAL;
        }

        let arp = (*skb).data.offset(ETH_HLEN as isize) as *mut arphdr;
        if (*arp).ar_op != htons(ARPOP_REQUEST) {
            return -EINVAL;
        }

        let saddr = (*arp).ar_sip as *mut in_addr;
        let daddr = (*arp).ar_tip as *mut in_addr;

        if (*saddr).s_addr == (*daddr).s_addr {
            return -EINVAL;
        }

        let dst = (*skb).dst as *mut neighbour;
        if dst.is_null() || (*dst).dev.is_null() || (*dst).dev != (*skb).dev as *mut net_device {
            return -EINVAL;
        }

        if (*dst).ops.is_null() {
            return -EINVAL;
        }

        let ops = (*dst).ops as *mut ndisc_ops;
        if (*ops).output.is_none() {
            return -EINVAL;
        }

        ((*ops).output.unwrap())(skb, dst)
    }
}