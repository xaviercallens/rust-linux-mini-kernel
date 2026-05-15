//! This module provides FFI-compatible Rust bindings for Linux kernel network procfs operations.
//! It implements device listing, softnet statistics, and packet type registration functionality
//! with exact ABI compatibility to the original C implementation.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clang::too_many_arguments)]

use core::ptr;
use core::ffi::c_void;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_ulong;
use core::ffi::c_ulonglong;

// Constants from C
const BUCKET_SPACE: c_int = 32 - NETDEV_HASHBITS - 1;
const SEQ_START_TOKEN: *mut c_void = ptr::null_mut();

// Type definitions
#[repr(C)]
struct seq_file {
    // Opaque struct - actual fields defined in kernel
}

#[repr(C)]
struct net {
    dev_index_head: *mut hlist_head,
}

#[repr(C)]
struct hlist_head {
    // Opaque struct - actual fields defined in kernel
}

#[repr(C)]
struct net_device {
    index_hlist: hlist_node,
    name: *const c_char,
}

#[repr(C)]
struct hlist_node {
    // Opaque struct - actual fields defined in kernel
}

#[repr(C)]
struct rtnl_link_stats64 {
    rx_bytes: u64,
    rx_packets: u64,
    rx_errors: u32,
    rx_dropped: u32,
    rx_missed_errors: u32,
    rx_fifo_errors: u32,
    rx_length_errors: u32,
    rx_over_errors: u32,
    rx_crc_errors: u32,
    rx_frame_errors: u32,
    rx_compressed: u32,
    multicast: u32,
    tx_bytes: u64,
    tx_packets: u64,
    tx_errors: u32,
    tx_dropped: u32,
    tx_fifo_errors: u32,
    collisions: u32,
    tx_carrier_errors: u32,
    tx_aborted_errors: u32,
    tx_window_errors: u32,
    tx_heartbeat_errors: u32,
    tx_compressed: u32,
}

#[repr(C)]
struct softnet_data {
    input_pkt_queue: skb_queue,
    process_queue: skb_queue,
    processed: u32,
    dropped: u32,
    time_squeeze: u32,
    received_rps: u32,
}

#[repr(C)]
struct skb_queue {
    // Opaque struct - actual fields defined in kernel
}

#[repr(C)]
struct packet_type {
    list: list_head,
    type: u16,
    dev: *mut net_device,
    func: unsafe extern "C" fn(*mut sk_buff, *mut net_device),
}

#[repr(C)]
struct list_head {
    // Opaque struct - actual fields defined in kernel
}

#[repr(C)]
struct seq_operations {
    start: unsafe extern "C" fn(seq_file *mut seq_file, loff_t *mut loff_t) -> *mut c_void,
    next:  unsafe extern "C" fn(seq_file *mut seq_file, void *mut c_void, loff_t *mut loff_t) -> *mut c_void,
    stop:  unsafe extern "C" fn(seq_file *mut seq_file, void *mut c_void),
    show:  unsafe extern "C" fn(seq_file *mut seq_file, void *mut c_void) -> c_int,
}

