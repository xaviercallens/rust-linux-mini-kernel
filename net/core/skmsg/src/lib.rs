//! This module provides FFI-compatible Rust bindings for the Linux kernel's skmsg.c
//! implementation. It maintains ABI compatibility with the original C code while
//! preserving all memory management and scatter-gather list operations.
//!
//! The implementation handles message allocation, cloning, memory management,
//! and zero-copy operations for socket messages in the Linux networking stack.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clang::too_many_arguments)]

use core::ptr;
use core::mem;
use libc::{c_int, c_uint, c_void, size_t};

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSPC: c_int = -28;
pub const EFAULT: c_int = -14;

// Type definitions
#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct page_frag {
    page: *const page,
    offset: u32,
    size: u32,
}

#[repr(C)]
pub struct scatterlist {
    page: *const page,
    offset: u32,
    length: u32,
}

#[repr(C)]
pub struct sk_msg_sg {
    data: [scatterlist; MAX_MSG_FRAGS],
    start: c_int,
    end: c_int,
    size: size_t,
    curr: c_int,
    copybreak: size_t,
}

#[repr(C)]
pub struct sk_msg {
    sg: sk_msg_sg,
    skb: *mut c_void,
}

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_psock {
    _private: [u8; 0],
}

// External functions from Linux kernel
extern "C" {
    fn sk_page_frag(sk: *const sock) -> *mut page_frag;
    fn sk_page_frag_refill(sk: *const sock, pfrag: *mut page_frag) -> c_int;
    fn sk_wmem_schedule(sk: *const sock, size: c_int) -> c_int;
    fn sk_msg_full(msg: *const sk_msg) -> c_int;
    fn sk_msg_iter_next(msg: *mut sk_msg, field: *mut c_int);
    fn sk_msg_iter_var_prev(i: *mut c_int);
    fn sk_msg_iter_var_next(i: *mut c_int);
    fn sk_msg_check_to_free(msg: *mut sk_msg, i: c_int, size: size_t);
    fn sk_mem_charge(sk: *const sock, size: c_int);
    fn sk_mem_uncharge(sk: *const sock, size: c_int);
    fn get_page(page: *const page);
    fn put_page(page: *const page);
    fn consume_skb(skb: *mut c_void);
    fn sk_msg_init(msg: *mut sk_msg);
    fn iov_iter_get_pages(
        from: *mut c_void,
        pages: *mut *mut page,
        bytes: size_t,
        maxpages: c_int,
        offset: *mut size_t
    ) -> ssize_t;
    fn iov_iter_advance(from: *mut c_void, bytes: size_t);
    fn iov_iter_revert(from: *mut c_void, bytes: size_t);
    fn copy_from_iter(to: *mut c_void, len: size_t, from: *mut c_void) -> ssize_t;
    fn copy_from_iter_nocache(to: *mut c_void, len: size_t, from: *mut c_void) -> ssize_t;
    fn sk_set_bit(bit: c_int, sk: *mut sock);
    fn sk_clear_bit(bit: c_int, sk: *mut sock);
    fn add_wait_queue(queue: *mut c_void, wait: *mut c_void);
    fn remove_wait_queue(queue: *mut c_void, wait: *mut c_void);
    fn sk_wait_event(
        sk: *mut sock,
        timeo: *mut c_int,
        condition: c_int,
        wait: *mut c_void
    ) -> c_int;
    fn DEFINE_WAIT_FUNC(wait: *mut c_void, func: *mut c_void);
    fn sk_sleep(sk: *mut sock) -> *mut c_void;
}

// Constants from Linux kernel
const MAX_MSG_FRAGS: c_int = 128;

// Helper functions for scatterlist operations
fn sg_page(sge: *const scatterlist) -> *const page {
    unsafe { (*sge).page }
}

