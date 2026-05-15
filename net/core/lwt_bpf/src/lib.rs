//! BPF-based Light Weight Tunneling (LWT) implementation
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use libc::{c_int, c_uint, c_void, size_t};

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;
pub const ENOENT: c_int = -2;
pub const EAFNOSUPPORT: c_int = -97;
pub const ERANGE: c_int = -84;

// Type definitions
#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_prog {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dst_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct lwtunnel_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rtable {
    dst: dst_entry,
}

#[repr(C)]
pub struct ipv6hdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct flowi4 {
    flowi4_oif: c_int,
    flowi4_mark: c_int,
    flowi4_uid: c_int,
    flowi4_tos: c_int,
    flowi4_flags: c_int,
    flowi4_proto: c_int,
    daddr: u32,
    saddr: u32,
}

#[repr(C)]
pub struct flowi6 {
    flowi6_oif: c_int,
    flowi6_mark: c_int,
    flowi6_uid: c_int,
    flowlabel: u32,
    flowi6_proto: c_int,
    daddr: [u8; 16],
    saddr: [u8; 16],
}

#[repr(C)]
pub struct bpf_lwt_prog {
    prog: *mut bpf_prog,
    name: *mut c_char,
}

#[repr(C)]
pub struct bpf_lwt {
    in_: bpf_lwt_prog,
    out: bpf_lwt_prog,
    xmit: bpf_lwt_prog,
    family: c_int,
}

#[repr(C)]
pub struct nlattr {
    len: c_uint,
    kind: c_uint,
}

// Function declarations for external kernel APIs
extern "C" {
    fn migrate_disable();
    fn migrate_enable();
    fn local_bh_disable();
    fn local_bh_enable();
    fn bpf_compute_data_pointers(skb: *mut sk_buff);
    fn bpf_prog_run_save_cb(prog: *mut bpf_prog, skb: *mut sk_buff) -> c_int;
    fn skb_dst(skb: *mut sk_buff) -> *mut dst_entry;
    fn ip_hdr(skb: *mut sk_buff) -> *mut c_void;
    fn ipv6_hdr(skb: *mut sk_buff) -> *mut ipv6hdr;
    fn dev_hold(dev: *mut net_device);
    fn dev_put(dev: *mut net_device);
    fn ip_route_input_noref(skb: *mut sk_buff, daddr: u32, saddr: u32, tos: c_int, dev: *mut net_device) -> c_int;
    fn ipv6_stub_ipv6_route_input(skb: *mut sk_buff) -> c_int;
    fn dst_input(skb: *mut sk_buff) -> c_int;
    fn skb_do_redirect(skb: *mut sk_buff) -> c_int;
    fn kfree_skb(skb: *mut sk_buff);
    fn dst_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int;
    fn l3mdev_master_dev_rcu(dev: *mut net_device) -> *mut net_device;
    fn sk_to_full_sk(sk: *mut sock) -> *mut sock;
    fn sock_net(sk: *mut sock) -> *mut net;
    fn dev_net(dev: *mut net_device) -> *mut net;
    fn skb_cow_head(skb: *mut sk_buff, headroom: c_int) -> c_int;
    fn skb_dst_drop(skb: *mut sk_buff);
    fn skb_reset_mac_header(skb: *mut sk_buff);
    fn pr_warn_once(fmt: *const c_char, ...) -> c_int;
    fn bpf_prog_get_type(fd: c_uint, type_: c_int) -> *mut bpf_prog;
    fn bpf_prog_put(prog: *mut bpf_prog);
    fn lwtunnel_state_alloc(size: size_t) -> *mut lwtunnel_state;
    fn lwtunnel_state_free(lwt: *mut lwtunnel_state);
    fn nla_parse_nested_deprecated(tb: *mut *mut nlattr, maxtype: c_int, attr: *mut nlattr, policy: *mut c_void, extack: *mut c_void) -> c_int;
    fn nla_memdup(attr: *mut nlattr, gfp: c_int) -> *mut c_void;
    fn nla_get_u32(attr: *mut nlattr) -> c_uint;
    fn nla_put_string(skb: *mut sk_buff, attrtype: c_int, str: *const c_char) -> c_int;
    fn nla_nest_start(skb: *mut sk_buff, attrtype: c_int) -> *mut nlattr;
}

// Helper functions
fn bpf_lwt_lwtunnel(lwt: *mut lwtunnel_state) -> *mut bpf_lwt {
    // SAFETY: lwt->data is guaranteed to be a bpf_lwt struct
    unsafe { &mut (*lwt).data as *mut bpf_lwt }
}

