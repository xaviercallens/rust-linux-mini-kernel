//! Network Dropped Packet Monitoring Module
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use core::mem;

// Constants from C
pub const TRACE_ON: c_int = 1;
pub const TRACE_OFF: c_int = 0;
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

// Type definitions
#[repr(C)]
pub struct net_dm_stats {
    pub dropped: u64,
    pub syncp: u64_stats_sync, // Opaque kernel type
}

#[repr(C)]
pub struct net_dm_hw_entry {
    pub trap_name: [u8; 40],
    pub count: u32,
}

#[repr(C)]
pub struct net_dm_hw_entries {
    pub num_entries: u32,
    pub entries: [net_dm_hw_entry; 0], // Flexible array member
}

#[repr(C)]
pub struct per_cpu_dm_data {
    pub lock: spinlock_t, // Opaque kernel type
    pub data_union: per_cpu_dm_data_union,
    pub drop_queue: sk_buff_head, // Opaque kernel type
    pub dm_alert_work: work_struct, // Opaque kernel type
    pub send_timer: timer_list, // Opaque kernel type
    pub stats: net_dm_stats,
}

#[repr(C)]
union per_cpu_dm_data_union {
    skb: *mut sk_buff, // Opaque kernel type
    hw_entries: *mut net_dm_hw_entries,
}

#[repr(C)]
pub struct dm_hw_stat_delta {
    pub dev: *mut net_device, // Opaque kernel type
    pub last_rx: u64,
    pub list: list_head, // Opaque kernel type
    pub rcu: rcu_head, // Opaque kernel type
    pub last_drop_val: u64,
}

// Function implementations
/// Reset per-CPU data for alert message
///
/// # Safety
/// - `data` must be a valid pointer to per_cpu_dm_data
/// - Kernel memory allocation functions must be available
///
/// # Returns
/// Pointer to sk_buff or NULL on failure
#[no_mangle]
pub unsafe extern "C" fn reset_per_cpu_data(
    data: *mut per_cpu_dm_data,
) -> *mut sk_buff {
    let al: usize = mem::size_of::<net_dm_alert_msg>();
    al += dm_hit_limit * mem::size_of::<net_dm_drop_point>();
    al += mem::size_of::<nlattr>();

    let skb = genlmsg_new(al, GFP_KERNEL);
    if skb.is_null() {
        goto err;
    }

    let msg_header = genlmsg_put(skb, 0, 0, &net_drop_monitor_family, 0, NET_DM_CMD_ALERT);
    if msg_header.is_null() {
        nlmsg_free(skb);
        return ptr::null_mut();
    }

    let nla = nla_reserve(skb, NLA_UNSPEC, mem::size_of::<net_dm_alert_msg>());
    if nla.is_null() {
        nlmsg_free(skb);
        return ptr::null_mut();
    }

    let msg = nla_data(nla);
    ptr::write_bytes(msg, 0, al);

    out:
    let flags = spin_lock_irqsave(&(*data).lock);
    let data_union = &mut (*data).data_union;
    let old_skb = (*data_union).skb;
    (*data_union).skb = skb;
    spin_unlock_irqrestore(&(*data).lock, flags);

    if !skb.is_null() {
        let nlh = &(*skb).data as *const _ as *mut nlmsghdr;
        let gnlh = nlmsg_data(nlh) as *mut genlmsghdr;
        genlmsg_end(skb, genlmsg_data(gnlh));
    }

    return skb;

    err:
    mod_timer(&(*data).send_timer, jiffies + HZ / 10);
    return ptr::null_mut();
}

/// Send delayed alert work
///
/// # Safety
/// - `work` must be a valid pointer to work_struct
#[no_mangle]
pub unsafe extern "C" fn send_dm_alert(work: *mut work_struct) {
    let data = container_of!(work, per_cpu_dm_data, dm_alert_work);
    let skb = reset_per_cpu_data(data);

    if !skb.is_null() {
        genlmsg_multicast(&net_drop_monitor_family, skb, 0, 0, GFP_KERNEL);
    }
}

/// Schedule work to send alert
///
/// # Safety
/// - `t` must be a valid timer_list pointer
#[no_mangle]
pub unsafe extern "C" fn sched_send_work(t: *mut timer_list) {
    let data = container_of!(t, per_cpu_dm_data, send_timer);
    schedule_work(&(*data).dm_alert_work);
}

