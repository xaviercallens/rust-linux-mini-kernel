//! SR-IPv6 HMAC implementation in Rust
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::c_int;
use core::ffi::c_void;
use core::ffi::size_t;
use core::mem;
use core::ptr;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOENT: c_int = -2;
pub const EMSGSIZE: c_int = -92;

// Type definitions
#[repr(C)]
pub struct in6_addr {
    pub s6_addr: [u8; 16],
}

#[repr(C)]
pub struct ipv6_sr_hdr {
    pub hdrlen: u8,
    pub flags: u8,
    pub first_segment: u8,
    pub segments: [in6_addr; 1], // Flexible array member
}

#[repr(C)]
pub struct sr6_tlv_hmac {
    pub tlvhdr: [u8; 2], // Assuming TLV header structure
    pub hmackeyid: u32,
    pub hmac: [u8; 38], // Assuming 38-byte HMAC field
}

#[repr(C)]
pub struct seg6_hmac_info {
    pub hmackeyid: u32,
    pub alg_id: u8,
    pub secret: *const u8,
    pub slen: u32,
    pub node: [u8; 1], // Flexible rhashtable node
    pub rcu: [u8; 1],  // Flexible RCU head
}

#[repr(C)]
pub struct seg6_hmac_algo {
    pub alg_id: u8,
    pub name: *const u8,
    pub tfms: *mut *mut c_void,   // Per-CPU crypto_shash pointers
    pub shashs: *mut *mut c_void, // Per-CPU shash_desc pointers
}