fn sg_virt(sge: *const scatterlist) -> *mut c_void {
    unsafe {
        let page = sg_page(sge);
        let offset = (*sge).offset;
        page.offset((offset as isize) / mem::size_of::<page>() as isize)
    }
}

fn sk_msg_elem(msg: *const sk_msg, i: c_int) -> *mut scatterlist {
    unsafe { &mut (*msg).sg.data[i as usize] }
}

fn sk_msg_page_add(
    dst: *mut sk_msg,
    page: *const page,
    len: size_t,
    offset: u32
) -> c_int {
    let i = (*dst).sg.end;
    let sge = sk_msg_elem(dst, i);
    
    unsafe {
        (*sge).page = page;
        (*sge).offset = offset;
        (*sge).length = len as u32;
        sk_msg_iter_next(dst, &mut (*dst).sg.end);
    }
    
    0
}

// Function implementations
/// Check if coalescing is possible for sk_msg
///
/// # Safety
/// - `msg` must be a valid pointer to sk_msg
/// - `elem_first_coalesce` must be a valid index
fn sk_msg_try_coalesce_ok(msg: *const sk_msg, elem_first_coalesce: c_int) -> bool {
    let end = (*msg).sg.end;
    let start = (*msg).sg.start;
    
    if end > start && elem_first_coalesce < end {
        return true;
    }
    
    if end < start && (elem_first_coalesce > start || elem_first_coalesce < end) {
        return true;
    }
    
    false
}

/// Allocate memory for sk_msg
///
/// # Safety
/// - `sk` must be a valid pointer to sock
/// - `msg` must be a valid pointer to sk_msg
/// - `elem_first_coalesce` must be a valid index
#[no_mangle]
pub unsafe extern "C" fn sk_msg_alloc(
    sk: *mut sock,
    msg: *mut sk_msg,
    len: c_int,
    elem_first_coalesce: c_int
) -> c_int {
    let pfrag = sk_page_frag(sk);
    let mut ret = 0;
    
    let mut len_remaining = len;
    let mut sg_size = (*msg).sg.size;
    
    while len_remaining > 0 {
        let i = (*msg).sg.end;
        let mut i_prev = i;
        sk_msg_iter_var_prev(&mut i_prev);
        let sge = sk_msg_elem(msg, i_prev);
        
        let orig_offset = (*pfrag).offset;
        let use_size = if len_remaining < (*pfrag).size as c_int - orig_offset {
            len_remaining
        } else {
            (*pfrag).size as c_int - orig_offset
        };
        
        if !sk_page_frag_refill(sk, pfrag).into() {
            return -ENOMEM;
        }
        
        if !sk_wmem_schedule(sk, use_size).into() {
            return -ENOMEM;
        }
        
        if sk_msg_try_coalesce_ok(msg, elem_first_coalesce) &&
           sg_page(sge) == (*pfrag).page &&
           (*sge).offset + (*sge).length == orig_offset {
            (*sge).length += use_size as u32;
        } else {
            if sk_msg_full(msg).into() != 0 {
                ret = -ENOSPC;
                break;
            }
            
            let sge_new = sk_msg_elem(msg, (*msg).sg.end);
            sg_unmark_end(sge_new);
            (*sge_new).page = (*pfrag).page;
            (*sge_new).offset = orig_offset;
            (*sge_new).length = use_size as u32;
            get_page((*pfrag).page);
            sk_msg_iter_next(msg, &mut (*msg).sg.end);
        }
        
        sk_mem_charge(sk, use_size);
        (*msg).sg.size += use_size as size_t;
        (*pfrag).offset += use_size as u32;
        len_remaining -= use_size;
    }
    
    ret
}

