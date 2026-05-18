#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::implicit_return_in_non_void_function)]

use core::alloc::{GlobalAlloc, Layout};
use core::ffi::{c_int, c_void};
use core::{mem, ptr};
use kernel_types::*;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOSYS: c_int = -38;
pub const ENOENT: c_int = -2;
pub const EINPROGRESS: c_int = -115;
pub const EOPNOTSUPP: c_int = -95;
pub const EREMCHG: c_int = -103;

struct DummyAlloc;
unsafe impl GlobalAlloc for DummyAlloc {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        ptr::null_mut()
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
#[global_allocator]
static GLOBAL_ALLOCATOR: DummyAlloc = DummyAlloc;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[cfg(target_os = "none")]
#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock {
    _private: [u8; 0],
}

pub type __be16 = u16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_skb_cb {
    pub xfrm: xfrm_skb_cb_inner,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_skb_cb_inner {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esp_skb_cb {
    pub xfrm: xfrm_skb_cb,
    pub tmp: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esp_output_extra {
    pub seqhi: u32,
    pub esphoff: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aead_request {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_aead {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scatterlist {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_state_props {
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_encap_tmpl {
    pub encap_type: u16,
    pub encap_sport: __be16,
    pub encap_dport: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_state {
    pub data: *mut crypto_aead,
    pub props: xfrm_state_props,
    pub encap: *mut xfrm_encap_tmpl,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_head {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esp_tcp_sk {
    pub sk: *mut sock,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_offload_seq {
    pub hi: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_offload {
    pub flags: u32,
    pub seq: xfrm_offload_seq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sec_path {
    pub len: u8,
    pub xvec: [*mut xfrm_state; 0],
}

#[inline]
fn align_up(v: usize, a: usize) -> usize {
    if a == 0 {
        v
    } else {
        (v + (a - 1)) & !(a - 1)
    }
}

unsafe extern "C" {
    fn crypto_aead_ivsize(aead: *mut crypto_aead) -> u32;
    fn crypto_aead_alignmask(aead: *mut crypto_aead) -> u32;
    fn crypto_tfm_ctx_alignment() -> u32;
    fn crypto_aead_reqsize(aead: *mut crypto_aead) -> u32;
    fn aead_request_set_tfm(req: *mut aead_request, tfm: *mut crypto_aead);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn esp_alloc_tmp(
    aead: *mut crypto_aead,
    nfrags: c_int,
    extralen: c_int,
) -> *mut c_void {
    if aead.is_null() {
        return ptr::null_mut();
    }

    let mut len = extralen.max(0) as usize;

    let ivsize = unsafe { crypto_aead_ivsize(aead) } as usize;
    len = len.saturating_add(ivsize);

    if ivsize > 0 {
        let tfm_align = unsafe { crypto_tfm_ctx_alignment() } as usize;
        let align_mask =
            (unsafe { crypto_aead_alignmask(aead) } as usize) & !(tfm_align.saturating_sub(1));
        len = len.saturating_add(align_mask);
        len = align_up(len, tfm_align);
    }

    len = len
        .saturating_add(mem::size_of::<aead_request>())
        .saturating_add(unsafe { crypto_aead_reqsize(aead) } as usize);
    len = align_up(len, mem::align_of::<scatterlist>());

    let frags = nfrags.max(0) as usize;
    len = len.saturating_add(mem::size_of::<scatterlist>().saturating_mul(frags));

    let layout = match Layout::from_size_align(len.max(1), mem::align_of::<usize>()) {
        Ok(l) => l,
        Err(_) => return ptr::null_mut(),
    };

    unsafe { GLOBAL_ALLOCATOR.alloc(layout) as *mut c_void }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn esp_tmp_extra(tmp: *mut c_void) -> *mut esp_output_extra {
    if tmp.is_null() {
        return ptr::null_mut();
    }
    let aligned = align_up(tmp as usize, mem::align_of::<esp_output_extra>());
    aligned as *mut esp_output_extra
}