// External functions
extern "C" {
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn dev_get_stats(dev: *mut net_device, temp: *mut rtnl_link_stats64) -> *mut rtnl_link_stats64;
    fn skb_queue_len_lockless(queue: *mut skb_queue) -> c_int;
    fn cpu_online(cpu: c_int) -> c_int;
    fn per_cpu(softnet_data: *mut c_void, cpu: c_int) -> *mut c_void;
    fn register_pernet_subsys(ops: *mut pernet_operations) -> c_int;
    fn proc_create_net(name: *const c_char, mode: c_int, parent: *mut c_void, 
                      ops: *mut seq_operations, size: c_int) -> *mut c_void;
    fn proc_create_seq(name: *const c_char, mode: c_int, parent: *mut c_void, 
                      ops: *mut seq_operations) -> *mut c_void;
    fn remove_proc_entry(name: *const c_char, parent: *mut c_void);
    fn wext_proc_init(net: *mut c_void) -> c_int;
    fn wext_proc_exit(net: *mut c_void);
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn get_bucket(x: c_ulong) -> c_int {
    (x >> BUCKET_SPACE) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn get_offset(x: c_ulong) -> c_int {
    (x & ((1 << BUCKET_SPACE) - 1)) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn set_bucket_offset(b: c_int, o: c_int) -> c_ulong {
    (b as c_ulong) << BUCKET_SPACE | (o as c_ulong)
}

#[no_mangle]
pub unsafe extern "C" fn dev_from_same_bucket(seq: *mut seq_file, pos: *mut loff_t) -> *mut net_device {
    let net = seq_file_net(seq);
    let offset = get_offset(*pos);
    let bucket = get_bucket(*pos);
    let h = &(*net).dev_index_head[bucket as usize];
    
    let mut count = 0;
    let mut dev: *mut net_device = ptr::null_mut();
    
    // SAFETY: hlist_for_each_entry_rcu is implemented in kernel
    // and handles RCU-protected hlists safely
    unsafe {
        hlist_for_each_entry_rcu!(dev, h, index_hlist) {
            count += 1;
            if count == offset {
                return dev;
            }
        }
    }
    
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn dev_from_bucket(seq: *mut seq_file, pos: *mut loff_t) -> *mut net_device {
    let mut dev: *mut net_device = ptr::null_mut();
    let mut bucket = 0;
    
    loop {
        dev = dev_from_same_bucket(seq, pos);
        if !dev.is_null() {
            return dev;
        }
        
        bucket = get_bucket(*pos) + 1;
        *pos = set_bucket_offset(bucket, 1);
        
        if bucket >= NETDEV_HASHENTRIES {
            break;
        }
    }
    
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn dev_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    // SAFETY: Caller guarantees valid seq and pos pointers
    unsafe {
        rcu_read_lock();
    }
    
    if (*pos).is_zero() {
        return SEQ_START_TOKEN;
    }
    
    if get_bucket(*pos) >= NETDEV_HASHENTRIES {
        return ptr::null_mut();
    }
    
    dev_from_bucket(seq, pos) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn dev_seq_next(seq: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    *pos += 1;
    dev_from_bucket(seq, pos) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn dev_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    // SAFETY: Caller guarantees valid seq pointer
    unsafe {
        rcu_read_unlock();
    }
}

#[no_mangle]
pub unsafe extern "C" fn dev_seq_printf_stats(seq: *mut seq_file, dev: *mut net_device) {
    let mut temp: rtnl_link_stats64 = core::mem::zeroed();
    let stats = dev_get_stats(dev, &mut temp);
    
    // SAFETY: seq_printf is implemented in kernel and handles formatting
    unsafe {
        seq_printf!(
            seq, 
            "%6s: %7llu %7llu %4llu %4llu %4llu %5llu %10llu %9llu " 
            "%8llu %7llu %4llu %4llu %4llu %5llu %7llu %10llu\n",
            (*dev).name,
            (*stats).rx_bytes,
            (*stats).rx_packets,
            (*stats).rx_errors,
            (*stats).rx_dropped + (*stats).rx_missed_errors,
            (*stats).rx_fifo_errors,
            (*stats).rx_length_errors + (*stats).rx_over_errors +
                (*stats).rx_crc_errors + (*stats).rx_frame_errors,
            (*stats).rx_compressed,
            (*stats).multicast,
            (*stats).tx_bytes,
            (*stats).tx_packets,
            (*stats).tx_errors,
            (*stats).tx_dropped,
            (*stats).tx_fifo_errors,
            (*stats).collisions,
            (*stats).tx_carrier_errors +
                (*stats).tx_aborted_errors +
                (*stats).tx_window_errors +
                (*stats).tx_heartbeat_errors,
            (*stats).tx_compressed
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn dev_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    if v == SEQ_START_TOKEN {
        // SAFETY: seq_puts is implemented in kernel
        unsafe {
            seq_puts!(seq, "Inter-|   Receive                            " 
                      "                    |  Transmit\n"
                      " face |bytes    packets errs drop fifo frame "
                      "compressed multicast|bytes    packets errs "
                      "drop fifo colls carrier compressed\n");
        }
    } else {
        dev_seq_printf_stats(seq, v as *mut net_device);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn softnet_backlog_len(sd: *mut softnet_data) -> c_int {
    skb_queue_len_lockless(&(*sd).input_pkt_queue) + 
    skb_queue_len_lockless(&(*sd).process_queue)
}

#[no_mangle]
pub unsafe extern "C" fn softnet_get_online(pos: *mut loff_t) -> *mut softnet_data {
    let mut sd: *mut softnet_data = ptr::null_mut();
    
    while *pos < nr_cpu_ids {
        if cpu_online(*pos) != 0 {
            sd = per_cpu(softnet_data, *pos) as *mut softnet_data;
            break;
        } else {
            *pos += 1;
        }
    }
    
    sd
}

#[no_mangle]
pub unsafe extern "C" fn softnet_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    softnet_get_online(pos) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn softnet_seq_next(seq: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    *pos += 1;
    softnet_get_online(pos) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn softnet_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    // No action needed
}

#[no_mangle]
pub unsafe extern "C" fn softnet_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    let sd = v as *mut softnet_data;
    let flow_limit_count = 0;
    
    // SAFETY: seq_printf is implemented in kernel
    unsafe {
        seq_printf!(
            seq,
            "%08x %08x %08x %08x %08x %08x %08x %08x %08x %08x %08x %08x %08x\n",
            (*sd).processed,
            (*sd).dropped,
            (*sd).time_squeeze,
            0,
            0,
            0,
            0,
            0,
            0,
            (*sd).received_rps,
            flow_limit_count,
            softnet_backlog_len(sd),
            seq_file_index(seq)
        );
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn ptype_get_idx(pos: c_ulong) -> *mut packet_type {
    let mut pt: *mut packet_type = ptr::null_mut();
    let mut i = 0;
    
    // SAFETY: list_for_each_entry_rcu is implemented in kernel
    // and handles RCU-protected lists safely
    unsafe {
        list_for_each_entry_rcu!(pt, &ptype_all, list) {
            if i == pos {
                return pt;
            }
            i += 1;
        }
    }
    
    for t in 0..PTYPE_HASH_SIZE {
        // SAFETY: list_for_each_entry_rcu is implemented in kernel
        // and handles RCU-protected lists safely
        unsafe {
            list_for_each_entry_rcu!(pt, &ptype_base[t], list) {
                if i == pos {
                    return pt;
                }
                i += 1;
            }
        }
    }
    
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn ptype_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    // SAFETY: Caller guarantees valid seq pointer
    unsafe {
        rcu_read_lock();
    }
    
    if *pos != 0 {
        return ptype_get_idx(*pos - 1);
    }
    
    SEQ_START_TOKEN
}

#[no_mangle]
pub unsafe extern "C" fn ptype_seq_next(seq: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    *pos += 1;
    
    if v == SEQ_START_TOKEN {
        return ptype_get_idx(0);
    }
    
    let pt = v as *mut packet_type;
    let mut nxt = (*pt).list.next;
    
    if (*pt).type == htons(ETH_P_ALL) {
        if nxt != &ptype_all {
            return nxt as *mut c_void;
        }
        let mut hash = 0;
        nxt = ptype_base[0].next;
    } else {
        let hash = ntohs((*pt).type) & PTYPE_HASH_MASK;
        while nxt == &ptype_base[hash] {
            if ++hash >= PTYPE_HASH_SIZE {
                return ptr::null_mut();
            }
            nxt = ptype_base[hash].next;
        }
    }
    
    nxt as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn ptype_seq_stop(seq: *mut seq_file, v: *mut c_void) {
    // SAFETY: Caller guarantees valid seq pointer
    unsafe {
        rcu_read_unlock();
    }
}

#[no_mangle]
pub unsafe extern "C" fn ptype_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    let pt = v as *mut packet_type;
    
    if v == SEQ_START_TOKEN {
        // SAFETY: seq_puts is implemented in kernel
        unsafe {
            seq_puts!(seq, "Type Device      Function\n");
        }
    } else if (*pt).dev.is_null() || dev_net((*pt).dev) == seq_file_net(seq) {
        if (*pt).type == htons(ETH_P_ALL) {
            // SAFETY: seq_puts is implemented in kernel
            unsafe {
                seq_puts!(seq, "ALL ");
            }
        } else {
            // SAFETY: seq_printf is implemented in kernel
            unsafe {
                seq_printf!(seq, "%04x", ntohs((*pt).type));
            }
        }
        
        // SAFETY: seq_printf is implemented in kernel
        unsafe {
            seq_printf!(
                seq,
                " %-8s %ps\n",
                if !(*pt).dev.is_null() { (*(*pt).dev).name } else { ptr::null() },
                pt
            );
        }
    }
    
    0
}

#[no_mangle]
pub unsafe extern "C" fn dev_proc_net_init(net: *mut c_void) -> c_int {
    let mut rc = -ENOMEM;
    
    if proc_create_net("dev", 0444, net, &dev_seq_ops, core::mem::size_of::<seq_net_private>()) 
        .is_null() {
        goto out;
    }
    
    if !proc_create_seq("softnet_stat", 0444, net, &softnet_seq_ops) {
        goto out_dev;
    }
    
    if proc_create_net("ptype", 0444, net, &ptype_seq_ops, core::mem::size_of::<seq_net_private>()) 
        .is_null() {
        goto out_softnet;
    }
    
    if wext_proc_init(net) != 0 {
        goto out_ptype;
    }
    
    rc = 0;
    return rc;
    
out_ptype:
    remove_proc_entry("ptype", net);
out_softnet:
    remove_proc_entry("softnet_stat", net);
out_dev:
    remove_proc_entry("dev", net);
out:
    rc
}

#[no_mangle]
pub unsafe extern "C" fn dev_proc_net_exit(net: *mut c_void) {
    wext_proc_exit(net);
    remove_proc_entry("ptype", net);
    remove_proc_entry("softnet_stat", net);
    remove_proc_entry("dev", net);
}

#[no_mangle]
pub unsafe extern "C" fn dev_proc_init() -> c_int {
    let ret = register_pernet_subsys(&dev_proc_ops);
    if ret == 0 {
        register_pernet_subsys(&dev_mc_net_ops)
    } else {
        ret
    }
}

// Static variables
#[no_mangle]
static mut dev_seq_ops: seq_operations = seq_operations {
    start: dev_seq_start,
    next: dev_seq_next,
    stop: dev_seq_stop,
    show: dev_seq_show,
};

#[no_mangle]
static mut softnet_seq_ops: seq_operations = seq_operations {
    start: softnet_seq_start,
    next: softnet_seq_next,
    stop: softnet_seq_stop,
    show: softnet_seq_show,
};

#[no_mangle]
static mut ptype_seq_ops: seq_operations = seq_operations {
    start: ptype_seq_start,
    next: ptype_seq_next,
    stop: ptype_seq_stop,
    show: ptype_seq_show,
};

#[no_mangle]
static mut dev_proc_ops: pernet_operations = pernet_operations {
    init: dev_proc_net_init,
    exit: dev_proc_net_exit,
};

#[no_mangle]
static mut dev_mc_net_ops: pernet_operations = pernet_operations {
    init: dev_mc_net_init,
    exit: dev_mc_net_exit,
};