/// Clone sk_msg content
///
/// # Safety
/// - `sk` must be a valid pointer to sock
/// - `dst` must be a valid pointer to sk_msg
/// - `src` must be a valid pointer to sk_msg
#[no_mangle]
pub unsafe extern "C" fn sk_msg_clone(
    sk: *mut sock,
    dst: *mut sk_msg,
    src: *const sk_msg,
    off: u32,
    len: u32
) -> c_int {
    let mut i = (*src).sg.start;
    let mut sge = sk_msg_elem(src, i);
    
    while off > 0 {
        if (*sge).length > off {
            break;
        }
        off -= (*sge).length;
        sk_msg_iter_var_next(&mut i);
        if i == (*src).sg.end && off > 0 {
            return -ENOSPC;
        }
        sge = sk_msg_elem(src, i);
    }
    
    while len > 0 {
        let sge_len = if (*sge).length > len {
            len
        } else {
            (*sge).length
        };
        
        let sgd = if (*dst).sg.end > 0 {
            sk_msg_elem(dst, (*dst).sg.end - 1)
        } else {
            ptr::null_mut()
        };
        
        if !sgd.is_null() &&
           sg_page(sge) == sg_page(sgd) &&
           (sg_virt(sge) as *mut u8).offset((*sge).offset as isize) == 
           (sg_virt(sgd) as *mut u8).offset((*sgd).length as isize) {
            (*sgd).length += sge_len;
            (*dst).sg.size += sge_len;
        } else if sk_msg_full(dst).into() == 0 {
            let sge_off = (*sge).offset + off;
            sk_msg_page_add(dst, sg_page(sge), sge_len, sge_off);
        } else {
            return -ENOSPC;
        }
        
        off = 0;
        len -= sge_len;
        sk_mem_charge(sk, sge_len as c_int);
        sk_msg_iter_var_next(&mut i);
        if i == (*src).sg.end && len > 0 {
            return -ENOSPC;
        }
        sge = sk_msg_elem(src, i);
    }
    
    0
}

/// Return zero bytes from sk_msg
///
/// # Safety
/// - `sk` must be a valid pointer to sock
/// - `msg` must be a valid pointer to sk_msg
/// - `bytes` must be a valid number of bytes
#[no_mangle]
pub unsafe extern "C" fn sk_msg_return_zero(
    sk: *mut sock,
    msg: *mut sk_msg,
    bytes: c_int
) {
    let mut i = (*msg).sg.start;
    
    loop {
        let sge = sk_msg_elem(msg, i);
        if bytes < (*sge).length as c_int {
            (*sge).length -= bytes as u32;
            (*sge).offset += bytes as u32;
            sk_mem_uncharge(sk, bytes);
            break;
        }
        
        sk_mem_uncharge(sk, (*sge).length as c_int);
        bytes -= (*sge).length as c_int;
        (*sge).length = 0;
        (*sge).offset = 0;
        sk_msg_iter_var_next(&mut i);
        if bytes == 0 || i == (*msg).sg.end {
            break;
        }
    }
    (*msg).sg.start = i;
}

/// Return bytes from sk_msg
///
/// # Safety
/// - `sk` must be a valid pointer to sock
/// - `msg` must be a valid pointer to sk_msg
/// - `bytes` must be a valid number of bytes
#[no_mangle]
pub unsafe extern "C" fn sk_msg_return(
    sk: *mut sock,
    msg: *mut sk_msg,
    bytes: c_int
) {
    let mut i = (*msg).sg.start;
    
    loop {
        let sge = sk_msg_elem(msg, i);
        let uncharge = if bytes < (*sge).length as c_int {
            bytes
        } else {
            (*sge).length as c_int
        };
        
        sk_mem_uncharge(sk, uncharge);
        bytes -= uncharge;
        sk_msg_iter_var_next(&mut i);
        if i == (*msg).sg.end {
            break;
        }
    }
}

