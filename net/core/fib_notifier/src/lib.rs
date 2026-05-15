//! FIB Notifier Module for Linux Kernel
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

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const EBUSY: c_int = -16;
pub const EEXIST: c_int = -17;

// Type definitions
#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct atomic_notifier_head {
    _private: [u8; 0],
}

#[repr(C)]
struct fib_notifier_net {
    fib_notifier_ops: list_head,
    fib_chain: atomic_notifier_head,
}

#[repr(C)]
struct fib_notifier_ops {
    list: list_head,
    owner: *mut c_void, // struct module*
    family: c_int,
    fib_seq_read: extern "C" fn(net: *mut c_void) -> c_uint,
    fib_dump: extern "C" fn(net: *mut c_void, nb: *mut c_void, extack: *mut c_void) -> c_int,
}

// External functions from Linux kernel
extern "C" {
    fn kmalloc(size: size_t, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn try_module_get(owner: *mut c_void) -> c_int;
    fn module_put(owner: *mut c_void);
    fn rtnl_lock();
    fn rtnl_unlock();
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn list_add_tail_rcu(new_entry: *mut list_head, head: *mut list_head);
    fn list_del_rcu(entry: *mut list_head);
    fn kmemdup(src: *const c_void, size: size_t, flags: c_int) -> *mut c_void;
    fn atomic_notifier_chain_register(
        head: *mut atomic_notifier_head,
        new: *mut c_void
    ) -> c_int;
    fn atomic_notifier_chain_unregister(
        head: *mut atomic_notifier_head,
        old: *mut c_void
    ) -> c_int;
    fn atomic_notifier_call_chain(
        head: *mut atomic_notifier_head,
        val: c_int,
        v: *mut c_void
    ) -> c_int;
    fn INIT_LIST_HEAD(head: *mut list_head);
    fn ATOMIC_INIT_NOTIFIER_HEAD(head: *mut atomic_notifier_head);
    fn WARN_ON_ONCE(condition: c_int);
    fn register_pernet_subsys(ops: *mut pernet_operations) -> c_int;
}

type size_t = usize;

#[repr(C)]
struct pernet_operations {
    init: extern "C" fn(net: *mut c_void) -> c_int,
    exit: extern "C" fn(net: *mut c_void),
    id: *mut c_ulong,
    size: size_t,
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn call_fib_notifier(
    nb: *mut c_void,
    event_type: c_int,
    info: *mut c_void,
) -> c_int {
    let err = {
        let call = (*nb).cast::<extern "C" fn(
            *mut c_void,
            c_int,
            *mut c_void,
        ) -> c_int>();
        call(nb, event_type, info)
    };
    notifier_to_errno(err)
}

#[no_mangle]
pub unsafe extern "C" fn call_fib_notifiers(
    net: *mut c_void,
    event_type: c_int,
    info: *mut c_void,
) -> c_int {
    let fib_notifier_net_id: c_ulong = 0; // Placeholder
    let fn_net = net_generic(net, fib_notifier_net_id);
    let err = atomic_notifier_call_chain(&mut (*fn_net).fib_chain, event_type, info);
    notifier_to_errno(err)
}

#[no_mangle]
pub unsafe extern "C" fn register_fib_notifier(
    net: *mut c_void,
    nb: *mut c_void,
    cb: extern "C" fn(*mut c_void),
    extack: *mut c_void,
) -> c_int {
    const FIB_DUMP_MAX_RETRIES: c_int = 5;
    let mut retries: c_int = 0;
    let mut err: c_int = 0;

    loop {
        let fib_seq = fib_seq_sum(net);
        err = fib_net_dump(net, nb, extack);
        if err != 0 {
            return err;
        }

        if fib_dump_is_consistent(net, nb, cb, fib_seq) {
            return 0;
        }

        if retries + 1 >= FIB_DUMP_MAX_RETRIES {
            break;
        }
        retries += 1;
    }

    -EBUSY
}

#[no_mangle]
pub unsafe extern "C" fn unregister_fib_notifier(
    net: *mut c_void,
    nb: *mut c_void,
) -> c_int {
    let fib_notifier_net_id: c_ulong = 0; // Placeholder
    let fn_net = net_generic(net, fib_notifier_net_id);
    atomic_notifier_chain_unregister(&mut (*fn_net).fib_chain, nb)
}

#[no_mangle]
pub unsafe extern "C" fn fib_notifier_ops_register(
    tmpl: *const fib_notifier_ops,
    net: *mut c_void,
) -> *mut fib_notifier_ops {
    let ops = kmemdup(tmpl, core::mem::size_of::<fib_notifier_ops>(), 0);
    if ops.is_null() {
        return ptr::addr_of_mut!(-ENOMEM).cast();
    }

    let err = __fib_notifier_ops_register(ops, net);
    if err != 0 {
        kfree(ops);
        return ptr::addr_of_mut!(err).cast();
    }

    ops
}

#[no_mangle]
pub unsafe extern "C" fn fib_notifier_ops_unregister(
    ops: *mut fib_notifier_ops,
) {
    list_del_rcu(&mut (*ops).list);
    kfree(ops);
}

unsafe extern "C" fn fib_notifier_net_init(
    net: *mut c_void,
) -> c_int {
    let fib_notifier_net_id: c_ulong = 0; // Placeholder
    let fn_net = net_generic(net, fib_notifier_net_id);
    INIT_LIST_HEAD(&mut (*fn_net).fib_notifier_ops);
    ATOMIC_INIT_NOTIFIER_HEAD(&mut (*fn_net).fib_chain);
    0
}

unsafe extern "C" fn fib_notifier_net_exit(
    net: *mut c_void,
) {
    let fib_notifier_net_id: c_ulong = 0; // Placeholder
    let fn_net = net_generic(net, fib_notifier_net_id);
    WARN_ON_ONCE(!list_empty(&(*fn_net).fib_notifier_ops));
}

unsafe extern "C" fn fib_notifier_init() -> c_int {
    let ops = &mut pernet_operations {
        init: fib_notifier_net_init,
        exit: fib_notifier_net_exit,
        id: &mut 0,
        size: core::mem::size_of::<fib_notifier_net>() as size_t,
    };
    register_pernet_subsys(ops)
}

// Helper functions
unsafe fn net_generic(net: *mut c_void, id: c_ulong) -> *mut fib_notifier_net {
    // Simplified implementation - actual implementation would use pernet data
    let offset = id as usize * core::mem::size_of::<fib_notifier_net>();
    (net as *mut u8).add(offset).cast()
}

unsafe fn __fib_notifier_ops_register(
    ops: *mut fib_notifier_ops,
    net: *mut c_void,
) -> c_int {
    let fib_notifier_net_id: c_ulong = 0; // Placeholder
    let fn_net = net_generic(net, fib_notifier_net_id);
    let mut pos = (*fn_net).fib_notifier_ops.next;
    
    while pos != &mut (*fn_net).fib_notifier_ops as *mut _ as *mut list_head {
        let entry = container_of(pos, ops, list);
        if (*entry).family == (*ops).family {
            return -EEXIST;
        }
        pos = (*pos).next;
    }
    
    list_add_tail_rcu(ops, &mut (*fn_net).fib_notifier_ops);
    0
}

unsafe fn fib_seq_sum(net: *mut c_void) -> c_uint {
    let fib_notifier_net_id: c_ulong = 0; // Placeholder
    let fn_net = net_generic(net, fib_notifier_net_id);
    let mut fib_seq: c_uint = 0;
    
    rtnl_lock();
    rcu_read_lock();
    
    let mut pos = (*fn_net).fib_notifier_ops.next;
    while pos != &mut (*fn_net).fib_notifier_ops as *mut _ as *mut list_head {
        let ops = container_of(pos, ops, list);
        if try_module_get((*ops).owner) == 0 {
            pos = (*pos).next;
            continue;
        }
        
        fib_seq += ((*ops).fib_seq_read)(net);
        module_put((*ops).owner);
        
        pos = (*pos).next;
    }
    
    rcu_read_unlock();
    rtnl_unlock();
    
    fib_seq
}

unsafe fn fib_net_dump(
    net: *mut c_void,
    nb: *mut c_void,
    extack: *mut c_void,
) -> c_int {
    let fib_notifier_net_id: c_ulong = 0; // Placeholder
    let fn_net = net_generic(net, fib_notifier_net_id);
    let mut err: c_int = 0;
    
    rcu_read_lock();
    
    let mut pos = (*fn_net).fib_notifier_ops.next;
    while pos != &mut (*fn_net).fib_notifier_ops as *mut _ as *mut list_head {
        let ops = container_of(pos, ops, list);
        if try_module_get((*ops).owner) == 0 {
            pos = (*pos).next;
            continue;
        }
        
        err = ((*ops).fib_dump)(net, nb, extack);
        module_put((*ops).owner);
        
        if err != 0 {
            break;
        }
        
        pos = (*pos).next;
    }
    
    rcu_read_unlock();
    
    err
}

unsafe fn fib_dump_is_consistent(
    net: *mut c_void,
    nb: *mut c_void,
    cb: extern "C" fn(*mut c_void),
    fib_seq: c_uint,
) -> bool {
    let fib_notifier_net_id: c_ulong = 0; // Placeholder
    let fn_net = net_generic(net, fib_notifier_net_id);
    
    if atomic_notifier_chain_register(&mut (*fn_net).fib_chain, nb) != 0 {
        return false;
    }
    
    if fib_seq == fib_seq_sum(net) {
        true
    } else {
        atomic_notifier_chain_unregister(&mut (*fn_net).fib_chain, nb);
        if !cb.is_null() {
            cb(nb);
        }
        false
    }
}

fn container_of(ptr: *mut list_head, container_type: *mut c_void, member: *mut list_head) -> *mut c_void {
    // SAFETY: This is a direct translation of the C container_of macro
    // Assumes that the member is a field in the container_type struct
    // and that the pointer is properly aligned.
    let offset = (member as *mut u8).offset_from(container_type as *mut u8);
    (ptr as *mut u8).offset(-offset) as *mut c_void
}

fn notifier_to_errno(err: c_int) -> c_int {
    // In real implementation, this would convert the notifier return code
    // to the appropriate errno value. For simplicity, we just return it.
    err
}

// Module initialization
#[no_mangle]
pub unsafe extern "C" fn init_module() -> c_int {
    fib_notifier_init()
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_module() {
    // Implementation would be needed for module cleanup
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_container_of() {
        // Simple test to verify container_of logic
        let mut obj = fib_notifier_ops {
            list: list_head { next: ptr::null_mut(), prev: ptr::null_mut() },
            owner: ptr::null_mut(),
            family: 0,
            fib_seq_read: |_| 0,
            fib_dump: |_, _, _| 0,
        };
        
        let ptr = &mut obj.list as *mut list_head;
        let container = container_of(ptr, &mut obj as *mut _ as *mut c_void, &mut obj.list as *mut _);
        assert_eq!(container as *mut fib_notifier_ops, &mut obj as *mut _);
    }
}
