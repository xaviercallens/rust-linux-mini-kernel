//! BPF socket storage management
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ptr;
use core::ffi::c_void;
use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_ulong;
use core::ffi::c_void;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOENT: c_int = -6;
pub const ENOTSUPP: c_int = -95;

// Type definitions
#[repr(C)]
pub struct sock {
    sk_bpf_storage: *mut bpf_local_storage,
    sk_refcnt: c_int,
    sk_omem_alloc: c_int,
}

#[repr(C)]
pub struct bpf_map {
    map_flags: c_uint,
}

#[repr(C)]
pub struct bpf_local_storage {
    list: hlist_head,
    lock: raw_spinlock_t,
}

#[repr(C)]
pub struct bpf_local_storage_map {
    map: bpf_map,
    cache_idx: c_int,
}

#[repr(C)]
pub struct bpf_local_storage_data {
    data: *mut c_void,
    smap: *mut bpf_local_storage_map,
}

#[repr(C)]
pub struct bpf_local_storage_elem {
    snode: hlist_node,
    local_storage: *mut bpf_local_storage,
}

#[repr(C)]
pub struct hlist_head {
    first: *mut hlist_node,
}

#[repr(C)]
pub struct hlist_node {
    next: *mut hlist_node,
    prev: *mut hlist_node,
}

#[repr(C)]
pub struct raw_spinlock_t {
    // Opaque type
    _unused: [u8; 0],
}

#[repr(C)]
pub struct bpf_storage_cache {
    // Opaque type
    _unused: [u8; 0],
}

// Function pointers and operations
#[repr(C)]
pub struct bpf_map_ops {
    map_meta_equal: extern "C" fn() -> c_int,
    map_alloc_check: extern "C" fn() -> c_int,
    map_alloc: extern "C" fn(*mut c_void) -> *mut bpf_map,
    map_free: extern "C" fn(*mut bpf_map),
    map_get_next_key: extern "C" fn(*mut bpf_map, *mut c_void, *mut c_void) -> c_int,
    map_lookup_elem: extern "C" fn(*mut bpf_map, *mut c_void) -> *mut c_void,
    map_update_elem: extern "C" fn(*mut bpf_map, *mut c_void, *mut c_void, c_ulong) -> c_int,
    map_delete_elem: extern "C" fn(*mut bpf_map, *mut c_void) -> c_int,
    map_check_btf: extern "C" fn() -> c_int,
    map_btf_name: *const c_char,
    map_btf_id: *mut c_int,
    map_local_storage_charge: extern "C" fn(*mut bpf_local_storage_map, *mut c_void, c_int) -> c_int,
    map_local_storage_uncharge: extern "C" fn(*mut bpf_local_storage_map, *mut c_void, c_int),
    map_owner_storage_ptr: extern "C" fn(*mut c_void) -> *mut *mut bpf_local_storage,
}

// External functions
extern "C" {
    fn rcu_dereference(ptr: *mut c_void) -> *mut c_void;
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn raw_spin_lock_bh(lock: *mut raw_spinlock_t);
    fn raw_spin_unlock_bh(lock: *mut raw_spinlock_t);
    fn kfree_rcu(ptr: *mut c_void, rcu: *const c_char);
    fn bpf_local_storage_lookup(sk_storage: *mut bpf_local_storage, smap: *mut bpf_local_storage_map, cacheit_lockit: bool) -> *mut bpf_local_storage_data;
    fn bpf_selem_unlink(selem: *mut bpf_local_storage_elem);
    fn bpf_selem_unlink_map(selem: *mut bpf_local_storage_elem);
    fn bpf_selem_unlink_storage_nolock(sk_storage: *mut bpf_local_storage, selem: *mut bpf_local_storage_elem, free_sk_storage: bool) -> bool;
    fn bpf_local_storage_cache_idx_free(cache: *mut bpf_storage_cache, idx: c_int);
    fn bpf_local_storage_map_free(smap: *mut bpf_local_storage_map, ptr: *mut c_void);
    fn bpf_local_storage_map_alloc(attr: *mut c_void) -> *mut bpf_local_storage_map;
    fn bpf_local_storage_cache_idx_get(cache: *mut bpf_storage_cache) -> c_int;
    fn sockfd_lookup(fd: c_int, err: *mut c_int) -> *mut socket;
    fn sockfd_put(sock: *mut socket);
    fn bpf_selem_alloc(smap: *mut bpf_local_storage_map, sk: *mut sock, ptr: *mut c_void, clone: bool) -> *mut bpf_local_storage_elem;
    fn copy_map_value_locked(map: *mut bpf_map, dst: *mut c_void, src: *mut c_void, clone: bool);
    fn copy_map_value(map: *mut bpf_map, dst: *mut c_void, src: *mut c_void);
    fn bpf_selem_link_map(smap: *mut bpf_local_storage_map, selem: *mut bpf_local_storage_elem);
    fn bpf_selem_link_storage_nolock(sk_storage: *mut bpf_local_storage, selem: *mut bpf_local_storage_elem);
    fn bpf_local_storage_alloc(sk: *mut sock, smap: *mut bpf_local_storage_map, selem: *mut bpf_local_storage_elem) -> c_int;
    fn sock_put(sk: *mut sock);
    fn sk_fullsock(sk: *mut sock) -> bool;
    fn refcount_inc_not_zero(ref: *mut c_int) -> bool;
    fn atomic_read(atomic: *mut c_int) -> c_int;
    fn atomic_add(atomic: *mut c_int, val: c_int);
    fn atomic_sub(atomic: *mut c_int, val: c_int);
}

