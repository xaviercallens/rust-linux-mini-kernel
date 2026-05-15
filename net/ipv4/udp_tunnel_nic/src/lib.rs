// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2020 Facebook Inc.

#![no_std]
#![allow(non_camel_case_types)]

use core::ptr;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_ulong;
use core::ffi::c_void;
use core::mem;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSPC: c_int = -28;
pub const EEXIST: c_int = -17;
pub const ENOENT: c_int = -2;

// Type definitions
#[repr(C)]
pub struct in6_addr {
    __in6_u: [u8; 16],
}

#[repr(C)]
pub struct sockaddr {
    sa_family: c_int,
    sa_data: [u8; 14],
}

#[repr(C)]
pub struct udp_tunnel_info {
    port: u16,
    type_: u8,
    sa_family: u16,
    __pad: u16,
    key: [u8; 16],
    hw_priv: u8,
}

#[repr(C)]
pub struct net_device;

#[repr(C)]
pub struct work_struct;

#[repr(C)]
pub struct workqueue_struct;

#[repr(C)]
pub struct udp_tunnel_nic_table_info {
    tunnel_types: u8,
    n_entries: c_uint,
};

#[repr(C)]
pub struct udp_tunnel_nic_info {
    flags: u8,
    sync_table: extern "C" fn(*mut net_device, c_uint) -> c_int,
    set_port: extern "C" fn(*mut net_device, c_uint, c_uint, *const udp_tunnel_info) -> c_int,
    unset_port: extern "C" fn(*mut net_device, c_uint, c_uint, *const udp_tunnel_info) -> c_int,
    tables: *const udp_tunnel_nic_table_info,
    n_tables: c_uint,
}

#[repr(C)]
pub struct udp_tunnel_nic_table_entry {
    port: u16,
    type_: u8,
    flags: u8,
    use_cnt: u16,
    hw_priv: u8,
}

#[repr(C)]
pub struct udp_tunnel_nic {
    work: *mut work_struct,
    dev: *mut net_device,
    need_sync: u8,
    need_replay: u8,
    work_pending: u8,
    n_tables: c_uint,
    missed: c_ulong,
    entries: *mut *mut udp_tunnel_nic_table_entry,
}

// Workqueue global
static mut udp_tunnel_nic_workqueue: *mut workqueue_struct = ptr::null_mut();

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn queue_work(wq: *mut workqueue_struct, work: *mut work_struct) -> bool {
    // Placeholder for actual implementation
    true
}

#[no_mangle]
pub unsafe extern "C" fn netdev_warn(dev: *mut net_device, fmt: *const c_char, ...) {
    // Placeholder for actual implementation
}

#[no_mangle]
pub unsafe extern "C" fn WARN_ON_ONCE(condition: bool) {
    if condition {
        // Placeholder for actual implementation
    }
}

#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_nic_tunnel_type_name(type: c_uint) -> *const c_char {
    match type {
        0x01 => "vxlan\0".as_ptr() as *const c_char,
        0x02 => "geneve\0".as_ptr() as *const c_char,
        0x03 => "vxlan-gpe\0".as_ptr() as *const c_char,
        _ => "unknown\0".as_ptr() as *const c_char,
    }
}

#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_nic_entry_is_free(entry: *const udp_tunnel_nic_table_entry) -> bool {
    (*entry).use_cnt == 0 && (*entry).flags == 0
}

#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_nic_entry_is_present(entry: *const udp_tunnel_nic_table_entry) -> bool {
    (*entry).use_cnt != 0 && !((*entry).flags & !(1 << 3))
}

#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_nic_entry_is_frozen(entry: *const udp_tunnel_nic_table_entry) -> bool {
    (*entry).flags & (1 << 3) != 0
}

#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_nic_entry_freeze_used(entry: *mut udp_tunnel_nic_table_entry) {
    if !udp_tunnel_nic_entry_is_free(entry) {
        (*entry).flags |= 1 << 3;
    }
}

#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_nic_entry_unfreeze(entry: *mut udp_tunnel_nic_table_entry) {
    (*entry).flags &= !(1 << 3);
}

#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_nic_entry_is_queued(entry: *const udp_tunnel_nic_table_entry) -> bool {
    let flags = (*entry).flags;
    flags & (1 << 0 | 1 << 1) != 0
}

#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_nic_entry_queue(
    utn: *mut udp_tunnel_nic,
    entry: *mut udp_tunnel_nic_table_entry,
    flag: c_uint,
) {
    (*entry).flags |= flag as u8;
    (*utn).need_sync = 1;
}

#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_nic_ti_from_entry(
    entry: *const udp_tunnel_nic_table_entry,
    ti: *mut udp_tunnel_info,
) {
    ptr::write_bytes(ti as *mut u8, 0, mem::size_of::<udp_tunnel_info>());
    (*ti).port = (*entry).port;
    (*ti).type_ = (*entry).type_;
    (*ti).hw_priv = (*entry).hw_priv;
}