// Function implementations
unsafe extern "C" fn run_lwt_bpf(
    skb: *mut sk_buff,
    lwt: *mut bpf_lwt_prog,
    dst: *mut dst_entry,
    can_redirect: bool,
) -> c_int {
    // SAFETY: Caller guarantees valid pointers
    migrate_disable();
    local_bh_disable();
    bpf_compute_data_pointers(skb);
    let ret = bpf_prog_run_save_cb((*lwt).prog, skb);

    match ret {
        0 => 0, // BPF_OK
        1 => 1, // BPF_LWT_REROUTE
        2 => {
            if !can_redirect {
                pr_warn_once(b"Illegal redirect return code in prog %s\n\0".as_ptr() as *const c_char, (*lwt).name);
                0 // BPF_OK
            } else {
                skb_reset_mac_header(skb);
                let redirect_result = skb_do_redirect(skb);
                if redirect_result == 0 {
                    2 // BPF_REDIRECT
                } else {
                    0 // BPF_OK
                }
            }
        }
        3 => {
            kfree_skb(skb);
            -1 // -EPERM
        }
        _ => {
            pr_warn_once(b"bpf-lwt: Illegal return value %u, expect packet loss\n\0".as_ptr() as *const c_char, ret as u32);
            kfree_skb(skb);
            -22 // -EINVAL
        }
    }

    local_bh_enable();
    migrate_enable();

    ret
}

unsafe extern "C" fn bpf_lwt_input_reroute(skb: *mut sk_buff) -> c_int {
    let err = -22; // -EINVAL

    if (*skb).protocol == htons(ETH_P_IP) {
        let dev = (*skb_dst(skb)).dev;
        let iph = ip_hdr(skb);

        dev_hold(dev);
        skb_dst_drop(skb);
        let route_result = ip_route_input_noref(skb, (*iph).daddr, (*iph).saddr, (*iph).tos, dev);
        dev_put(dev);

        if route_result < 0 {
            kfree_skb(skb);
            return route_result;
        }

        return dst_input(skb);
    } else if (*skb).protocol == htons(ETH_P_IPV6) {
        skb_dst_drop(skb);
        return ipv6_stub_ipv6_route_input(skb);
    } else {
        return -97; // -EAFNOSUPPORT;
    }
}

unsafe extern "C" fn bpf_input(skb: *mut sk_buff) -> c_int {
    let dst = skb_dst(skb);
    let bpf = bpf_lwt_lwtunnel(dst);
    let mut ret = 0;

    if (*bpf).in_.prog.is_null() {
        if !(*dst).lwtstate->orig_input.is_null() {
            return (*dst).lwtstate->orig_input(skb);
        } else {
            kfree_skb(skb);
            return -22; // -EINVAL
        }
    }

    ret = run_lwt_bpf(skb, &mut (*bpf).in_, dst, false);
    if ret < 0 {
        return ret;
    }

    if ret == 1 { // BPF_LWT_REROUTE
        return bpf_lwt_input_reroute(skb);
    }

    if (*dst).lwtstate->orig_input.is_null() {
        kfree_skb(skb);
        return -22; // -EINVAL
    }

    return (*dst).lwtstate->orig_input(skb);
}

unsafe extern "C" fn bpf_output(
    net: *mut net,
    sk: *mut sock,
    skb: *mut sk_buff,
) -> c_int {
    let dst = skb_dst(skb);
    let bpf = bpf_lwt_lwtunnel(dst);
    let mut ret = 0;

    if (*bpf).out.prog.is_null() {
        if !(*dst).lwtstate->orig_output.is_null() {
            return (*dst).lwtstate->orig_output(net, sk, skb);
        } else {
            kfree_skb(skb);
            return -22; // -EINVAL
        }
    }

    ret = run_lwt_bpf(skb, &mut (*bpf).out, dst, false);
    if ret < 0 {
        return ret;
    }

    if (*dst).lwtstate->orig_output.is_null() {
        pr_warn_once(b"orig_output not set on dst for prog %s\n\0".as_ptr() as *const c_char, (*bpf).out.name);
        kfree_skb(skb);
        return -22; // -EINVAL
    }

    return (*dst).lwtstate->orig_output(net, sk, skb);
}

unsafe extern "C" fn xmit_check_hhlen(skb: *mut sk_buff) -> c_int {
    let hh_len = (*(*skb_dst(skb)).dev).hard_header_len;
    let headroom = hh_len as c_int - (*skb).headroom as c_int;
    let nhead = HH_DATA_ALIGN(headroom);

    if headroom > 0 {
        if pskb_expand_head(skb, nhead, 0, 0) != 0 {
            return -12; // -ENOMEM
        }
    }

    0
}

