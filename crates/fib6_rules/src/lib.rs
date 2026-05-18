use kernel_types::*;

const FR_ACT_TO_TBL: u32 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib6_rule {
    pub common: fib_rule_common,
    pub src: in6_addr,
    pub src_len: u8,
    pub dst: in6_addr,
    pub dst_len: u8,
    pub tos: u8,
    pub table: u32,
    pub l3mdev: u32,
    pub pref: u32,
    pub action: u32,
    pub flags: u32,
    pub suppress_ifgroup: u32,
    pub suppress_prefixlen: u32,
    pub priority: u32,
    pub fwmark: u32,
    pub fwmask: u32,
    pub ifname: [c_char; 16],
    pub goto: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_rule_common {
    pub action: u32,
    pub flags: u32,
    pub suppress_ifgroup: u32,
    pub suppress_prefixlen: u32,
    pub priority: u32,
    pub fwmark: u32,
    pub fwmask: u32,
    pub ifname: [c_char; 16],
    pub goto: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib6_info {
    pub f6i_family: u16,
    pub f6i_tclassid: u32,
    pub f6i_flowinfo: u32,
    pub f6i_secid: u32,
    pub f6i_mark: u32,
    pub f6i_ifindex: i32,
    pub f6i_nh_sel: u32,
    pub f6i_nh: *mut fib6_nh,
    pub f6i_dev: *mut c_void,
    pub f6i_flags: u32,
    pub f6i_expires: u64,
    pub f6i_protocol: u8,
    pub f6i_pmtu: u32,
    pub f6i_advmss: u32,
    pub f6i_mtu: u32,
    pub f6i_idev: *mut c_void,
    pub f6i_rt: *mut c_void,
    pub f6i_rtnl: *mut c_void,
    pub f6i_nh_sel_cnt: u32,
    pub f6i_nh_cnt: u32,
    pub f6i_nhs: *mut fib6_nh,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib6_nh {
    pub nh_common: fib_nh_common,
    pub nh_gw: in6_addr,
    pub nh_oif: i32,
    pub nh_hops: u8,
    pub nh_flags: u8,
    pub nh_scope: u8,
    pub nh_dev: *mut c_void,
    pub nh_idev: *mut c_void,
    pub nh_sdev: *mut c_void,
    pub nh_rt: *mut c_void,
    pub nh_rtnl: *mut c_void,
    pub nh_expires: u64,
    pub nh_pmtu: u32,
    pub nh_advmss: u32,
    pub nh_mtu: u32,
    pub nh_protocol: u8,
    pub nh_nh_sel: u32,
    pub nh_nh: *mut fib6_nh,
    pub nh_nh_cnt: u32,
    pub nh_nhs: *mut fib6_nh,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_nh_common {
    pub nh_flags: u8,
    pub nh_scope: u8,
    pub nh_protocol: u8,
    pub nh_nh_sel: u32,
    pub nh_nh: *mut c_void,
    pub nh_nh_cnt: u32,
    pub nh_nhs: *mut c_void,
}

#[no_mangle]
pub extern "C" fn fib6_rule_match(
    rule: *const fib6_rule,
    fl6: *const flowi6,
    flags: u32,
) -> bool {
    unsafe {
        let rule = &*rule;
        let fl6 = &*fl6;

        if rule.action != FR_ACT_TO_TBL {
            return false;
        }

        if rule.src_len > 0 {
            let src_mask = !((1 << (128 - rule.src_len)) - 1);
            let src = rule.src.in6_u.u6_addr32[0] & src_mask;
            let fl6_src = fl6.saddr.in6_u.u6_addr32[0] & src_mask;

            if src != fl6_src {
                return false;
            }
        }

        if rule.dst_len > 0 {
            let dst_mask = !((1 << (128 - rule.dst_len)) - 1);
            let dst = rule.dst.in6_u.u6_addr32[0] & dst_mask;
            let fl6_dst = fl6.daddr.in6_u.u6_addr32[0] & dst_mask;

            if dst != fl6_dst {
                return false;
            }
        }

        if rule.tos != 0 && rule.tos != fl6.flowi6_tos {
            return false;
        }

        if rule.fwmark != 0 && (rule.fwmark & rule.fwmask) != (fl6.flowi6_mark & rule.fwmask) {
            return false;
        }

        if rule.ifname[0] != 0 {
            let ifname = core::ffi::CStr::from_ptr(rule.ifname.as_ptr());
            let fl6_ifname = core::ffi::CStr::from_ptr(fl6.fl6_iifname.as_ptr());

            if ifname != fl6_ifname {
                return false;
            }
        }

        true
    }
}

#[no_mangle]
pub extern "C" fn fib6_rule_action(
    rule: *const fib6_rule,
    fl6: *const flowi6,
    res: *mut fib6_rule_action_result,
) -> c_int {
    unsafe {
        let rule = &*rule;
        let fl6 = &*fl6;
        let res = &mut *res;

        if rule.action != FR_ACT_TO_TBL {
            return -EINVAL;
        }

        res.table = rule.table;
        res.oif = fl6.oif;
        res.mark = fl6.flowi6_mark;
        res.tos = fl6.flowi6_tos;

        0
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib6_rule_action_result {
    pub table: u32,
    pub oif: i32,
    pub mark: u32,
    pub tos: u8,
}