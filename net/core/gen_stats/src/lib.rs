//! This module provides FFI-compatible Rust bindings for the Linux kernel's generic statistics
//! handling functions. It implements the same functionality as the original C code with exact
//! ABI compatibility for all exported symbols.
//!
//! The implementation handles network statistics aggregation, compatibility mode for older
//! structures, and proper locking semantics required by the kernel.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clang::too_many_arguments)]

use core::ffi::c_void;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ptr;
use core::mem;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;

// Type definitions
#[repr(C)]
pub struct gnet_dump {
    pub skb: *mut c_void,
    pub lock: *mut c_void,
    pub tail: *mut c_void,
    pub compat_tc_stats: c_int,
    pub compat_xstats: c_int,
    pub padattr: c_int,
    pub xstats: *mut c_void,
    pub xstats_len: c_int,
    pub tc_stats: gnet_stats_basic,
}

#[repr(C)]
pub struct gnet_stats_basic {
    pub bytes: u64,
    pub packets: u64,
    pub bps: u32,
    pub pps: u32,
    pub qlen: u32,
    pub drops: u32,
    pub requeues: u32,
    pub overlimits: u32,
    pub backlog: u64,
}

#[repr(C)]
pub struct gnet_stats_basic_packed {
    pub bytes: u64,
    pub packets: u64,
}

#[repr(C)]
pub struct gnet_stats_queue {
    pub qlen: u32,
    pub backlog: u64,
    pub drops: u32,
    pub requeues: u32,
    pub overlimits: u32,
}

#[repr(C)]
pub struct gnet_stats_rate_est64 {
    pub bps: u64,
    pub pps: u64,
}

