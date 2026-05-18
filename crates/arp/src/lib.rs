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

#[no_mangle]
pub extern "C" fn arp_send(
    skb: *mut sk_buff,
    ip: *mut c_void,
) -> c_int {
    unsafe {
        if skb.is_null() || ip.is_null() {
            return -EINVAL;
        }

        let skb = &mut *skb;
        let ip = &mut *(ip as *mut iphdr);

        if skb.dev.is_null() {
            return -EINVAL;
        }

        let dev = &mut *(skb.dev as *mut net_device);

        if dev.type_ != ARPHRD_ETHER {
            return -EINVAL;
        }

        let eth = &mut *(skb.data as *mut ethhdr);

        if eth.h_proto != htons(ETH_P_IP) {
            return -EINVAL;
        }

        let arp = &mut *(skb.data.offset(ETH_HLEN as isize) as *mut arphdr);

        if arp.ar_op != htons(ARPOP_REQUEST) {
            return -EINVAL;
        }

        let saddr = &mut *(arp.ar_sip as *mut in_addr);
        let daddr = &mut *(arp.ar_tip as *mut in_addr);

        if saddr.s_addr == daddr.s_addr {
            return -EINVAL;
        }

        let dst = &mut *(skb.dst as *mut neighbour);

        if dst.is_null() {
            return -EINVAL;
        }

        if dst.dev.is_null() {
            return -EINVAL;
        }

        if dst.dev != skb.dev {
            return -EINVAL;
        }

        if dst.ops.is_null() {
            return -EINVAL;
        }

        let ops = &mut *(dst.ops as *mut ndisc_ops);

        if ops.output.is_null() {
            return -EINVAL;
        }

        let output = ops.output;

        let ret = output(skb, dst);

        if ret != 0 {
            return ret;
        }

        return 0;
    }
}