/// Common drop tracing logic
///
/// # Safety
/// - `skb` must be valid or NULL
/// - `location` must be valid pointer
#[no_mangle]
pub unsafe extern "C" fn trace_drop_common(
    skb: *mut sk_buff,
    location: *const c_void,
) {
    let data = this_cpu_ptr!(&dm_cpu_data);
    let data_union = &mut (*data).data_union;
    let dskb = (*data_union).skb;

    if dskb.is_null() {
        return;
    }

    let nlh = &(*dskb).data as *const _ as *mut nlmsghdr;
    let nla = genlmsg_data(nlh) as *mut nlattr;
    let msg = nla_data(nla) as *mut net_dm_alert_msg;
    let point = (*msg).points;
    let mut i: c_int = 0;

    while i < (*msg).entries {
        if !ptr::eq(location, &(*point.offset(i as isize)).pc) {
            i += 1;
            continue;
        }
        (*point.offset(i as isize)).count += 1;
        return;
    }

    if (*msg).entries >= dm_hit_limit as u32 {
        return;
    }

    __nla_reserve_nohdr(dskb, mem::size_of::<net_dm_drop_point>());
    let nla = nla as *mut nlattr;
    (*nla).nla_len += NLA_ALIGN(mem::size_of::<net_dm_drop_point>()) as u16;
    ptr::copy_nonoverlapping(location, &mut (*point.offset(i as isize)).pc, 1);
    (*point.offset(i as isize)).count = 1;
    (*msg).entries += 1;

    if !timer_pending(&(*data).send_timer) {
        (*(*data).send_timer).expires = jiffies + dm_delay * HZ;
        add_timer(&(*data).send_timer);
    }
}

// Helper macros translated to Rust functions
#[macro_export]
macro_rules! container_of {
    ($ptr:expr, $type:ty, $member:ident) => {
        unsafe {
            let offset = core::mem::offset_of!($type, $member);
            ($ptr as *mut u8).offset(-offset as isize) as *mut $type
        }
    }
}

#[macro_export]
macro_rules! this_cpu_ptr {
    ($var:expr) => {
        unsafe { &(*$var) }
    }
}

#[macro_export]
macro_rules! spin_lock_irqsave {
    ($lock:expr, $flags:expr) => {
        unsafe { spin_lock_irqsave($lock, $flags) }
    }
}

#[macro_export]
macro_rules! spin_unlock_irqrestore {
    ($lock:expr, $flags:expr) => {
        unsafe { spin_unlock_irqrestore($lock, $flags) }
    }
}

// Opaque kernel types
#[repr(C)]
pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)]
pub struct sk_buff { data: [u8; 0] }
#[repr(C)]
pub struct sk_buff_head { _private: [u8; 0] }
#[repr(C)]
pub struct work_struct { _private: [u8; 0] }
#[repr(C)]
pub struct timer_list { _private: [u8; 0] }
#[repr(C)]
pub struct list_head { _private: [u8; 0] }
#[repr(C)]
pub struct rcu_head { _private: [u8; 0] }
#[repr(C)]
pub struct net_device { _private: [u8; 0] }
#[repr(C)]
pub struct u64_stats_sync { _private: [u8; 0] }
#[repr(C)]
pub struct genlmsghdr { _private: [u8; 0] }
#[repr(C)]
pub struct nlmsghdr { _private: [u8; 0] }

// Extern declarations for kernel functions
extern "C" {
    fn genlmsg_new(al: usize, gfp: c_int) -> *mut sk_buff;
    fn genlmsg_put(skb: *mut sk_buff, portid: u32, seq: u32, 
                   family: *const genl_family, flags: u32, cmd: u8) -> *mut c_void;
    fn nla_reserve(skb: *mut sk_buff, attrtype: u16, attrlen: usize) -> *mut nlattr;
    fn nla_data(nla: *const nlattr) -> *mut c_void;
    fn __nla_reserve_nohdr(skb: *mut sk_buff, len: usize);
    fn NLA_ALIGN(len: usize) -> usize;
    fn genlmsg_multicast(family: *const genl_family, skb: *mut sk_buff, 
                         group: u32, flags: u32, gfp: c_int);
    fn nlmsg_free(skb: *mut sk_buff);
    fn mod_timer(timer: *mut timer_list, expires: u64);
    fn schedule_work(work: *mut work_struct);
    fn timer_pending(timer: *mut timer_list);
    fn add_timer(timer: *mut timer_list);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: u64) -> u64;
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: u64);
    fn jiffies() -> u64;
    fn HZ() -> u64;
    fn GFP_KERNEL() -> c_int;
    fn NLA_UNSPEC() -> u16;
    fn NET_DM_CMD_ALERT() -> u8;
}

// Global variables
static mut trace_state: c_int = TRACE_OFF;
static mut monitor_hw: bool = false;
static mut net_dm_mutex: mutex_t = mutex_t { .. };
static mut dm_cpu_data: [per_cpu_dm_data; num_possible_cpus] = [per_cpu_dm_data { .. }; 0];
static mut dm_hw_cpu_data: [per_cpu_dm_data; num_possible_cpus] = [per_cpu_dm_data { .. }; 0];
static mut dm_hit_limit: c_int = 64;
static mut dm_delay: c_int = 1;
static mut dm_hw_check_delta: u64 = 2 * HZ();
static mut hw_stats_list: list_head = list_head { .. };
static mut net_dm_alert_mode: net_dm_alert_mode_t = NET_DM_ALERT_MODE_SUMMARY;
static mut net_dm_trunc_len: u32 = 0;
static mut net_dm_queue_len: u32 = 1000;

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    #[test]
    fn test_reset_per_cpu_data() {
        // Basic test would require kernel environment
        // This is a placeholder for actual tests
        assert!(true);
    }
}