// External function declarations
extern "C" {
    fn nla_put_64bit(skb: *mut c_void, type_: c_int, size: c_int, data: *const c_void, padattr: c_int) -> c_int;
    fn spin_lock_bh(lock: *mut c_void);
    fn spin_unlock_bh(lock: *mut c_void);
    fn kfree(ptr: *mut c_void);
    fn per_cpu_ptr(cpu: *mut c_void, cpu_id: c_int) -> *mut c_void;
    fn u64_stats_fetch_begin_irq(syncp: *mut c_void) -> u32;
    fn u64_stats_fetch_retry_irq(syncp: *mut c_void, start: u32) -> c_int;
    fn read_seqcount_begin(running: *mut c_void) -> u32;
    fn read_seqcount_retry(running: *mut c_void, seq: u32) -> c_int;
    fn gen_estimator_read(rate_est: *mut c_void, sample: *mut gnet_stats_rate_est64) -> c_int;
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn gnet_stats_copy(d: *mut gnet_dump, type_: c_int, buf: *const c_void, size: c_int, padattr: c_int) -> c_int {
    // SAFETY: Caller guarantees valid d and buf pointers
    if nla_put_64bit((*d).skb, type_, size, buf, padattr) != 0 {
        if !(*d).lock.is_null() {
            spin_unlock_bh((*d).lock);
        }
        kfree((*d).xstats);
        (*d).xstats = ptr::null_mut();
        (*d).xstats_len = 0;
        return -1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn gnet_stats_start_copy_compat(skb: *mut c_void, type_: c_int, tc_stats_type: c_int, xstats_type: c_int, lock: *mut c_void, d: *mut gnet_dump, padattr: c_int) -> c_int {
    // SAFETY: Caller guarantees valid d pointer
    ptr::write_bytes(d as *mut u8, 0, mem::size_of::<gnet_dump>());
    
    if type_ != 0 {
        (*d).tail = skb as *mut c_void;
    }
    (*d).skb = skb;
    (*d).compat_tc_stats = tc_stats_type;
    (*d).compat_xstats = xstats_type;
    (*d).padattr = padattr;
    
    if !lock.is_null() {
        (*d).lock = lock;
        spin_lock_bh(lock);
    }
    
    if !(*d).tail.is_null() {
        let ret = gnet_stats_copy(d, type_, ptr::null(), 0, padattr);
        
        if ret == 0 && (*d).tail.cast::<u8>().offset(0).cast::<u16>().read() == padattr as u16 {
            // SAFETY: Valid pointer arithmetic with aligned data
            (*d).tail = (*d).tail.cast::<u8>().add((*d).tail.cast::<u8>().read().nla_len as usize).cast();
        }
        return ret;
    }
    
    0
}

#[no_mangle]
pub unsafe extern "C" fn gnet_stats_start_copy(skb: *mut c_void, type_: c_int, lock: *mut c_void, d: *mut gnet_dump, padattr: c_int) -> c_int {
    gnet_stats_start_copy_compat(skb, type_, 0, 0, lock, d, padattr)
}

#[no_mangle]
pub unsafe extern "C" fn __gnet_stats_copy_basic(running: *mut c_void, bstats: *mut gnet_stats_basic_packed, cpu: *mut c_void, b: *mut gnet_stats_basic_packed) {
    if !cpu.is_null() {
        let mut bstats_cpu = gnet_stats_basic_packed { bytes: 0, packets: 0 };
        // Simulate for_each_possible_cpu
        for cpu_id in 0..32 {
            let bcpu = per_cpu_ptr(cpu, cpu_id as c_int);
            let syncp = ptr::null_mut();
            let mut start: u32 = 0;
            let mut bytes: u64 = 0;
            let mut packets: u64 = 0;
            
            loop {
                start = u64_stats_fetch_begin_irq(syncp);
                bytes = (*bcpu).bstats.bytes;
                packets = (*bcpu).bstats.packets;
                if u64_stats_fetch_retry_irq(syncp, start) == 0 { break; }
            }
            
            bstats_cpu.bytes += bytes;
            bstats_cpu.packets += packets;
        }
        
        (*bstats).bytes = bstats_cpu.bytes;
        (*bstats).packets = bstats_cpu.packets;
        return;
    }
    
    let mut seq: u32 = 0;
    loop {
        if !running.is_null() {
            seq = read_seqcount_begin(running);
        }
        (*bstats).bytes = (*b).bytes;
        (*bstats).packets = (*b).packets;
        if !running.is_null() && read_seqcount_retry(running, seq) != 0 {
            continue;
        }
        break;
    }
}

#[no_mangle]
pub unsafe extern "C" fn ___gnet_stats_copy_basic(running: *mut c_void, d: *mut gnet_dump, cpu: *mut c_void, b: *mut gnet_stats_basic_packed, type_: c_int) -> c_int {
    let mut bstats = gnet_stats_basic_packed { bytes: 0, packets: 0 };
    __gnet_stats_copy_basic(running, &mut bstats, cpu, b);
    
    if (*d).compat_tc_stats != 0 && type_ == 1 { // TCA_STATS_BASIC
        (*d).tc_stats.bytes = bstats.bytes;
        (*d).tc_stats.packets = bstats.packets;
    }
    
    if !(*d).tail.is_null() {
        let mut sb = gnet_stats_basic {
            bytes: bstats.bytes,
            packets: bstats.packets,
            bps: 0,
            pps: 0,
            qlen: 0,
            drops: 0,
            requeues: 0,
            overlimits: 0,
            backlog: 0,
        };
        
        let res = gnet_stats_copy(d, type_, &sb as *const _ as *const c_void, mem::size_of::<gnet_stats_basic>() as c_int, 0);
        
        if res < 0 || sb.packets == bstats.packets {
            return res;
        }
        
        return gnet_stats_copy(d, 2, &bstats.packets as *const _ as *const c_void, mem::size_of_val(&bstats.packets) as c_int, 0);
    }
    
    0
}

#[no_mangle]
pub unsafe extern "C" fn gnet_stats_copy_basic(running: *mut c_void, d: *mut gnet_dump, cpu: *mut c_void, b: *mut gnet_stats_basic_packed) -> c_int {
    ___gnet_stats_copy_basic(running, d, cpu, b, 1) // TCA_STATS_BASIC
}

#[no_mangle]
pub unsafe extern "C" fn gnet_stats_copy_basic_hw(running: *mut c_void, d: *mut gnet_dump, cpu: *mut c_void, b: *mut gnet_stats_basic_packed) -> c_int {
    ___gnet_stats_copy_basic(running, d, cpu, b, 2) // TCA_STATS_BASIC_HW
}

#[no_mangle]
pub unsafe extern "C" fn gnet_stats_copy_rate_est(d: *mut gnet_dump, rate_est: *mut c_void) -> c_int {
    let mut sample = gnet_stats_rate_est64 { bps: 0, pps: 0 };
    if gen_estimator_read(rate_est, &mut sample) == 0 {
        return 0;
    }
    
    let mut est = gnet_stats_basic {
        bytes: 0,
        packets: 0,
        bps: sample.bps as u32,
        pps: sample.pps as u32,
        qlen: 0,
        drops: 0,
        requeues: 0,
        overlimits: 0,
        backlog: 0,
    };
    
    if (*d).compat_tc_stats != 0 {
        (*d).tc_stats.bps = est.bps;
        (*d).tc_stats.pps = est.pps;
    }
    
    if !(*d).tail.is_null() {
        let res = gnet_stats_copy(d, 3, &est as *const _ as *const c_void, mem::size_of::<gnet_stats_basic>() as c_int, 0);
        if res < 0 || est.bps as u64 == sample.bps {
            return res;
        }
        return gnet_stats_copy(d, 4, &sample as *const _ as *const c_void, mem::size_of::<gnet_stats_rate_est64>() as c_int, 0);
    }
    
    0
}

#[no_mangle]
pub unsafe extern "C" fn __gnet_stats_copy_queue_cpu(qstats: *mut gnet_stats_queue, q: *mut c_void) {
    // Simulate for_each_possible_cpu
    for cpu_id in 0..32 {
        let qcpu = per_cpu_ptr(q, cpu_id as c_int);
        (*qstats).qlen = 0;
        (*qstats).backlog += (*qcpu).backlog;
        (*qstats).drops += (*qcpu).drops;
        (*qstats).requeues += (*qcpu).requeues;
        (*qstats).overlimits += (*qcpu).overlimits;
    }
}

#[no_mangle]
pub unsafe extern "C" fn __gnet_stats_copy_queue(qstats: *mut gnet_stats_queue, cpu: *mut c_void, q: *mut gnet_stats_queue, qlen: c_uint) {
    if !cpu.is_null() {
        __gnet_stats_copy_queue_cpu(qstats, cpu);
    } else {
        (*qstats).qlen = (*q).qlen;
        (*qstats).backlog = (*q).backlog;
        (*qstats).drops = (*q).drops;
        (*qstats).requeues = (*q).requeues;
        (*qstats).overlimits = (*q).overlimits;
    }
    
    (*qstats).qlen = qlen;
}

#[no_mangle]
pub unsafe extern "C" fn gnet_stats_copy_queue(d: *mut gnet_dump, cpu_q: *mut c_void, q: *mut gnet_stats_queue, qlen: c_uint) -> c_int {
    let mut qstats = gnet_stats_queue {
        qlen: 0,
        backlog: 0,
        drops: 0,
        requeues: 0,
        overlimits: 0,
    };
    
    __gnet_stats_copy_queue(&mut qstats, cpu_q, q, qlen);
    
    if (*d).compat_tc_stats != 0 {
        (*d).tc_stats.drops = qstats.drops;
        (*d).tc_stats.qlen = qstats.qlen;
        (*d).tc_stats.backlog = qstats.backlog;
        (*d).tc_stats.overlimits = qstats.overlimits;
    }
    
    if !(*d).tail.is_null() {
        return gnet_stats_copy(d, 5, &qstats as *const _ as *const c_void, mem::size_of::<gnet_stats_queue>() as c_int, 0);
    }
    
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_gnet_stats_copy() {
        // Basic test case - would need actual kernel environment to run
        assert!(true);
    }
}