#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_nic_is_empty(
    dev: *mut net_device,
    utn: *mut udp_tunnel_nic,
) -> bool {
    let info = (*dev).udp_tunnel_nic_info;
    let mut i = 0;
    while i < (*utn).n_tables {
        let table = &*info.add(8 * i) as *const udp_tunnel_nic_table_info;
        let mut j = 0;
        while j < (*table).n_entries {
            let entry = &*(*utn).entries.add(i).read() as *mut udp_tunnel_nic_table_entry;
            if !udp_tunnel_nic_entry_is_free(entry) {
                return false;
            }
            j += 1;
        }
        i += 1;
    }
    true
}

#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_nic_should_replay(
    dev: *mut net_device,
    utn: *mut udp_tunnel_nic,
) -> bool {
    if (*utn).missed == 0 {
        return false;
    }

    let mut i = 0;
    while i < (*utn).n_tables {
        let info = (*dev).udp_tunnel_nic_info;
        let table = &*info.add(8 * i) as *const udp_tunnel_nic_table_info;
        if !(*utn).missed & (1 << i) {
            i += 1;
            continue;
        }

        let mut j = 0;
        while j < (*table).n_entries {
            let entry = &*(*utn).entries.add(i).read() as *mut udp_tunnel_nic_table_entry;
            if udp_tunnel_nic_entry_is_free(entry) {
                return true;
            }
            j += 1;
        }
        i += 1;
    }

    false
}

#[no_mangle]
pub unsafe extern "C" fn __udp_tunnel_nic_get_port(
    dev: *mut net_device,
    table: c_uint,
    idx: c_uint,
    ti: *mut udp_tunnel_info,
) {
    let utn = (*dev).udp_tunnel_nic;
    let entry = &*(*utn).entries.add(table).read() as *mut udp_tunnel_nic_table_entry;
    if (*entry).use_cnt != 0 {
        udp_tunnel_nic_ti_from_entry(entry, ti);
    }
}

#[no_mangle]
pub unsafe extern "C" fn __udp_tunnel_nic_set_port_priv(
    dev: *mut net_device,
    table: c_uint,
    idx: c_uint,
    priv_: u8,
) {
    let utn = (*dev).udp_tunnel_nic;
    (*(*utn).entries.add(table).read()).add(idx).write().hw_priv = priv_;
}

#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_nic_entry_update_done(
    entry: *mut udp_tunnel_nic_table_entry,
    err: c_int,
) {
    if (*entry).flags & (1 << 0) != 0 && (*entry).flags & (1 << 1) != 0 {
        WARN_ON_ONCE(true);
    }

    if (*entry).flags & (1 << 0) != 0 && (err == 0 || (err == -EEXIST && (*entry).flags & (1 << 2) != 0)) {
        (*entry).flags &= !(1 << 0);
    }

    if (*entry).flags & (1 << 1) != 0 && (err == 0 || (err == -ENOENT && (*entry).flags & (1 << 2) != 0)) {
        (*entry).flags &= !(1 << 1);
    }

    if err == 0 {
        (*entry).flags &= !(1 << 2);
    } else {
        (*entry).flags |= 1 << 2;
    }
}

#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_nic_device_sync_one(
    dev: *mut net_device,
    utn: *mut udp_tunnel_nic,
    table: c_uint,
    idx: c_uint,
) {
    let entry = &*(*utn).entries.add(table).read() as *mut udp_tunnel_nic_table_entry;
    if !udp_tunnel_nic_entry_is_queued(entry) {
        return;
    }

    let mut ti: udp_tunnel_info = mem::zeroed();
    udp_tunnel_nic_ti_from_entry(entry, &mut ti);
    let err = if (*entry).flags & (1 << 0) != 0 {
        (*(*dev).udp_tunnel_nic_info).set_port(dev, table, idx, &ti)
    } else {
        (*(*dev).udp_tunnel_nic_info).unset_port(dev, table, idx, &ti)
    };
    udp_tunnel_nic_entry_update_done(entry, err);

    if err != 0 {
        netdev_warn(dev, "UDP tunnel port sync failed port %d type %s: %d\0".as_ptr() as *const c_char, (*entry).port as c_int, udp_tunnel_nic_tunnel_type_name((*entry).type_), err);
    }
}