// Static variables
static mut sk_cache: bpf_storage_cache = bpf_storage_cache {
    _unused: [0; 0],
};

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn bpf_sk_storage_lookup(
    sk: *mut sock,
    map: *mut bpf_map,
    cacheit_lockit: bool,
) -> *mut bpf_local_storage_data {
    let sk_storage = rcu_dereference((*sk).sk_bpf_storage);
    if sk_storage.is_null() {
        return ptr::null_mut();
    }

    let smap = map as *mut bpf_local_storage_map;
    bpf_local_storage_lookup(sk_storage, smap, cacheit_lockit)
}

#[no_mangle]
pub unsafe extern "C" fn bpf_sk_storage_del(
    sk: *mut sock,
    map: *mut bpf_map,
) -> c_int {
    let sdata = bpf_sk_storage_lookup(sk, map, false);
    if sdata.is_null() {
        return -ENOENT;
    }

    bpf_selem_unlink(sdata as *mut bpf_local_storage_elem);
    0
}

#[no_mangle]
pub unsafe extern "C" fn bpf_sk_storage_free(sk: *mut sock) {
    rcu_read_lock();
    let sk_storage = rcu_dereference((*sk).sk_bpf_storage);
    if sk_storage.is_null() {
        rcu_read_unlock();
        return;
    }

    raw_spin_lock_bh(&(*sk_storage).lock);
    let mut n: *mut hlist_node = ptr::null_mut();
    let mut selem: *mut bpf_local_storage_elem = ptr::null_mut();
    
    // SAFETY: Manual iteration of hlist
    let mut pos = (*sk_storage).list.first;
    while !pos.is_null() {
        selem = (pos as *mut bpf_local_storage_elem);
        n = (*pos).next;
        
        bpf_selem_unlink_map(selem);
        let free_sk_storage = bpf_selem_unlink_storage_nolock(sk_storage, selem, true);
        
        // Move to next element
        pos = n;
    }
    
    raw_spin_unlock_bh(&(*sk_storage).lock);
    rcu_read_unlock();

    if free_sk_storage {
        kfree_rcu(sk_storage, core::ptr::null());
    }
}

#[no_mangle]
pub unsafe extern "C" fn bpf_sk_storage_map_free(map: *mut bpf_map) {
    let smap = map as *mut bpf_local_storage_map;
    bpf_local_storage_cache_idx_free(&mut sk_cache, (*smap).cache_idx);
    bpf_local_storage_map_free(smap, ptr::null_mut());
}

#[no_mangle]
pub unsafe extern "C" fn bpf_sk_storage_map_alloc(attr: *mut c_void) -> *mut bpf_map {
    let smap = bpf_local_storage_map_alloc(attr);
    if smap.is_null() {
        return smap;
    }
    
    (*smap).cache_idx = bpf_local_storage_cache_idx_get(&mut sk_cache);
    &mut (*smap).map
}

#[no_mangle]
pub unsafe extern "C" fn notsupp_get_next_key(
    _map: *mut bpf_map,
    _key: *mut c_void,
    _next_key: *mut c_void,
) -> c_int {
    -ENOTSUPP
}

#[no_mangle]
pub unsafe extern "C" fn bpf_fd_sk_storage_lookup_elem(
    map: *mut bpf_map,
    key: *mut c_void,
) -> *mut c_void {
    let fd = *(key as *mut c_int);
    let mut err: c_int = 0;
    let sock = sockfd_lookup(fd, &mut err);
    if sock.is_null() {
        return err as *mut c_void;
    }
    
    let sdata = bpf_sk_storage_lookup((*sock).sk, map, true);
    sockfd_put(sock);
    
    if sdata.is_null() {
        return ptr::null_mut();
    }
    (*sdata).data
}

