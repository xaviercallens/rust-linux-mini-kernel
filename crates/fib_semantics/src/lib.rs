#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(clippy::all)]

use core::ffi::{c_int, c_void};
use core::sync::atomic::{AtomicUsize, Ordering};
use kernel_types::*;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NetDevice(pub *mut c_void);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_prop {
    pub error: c_int,
    pub scope: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtable {
    pub dst: [u8; 1],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_head {
    pub next: *mut c_void,
    pub func: Option<unsafe extern "C" fn(*mut rcu_head)>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_nh_common {
    pub nhc_dev: *mut NetDevice,
    pub nhc_lwtstate: *mut c_void,
    pub nhc_pcpu_rth_output: *mut c_void,
    pub nhc_rth_input: *mut rtable,
    pub nhc_exceptions: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_nh {
    pub nh_common: fib_nh_common,
    pub fib_nh_oif: c_int,
    pub fib_nh_gw_family: c_int,
    pub fib_nh_scope: c_int,
    pub fib_nh_weight: c_int,
    pub nh_tclassid: c_int,
    pub fib_nh_lws: *mut c_void,
    pub fib_nh_flags: c_int,
    pub fib_nh_gw4: u32,
    pub fib_nh_gw6: [u8; 16],
}

#[repr(C)]
pub struct fib_info {
    pub fib_net: *mut c_void,
    pub fib_nhs: c_int,
    pub fib_protocol: c_int,
    pub fib_scope: c_int,
    pub fib_prefsrc: u32,
    pub fib_priority: u32,
    pub fib_type: c_int,
    pub fib_tb_id: u32,
    pub fib_flags: c_int,
    pub fib_metrics: [u32; 1],
    pub fib_treeref: AtomicUsize,
    pub fib_dead: c_int,
    pub fib_hash: *mut c_void,
    pub fib_lhash: *mut c_void,
    pub nh_list: *mut c_void,
    pub nh: *mut c_void,
    pub fib_nh: *mut fib_nh,
    pub rcu: rcu_head,
}

static fib_info_cnt: AtomicUsize = AtomicUsize::new(0);

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn fib_nh_common_release(nhc: *mut fib_nh_common) {
    if nhc.is_null() {
        return;
    }
    let _ = &*nhc;
}

#[no_mangle]
pub unsafe extern "C" fn free_fib_info(fi: *mut fib_info) {
    if fi.is_null() {
        return;
    }

    if (*fi).fib_dead == 0 {
        return;
    }

    let _ = fib_info_cnt.fetch_sub(1, Ordering::Relaxed);
    free_fib_info_rcu(&mut (*fi).rcu as *mut rcu_head);
}

unsafe fn free_fib_info_rcu(head: *mut rcu_head) {
    if head.is_null() {
        return;
    }

    let fi = head as *mut fib_info;

    if !(*fi).nh.is_null() {
    } else if !(*fi).fib_nh.is_null() && (*fi).fib_nhs > 0 {
        let nhs = (*fi).fib_nhs as isize;
        let base = (*fi).fib_nh;
        let mut i = 0isize;
        while i < nhs {
            let nhp = base.offset(i);
            let _oif = (*nhp).fib_nh_oif;
            let _ = _oif;
            i += 1;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn fib_release_info(fi: *mut fib_info) {
    if fi.is_null() {
        return;
    }

    if (*fi).fib_treeref.fetch_sub(1, Ordering::Relaxed) == 1 {
        if !(*fi).nh.is_null() {
        } else if !(*fi).fib_nh.is_null() && (*fi).fib_nhs > 0 {
            let nhs = (*fi).fib_nhs as isize;
            let base = (*fi).fib_nh;
            let mut i = 0isize;
            while i < nhs {
                let nhp = base.offset(i);
                let _oif = (*nhp).fib_nh_oif;
                let _proto = (*fi).fib_protocol;
                let _scope = (*fi).fib_scope;
                let _flags = (*nhp).fib_nh_flags;
                let _ = (_oif, _proto, _scope, _flags);
                i += 1;
            }
        }
        (*fi).fib_dead = 1;
        free_fib_info(fi);
    }
}