#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_nic_device_sync_by_port(
    dev: *mut net_device,
    utn: *mut udp_tunnel_nic,
) {
    let info = (*dev).udp_tunnel_nic_info;
    let mut i = 0;
    while i < (*utn).n_tables {
        let mut j = 0;
        while j < (*info).tables.add(i).read().n_entries {
            udp_tunnel_nic_device_sync_one(dev, utn, i as c_uint, j as c_uint);
            j += 1;
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_nic_device_sync_by_table(
    dev: *mut net_device,
    utn: *mut udp_tunnel_nic,
) {
    let info = (*dev).udp_tunnel_nic_info;
    let mut i = 0;
    while i < (*utn).n_tables {
        let mut j = 0;
        while j < (*info).tables.add(i).read().n_entries {
            if udp_tunnel_nic_entry_is_queued(&*(*utn).entries.add(i).read().add(j)) {
                break;
            }
            j += 1;
        }
        if j == (*info).tables.add(i).read().n_entries {
            i += 1;
            continue;
        }

        let err = (*(*dev).udp_tunnel_nic_info).sync_table(dev, i as c_uint);
        if err != 0 {
            netdev_warn(dev, "UDP tunnel port sync failed for table %d: %d\0".as_ptr() as *const c_char, i as c_int, err);
        }

        let mut j = 0;
        while j < (*info).tables.add(i).read().n_entries {
            let entry = &*(*utn).entries.add(i).read().add(j) as *mut udp_tunnel_nic_table_entry;
            if udp_tunnel_nic_entry_is_queued(entry) {
                udp_tunnel_nic_entry_update_done(entry, err);
            }
            j += 1;
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn __udp_tunnel_nic_device_sync(
    dev: *mut net_device,
    utn: *mut udp_tunnel_nic,
) {
    if (*utn).need_sync == 0 {
        return;
    }

    if (*(*dev).udp_tunnel_nic_info).sync_table != ptr::null() {
        udp_tunnel_nic_device_sync_by_table(dev, utn);
    } else {
        udp_tunnel_nic_device_sync_by_port(dev, utn);
    }

    (*utn).need_sync = 0;
    (*utn).need_replay = if udp_tunnel_nic_should_replay(dev, utn) { 1 } else { 0 };
}

#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_nic_device_sync(
    dev: *mut net_device,
    utn: *mut udp_tunnel_nic,
) {
    let info = (*dev).udp_tunnel_nic_info;
    if (*utn).need_sync == 0 {
        return;
    }

    let may_sleep = (*info).flags & (1 << 0) != 0;
    if !may_sleep {
        __udp_tunnel_nic_device_sync(dev, utn);
    }
    if may_sleep || (*utn).need_replay != 0 {
        queue_work(udp_tunnel_nic_workqueue, (*utn).work);
        (*utn).work_pending = 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_nic_table_is_capable(
    table: *const udp_tunnel_nic_table_info,
    ti: *const udp_tunnel_info,
) -> bool {
    (*table).tunnel_types & (*ti).type_ != 0
}

#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_nic_is_capable(
    dev: *mut net_device,
    utn: *mut udp_tunnel_nic,
    ti: *const udp_tunnel_info,
) -> bool {
    let info = (*dev).udp_tunnel_nic_info;
    if (*info).flags & (1 << 1) != 0 && (*ti).sa_family != 2 {
        return false;
    }

    let mut i = 0;
    while i < (*utn).n_tables {
        if udp_tunnel_nic_table_is_capable(&*info.add(8 * i) as *const udp_tunnel_nic_table_info, ti) {
            return true;
        }
        i += 1;
    }
    false
}

#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_nic_has_collision(
    dev: *mut net_device,
    utn: *mut udp_tunnel_nic,
    ti: *const udp_tunnel_info,
) -> bool {
    let info = (*dev).udp_tunnel_nic_info;
    let mut i = 0;
    while i < (*utn).n_tables {
        let mut j = 0;
        while j < (*info).tables.add(i).read().n_entries {
            let entry = &*(*utn).entries.add(i).read().add(j) as *mut udp_tunnel_nic_table_entry;
            if !udp_tunnel_nic_entry_is_free(entry) && (*entry).port == (*ti).port && (*entry).type_ != (*ti).type_ {
                (*utn).missed |= 1 << i;
                return true;
            }
            j += 1;
        }
        i += 1;
    }
    false
}

#[no_mangle]
pub unsafe extern "C" fn udp_tunnel_nic_entry_adj(
    utn: *mut udp_tunnel_nic,
    table: c_uint,
    idx: c_uint,
    use_cnt_adj: c_int,
) {
    let entry = &*(*utn).entries.add(table).read().add(idx) as *mut udp_tunnel_nic_table_entry;
    let dodgy = (*entry).flags & (1 << 2) != 0;
    if (*entry).use_cnt as c_int + use_cnt_adj > u16::MAX as c_int {
        WARN_ON_ONCE(true);
    }

    (*entry).use_cnt = (*entry).use_cnt.wrapping_add(use_cnt_adj as u16);
    if !dodgy && (*entry).use_cnt == 0 {
        (*entry).flags &= !(1 << 0 | 1 << 1);
    }
}
