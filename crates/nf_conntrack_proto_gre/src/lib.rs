#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_int, c_uint, c_void};
use core::ptr;
use kernel_types::*;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;
pub const EEXIST: c_int = -17;

// Constants
pub const GRE_CT_UNREPLIED: usize = 0;
pub const GRE_CT_REPLIED: usize = 1;
pub const GRE_CT_MAX: usize = 2;
pub const IP_CT_DIR_ORIGINAL: usize = 0;
pub const IP_CT_DIR_REPLY: usize = 1;
pub const IP_CT_DIR_MAX: usize = 2;
pub const IPPROTO_GRE: u8 = 47;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_inet_addr {
    pub all: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_ipv4 {
    pub u3: nf_inet_addr,
    pub protonum: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple {
    pub src: nf_conntrack_tuple_ipv4,
    pub dst: nf_conntrack_tuple_ipv4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_gre {
    pub key: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_head {
    pub next: *mut rcu_head,
    pub func: Option<unsafe extern "C" fn(*mut rcu_head)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_gre_keymap {
    pub tuple: nf_conntrack_tuple,
    pub list: list_head,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_gre_net {
    pub keymap_list: list_head,
    pub timeouts: [c_uint; GRE_CT_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn_gre {
    pub timeout: c_uint,
    pub stream_timeout: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn_proto {
    pub gre: nf_conn_gre,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conn {
    pub status: u32,
    pub proto: nf_conn_proto,
    pub _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_pptp_master {
    pub keymap: [*mut nf_ct_gre_keymap; IP_CT_DIR_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_l4proto {
    pub l4proto: u8,
}

#[repr(C)]
pub struct sk_buff {
    _priv: [u8; 0],
}

unsafe extern "C" {
    fn nf_ct_net(ct: *const nf_conn) -> *mut c_void;
    fn nfct_help_data(ct: *const nf_conn) -> *mut nf_ct_pptp_master;
    fn nf_ct_timeout_lookup(ct: *const nf_conn) -> *const c_uint;
    fn nf_ct_refresh_acct(ct: *mut nf_conn, ctinfo: c_int, skb: *mut sk_buff, timeout: c_uint);
    fn nf_conntrack_event_cache(event: c_int, ct: *mut nf_conn);
    fn skb_header_pointer(
        skb: *const sk_buff,
        dataoff: c_int,
        size: usize,
        ptr: *mut c_void,
    ) -> *mut c_void;
    fn nf_ct_dump_tuple(tuple: *const nf_conntrack_tuple);

    fn gre_pernet(net: *mut c_void) -> *mut nf_gre_net;
    fn kmalloc(size: usize) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
}

static mut KEYMAP_LOCK: c_int = 0;

fn spin_lock_bh(lock: *mut c_int) {
    unsafe { *lock = 1 }
}

fn spin_unlock_bh(lock: *mut c_int) {
    unsafe { *lock = 0 }
}

fn gre_key_cmpfn(a: *const nf_ct_gre_keymap, t: *const nf_conntrack_tuple) -> bool {
    unsafe { ptr::eq(&(*a).tuple, t) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nf_ct_gre_keymap_add(
    ct: *mut nf_conn,
    dir: c_int,
    t: *mut nf_conntrack_tuple,
) -> c_int {
    if ct.is_null() || t.is_null() {
        return EINVAL;
    }

    let dir_idx = dir as usize;
    if dir_idx >= IP_CT_DIR_MAX {
        return EINVAL;
    }

    let net = unsafe { nf_ct_net(ct) };
    if net.is_null() {
        return EINVAL;
    }

    let net_gre = unsafe { gre_pernet(net) };
    if net_gre.is_null() {
        return EINVAL;
    }

    let ct_pptp_info = unsafe { nfct_help_data(ct) };
    if ct_pptp_info.is_null() {
        return EINVAL;
    }

    let kmp = unsafe { &mut (*ct_pptp_info).keymap[dir_idx] };
    if !(*kmp).is_null() {
        let km = *kmp;
        if gre_key_cmpfn(km as *const nf_ct_gre_keymap, t as *const nf_conntrack_tuple) {
            return 0;
        }
        return EEXIST;
    }

    let km = unsafe { kmalloc(core::mem::size_of::<nf_ct_gre_keymap>()) as *mut nf_ct_gre_keymap };
    if km.is_null() {
        return ENOMEM;
    }

    unsafe {
        ptr::copy_nonoverlapping(t as *const nf_conntrack_tuple, &mut (*km).tuple, 1);
        (*km).list.next = ptr::null_mut();
        (*km).list.prev = ptr::null_mut();
        (*km).rcu.next = ptr::null_mut();
        (*km).rcu.func = None;

        spin_lock_bh(core::ptr::addr_of_mut!(KEYMAP_LOCK));
        *kmp = km;
        spin_unlock_bh(core::ptr::addr_of_mut!(KEYMAP_LOCK));
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn nf_ct_gre_keymap_destroy(ct: *mut nf_conn) {
    if ct.is_null() {
        return;
    }

    let ct_pptp_info = unsafe { nfct_help_data(ct) };
    if ct_pptp_info.is_null() {
        return;
    }

    for i in 0..IP_CT_DIR_MAX {
        let km = unsafe { (*ct_pptp_info).keymap[i] };
        if !km.is_null() {
            unsafe {
                spin_lock_bh(core::ptr::addr_of_mut!(KEYMAP_LOCK));
                (*ct_pptp_info).keymap[i] = ptr::null_mut();
                spin_unlock_bh(core::ptr::addr_of_mut!(KEYMAP_LOCK));
                kfree(km as *mut c_void);
            }
        }
    }
}