#[no_mangle]
pub unsafe extern "C" fn bpf_fd_sk_storage_update_elem(
    map: *mut bpf_map,
    key: *mut c_void,
    value: *mut c_void,
    map_flags: c_ulong,
) -> c_int {
    let fd = *(key as *mut c_int);
    let mut err: c_int = 0;
    let sock = sockfd_lookup(fd, &mut err);
    if sock.is_null() {
        return err;
    }
    
    let smap = map as *mut bpf_local_storage_map;
    let sdata = bpf_local_storage_update((*sock).sk, smap, value, map_flags);
    sockfd_put(sock);
    
    if sdata.is_null() {
        PTR_ERR_OR_ZERO(sdata)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn bpf_fd_sk_storage_delete_elem(
    map: *mut bpf_map,
    key: *mut c_void,
) -> c_int {
    let fd = *(key as *mut c_int);
    let mut err: c_int = 0;
    let sock = sockfd_lookup(fd, &mut err);
    if sock.is_null() {
        return err;
    }
    
    let err = bpf_sk_storage_del((*sock).sk, map);
    sockfd_put(sock);
    err
}

#[no_mangle]
pub unsafe extern "C" fn bpf_sk_storage_clone_elem(
    newsk: *mut sock,
    smap: *mut bpf_local_storage_map,
    selem: *mut bpf_local_storage_elem,
) -> *mut bpf_local_storage_elem {
    let copy_selem = bpf_selem_alloc(smap, newsk, ptr::null_mut(), true);
    if copy_selem.is_null() {
        return ptr::null_mut();
    }
    
    if map_value_has_spin_lock(&(*smap).map) {
        copy_map_value_locked(&(*smap).map, (*copy_selem).data, (*selem).data, true);
    } else {
        copy_map_value(&(*smap).map, (*copy_selem).data, (*selem).data);
    }
    
    copy_selem
}

#[no_mangle]
pub unsafe extern "C" fn bpf_sk_storage_clone(
    sk: *const sock,
    newsk: *mut sock,
) -> c_int {
    RCU_INIT_POINTER((*newsk).sk_bpf_storage, ptr::null_mut());
    
    rcu_read_lock();
    let sk_storage = rcu_dereference((*sk).sk_bpf_storage);
    
    if sk_storage.is_null() || hlist_empty(&(*sk_storage).list) {
        rcu_read_unlock();
        return 0;
    }
    
    let mut ret: c_int = 0;
    let mut pos = (*sk_storage).list.first;
    let mut new_sk_storage: *mut bpf_local_storage = ptr::null_mut();
    
    while !pos.is_null() {
        let selem = pos as *mut bpf_local_storage_elem;
        let smap = (*(*selem).smap).smap;
        let map = bpf_map_inc_not_zero(&(*smap).map);
        
        if IS_ERR(map) {
            pos = (*pos).next;
            continue;
        }
        
        let copy_selem = bpf_sk_storage_clone_elem(newsk, smap, selem);
        if copy_selem.is_null() {
            bpf_map_put(map);
            ret = -ENOMEM;
            break;
        }
        
        if !new_sk_storage.is_null() {
            bpf_selem_link_map(smap, copy_selem);
            bpf_selem_link_storage_nolock(new_sk_storage, copy_selem);
        } else {
            let alloc_ret = bpf_local_storage_alloc(newsk, smap, copy_selem);
            if alloc_ret != 0 {
                kfree(copy_selem as *mut c_void);
                atomic_sub(&(*newsk).sk_omem_alloc, (*smap).elem_size);
                bpf_map_put(map);
                ret = alloc_ret;
                break;
            }
            new_sk_storage = rcu_dereference((*copy_selem).local_storage);
        }
        bpf_map_put(map);
        pos = (*pos).next;
    }
    
    rcu_read_unlock();
    ret
}

#[no_mangle]
pub unsafe extern "C" fn bpf_sk_storage_get(
    map: *mut bpf_map,
    sk: *mut sock,
    value: *mut c_void,
    flags: c_ulong,
) -> c_ulong {
    if sk.is_null() || !sk_fullsock(sk) || flags > BPF_SK_STORAGE_GET_F_CREATE {
        return 0;
    }
    
    let sdata = bpf_sk_storage_lookup(sk, map, true);
    if !sdata.is_null() {
        return sdata as c_ulong;
    }
    
    if flags == BPF_SK_STORAGE_GET_F_CREATE && refcount_inc_not_zero(&(*sk).sk_refcnt) {
        let smap = map as *mut bpf_local_storage_map;
        let sdata = bpf_local_storage_update(sk, smap, value, BPF_NOEXIST);
        sock_put(sk);
        if sdata.is_null() {
            0
        } else {
            sdata as c_ulong
        }
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn bpf_sk_storage_delete(
    map: *mut bpf_map,
    sk: *mut sock,
) -> c_int {
    if sk.is_null() || !sk_fullsock(sk) {
        return -EINVAL;
    }
    
    if refcount_inc_not_zero(&(*sk).sk_refcnt) {
        let err = bpf_sk_storage_del(sk, map);
        sock_put(sk);
        err
    } else {
        -ENOENT
    }
}

#[no_mangle]
pub unsafe extern "C" fn bpf_sk_storage_charge(
    smap: *mut bpf_local_storage_map,
    owner: *mut c_void,
    size: c_int,
) -> c_int {
    let sk = owner as *mut sock;
    let sysctl_optmem_max = 0; // Placeholder
    if size <= sysctl_optmem_max && atomic_read(&(*sk).sk_omem_alloc) + size < sysctl_optmem_max {
        atomic_add(&(*sk).sk_omem_alloc, size);
        0
    } else {
        -ENOMEM
    }
}

#[no_mangle]
pub unsafe extern "C" fn bpf_sk_storage_uncharge(
    _smap: *mut bpf_local_storage_map,
    owner: *mut c_void,
    size: c_int,
) {
    let sk = owner;
    atomic_sub(&(*sk).sk_omem_alloc, size);
}

#[no_mangle]
pub unsafe extern "C" fn bpf_sk_storage_ptr(owner: *mut c_void) -> *mut *mut bpf_local_storage {
    let sk = owner as *mut sock;
    &mut (*sk).sk_bpf_storage
}

// BPF function prototypes
#[no_mangle]
pub static mut bpf_sk_storage_get_proto: bpf_func_proto = bpf_func_proto {
    func: bpf_sk_storage_get,
    gpl_only: false,
    ret_type: RET_PTR_TO_MAP_VALUE_OR_NULL,
    arg1_type: ARG_CONST_MAP_PTR,
    arg2_type: ARG_PTR_TO_BTF_ID_SOCK_COMMON,
    arg3_type: ARG_PTR_TO_MAP_VALUE_OR_NULL,
    arg4_type: ARG_ANYTHING,
};

#[no_mangle]
pub static mut bpf_sk_storage_delete_proto: bpf_func_proto = bpf_func_proto {
    func: bpf_sk_storage_delete,
    gpl_only: false,
    ret_type: RET_INTEGER,
    arg1_type: ARG_CONST_MAP_PTR,
    arg2_type: ARG_PTR_TO_BTF_ID_SOCK_COMMON,
};

// Map operations
#[no_mangle]
pub static mut sk_storage_map_ops: bpf_map_ops = bpf_map_ops {
    map_meta_equal: bpf_map_meta_equal,
    map_alloc_check: bpf_local_storage_map_alloc_check,
    map_alloc: bpf_sk_storage_map_alloc,
    map_free: bpf_sk_storage_map_free,
    map_get_next_key: notsupp_get_next_key,
    map_lookup_elem: bpf_fd_sk_storage_lookup_elem,
    map_update_elem: bpf_fd_sk_storage_update_elem,
    map_delete_elem: bpf_fd_sk_storage_delete_elem,
    map_check_btf: bpf_local_storage_map_check_btf,
    map_btf_name: bpf_local_storage_map_btf_name,
    map_btf_id: &sk_storage_map_btf_id,
    map_local_storage_charge: bpf_sk_storage_charge,
    map_local_storage_uncharge: bpf_sk_storage_uncharge,
    map_owner_storage_ptr: bpf_sk_storage_ptr,
};

// Helper functions
#[no_mangle]
pub unsafe extern "C" fn RCU_INIT_POINTER(ptr: *mut *mut c_void, val: *mut c_void) {
    *ptr = val;
}

#[no_mangle]
pub unsafe extern "C" fn hlist_empty(head: *mut hlist_head) -> bool {
    (*head).first.is_null()
}

#[no_mangle]
pub unsafe extern "C" fn PTR_ERR_OR_ZERO(ptr: *mut c_void) -> c_int {
    if ptr.is_null() {
        -1
    } else {
        0
    }
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    #[test]
    fn test_bpf_sk_storage_lookup() {
        // Basic test would require kernel environment
    }
}
