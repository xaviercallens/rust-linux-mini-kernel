#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]

use core::ffi::{c_int, c_uint, c_void};
use core::ptr;
use kernel_types::*;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const E2BIG: c_int = -75;
pub const INT_MIN: c_int = -2147483648;
pub const MAX_HOOK_COUNT: c_int = 1024;

pub const NFPROTO_NUMPROTO: usize = 32;
pub const NF_MAX_HOOKS: usize = 32;
pub const NF_INET_INGRESS: c_int = 0;
pub const NF_NETDEV_INGRESS: c_int = 1;
pub const NFPROTO_NETDEV: c_int = 5;
pub const NFPROTO_ARP: c_int = 3;
pub const NFPROTO_BRIDGE: c_int = 4;
pub const NFPROTO_IPV4: c_int = 2;
pub const NFPROTO_IPV6: c_int = 10;
pub const NFPROTO_INET: c_int = 14;

#[repr(C)]
pub struct nf_hook_state {
    _private: [u8; 0],
}

pub type HookFn =
    extern "C" fn(priv_data: *mut c_void, skb: *mut c_void, state: *const nf_hook_state) -> c_uint;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_hook_ops {
    pub hook: HookFn,
    pub priority: c_int,
    pub priv_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_hook_entry {
    pub hook: HookFn,
    pub priv_data: *mut c_void,
}

#[repr(C)]
pub struct nf_hook_entries_rcu_head {
    pub allocation: *mut c_void,
    pub head: *mut c_void,
}

#[repr(C)]
pub struct nf_hook_entries {
    pub num_hook_entries: c_uint,
    pub hooks: [nf_hook_entry; 0],
}

#[repr(C)]
pub struct nf_net {
    pub hooks_arp: *mut nf_hook_entries,
    pub hooks_bridge: *mut nf_hook_entries,
    pub hooks_ipv4: *mut nf_hook_entries,
    pub hooks_ipv6: *mut nf_hook_entries,
    pub hooks_decnet: *mut nf_hook_entries,
}

#[repr(C)]
pub struct net {
    pub nf: nf_net,
}

#[repr(C)]
pub struct net_device {
    pub nf_hooks_ingress: *mut nf_hook_entries,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct static_key {
    _private: [u8; 0],
}

unsafe impl Sync for static_key {}
unsafe impl Sync for mutex {}

#[no_mangle]
pub static mut nf_ipv6_ops: *mut c_void = ptr::null_mut();

#[no_mangle]
pub static mut nf_skb_duplicated: [u8; 0] = [];

#[no_mangle]
pub static mut nf_hooks_needed: [[static_key; NF_MAX_HOOKS]; NFPROTO_NUMPROTO] =
    [[static_key { _private: [] }; NF_MAX_HOOKS]; NFPROTO_NUMPROTO];

#[no_mangle]
pub static mut nf_hook_mutex: mutex = mutex { _private: [] };

#[repr(C)]
pub struct hook_ops_ptr {
    pub ptr: *const nf_hook_ops,
}

unsafe impl Sync for hook_ops_ptr {}

#[no_mangle]
pub static dummy_ops: nf_hook_ops = nf_hook_ops {
    hook: dummy_hook,
    priority: 0,
    priv_data: ptr::null_mut(),
};

unsafe impl Sync for nf_hook_ops {}

extern "C" fn dummy_hook(
    _priv_data: *mut c_void,
    _skb: *mut c_void,
    _state: *const nf_hook_state,
) -> c_uint {
    0
}

#[inline]
unsafe fn rcu_dereference_raw(pp: *mut *mut nf_hook_entries) -> *mut nf_hook_entries {
    if pp.is_null() {
        ptr::null_mut()
    } else {
        *pp
    }
}

#[inline]
unsafe fn rcu_assign_pointer(pp: *mut *mut nf_hook_entries, p: *mut nf_hook_entries) {
    if !pp.is_null() {
        *pp = p;
    }
}

#[inline]
fn hooks_validate(_p: *mut nf_hook_entries) {}

#[inline]
fn nf_hook_entries_free(_p: *mut nf_hook_entries) {}

#[inline]
fn nf_hook_entries_get_hook_ops(_old: *mut nf_hook_entries) -> &'static [*const nf_hook_ops] {
    &[]
}

fn allocate_hook_entries_size(num: c_uint) -> *mut nf_hook_entries {
    if num == 0 {
        return ptr::null_mut();
    }
    ptr::null_mut()
}

fn nf_hook_entries_grow(old: *mut nf_hook_entries, _reg: *const nf_hook_ops) -> *mut nf_hook_entries {
    let mut alloc_entries: c_int = 1;
    let old_entries: c_uint = if old.is_null() {
        0
    } else {
        unsafe { (*old).num_hook_entries }
    };

    if !old.is_null() {
        let orig_ops = nf_hook_entries_get_hook_ops(old);
        let mut i: usize = 0;
        while i < old_entries as usize {
            let p = if i < orig_ops.len() { orig_ops[i] } else { ptr::null() };
            if !p.is_null() && ptr::eq(p, &dummy_ops as *const nf_hook_ops) {
                alloc_entries += 1;
            }
            i += 1;
        }
    }

    if alloc_entries > MAX_HOOK_COUNT {
        return ptr::null_mut();
    }

    allocate_hook_entries_size(alloc_entries as c_uint)
}

#[no_mangle]
pub unsafe extern "C" fn nf_hook_entries_insert_raw(
    pp: *mut *mut nf_hook_entries,
    reg: *const nf_hook_ops,
) -> c_int {
    let p = rcu_dereference_raw(pp);
    let new_hooks = nf_hook_entries_grow(p, reg);

    if new_hooks.is_null() {
        return ENOMEM;
    }

    if core::ptr::eq(new_hooks, p) {
        return 0;
    }

    hooks_validate(new_hooks);
    rcu_assign_pointer(pp, new_hooks);
    nf_hook_entries_free(p);
    0
}

#[no_mangle]
pub unsafe extern "C" fn nf_unregister_net_hook(
    _net: *mut net,
    _pf: c_int,
    _reg: *const nf_hook_ops,
) -> c_int {
    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}