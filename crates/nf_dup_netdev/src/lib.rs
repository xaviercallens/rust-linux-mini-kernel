```rust
#![no_std]
#![no_main]
#![allow(non_camel_case_types)]

use core::panic::PanicInfo;

mod kernel_types {
    pub use core::ffi::{c_char, c_int, c_uint, c_void};
    pub type size_t = usize;
    pub type c_size_t = usize;
    pub type socklen_t = u32;
}

use kernel_types::*;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const EOPNOTSUPP: c_int = -95;
pub const GFP_ATOMIC: c_int = 0x20;

#[repr(C)]
pub struct sk_buff {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct net_device {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct nft_pktinfo {
    pub skb: *mut sk_buff,
    pub net: *mut c_void,
}

#[repr(C)]
pub struct nft_offload_ctx {
    pub net: *mut c_void,
    pub num_actions: c_int,
}

#[repr(C)]
pub struct nft_flow_rule {
    pub rule: *mut c_void,
}

#[repr(C)]
pub struct flow_action_entry {
    pub id: c_int,
    pub dev: *mut net_device,
}

unsafe extern "C" {
    fn dev_get_by_index_rcu(net: *mut c_void, ifindex: c_int) -> *mut net_device;
    fn dev_get_by_index(net: *mut c_void, ifindex: c_int) -> *mut net_device;
    fn kfree_skb(skb: *mut sk_buff);
    fn skb_clone(skb: *mut sk_buff, gfp_mask: c_int) -> *mut sk_buff;
    fn nf_do_netdev_egress(skb: *mut sk_buff, dev: *mut net_device);
    fn nft_flow_rule_action_entry(rule: *mut c_void, index: c_int) -> *mut flow_action_entry;
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nf_fwd_netdev_egress(pkt: *const nft_pktinfo, oif: c_int) {
    if pkt.is_null() {
        return;
    }

    let net = unsafe { (*pkt).net };
    let skb = unsafe { (*pkt).skb };
    let dev = unsafe { dev_get_by_index_rcu(net, oif) };
    if dev.is_null() {
        unsafe { kfree_skb(skb) };
        return;
    }

    unsafe { nf_do_netdev_egress(skb, dev) };
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nf_dup_netdev_egress(pkt: *const nft_pktinfo, oif: c_int) {
    if pkt.is_null() {
        return;
    }

    let net = unsafe { (*pkt).net };
    let orig_skb = unsafe { (*pkt).skb };
    let dev = unsafe { dev_get_by_index_rcu(net, oif) };
    if dev.is_null() {
        return;
    }

    let skb = unsafe { skb_clone(orig_skb, GFP_ATOMIC) };
    if !skb.is_null() {
        unsafe { nf_do_netdev_egress(skb, dev) };
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nft_fwd_dup_netdev_offload(
    ctx: *mut nft_offload_ctx,
    flow: *mut nft_flow_rule,
    id: c_int,
    oif: c_int,
) -> c_int {
    if ctx.is_null() || flow.is_null() {
        return EINVAL;
    }

    let net = unsafe { (*ctx).net };
    let dev = unsafe { dev_get_by_index(net, oif) };
    if dev.is_null() {
        return EOPNOTSUPP;
    }

    let rule = unsafe { (*flow).rule };
    let index = unsafe { (*ctx).num_actions };
    let entry = unsafe { nft_flow_rule_action_entry(rule, index) };
    if entry.is_null() {
        return ENOMEM;
    }

    unsafe {
        (*entry).id = id;
        (*entry).dev = dev;
        (*ctx).num_actions += 1;
    }

    0
}
```