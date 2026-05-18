//! IPv4 Forwarding Information Base: policy rules
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::all)]

use core::ffi::{c_int, c_uint, c_void};
use core::ptr::{self, NonNull};
use core::mem;
use kernel_types::*;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;
pub const ENETUNREACH: c_int = -101;
pub const EACCES: c_int = -13;
pub const EAGAIN: c_int = -11;
pub const ENOBUFS: c_int = -105;

// FRA constants
pub const FRA_SRC: c_int = 1;
pub const FRA_DST: c_int = 2;
pub const FRA_FLOW: c_int = 3;

// Type definitions

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib4_rule {
    common: fib_rule,
    dst_len: u8,
    src_len: u8,
    tos: u8,
    src: u32,
    srcmask: u32,
    dst: u32,
    dstmask: u32,
    #[cfg(CONFIG_IP_ROUTE_CLASSID)]
    tclassid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_lookup_arg {
    result: *mut fib_result,
    flags: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_rules_ops {
    family: c_int,
    rule_size: c_uint,
    addr_size: c_uint,
    action: extern "C" fn(*mut fib_rule, *mut c_void, c_int, *mut fib_lookup_arg) -> c_int,
    suppress: extern "C" fn(*mut fib_rule, *mut fib_lookup_arg) -> bool,
    match_: extern "C" fn(*mut fib_rule, *mut c_void, c_int) -> bool,
    configure: extern "C" fn(*mut fib_rule, *mut c_void, *mut c_void, *mut *mut c_void, *mut c_void) -> c_int,
    delete: extern "C" fn(*mut fib_rule) -> c_int,
    compare: extern "C" fn(*mut fib_rule, *mut c_void, *mut *mut c_void) -> c_int,
    fill: extern "C" fn(*mut fib_rule, *mut c_void, *mut c_void) -> c_int,
    nlmsg_payload: extern "C" fn(*mut fib_rule) -> size_t,
    flush_cache: extern "C" fn(*mut fib_rules_ops),
    nlgroup: c_int,
    policy: *const c_void,
    owner: *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_ipv4 {
    rules_ops: *mut fib_rules_ops,
    fib_has_custom_rules: bool,
    fib_rules_require_fldissect: c_int,
    #[cfg(CONFIG_IP_ROUTE_CLASSID)]
    fib_num_tclassid_users: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flowi4 {
    daddr: u32,
    saddr: u32,
    flowi4_tos: u8,
    flowi4_proto: u8,
    fl4_sport: u16,
    fl4_dport: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_rule_hdr {
    dst_len: u8,
    src_len: u8,
    tos: u8,
    ip_proto: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nlattr {
    len: c_ushort,
    type_: c_ushort,
}

// Helper functions for container_of pattern
#[inline]
unsafe fn container_of(ptr: *const c_void, offset: usize) -> *const c_void {
    (ptr as usize - offset) as *const c_void
}

#[inline]
unsafe fn offset_of<T, U>(_: *const T, _: *const U) -> usize {
    let t: *const T = ptr::null();
    let u: *const U = &(*t).tos as *const _;
    (u as usize) - (t as usize)
}

// Exported functions
#[no_mangle]
pub unsafe extern "C" fn fib4_rule_matchall(rule: *const fib_rule) -> bool {
    let offset = offset_of::<fib4_rule, u8>(ptr::null(), &(*ptr::null::<fib4_rule>()).tos);
    let r = container_of(rule as *const c_void, offset) as *const fib4_rule;

    if (*r).dst_len != 0 || (*r).src_len != 0 || (*r).tos != 0 {
        return false;
    }

    let c_rule = &(*r).common as *const fib_rule;
    fib_rule_matchall(c_rule)
}

#[no_mangle]
pub unsafe extern "C" fn fib4_rule_default(rule: *const fib_rule) -> bool {
    if !fib4_rule_matchall(rule) || (*rule).action != 0 || (*rule).l3mdev != 0 {
        return false;
    }

    let table = (*rule).table;
    if table != 254 && table != 253 && table != 255 {
        return false;
    }

    true
}

#[no_mangle]
pub unsafe extern "C" fn __fib_lookup(
    net: *mut net,
    flp: *mut flowi4,
    res: *mut fib_result,
    flags: c_uint,
) -> c_int {
    let mut arg = fib_lookup_arg {
        result: res,
        flags,
    };
    let mut err = 0;

    // update flow if oif or iif point to device enslaved to l3mdev
    l3mdev_update_flow((*net).ipv4, flp as *mut flowi);

    err = fib_rules_lookup((*net).ipv4.rules_ops, flp as *mut flowi, 0, &mut arg);

    #[cfg(CONFIG_IP_ROUTE_CLASSID)]
    {
        if !arg.rule.is_null() {
            let rule4 = container_of(
                arg.rule as *const c_void,
                offset_of::<fib4_rule, u8>(ptr::null(), &(*ptr::null::<fib4_rule>()).tclassid),
            ) as *const fib4_rule;
            (*res).tclassid = (*rule4).tclassid;
        } else {
            (*res).tclassid = 0;
        }
    }

    if err == -13 {
        err = -ENETUNREACH;
    }

    err
}

#[no_mangle]
pub unsafe extern "C" fn fib4_rule_action(
    rule: *mut fib_rule,
    flp: *mut c_void,
    flags: c_int,
    arg: *mut fib_lookup_arg,
) -> c_int {
    let mut err = -EAGAIN;
    let mut tb_id = 0;
    let mut tbl: *mut fib_table = ptr::null_mut();

    match (*rule).action {
        0 => {} // FR_ACT_TO_TBL
        1 => return -ENETUNREACH, // FR_ACT_UNREACHABLE
        2 => return -EACCES, // FR_ACT_PROHIBIT
        3 => return -EINVAL, // FR_ACT_BLACKHOLE
        _ => return -EINVAL,
    }

    rcu_read_lock();

    tb_id = fib_rule_get_table(rule, arg);
    tbl = fib_get_table((*rule).fr_net, tb_id);
    if !tbl.is_null() {
        err = fib_table_lookup(
            tbl,
            &(*flp.cast::<flowi>()).u.ip4,
            (*arg).result as *mut fib_result,
            (*arg).flags,
        );
    }

    rcu_read_unlock();

    err
}

#[no_mangle]
pub unsafe extern "C" fn fib4_rule_suppress(
    rule: *mut fib_rule,
    arg: *mut fib_lookup_arg,
) -> bool {
    let result = (*arg).result as *mut fib_result;
    let dev: *mut c_void = ptr::null_mut();

    if !(*result).fi.is_null() {
        let nhc = fib_info_nhc((*result).fi, 0);
        dev = (*nhc).nhc_dev;
    }

    if (*result).prefixlen <= (*rule).suppress_prefixlen {
        suppress_route(result, arg, dev);
        return true;
    }

    if (*rule).suppress_ifgroup != -1 && !dev.is_null() && (*dev).group == (*rule).suppress_ifgroup {
        suppress_route(result, arg, dev);
        return true;
    }

    false
}

fn suppress_route(result: *mut fib_result, arg: *mut fib_lookup_arg, dev: *mut c_void) {
    if !((*arg).flags & 1) != 0 {
        fib_info_put((*result).fi);
    }
}

#[no_mangle]
pub unsafe extern "C" fn fib4_rule_match(
    rule: *mut fib_rule,
    fl: *mut c_void,
    flags: c_int,
) -> bool {
    let r = rule as *mut fib4_rule;
    let fl4 = &(*fl.cast::<flowi>()).u.ip4;

    if ((((*fl4).saddr ^ (*r).src) & (*r).srcmask) != 0) ||
       ((((*fl4).daddr ^ (*r).dst) & (*r).dstmask) != 0) {
        return false;
    }

    if (*r).tos != 0 && (*r).tos != (*fl4).flowi4_tos {
        return false;
    }

    if (*rule).ip_proto != 0 && (*rule).ip_proto != (*fl4).flowi4_proto {
        return false;
    }

    if fib_rule_port_range_set(&(*rule).sport_range) &&
       !fib_rule_port_inrange(&(*rule).sport_range, (*fl4).fl4_sport) {
        return false;
    }

    if fib_rule_port_range_set(&(*rule).dport_range) &&
       !fib_rule_port_inrange(&(*rule).dport_range, (*fl4).fl4_dport) {
        return false;
    }

    true
}

// External functions (declared in other modules)
extern "C" {
    fn fib_rule_matchall(rule: *const fib_rule) -> bool;
    fn l3mdev_update_flow(net: *mut net_ipv4, fl: *mut flowi);
    fn fib_rules_lookup(ops: *mut fib_rules_ops, fl: *mut flowi, flags: c_int, arg: *mut fib_lookup_arg) -> c_int;
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn fib_rule_get_table(rule: *mut fib_rule, arg: *mut fib_lookup_arg) -> u32;
    fn fib_get_table(net: *mut net, id: u32) -> *mut fib_table;
    fn fib_table_lookup(
        tbl: *mut fib_table,
        flp: *mut flowi4,
        res: *mut fib_result,
        flags: c_uint,
    ) -> c_int;
    fn fib_info_nhc(fi: *mut c_void, index: c_int) -> *mut c_void;
    fn fib_info_put(fi: *mut c_void);
    fn fib_rule_port_range_set(range: *mut c_void) -> bool;
    fn fib_rule_port_inrange(range: *mut c_void, port: u16) -> bool;
    fn fib_unmerge(net: *mut net) -> c_int;
    fn fib_default_rule_add(ops: *mut fib_rules_ops, priority: u16, table: c_int, flags: c_int) -> c_int;
    fn fib_rules_unregister(ops: *mut fib_rules_ops);
    fn rt_cache_flush(net: *mut net);
}

// Module initialization
#[no_mangle]
pub unsafe extern "C" fn fib4_rules_init(net: *mut net) -> c_int {
    let mut ops: *mut fib_rules_ops = ptr::null_mut();
    let mut err = 0;

    ops = fib_rules_register(&fib4_rules_ops_template, net);
    if ops.is_null() {
        return -ENOMEM;
    }

    err = fib_default_rules_init(ops);
    if err < 0 {
        fib_rules_unregister(ops);
        return err;
    }

    (*net).ipv4.rules_ops = ops;
    (*net).ipv4.fib_has_custom_rules = false;
    (*net).ipv4.fib_rules_require_fldissect = 0;

    0
}

#[no_mangle]
pub unsafe extern "C" fn fib4_rules_exit(net: *mut net) {
    fib_rules_unregister((*net).ipv4.rules_ops);
}

// Static rules_ops template
#[repr(C)]
static fib4_rules_ops_template: fib_rules_ops = fib_rules_ops {
    family: 2, // AF_INET
    rule_size: mem::size_of::<fib4_rule>() as c_uint,
    addr_size: 4, // sizeof(u32)
    action: fib4_rule_action,
    suppress: fib4_rule_suppress,
    match_: fib4_rule_match,
    configure: fib4_rule_configure,
    delete: fib4_rule_delete,
    compare: fib4_rule_compare,
    fill: fib4_rule_fill,
    nlmsg_payload: fib4_rule_nlmsg_payload,
    flush_cache: fib4_rule_flush_cache,
    nlgroup: 5, // RTNLGRP_IPV4_RULE
    policy: ptr::null(),
    owner: ptr::null(),
};

// Helper functions
#[no_mangle]
pub unsafe extern "C" fn fib4_rules_seq_read(net: *mut net) -> c_uint {
    fib_rules_seq_read(net, 2) // AF_INET
}

#[no_mangle]
pub unsafe extern "C" fn fib4_rules_dump(
    net: *mut net,
    nb: *mut c_void,
    extack: *mut netlink_ext_ack,
) -> c_int {
    fib_rules_dump(net, nb, 2, extack) // AF_INET
}

#[no_mangle]
pub unsafe extern "C" fn fib4_rule_flush_cache(ops: *mut fib_rules_ops) {
    rt_cache_flush((*ops).fro_net);
}

#[no_mangle]
pub unsafe extern "C" fn fib4_rule_configure(
    rule: *mut fib_rule,
    skb: *mut c_void,
    frh: *mut fib_rule_hdr,
    tb: *mut *mut c_void,
    extack: *mut netlink_ext_ack,
) -> c_int {
    let net = sock_net((*skb).sk);
    let rule4 = rule as *mut fib4_rule;
    let mut err = -EINVAL;

    if (*frh).tos & !0x0F != 0 {
        NL_SET_ERR_MSG(extack, "Invalid tos\0".as_ptr() as *const c_char);
        return err;
    }

    err = fib_unmerge(net);
    if err < 0 {
        return err;
    }

    if (*rule).table == 0 && (*rule).l3mdev == 0 && (*rule).action == 0 {
        let table = fib_empty_table(net);
        if table.is_null() {
            return -ENOBUFS;
        }
        (*rule).table = (*table).tb_id;
    }

    if (*frh).src_len != 0 {
        (*rule4).src = nla_get_in_addr(*tb.offset(FRA_SRC as isize));
    }

    if (*frh).dst_len != 0 {
        (*rule4).dst = nla_get_in_addr(*tb.offset(FRA_DST as isize));
    }

    #[cfg(CONFIG_IP_ROUTE_CLASSID)]
    {
        if !(*tb.offset(FRA_FLOW as isize)).is_null() {
            (*rule4).tclassid = nla_get_u32(*tb.offset(FRA_FLOW as isize));
            if (*rule4).tclassid != 0 {
                (*net).ipv4.fib_num_tclassid_users += 1;
            }
        }
    }

    if fib_rule_requires_fldissect(rule) {
        (*net).ipv4.fib_rules_require_fldissect += 1;
    }

    (*rule4).src_len = (*frh).src_len;
    (*rule4).srcmask = inet_make_mask((*rule4).src_len);
    (*rule4).dst_len = (*frh).dst_len;
    (*rule4).dstmask = inet_make_mask((*rule4).dst_len);
    (*rule4).tos = (*frh).tos;

    (*net).ipv4.fib_has_custom_rules = true;

    0
}

#[no_mangle]
pub unsafe extern "C" fn fib4_rule_delete(rule: *mut fib_rule) -> c_int {
    let net = (*rule).fr_net;
    let mut err = 0;

    err = fib_unmerge(net);
    if err < 0 {
        return err;
    }

    #[cfg(CONFIG_IP_ROUTE_CLASSID)]
    {
        let rule4 = rule as *mut fib4_rule;
        if (*rule4).tclassid != 0 {
            (*net).ipv4.fib_num_tclassid_users -= 1;
        }
    }

    (*net).ipv4.fib_has_custom_rules = true;

    if (*net).ipv4.fib_rules_require_fldissect != 0 && fib_rule_requires_fldissect(rule) {
        (*net).ipv4.fib_rules_require_fldissect -= 1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn fib4_rule_compare(
    rule: *mut fib_rule,
    frh: *mut fib_rule_hdr,
    tb: *mut *mut c_void,
) -> c_int {
    let rule4 = rule as *mut fib4_rule;

    if (*frh).src_len != 0 && (*rule4).src_len != (*frh).src_len {
        return 0;
    }

    if (*frh).dst_len != 0 && (*rule4).dst_len != (*frh).dst_len {
        return 0;
    }

    if (*frh).tos != 0 && (*rule4).tos != (*frh).tos {
        return 0;
    }

    #[cfg(CONFIG_IP_ROUTE_CLASSID)]
    {
        if !(*tb.offset(FRA_FLOW as isize)).is_null() && (*rule4).tclassid != nla_get_u32(*tb.offset(FRA_FLOW as isize)) {
            return 0;
        }
    }

    if (*frh).src_len != 0 && (*rule4).src != nla_get_in_addr(*tb.offset(FRA_SRC as isize)) {
        return 0;
    }

    if (*frh).dst_len != 0 && (*rule4).dst != nla_get_in_addr(*tb.offset(FRA_DST as isize)) {
        return 0;
    }

    1
}

#[no_mangle]
pub unsafe extern "C" fn fib4_rule_fill(
    rule: *mut fib_rule,
    skb: *mut c_void,
    frh: *mut fib_rule_hdr,
) -> c_int {
    let rule4 = rule as *mut fib4_rule;

    (*frh).dst_len = (*rule4).dst_len;
    (*frh).src_len = (*rule4).src_len;
    (*frh).tos = (*rule4).tos;

    if (*rule4).dst_len != 0 && nla_put_in_addr(skb, FRA_DST, (*rule4).dst) != 0 {
        return -ENOBUFS;
    }

    if (*rule4).src_len != 0 && nla_put_in_addr(skb, FRA_SRC, (*rule4).src) != 0 {
        return -ENOBUFS;
    }

    #[cfg(CONFIG_IP_ROUTE_CLASSID)]
    {
        if (*rule4).tclassid != 0 && nla_put_u32(skb, FRA_FLOW, (*rule4).tclassid) != 0 {
            return -ENOBUFS;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn fib4_rule_nlmsg_payload(rule: *mut fib_rule) -> size_t {
    nla_total_size(4) /* dst */
        + nla_total_size(4) /* src */
        + nla_total_size(4) /* flow */
}

// External helper functions
extern "C" {
    fn sock_net(sk: *mut c_void) -> *mut net;
    fn fib_empty_table(net: *mut net) -> *mut fib_table;
    fn nla_get_in_addr(attr: *mut c_void) -> u32;
    fn nla_get_u32(attr: *mut c_void) -> u32;
    fn nla_put_in_addr(skb: *mut c_void, type_: c_int, data: u32) -> c_int;
    fn nla_total_size(len: size_t) -> size_t;
    fn fib_rule_requires_fldissect(rule: *mut fib_rule) -> bool;
    fn NL_SET_ERR_MSG(extack: *mut netlink_ext_ack, msg: *const c_char);
}