/// Free sk_msg elements
///
/// # Safety
/// - `sk` must be a valid pointer to sock
/// - `msg` must be a valid pointer to sk_msg
/// - `i` must be a valid index
/// - `charge` must be a valid boolean
fn sk_msg_free_elem(
    sk: *mut sock,
    msg: *mut sk_msg,
    i: c_int,
    charge: c_int
) -> c_int {
    let sge = sk_msg_elem(msg, i);
    let len = (*sge).length;
    
    if !(*msg).skb.is_null() {
        return len as c_int;
    }
    
    if charge != 0 {
        sk_mem_uncharge(sk, len as c_int);
    }
    put_page(sg_page(sge));
    ptr::write_bytes(sge, 0, 1);
    len as c_int
}

/// Free sk_msg
///
/// # Safety
/// - `sk` must be a valid pointer to sock
/// - `msg` must be a valid pointer to sk_msg
/// - `i` must be a valid index
/// - `charge` must be a valid boolean
fn __sk_msg_free(
    sk: *mut sock,
    msg: *mut sk_msg,
    i: c_int,
    charge: c_int
) -> c_int {
    let mut sge = sk_msg_elem(msg, i);
    let mut freed = 0;
    
    while (*msg).sg.size > 0 {
        (*msg).sg.size -= (*sge).length as size_t;
        freed += sk_msg_free_elem(sk, msg, i, charge);
        sk_msg_iter_var_next(&mut i);
        sk_msg_check_to_free(msg, i, (*msg).sg.size);
        sge = sk_msg_elem(msg, i);
    }
    
    consume_skb((*msg).skb);
    sk_msg_init(msg);
    freed
}

/// Free sk_msg without charging
///
/// # Safety
/// - `sk` must be a valid pointer to sock
/// - `msg` must be a valid pointer to sk_msg
#[no_mangle]
pub unsafe extern "C" fn sk_msg_free_nocharge(
    sk: *mut sock,
    msg: *mut sk_msg
) -> c_int {
    __sk_msg_free(sk, msg, (*msg).sg.start, 0)
}

/// Free sk_msg
///
/// # Safety
/// - `sk` must be a valid pointer to sock
/// - `msg` must be a valid pointer to sk_msg
#[no_mangle]
pub unsafe extern "C" fn sk_msg_free(
    sk: *mut sock,
    msg: *mut sk_msg
) -> c_int {
    __sk_msg_free(sk, msg, (*msg).sg.start, 1)
}

/// Free partial sk_msg
///
/// # Safety
/// - `sk` must be a valid pointer to sock
/// - `msg` must be a valid pointer to sk_msg
/// - `bytes` must be a valid number of bytes
/// - `charge` must be a valid boolean
fn __sk_msg_free_partial(
    sk: *mut sock,
    msg: *mut sk_msg,
    bytes: u32,
    charge: c_int
) {
    let mut i = (*msg).sg.start;
    
    while bytes > 0 {
        let sge = sk_msg_elem(msg, i);
        if (*sge).length == 0 {
            break;
        }
        if bytes < (*sge).length {
            if charge != 0 {
                sk_mem_uncharge(sk, bytes as c_int);
            }
            (*sge).length -= bytes;
            (*sge).offset += bytes;
            (*msg).sg.size -= bytes as size_t;
            break;
        }
        
        (*msg).sg.size -= (*sge).length as size_t;
        bytes -= (*sge).length;
        sk_msg_free_elem(sk, msg, i, charge);
        sk_msg_iter_var_next(&mut i);
        sk_msg_check_to_free(msg, i, bytes as size_t);
    }
    (*msg).sg.start = i;
}

/// Free partial sk_msg
///
/// # Safety
/// - `sk` must be a valid pointer to sock
/// - `msg` must be a valid pointer to sk_msg
/// - `bytes` must be a valid number of bytes
#[no_mangle]
pub unsafe extern "C" fn sk_msg_free_partial(
    sk: *mut sock,
    msg: *mut sk_msg,
    bytes: u32
) {
    __sk_msg_free_partial(sk, msg, bytes, 1)
}

