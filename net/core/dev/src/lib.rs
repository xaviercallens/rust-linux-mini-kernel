#![no_std]
#![allow(non_camel_case_types)]

use core::ptr;
use core::ffi::c_int;
use core::ffi::c_void;
use core::ffi::c_char;
use core::ffi::size_t;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;

// Type definitions
#[repr(C)]
struct hlist_node {
    next: *mut hlist_node,
}

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct netdev_name_node {
    hlist: hlist_node,
    list: list_head,
    dev: *mut net_device,
    name: *mut c_char,
}

#[repr(C)]
struct net_device {
    name: *mut c_char,
    name_node: list_head,
    // ... other fields omitted for brevity
}

#[repr(C)]
struct net {
    dev_name_head: [*mut hlist_head; 1 << 16], // Assuming NETDEV_HASHBITS is 16
    // ... other fields omitted for brevity
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn netdev_name_node_alt_create(
    dev: *mut net_device,
    name: *const c_char,
) -> c_int {
    // SAFETY: Check for null pointers
    if dev.is_null() || name.is_null() {
        return -EINVAL;
    }

    let net = dev_net(dev);
    let existing = netdev_name_node_lookup(net, name);
    if !existing.is_null() {
        return -EEXIST;
    }

    let name_node = netdev_name_node_alloc(dev, name as *mut c_char);
    if name_node.is_null() {
        return -ENOMEM;
    }

    netdev_name_node_add(net, name_node);
    list_add_tail(&mut (*name_node).list, &mut (*dev).name_node);

    0
}

#[no_mangle]
pub unsafe extern "C" fn netdev_name_node_alt_destroy(
    name_node: *mut netdev_name_node,
) {
    if !name_node.is_null() {
        list_del(&mut (*name_node).list);
        netdev_name_node_del(name_node);
        libc::free(name_node as *mut c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn dev_add_pack(
    ptype: *mut packet_type,
) {
    // Implementation would go here
}

#[no_mangle]
pub unsafe extern "C" fn __dev_remove_pack(
    ptype: *mut packet_type,
) {
    // Implementation would go here
}

// Helper functions
#[no_mangle]
pub unsafe extern "C" fn dev_net(
    dev: *mut net_device,
) -> *mut net {
    // Simplified for example - actual implementation would retrieve the net namespace
    ptr::null_mut()
}

unsafe fn netdev_name_node_lookup(
    net: *mut net,
    name: *const c_char,
) -> *mut netdev_name_node {
    let head = dev_name_hash(net, name);
    let mut node = hlist_first_entry(head, netdev_name_node, hlist);
    while !node.is_null() {
        if strcmp((*node).name, name as *mut c_char) == 0 {
            return node;
        }
        node = hlist_next_entry(node, hlist);
    }
    ptr::null_mut()
}

unsafe fn netdev_name_node_alloc(
    dev: *mut net_device,
    name: *mut c_char,
) -> *mut netdev_name_node {
    let name_node = libc::malloc(core::mem::size_of::<netdev_name_node>()) as *mut netdev_name_node;
    if name_node.is_null() {
        return ptr::null_mut();
    }
    
    // SAFETY: Initialize hlist_node's next pointer to null
    (*name_node).hlist.next = ptr::null_mut();
    (*name_node).dev = dev;
    (*name_node).name = name;
    
    name_node
}

unsafe fn netdev_name_node_add(
    net: *mut net,
    name_node: *mut netdev_name_node,
) {
    let head = dev_name_hash(net, (*name_node).name);
    hlist_add_head_rcu(&mut (*name_node).hlist, head);
}

unsafe fn dev_name_hash(
    net: *mut net,
    name: *const c_char,
) -> *mut hlist_head {
    // Simplified hash calculation
    let hash = full_name_hash(name);
    let index = hash & (1 << 16) - 1; // Assuming NETDEV_HASHBITS is 16
    &(*net).dev_name_head[index]
}

unsafe fn full_name_hash(
    name: *const c_char,
) -> u32 {
    // Simplified hash function
    let mut hash: u32 = 0;
    let mut i = 0;
    while *name.offset(i) != 0 {
        hash = hash.wrapping_mul(31).wrapping_add(*name.offset(i) as u32);
        i += 1;
    }
    hash
}

unsafe fn hlist_add_head_rcu(
    node: *mut hlist_node,
    head: *mut hlist_head,
) {
    // Simplified RCU-safe addition
    (*node).next = (*head).next;
    (*head).next = node;
}

unsafe fn hlist_first_entry(
    head: *mut hlist_head,
    ty: *mut netdev_name_node,
    member: &'static str,
) -> *mut netdev_name_node {
    // Simplified container_of implementation
    head as *mut netdev_name_node
}

unsafe fn hlist_next_entry(
    node: *mut netdev_name_node,
    member: &'static str,
) -> *mut netdev_name_node {
    // Simplified next entry retrieval
    (*node).hlist.next as *mut netdev_name_node
}

unsafe fn list_add_tail(
    new: *mut list_head,
    head: *mut list_head,
) {
    // Simplified list_add_tail implementation
    let prev = (*head).prev;
    (*new).prev = prev;
    (*new).next = head;
    (*prev).next = new;
    (*head).prev = new;
}

unsafe fn list_del(
    entry: *mut list_head,
) {
    // Simplified list_del implementation
    let next = (*entry).next;
    let prev = (*entry).prev;
    (*next).prev = prev;
    (*prev).next = next;
}

unsafe fn netdev_name_node_del(
    name_node: *mut netdev_name_node,
) {
    // Simplified hlist_del_rcu implementation
    let node = &mut (*name_node).hlist;
    let next = (*node).next;
    let prev = (*node).next; // Simplified
    if !prev.is_null() {
        (*prev).next = next;
    }
}

// External functions
extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}