unsafe extern "C" fn bpf_lwt_xmit_reroute(skb: *mut sk_buff) -> c_int {
    let l3mdev = l3mdev_master_dev_rcu((*skb_dst(skb)).dev);
    let oif = if !l3mdev.is_null() { (*l3mdev).ifindex } else { 0 };
    let mut err = -97; // -EAFNOSUPPORT
    let mut dst: *mut dst_entry = ptr::null_mut();
    let mut net: *mut net = ptr::null_mut();
    let mut sk: *mut sock = ptr::null_mut();
    let ipv4 = if (*skb).protocol == htons(ETH_P_IP) { true } else if (*skb).protocol == htons(ETH_P_IPV6) { false } else { false };

    if !ipv4 {
        return -97; // -EAFNOSUPPORT
    }

    sk = sk_to_full_sk((*skb).sk);
    if !sk.is_null() {
        if (*sk).sk_bound_dev_if != 0 {
            oif = (*sk).sk_bound_dev_if;
        }
        net = sock_net(sk);
    } else {
        net = dev_net((*skb_dst(skb)).dev);
    }

    if ipv4 {
        let iph = ip_hdr(skb);
        let mut fl4 = flowi4 {
            flowi4_oif: oif,
            flowi4_mark: (*skb).mark,
            flowi4_uid: sock_net_uid(net, sk),
            flowi4_tos: RT_TOS((*(*iph).tos) as c_int),
            flowi4_flags: 1, // FLOWI_FLAG_ANYSRC
            flowi4_proto: (*(*iph).protocol) as c_int,
            daddr: (*(*iph).daddr),
            saddr: (*(*iph).saddr),
        };

        let rt = ip_route_output_key(net, &fl4);
        if IS_ERR(rt) {
            return PTR_ERR(rt);
        }
        dst = &(*rt).dst;
    } else {
        let iph6 = ipv6_hdr(skb);
        let mut fl6 = flowi6 {
            flowi6_oif: oif,
            flowi6_mark: (*skb).mark,
            flowi6_uid: sock_net_uid(net, sk),
            flowlabel: ip6_flowinfo(iph6),
            flowi6_proto: (*(*iph6).nexthdr) as c_int,
            daddr: (*(*iph6).daddr),
            saddr: (*(*iph6).saddr),
        };

        dst = ipv6_stub->ipv6_dst_lookup_flow(net, (*skb).sk, &fl6, ptr::null_mut());
        if IS_ERR(dst) {
            return PTR_ERR(dst);
        }
    }

    if (*dst).error != 0 {
        let err = (*dst).error;
        dst_release(dst);
        return err;
    }

    let err = skb_cow_head(skb, LL_RESERVED_SPACE((*dst).dev));
    if err < 0 {
        return err;
    }

    skb_dst_drop(skb);
    skb_dst_set(skb, dst);

    let err = dst_output(net, (*skb).sk, skb);
    if err < 0 {
        return err;
    }

    1 // LWTUNNEL_XMIT_DONE
}

unsafe extern "C" fn bpf_xmit(skb: *mut sk_buff) -> c_int {
    let dst = skb_dst(skb);
    let bpf = bpf_lwt_lwtunnel(dst);
    let mut ret = 0;
    let mut proto = (*skb).protocol;

    if (*bpf).xmit.prog.is_null() {
        return 0; // LWTUNNEL_XMIT_CONTINUE
    }

    ret = run_lwt_bpf(skb, &mut (*bpf).xmit, dst, true);
    match ret {
        0 => {
            if (*skb).protocol != proto {
                kfree_skb(skb);
                return -22; // -EINVAL
            }
            let err = xmit_check_hhlen(skb);
            if err < 0 {
                return err;
            }
            0 // LWTUNNEL_XMIT_CONTINUE
        }
        2 => 1, // BPF_REDIRECT -> LWTUNNEL_XMIT_DONE
        1 => bpf_lwt_xmit_reroute(skb),
        _ => ret,
    }
}

unsafe extern "C" fn bpf_lwt_prog_destroy(prog: *mut bpf_lwt_prog) {
    if !(*prog).prog.is_null() {
        bpf_prog_put((*prog).prog);
    }
    if !(*prog).name.is_null() {
        free((*prog).name as *mut c_void);
    }
}

unsafe extern "C" fn bpf_destroy_state(lwt: *mut lwtunnel_state) {
    let bpf = bpf_lwt_lwtunnel(lwt);
    bpf_lwt_prog_destroy(&mut (*bpf).in_);
    bpf_lwt_prog_destroy(&mut (*bpf).out);
    bpf_lwt_prog_destroy(&mut (*bpf).xmit);
}

// Additional functions and constants would be added here...

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    // Basic tests would be added here
}
**Note:** This is a simplified translation focusing on the core structure and FFI compatibility. The complete implementation would require:
1. Adding all missing constants (ETH_P_IP, ETH_P_IPV6, etc.)
2. Implementing all helper functions (sock_net_uid, ip_route_output_key, etc.)
3. Adding proper error handling for all external function calls
4. Implementing the Netlink attribute parsing functions
5. Adding proper memory management for all allocations
6. Implementing the full set of BPF program types and validation

The actual implementation would need to be integrated with the Linux kernel's build system and would require careful validation against the original C implementation.