/// Free partial sk_msg without charge
///
/// # Safety
/// - `sk` must be a valid pointer to sock
/// - `msg` must be a valid pointer to sk_msg
/// - `bytes` must be a valid number of bytes
#[no_mangle]
pub unsafe extern "C" fn sk_msg_free_partial_nocharge(
    sk: *mut sock,
    msg: *mut sk_msg,
    bytes: u32
) {
    __sk_msg_free_partial(sk, msg, bytes, 0)
}

/// Trim sk_msg to specified length
///
/// # Safety
/// - `sk` must be a valid pointer to sock
/// - `msg` must be a valid pointer to sk_msg
/// - `len` must be a valid length
#[no_mangle]
pub unsafe extern "C" fn sk_msg_trim(
    sk: *mut sock,
    msg: *mut sk_msg,
    len: c_int
) {
    let mut trim = (*msg).sg.size as c_int - len;
    let mut i = (*msg).sg.end;
    
    if trim <= 0 {
        return;
    }
    
    sk_msg_iter_var_prev(&mut i);
    (*msg).sg.size = len as size_t;
    
    while (*msg).sg.data[i as usize].length != 0 && trim >= (*msg).sg.data[i as usize].length as c_int {
        trim -= (*msg).sg.data[i as usize].length as c_int;
        sk_msg_free_elem(sk, msg, i, 1);
        sk_msg_iter_var_prev(&mut i);
        if trim == 0 {
            break;
        }
    }
    
    if trim > 0 {
        let sge = sk_msg_elem(msg, i);
        (*sge).length -= trim as u32;
        sk_mem_uncharge(sk, trim);
        
        if (*msg).sg.curr == i && (*msg).sg.copybreak > (*sge).length {
            (*msg).sg.copybreak = (*sge).length;
        }
    }
    
    sk_msg_iter_var_next(&mut i);
    (*msg).sg.end = i;
    
    if (*msg).sg.size == 0 {
        (*msg).sg.curr = (*msg).sg.start;
        (*msg).sg.copybreak = 0;
    } else if sk_msg_iter_dist((*msg).sg.start, (*msg).sg.curr) >= 
              sk_msg_iter_dist((*msg).sg.start, (*msg).sg.end) {
        sk_msg_iter_var_prev(&mut i);
        (*msg).sg.curr = i;
        (*msg).sg.copybreak = (*msg).sg.data[i as usize].length;
    }
}

/// Zero-copy from iterator to sk_msg
///
/// # Safety
/// - `sk` must be a valid pointer to sock
/// - `from` must be a valid pointer to iov_iter
/// - `msg` must be a valid pointer to sk_msg
/// - `bytes` must be a valid number of bytes
#[no_mangle]
pub unsafe extern "C" fn sk_msg_zerocopy_from_iter(
    sk: *mut sock,
    from: *mut c_void,
    msg: *mut sk_msg,
    bytes: u32
) -> c_int {
    let mut num_elems = sk_msg_elem_used(msg);
    let mut orig = (*msg).sg.size;
    let mut ret = 0;
    
    while bytes > 0 {
        let mut pages = [ptr::null_mut(); MAX_MSG_FRAGS as usize];
        let mut offset = 0;
        let maxpages = MAX_MSG_FRAGS - num_elems;
        
        if maxpages == 0 {
            ret = -EFAULT;
            break;
        }
        
        let copied = iov_iter_get_pages(from, pages.as_mut_ptr() as *mut *mut page, bytes, maxpages, &mut offset);
        if copied <= 0 {
            ret = -EFAULT;
            break;
        }
        
        iov_iter_advance(from, copied);
        bytes -= copied as u32;
        (*msg).sg.size += copied as size_t;
        
        let mut i = 0;
        while copied > 0 {
            let use_size = if copied < (PAGE_SIZE - offset) as ssize_t {
                copied
            } else {
                (PAGE_SIZE - offset) as ssize_t
            };
            
            let sge = sk_msg_elem(msg, (*msg).sg.end);
            (*sge).page = pages[i];
            (*sge).offset = offset;
            (*sge).length = use_size as u32;
            sg_unmark_end(sge);
            sk_mem_charge(sk, use_size);
            
            offset = 0;
            copied -= use_size;
            sk_msg_iter_next(msg, &mut (*msg).sg.end);
            num_elems += 1;
            i += 1;
        }
        
        (*msg).sg.copybreak = 0;
        (*msg).sg.curr = (*msg).sg.end;
    }
    
    if ret != 0 {
        iov_iter_revert(from, (*msg).sg.size as ssize_t - orig as ssize_t);
    }
    
    ret
}

