#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_int, c_uint, c_void};
use core::mem;
use core::ptr;
use kernel_types::*;

const ECACHE_RETRY_WAIT: u32 = 1;
const ECACHE_STACK_ALLOC: usize = 256 / mem::size_of::<*mut c_void>();

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const EBUSY: c_int = -16;

const NFCT_ECACHE_DESTROY_FAIL: u32 = 1;
const NFCT_ECACHE_DESTROY_SENT: u32 = 2;
const IPCT_DESTROY: u32 = 4;

#[repr(u8)]
enum retry_state {
    STATE_CONGESTED = 0,
    STATE_RESTART = 1,
    STATE_DONE = 2,
}

#[repr(C)]
struct nf_conntrack_tuple_hash {
    _unused: [u8; 0],
}

#[repr(C)]
struct hlist_nulls_node {
    next: *mut hlist_nulls_node,
}

#[repr(C)]
struct nf_conn {
    _unused: [u8; 0],
}

#[repr(C)]
struct nf_ct_event {
    ct: *mut nf_conn,
    portid: u32,
    report: c_int,
}

#[repr(C)]
struct nf_ct_event_notifier {
    fcn: extern "C" fn(c_uint, *mut nf_ct_event),
}

#[repr(C)]
struct nf_conntrack_ecache {
    state: c_uint,
    portid: u32,
    ctmask: u16,
    missed: u16,
    _pad: [u8; 2],
}

#[repr(C)]
struct ct_pcpu {
    lock: *mut c_void,
    dying: *mut hlist_nulls_node,
}

#[repr(C)]
struct delayed_work {
    _unused: [u8; 0],
}

#[repr(C)]
struct netns_ct {
    ecache_dwork_pending: u8,
    _pad: [u8; 7],
    pcpu: *mut ct_pcpu,
    pcpu_count: c_uint,
}

#[repr(C)]
struct nf_conntrack_net {
    ecache_dwork: delayed_work,
    ct_net: *mut netns_ct,
}

unsafe extern "C" {
    fn nf_ct_tuplehash_to_ctrack(h: *mut nf_conntrack_tuple_hash) -> *mut nf_conn;
    fn nf_ct_ecache_find(ct: *mut nf_conn) -> *mut nf_conntrack_ecache;
    fn nf_conntrack_event(event: c_uint, ct: *mut nf_conn) -> c_int;
    fn nf_ct_put(ct: *mut nf_conn);
    fn nf_ct_is_confirmed(ct: *mut nf_conn) -> c_int;
    fn local_bh_disable();
    fn local_bh_enable();
    fn schedule_delayed_work(work: *mut delayed_work, delay: u32);
}

fn ecache_work_evict_list(pcpu: *mut ct_pcpu) -> retry_state {
    let mut refs: [*mut nf_conn; ECACHE_STACK_ALLOC] = [ptr::null_mut(); ECACHE_STACK_ALLOC];
    let mut evicted: usize = 0;
    let mut ret = retry_state::STATE_DONE;

    unsafe {
        let mut n = (*pcpu).dying;

        while !n.is_null() {
            let h = n as *mut nf_conntrack_tuple_hash;
            let ct = nf_ct_tuplehash_to_ctrack(h);

            if nf_ct_is_confirmed(ct) == 0 {
                n = (*n).next;
                continue;
            }

            let e = nf_ct_ecache_find(ct);
            if e.is_null() || (*e).state != NFCT_ECACHE_DESTROY_FAIL as c_uint {
                n = (*n).next;
                continue;
            }

            if nf_conntrack_event(IPCT_DESTROY as c_uint, ct) != 0 {
                ret = retry_state::STATE_CONGESTED;
                break;
            }

            (*e).state = NFCT_ECACHE_DESTROY_SENT as c_uint;
            refs[evicted] = ct;
            evicted += 1;

            if evicted >= ECACHE_STACK_ALLOC {
                ret = retry_state::STATE_RESTART;
                break;
            }

            n = (*n).next;
        }
    }

    while evicted > 0 {
        unsafe {
            evicted -= 1;
            nf_ct_put(refs[evicted]);
        }
    }

    ret
}

#[unsafe(no_mangle)]
extern "C" fn ecache_work(work: *mut delayed_work) {
    unsafe {
        let cnet =
            (work as *mut u8).sub(mem::offset_of!(nf_conntrack_net, ecache_dwork)) as *mut nf_conntrack_net;
        let ctnet = (*cnet).ct_net;
        if ctnet.is_null() {
            return;
        }

        local_bh_disable();

        let mut delay: c_int = -1;
        let mut cpu: c_uint = 0;
        while cpu < (*ctnet).pcpu_count {
            let pcpu = (*ctnet).pcpu.add(cpu as usize);
            match ecache_work_evict_list(pcpu) {
                retry_state::STATE_CONGESTED => {
                    delay = ECACHE_RETRY_WAIT as c_int;
                    break;
                }
                retry_state::STATE_RESTART => {
                    delay = 0;
                }
                retry_state::STATE_DONE => {}
            }
            cpu += 1;
        }

        local_bh_enable();

        if delay >= 0 {
            schedule_delayed_work(&mut (*cnet).ecache_dwork, delay as u32);
        }
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}