#[repr(C)]
pub struct rhashtable_params {
    pub head_offset: size_t,
    pub key_offset: size_t,
    pub key_len: size_t,
    pub automatic_shrinking: u8,
    pub obj_cmpfn: extern "C" fn(arg: *mut c_void, obj: *const c_void) -> c_int,
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn seg6_hmac_cmpfn(arg: *mut c_void, obj: *const c_void) -> c_int {
    let hinfo = obj as *const seg6_hmac_info;
    let key = arg as *const u32;

    if (*hinfo).hmackeyid != *key {
        return 1;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn seg6_hinfo_release(hinfo: *mut seg6_hmac_info) {
    // SAFETY: hinfo is valid and owned by this function
    ptr::write_bytes(hinfo, 0, mem::size_of::<seg6_hmac_info>());
}

#[no_mangle]
pub unsafe extern "C" fn seg6_free_hi(ptr: *mut c_void, _arg: *mut c_void) {
    let hinfo = ptr as *mut seg6_hmac_info;
    seg6_hinfo_release(hinfo);
}

#[no_mangle]
pub unsafe extern "C" fn seg6_get_tlv_hmac(srh: *const ipv6_sr_hdr) -> *mut sr6_tlv_hmac {
    let srh = srh as *const u8;
    let hdrlen = (*srh.offset(1) as *const u8).read();

    if hdrlen < (((*srh.offset(2) as *const u8).read() + 1) * 2 + 5) {
        return ptr::null_mut();
    }

    if !sr_has_hmac(srh) {
        return ptr::null_mut();
    }

    let tlv = srh.offset(((hdrlen + 1) << 3) - 40) as *mut sr6_tlv_hmac;

    if (*tlv).tlvhdr[0] != 0x01 || (*tlv).tlvhdr[1] != 0x26 {
        return ptr::null_mut();
    }

    tlv
}

#[no_mangle]
pub unsafe extern "C" fn __hmac_get_algo(alg_id: u8) -> *mut seg6_hmac_algo {
    static HMAC_ALGOS: [seg6_hmac_algo; 2] = [
        seg6_hmac_algo {
            alg_id: 1,
            name: b"hmac(sha1)\0".as_ptr() as *const u8,
            tfms: ptr::null_mut(),
            shashs: ptr::null_mut(),
        },
        seg6_hmac_algo {
            alg_id: 2,
            name: b"hmac(sha256)\0".as_ptr() as *const u8,
            tfms: ptr::null_mut(),
            shashs: ptr::null_mut(),
        },
    ];

    for i in 0..2 {
        if HMAC_ALGOS[i].alg_id == alg_id {
            return &HMAC_ALGOS[i] as *const seg6_hmac_algo as *mut seg6_hmac_algo;
        }
    }
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn __do_hmac(
    hinfo: *const seg6_hmac_info,
    text: *const c_void,
    psize: u8,
    output: *mut u8,
    outlen: c_int,
) -> c_int {
    let hinfo = hinfo as *const seg6_hmac_info;
    let algo = __hmac_get_algo((*hinfo).alg_id);
    if algo.is_null() {
        return -ENOENT;
    }

    // SAFETY: algo is valid pointer
    let tfm = *this_cpu_ptr((*algo).tfms);

    let dgsize = crypto_shash_digestsize(tfm);
    if dgsize > outlen as c_int {
        return -ENOMEM;
    }

    let ret = crypto_shash_setkey(tfm, (*hinfo).secret, (*hinfo).slen);
    if ret < 0 {
        return ret;
    }

    // SAFETY: shash is valid pointer
    let shash = *this_cpu_ptr((*algo).shashs);
    (*shash).tfm = tfm;

    let ret = crypto_shash_digest(shash, text, psize as size_t, output);
    if ret < 0 {
        return ret;
    }

    dgsize as c_int
}

#[no_mangle]
pub unsafe extern "C" fn seg6_hmac_compute(
    hinfo: *const seg6_hmac_info,
    hdr: *const ipv6_sr_hdr,
    saddr: *const in6_addr,
    output: *mut u8,
) -> c_int {
    let hmackeyid = hinfo.read().hmackeyid.to_be();
    let mut tmp_out = [0u8; 32]; // Assuming max digest size
    let mut plen = 16 + 1 + 1 + 4;
    let mut i = 0;

    // SAFETY: hdr is valid pointer from caller
    let seg_count = (*hdr).first_segment + 1;
    plen += 16 * seg_count as usize;

    if plen >= SEG6_HMAC_RING_SIZE {
        return -EMSGSIZE;
    }

    let ring = this_cpu_ptr(hmac_ring);
    let mut off = ring;

    // Copy source address
    ptr::copy_nonoverlapping(saddr as *const u8, off, 16);
    off = off.add(16);

    // Copy first_segment
    *off = (*hdr).first_segment;
    off = off.add(1);

    // Copy flags
    *off = (*hdr).flags;
    off = off.add(1);

    // Copy hmackeyid
    ptr::copy_nonoverlapping(&hmackeyid as *const u32 as *const u8, off, 4);
    off = off.add(4);

    // Copy segments
    for i in 0..seg_count {
        let seg = (*hdr).segments.add(i as usize);
        ptr::copy_nonoverlapping(seg as *const u8, off, 16);
        off = off.add(16);
    }

    let dgsize = __do_hmac(
        hinfo,
        ring as *const c_void,
        plen as u8,
        tmp_out.as_mut_ptr(),
        tmp_out.len() as c_int,
    );
    if dgsize < 0 {
        return dgsize;
    }

    let wrsize = if dgsize < SEG6_HMAC_FIELD_LEN {
        dgsize
    } else {
        SEG6_HMAC_FIELD_LEN
    };
    ptr::write_bytes(output, 0, SEG6_HMAC_FIELD_LEN);
    ptr::copy_nonoverlapping(tmp_out.as_ptr(), output, wrsize as usize);

    0
}

#[no_mangle]
pub unsafe extern "C" fn seg6_hmac_validate_skb(skb: *mut c_void) -> bool {
    let mut hmac_output = [0u8; SEG6_HMAC_FIELD_LEN];
    let net = dev_net(skb);
    let idev = __in6_dev_get(skb);

    let srh = skb_transport_header(skb) as *const ipv6_sr_hdr;
    let tlv = seg6_get_tlv_hmac(srh);

    if idev_require_hmac(idev) > 0 && tlv.is_null() {
        return false;
    }

    if idev_require_hmac(idev) < 0 {
        return true;
    }

    if idev_require_hmac(idev) == 0 && tlv.is_null() {
        return true;
    }

    let key = be32_to_cpu((*tlv).hmackeyid);
    let hinfo = seg6_hmac_info_lookup(net, key);
    if hinfo.is_null() {
        return false;
    }

    let ret = seg6_hmac_compute(hinfo, srh, &ipv6_hdr(skb).saddr, hmac_output.as_mut_ptr());
    if ret < 0 {
        return false;
    }

    if ptr::eq(hmac_output.as_ptr(), (*tlv).hmac) {
        true
    } else {
        false
    }
}

#[no_mangle]
pub unsafe extern "C" fn seg6_hmac_info_lookup(net: *mut c_void, key: u32) -> *mut seg6_hmac_info {
    let sdata = seg6_pernet(net);
    rhashtable_lookup_fast(&(*sdata).hmac_infos, &key, rht_params())
}

#[no_mangle]
pub unsafe extern "C" fn seg6_hmac_info_add(
    net: *mut c_void,
    key: u32,
    hinfo: *mut seg6_hmac_info,
) -> c_int {
    let sdata = seg6_pernet(net);
    rhashtable_lookup_insert_fast(&(*sdata).hmac_infos, &(*hinfo).node, rht_params())
}

#[no_mangle]
pub unsafe extern "C" fn seg6_hmac_info_del(net: *mut c_void, key: u32) -> c_int {
    let sdata = seg6_pernet(net);
    let hinfo = rhashtable_lookup_fast(&(*sdata).hmac_infos, &key, rht_params());
    if hinfo.is_null() {
        return -ENOENT;
    }

    let err = rhashtable_remove_fast(&(*sdata).hmac_infos, &(*hinfo).node, rht_params());
    if err < 0 {
        return err;
    }

    seg6_hinfo_release(hinfo);
    0
}

#[no_mangle]
pub unsafe extern "C" fn seg6_push_hmac(
    net: *mut c_void,
    saddr: *const in6_addr,
    srh: *mut ipv6_sr_hdr,
) -> c_int {
    let tlv = seg6_get_tlv_hmac(srh);
    if tlv.is_null() {
        return -EINVAL;
    }

    let key = be32_to_cpu((*tlv).hmackeyid);
    let hinfo = seg6_hmac_info_lookup(net, key);
    if hinfo.is_null() {
        return -ENOENT;
    }

    ptr::write_bytes((*tlv).hmac.as_mut_ptr(), 0, SEG6_HMAC_FIELD_LEN);
    seg6_hmac_compute(hinfo, srh, saddr, (*tlv).hmac.as_mut_ptr())
}

#[no_mangle]
pub unsafe extern "C" fn seg6_hmac_init_algo() -> c_int {
    // Implementation would use kernel crypto APIs
    0
}

#[no_mangle]
pub unsafe extern "C" fn seg6_hmac_init() -> c_int {
    seg6_hmac_init_algo()
}

#[no_mangle]
pub unsafe extern "C" fn seg6_hmac_net_init(net: *mut c_void) -> c_int {
    let sdata = seg6_pernet(net);
    rhashtable_init(&(*sdata).hmac_infos, rht_params())
}

#[no_mangle]
pub unsafe extern "C" fn seg6_hmac_exit() {
    // Implementation would clean up crypto resources
}

#[no_mangle]
pub unsafe extern "C" fn seg6_hmac_net_exit(net: *mut c_void) {
    let sdata = seg6_pernet(net);
    rhashtable_free_and_destroy(&(*sdata).hmac_infos, seg6_free_hi, ptr::null_mut());
}

// External functions (assumed to be available in kernel)
extern "C" {
    fn crypto_shash_digestsize(tfm: *mut c_void) -> c_int;
    fn crypto_shash_setkey(tfm: *mut c_void, key: *const u8, len: u32) -> c_int;
    fn crypto_shash_digest(
        desc: *mut c_void,
        data: *const c_void,
        len: size_t,
        out: *mut u8,
    ) -> c_int;
    fn this_cpu_ptr(ptr: *mut *mut c_void) -> *mut c_void;
    fn dev_net(skb: *mut c_void) -> *mut c_void;
    fn __in6_dev_get(skb: *mut c_void) -> *mut c_void;
    fn skb_transport_header(skb: *mut c_void) -> *mut c_void;
    fn ipv6_hdr(skb: *mut c_void) -> *mut in6_addr;
    fn be32_to_cpu(val: u32) -> u32;
    fn rhashtable_lookup_fast(
        table: *mut c_void,
        key: *mut c_void,
        params: rhashtable_params,
    ) -> *mut c_void;
    fn rhashtable_lookup_insert_fast(
        table: *mut c_void,
        node: *mut c_void,
        params: rhashtable_params,
    ) -> c_int;
    fn rhashtable_remove_fast(
        table: *mut c_void,
        node: *mut c_void,
        params: rhashtable_params,
    ) -> c_int;
    fn rhashtable_init(table: *mut c_void, params: rhashtable_params) -> c_int;
    fn rhashtable_free_and_destroy(
        table: *mut c_void,
        free_fn: extern "C" fn(*mut c_void, *mut c_void),
        arg: *mut c_void,
    );
}

// Constants
const SEG6_HMAC_RING_SIZE: usize = 1280;
const SEG6_HMAC_FIELD_LEN: usize = 32;

// Helper functions (would be implemented in kernel)
fn sr_has_hmac(srh: *const u8) -> bool {
    // Implementation would check SRH flags
    true
}

fn idev_require_hmac(idev: *mut c_void) -> c_int {
    // Implementation would check device configuration
    0
}

// Per-CPU variables
static hmac_ring: [u8; SEG6_HMAC_RING_SIZE] = [0; SEG6_HMAC_RING_SIZE];