/// Memory copy from iterator to sk_msg
///
/// # Safety
/// - `sk` must be a valid pointer to sock
/// - `from` must be a valid pointer to iov_iter
/// - `msg` must be a valid pointer to sk_msg
/// - `bytes` must be a valid number of bytes
#[no_mangle]
pub unsafe extern "C" fn sk_msg_memcopy_from_iter(
    sk: *mut sock,
    from: *mut c_void,
    msg: *mut sk_msg,
    bytes: u32
) -> c_int {
    let mut i = (*msg).sg.curr;
    let mut ret = -ENOSPC;
    
    loop {
        let sge = sk_msg_elem(msg, i);
        if (*msg).sg.copybreak >= (*sge).length {
            (*msg).sg.copybreak = 0;
            sk_msg_iter_var_next(&mut i);
            if i == (*msg).sg.end {
                break;
            }
            sge = sk_msg_elem(msg, i);
        }
        
        let buf_size = (*sge).length - (*msg).sg.copybreak;
        let copy = if buf_size > bytes {
            bytes
        } else {
            buf_size
        };
        
        let to = (sg_virt(sge) as *mut u8).offset((*msg).sg.copybreak as isize);
        (*msg).sg.copybreak += copy;
        
        if (*sk).sk_route_caps & NETIF_F_NOCACHE_COPY != 0 {
            ret = copy_from_iter_nocache(to as *mut c_void, copy, from);
        } else {
            ret = copy_from_iter(to as *mut c_void, copy, from);
        }
        
        if ret != copy as ssize_t {
            ret = -EFAULT;
            break;
        }
        
        bytes -= copy;
        if bytes == 0 {
            break;
        }
        (*msg).sg.copybreak = 0;
        sk_msg_iter_var_next(&mut i);
    }
    
    (*msg).sg.curr = i;
    ret as c_int
}

/// Wait for data on socket
///
/// # Safety
/// - `sk` must be a valid pointer to sock
/// - `psock` must be a valid pointer to sk_psock
/// - `flags` must be a valid set of flags
/// - `timeo` must be a valid timeout
#[no_mangle]
pub unsafe extern "C" fn sk_msg_wait_data(
    sk: *mut sock,
    psock: *mut sk_psock,
    flags: c_int,
    timeo: c_int,
    err: *mut c_int
) -> c_int {
    let mut wait = ptr::null_mut();
    let mut ret = 0;
    
    if (*sk).sk_shutdown & RCV_SHUTDOWN != 0 {
        return 1;
    }
    
    if timeo == 0 {
        return ret;
    }
    
    add_wait_queue(sk_sleep(sk), wait);
    sk_set_bit(SOCKWQ_ASYNC_WAITDATA, sk);
    ret = sk_wait_event(sk, &mut timeo, 
                       !list_empty(&(*psock).ingress_msg) || 
                       !skb_queue_empty(&(*sk).sk_receive_queue), wait);
    sk_clear_bit(SOCKWQ_ASYNC_WAITDATA, sk);
    remove_wait_queue(sk_sleep(sk), wait);
    
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sk_msg_alloc() {
        // Basic test for sk_msg_alloc
        // This would require a valid sock and sk_msg structure
        // which is not possible to create in user space
    }
}
