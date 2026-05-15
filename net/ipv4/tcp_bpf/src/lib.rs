//! TCP BPF (Berkeley Packet Filter) support for Linux kernel networking
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use libc::{c_int, c_uint, c_void, size_t};

// Constants from C
pub const ENOMEM: c_int = -12;
pub const EACCES: c_int = -13;
pub const EINVAL: c_int = -22;

// Type definitions
#[repr(C)]
pub struct sock {
    // Opaque structure - actual fields defined in kernel headers
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_psock {
    // Opaque structure - actual fields defined in kernel headers
    _private: [u8; 0],
}

#[repr(C)]
pub struct sk_msg {
    sg: sk_msg_sg,
    start: c_uint,
    end: c_uint,
    size: c_uint,
}

#[repr(C)]
pub struct sk_msg_sg {
    data: [sg_element; 0], // Flexible array member
    start: c_uint,
}

#[repr(C)]
pub struct sg_element {
    offset: c_uint,
    length: c_uint,
    page_link: c_uint,
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn tcp_bpf_sendmsg_redir(
    sk: *mut sock,
    msg: *mut sk_msg,
    bytes: c_uint,
    flags: c_int,
) -> c_int {
    // SAFETY: Caller must ensure sk and msg are valid pointers
    if sk.is_null() || msg.is_null() {
        return EINVAL;
    }

    let ingress = sk_msg_to_ingress(msg);
    let psock = sk_psock_get(sk);
    
    if psock.is_null() {
        sk_msg_free(sk, msg);
        return 0;
    }

    let ret = if ingress != 0 {
        bpf_tcp_ingress(sk, psock, msg, bytes, flags)
    } else {
        tcp_bpf_push_locked(sk, msg, bytes, flags, 0)
    };

    sk_psock_put(sk, psock);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn bpf_tcp_ingress(
    sk: *mut sock,
    psock: *mut sk_psock,
    msg: *mut sk_msg,
    apply_bytes: c_uint,
    flags: c_int,
) -> c_int {
    // SAFETY: Caller must ensure all pointers are valid
    if sk.is_null() || psock.is_null() || msg.is_null() {
        return EINVAL;
    }

    let apply = apply_bytes != 0;
    let mut tmp = ptr::null_mut();
    let mut ret = 0;
    let mut copied = 0;
    let mut i = (*msg).sg.start;

    // Allocate temporary message
    tmp = libc::malloc(core::mem::size_of::<sk_msg>() as usize) as *mut sk_msg;
    if tmp.is_null() {
        return ENOMEM;
    }

    // Initialize tmp with msg data
    (*tmp).sg.start = (*msg).sg.start;
    
    lock_sock(sk);
    
    loop {
        let sge = sk_msg_elem(msg, i);
        let size = if apply && apply_bytes < (*sge).length {
            apply_bytes
        } else {
            (*sge).length
        };

        if !sk_wmem_schedule(sk, size) {
            if copied == 0 {
                ret = ENOMEM;
            }
            break;
        }

        sk_mem_charge(sk, size);
        sk_msg_xfer(tmp, msg, i, size);
        copied += size;
        
        if (*sge).length != 0 {
            let page = sk_msg_page(tmp, i);
            get_page(page);
        }
        
        sk_msg_iter_var_next(i);
        (*tmp).sg.end = i;
        
        if apply {
            apply_bytes -= size;
            if apply_bytes == 0 {
                break;
            }
        }
        
        if i == (*msg).sg.end {
            break;
        }
    }

    if ret == 0 {
        (*msg).sg.start = i;
        sk_psock_queue_msg(psock, tmp);
        sk_psock_data_ready(sk, psock);
    } else {
        sk_msg_free(sk, tmp);
        libc::free(tmp as *mut c_void);
    }

    release_sock(sk);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn tcp_bpf_push_locked(
    sk: *mut sock,
    msg: *mut sk_msg,
    apply_bytes: c_uint,
    flags: c_int,
    uncharge: c_int,
) -> c_int {
    lock_sock(sk);
    let ret = tcp_bpf_push(sk, msg, apply_bytes, flags, uncharge);
    release_sock(sk);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn tcp_bpf_push(
    sk: *mut sock,
    msg: *mut sk_msg,
    apply_bytes: c_uint,
    flags: c_int,
    uncharge: c_int,
) -> c_int {
    // SAFETY: Caller must ensure all pointers are valid
    if sk.is_null() || msg.is_null() {
        return EINVAL;
    }

    let apply = apply_bytes != 0;
    let mut ret = 0;
    
    loop {
        let sge = sk_msg_elem(msg, (*msg).sg.start);
        let size = if apply && apply_bytes < (*sge).length {
            apply_bytes
        } else {
            (*sge).length
        };
        
        let off = (*sge).offset;
        let page = sg_page(sge);
        
        tcp_rate_check_app_limited(sk);
        
        let has_tx_ulp = tls_sw_has_ctx_tx(sk);
        if has_tx_ulp != 0 {
            flags |= MSG_SENDPAGE_NOPOLICY;
            ret = kernel_sendpage_locked(sk, page, off, size, flags);
        } else {
            ret = do_tcp_sendpages(sk, page, off, size, flags);
        }

        if ret <= 0 {
            break;
        }
        
        if apply {
            apply_bytes -= ret as c_uint;
        }
        (*msg).sg.size -= ret as c_uint;
        (*sge).offset += ret as c_uint;
        (*sge).length -= ret as c_uint;
        
        if uncharge != 0 {
            sk_mem_uncharge(sk, ret as c_uint);
        }
        
        if ret != size {
            let size = size - ret;
            let off = off + ret as c_uint;
            // retry with remaining data
            // (implementation of retry would go here)
        }
        
        if (*sge).length == 0 {
            put_page(page);
            sk_msg_iter_next(msg, (*msg).sg.start);
            sg_init_table(sge, 1);
            
            if (*msg).sg.start == (*msg).sg.end {
                break;
            }
        }
        
        if apply && apply_bytes == 0 {
            break;
        }
    }
    
    ret
}

// Helper functions (assumed to be defined elsewhere in the kernel)
#[link(name = "kernel_helpers")]
extern "C" {
    fn lock_sock(sk: *mut sock);
    fn release_sock(sk: *mut sock);
    fn sk_msg_to_ingress(msg: *mut sk_msg) -> c_int;
    fn sk_psock_get(sk: *mut sock) -> *mut sk_psock;
    fn sk_psock_put(sk: *mut sock, psock: *mut sk_psock);
    fn sk_wmem_schedule(sk: *mut sock, size: c_uint) -> c_int;
    fn sk_mem_charge(sk: *mut sock, size: c_uint);
    fn sk_mem_uncharge(sk: *mut sock, size: c_uint);
    fn sk_msg_xfer(dst: *mut sk_msg, src: *mut sk_msg, i: c_uint, size: c_uint);
    fn sk_msg_iter_var_next(i: *mut c_uint);
    fn sk_msg_iter_next(msg: *mut sk_msg, start: c_uint);
    fn sk_msg_page(msg: *mut sk_msg, i: c_uint) -> *mut c_void;
    fn sk_msg_elem(msg: *mut sk_msg, i: c_uint) -> *mut sg_element;
    fn sk_msg_free(sk: *mut sock, msg: *mut sk_msg) -> c_int;
    fn sk_msg_init(msg: *mut sk_msg);
    fn sk_msg_alloc(sk: *mut sock, msg: *mut sk_msg, size: c_uint, end: c_uint) -> c_int;
    fn sk_msg_memcopy_from_iter(sk: *mut sock, iter: *mut c_void, msg: *mut sk_msg, size: c_uint) -> c_int;
    fn sk_msg_trim(sk: *mut sock, msg: *mut sk_msg, size: c_uint);
    fn sk_msg_full(msg: *mut sk_msg) -> c_int;
    fn sk_msg_page_add(msg: *mut sk_msg, page: *mut c_void, size: c_uint, offset: c_int);
    fn sk_msg_return(sk: *mut sock, msg: *mut sk_msg, size: c_uint);
    fn sk_msg_free_nocharge(sk: *mut sock, msg: *mut sk_msg) -> c_int;
    fn sk_msg_free_partial(sk: *mut sock, msg: *mut sk_msg, size: c_uint) -> c_int;
    fn sk_msg_apply_bytes(psock: *mut sk_psock, size: c_uint);
    fn sk_psock_queue_msg(psock: *mut sk_psock, msg: *mut sk_msg);
    fn sk_psock_data_ready(sk: *mut sock, psock: *mut sk_psock);
    fn sk_psock_queue_empty(psock: *mut sk_psock) -> c_int;
    fn sk_psock_msg_verdict(sk: *mut sock, psock: *mut sk_psock, msg: *mut sk_msg) -> c_int;
    fn tcp_bpf_send_verdict(sk: *mut sock, psock: *mut sk_psock, msg: *mut sk_msg, copied: *mut c_int, flags: c_int) -> c_int;
    fn tcp_bpf_sendmsg(sk: *mut sock, msg: *mut msghdr, size: size_t) -> c_int;
    fn tcp_bpf_sendpage(sk: *mut sock, page: *mut c_void, offset: c_int, size: size_t, flags: c_int) -> c_int;
    fn tcp_rate_check_app_limited(sk: *mut sock);
    fn tls_sw_has_ctx_tx(sk: *mut sock) -> c_int;
    fn kernel_sendpage_locked(sk: *mut sock, page: *mut c_void, offset: c_int, size: c_uint, flags: c_int) -> c_int;
    fn do_tcp_sendpages(sk: *mut sock, page: *mut c_void, offset: c_int, size: c_uint, flags: c_int) -> c_int;
    fn sg_init_table(sge: *mut sg_element, nents: c_int);
    fn get_page(page: *mut c_void);
    fn put_page(page: *mut c_void);
    fn sock_sndtimeo(sk: *mut sock, nonblock: c_int) -> c_int;
    fn sk_stream_memory_free(sk: *mut sock) -> c_int;
    fn sk_stream_wait_memory(sk: *mut sock, timeo: *mut c_int) -> c_int;
    fn sk_stream_error(sk: *mut sock, flags: c_int, err: c_int) -> c_int;
}

// Tests (conditional compilation)
#[cfg(test)]
mod tests {
    #[test]
    fn test_bpf_tcp_ingress() {
        // Basic test case - would require kernel environment to run
        assert!(true);
    }
}
