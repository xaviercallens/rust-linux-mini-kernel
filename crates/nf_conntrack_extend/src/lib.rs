use kernel_types::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_extend {
    pub ct: *mut c_void,
    pub timeout: u32,
    pub flags: u32,
    pub helper: *mut c_void,
    pub master: *mut c_void,
    pub tstamp: u64,
    pub status: u32,
    pub nat: nf_nat_extend,
    pub timeout_data: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_nat_extend {
    pub nat_ipv4: nf_nat_ipv4,
    pub nat_ipv6: nf_nat_ipv6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_nat_ipv4 {
    pub min_addr: __be32,
    pub max_addr: __be32,
    pub min_proto: __be16,
    pub max_proto: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_nat_ipv6 {
    pub min_addr: in6_addr,
    pub max_addr: in6_addr,
    pub min_proto: __be16,
    pub max_proto: __be16,
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_extend_init(ct: *mut c_void) -> *mut nf_conntrack_extend {
    let extend = core::ptr::null_mut();
    if extend.is_null() {
        return core::ptr::null_mut();
    }

    (*extend).ct = ct;
    (*extend).timeout = 0;
    (*extend).flags = 0;
    (*extend).helper = core::ptr::null_mut();
    (*extend).master = core::ptr::null_mut();
    (*extend).tstamp = 0;
    (*extend).status = 0;
    (*extend).nat.nat_ipv4.min_addr = 0;
    (*extend).nat.nat_ipv4.max_addr = 0;
    (*extend).nat.nat_ipv4.min_proto = 0;
    (*extend).nat.nat_ipv4.max_proto = 0;
    (*extend).nat.nat_ipv6.min_addr = in6_addr { in6_u: in6_addr_union { u6_addr32: [0; 4] } };
    (*extend).nat.nat_ipv6.max_addr = in6_addr { in6_u: in6_addr_union { u6_addr32: [0; 4] } };
    (*extend).nat.nat_ipv6.min_proto = 0;
    (*extend).nat.nat_ipv6.max_proto = 0;
    (*extend).timeout_data = [0; 4];

    extend
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_extend_destroy(extend: *mut nf_conntrack_extend) {
    if !extend.is_null() {
        // Additional cleanup if needed
    }
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_extend_set_timeout(extend: *mut nf_conntrack_extend, timeout: u32) {
    if !extend.is_null() {
        (*extend).timeout = timeout;
    }
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_extend_set_flags(extend: *mut nf_conntrack_extend, flags: u32) {
    if !extend.is_null() {
        (*extend).flags = flags;
    }
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_extend_set_helper(extend: *mut nf_conntrack_extend, helper: *mut c_void) {
    if !extend.is_null() {
        (*extend).helper = helper;
    }
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_extend_set_master(extend: *mut nf_conntrack_extend, master: *mut c_void) {
    if !extend.is_null() {
        (*extend).master = master;
    }
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_extend_set_tstamp(extend: *mut nf_conntrack_extend, tstamp: u64) {
    if !extend.is_null() {
        (*extend).tstamp = tstamp;
    }
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_extend_set_status(extend: *mut nf_conntrack_extend, status: u32) {
    if !extend.is_null() {
        (*extend).status = status;
    }
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_extend_set_nat_ipv4(
    extend: *mut nf_conntrack_extend,
    min_addr: __be32,
    max_addr: __be32,
    min_proto: __be16,
    max_proto: __be16,
) {
    if !extend.is_null() {
        (*extend).nat.nat_ipv4.min_addr = min_addr;
        (*extend).nat.nat_ipv4.max_addr = max_addr;
        (*extend).nat.nat_ipv4.min_proto = min_proto;
        (*extend).nat.nat_ipv4.max_proto = max_proto;
    }
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_extend_set_nat_ipv6(
    extend: *mut nf_conntrack_extend,
    min_addr: in6_addr,
    max_addr: in6_addr,
    min_proto: __be16,
    max_proto: __be16,
) {
    if !extend.is_null() {
        (*extend).nat.nat_ipv6.min_addr = min_addr;
        (*extend).nat.nat_ipv6.max_addr = max_addr;
        (*extend).nat.nat_ipv6.min_proto = min_proto;
        (*extend).nat.nat_ipv6.max_proto = max_proto;
    }
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_extend_set_timeout_data(extend: *mut nf_conntrack_extend, data: [u32; 4]) {
    if !extend.is_null() {
        (*extend).timeout_data = data;
    }
}