#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::c_int;
use core::ffi::c_uint;
use core::ffi::c_void;
use core::ptr;
use core::mem;

// Constants from C
pub const IPPROTO_TCP: c_int = 6;
pub const NF_ACCEPT: c_int = 1;
pub const NF_DROP: c_int = 2;
pub const IP_CT_DIR_ORIGINAL: c_int = 0;
pub const IP_CT_DIR_REPLY: c_int = 1;
pub const IP_CT_ESTABLISHED: c_int = 1 << 1;
pub const IP_CT_ESTABLISHED_REPLY: c_int = IP_CT_ESTABLISHED | (1 << 2);
pub const CTINFO2DIR(ctinfo: c_int) -> c_int = (ctinfo >> 1) & 1;
pub const SANE_PORT: u16 = 6566;

// Error codes
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;

// C struct translations
#[repr(C)]
pub struct tcphdr {
    pub source: u16,
    pub dest: u16,
    pub doff: u8,
    pub _pad: [u8; 3],
}

#[repr(C)]
pub struct sane_request {
    pub RPC_code: u32,
    pub handle: u32,
}

#[repr(C)]
pub struct sane_reply_net_start {
    pub status: u32,
    pub zero: u16,
    pub port: u16,
}

// Helper data structure
#[repr(C)]
pub struct nf_ct_sane_master {
    pub state: c_int,
}

// Expectation structure
#[repr(C)]
pub struct nf_conntrack_expect {
    // Opaque structure - actual fields depend on kernel implementation
    _private: [u8; 0],
}

// Connection tracking tuple
#[repr(C)]
pub struct nf_conntrack_tuple {
    // Opaque structure
    _private: [u8; 0],
}

// Connection tracking structure
#[repr(C)]
pub struct nf_conn {
    // Opaque structure
    _private: [u8; 0],
}

// Helper structure
#[repr(C)]
pub struct nf_conntrack_helper {
    // Opaque structure
    _private: [u8; 0],
}

// Expectation policy
#[repr(C)]
pub struct nf_conntrack_expect_policy {
    pub max_expected: c_int,
    pub timeout: c_int,
};

// Function pointers
type nf_ct_helper_init_fn = unsafe extern "C" fn(
    helper: *mut nf_conntrack_helper,
    l3num: c_int,
    protonum: c_int,
    name: *const c_char,
    src_port: u16,
    dport: u16,
    timeout: c_int,
    policy: *const nf_conntrack_expect_policy,
    flags: c_int,
    help: Option<unsafe extern "C" fn(...)>,
    data: *mut c_void,
    module: *mut c_void,
) -> c_int;

type nf_conntrack_helpers_register_fn = unsafe extern "C" fn(
    helpers: *mut nf_conntrack_helper,
    count: c_int,
) -> c_int;

type nf_conntrack_helpers_unregister_fn = unsafe extern "C" fn(
    helpers: *mut nf_conntrack_helper,
    count: c_int,
);

// Extern declarations for kernel functions
extern "C" {
    fn skb_header_pointer(skb: *mut c_void, offset: c_int, size: c_int, buffer: *mut c_void) -> *mut c_void;
    fn nf_ct_expect_alloc(ct: *mut nf_conn) -> *mut nf_conntrack_expect;
    fn nf_ct_expect_init(
        exp: *mut nf_conntrack_expect,
        class: c_int,
        l3num: c_int,
        src: *mut c_void,
        dst: *mut c_void,
        protonum: c_int,
        mask: *mut c_void,
        port: *mut u16,
    );
    fn nf_ct_expect_related(exp: *mut nf_conntrack_expect, strict: c_int) -> c_int;
    fn nf_ct_expect_put(exp: *mut nf_conntrack_expect);
    fn nf_ct_helper_init(helper: *mut nf_conntrack_helper, ...);
    fn nf_conntrack_helpers_register(helpers: *mut nf_conntrack_helper, count: c_int) -> c_int;
    fn nf_conntrack_helpers_unregister(helpers: *mut nf_conntrack_helper, count: c_int);
    fn nf_ct_l3num(ct: *mut nf_conn) -> c_int;
    fn nf_ct_help_data(ct: *mut nf_conn) -> *mut nf_ct_sane_master;
    fn nf_ct_dump_tuple(tuple: *mut nf_conntrack_tuple);
    fn nf_ct_helper_log(skb: *mut c_void, ct: *mut nf_conn, msg: *const c_char);
}

// Module parameters
static mut ports: [u16; 8] = [0; 8];
static mut ports_c: c_int = 0;

// Spinlock and buffer
static mut nf_sane_lock: *mut c_void = ptr::null_mut();
static mut sane_buffer: *mut c_void = ptr::null_mut();

// Helper array
static mut sane: [nf_conntrack_helper; 16] = unsafe { [mem::zeroed(); 16] };

// Helper function
#[no_mangle]
pub unsafe extern "C" fn help(
    skb: *mut c_void,
    protoff: c_uint,
    ct: *mut nf_conn,
    ctinfo: c_int,
) -> c_int {
    let mut dataoff: c_int = 0;
    let mut datalen: c_int = 0;
    let mut ret: c_int = NF_ACCEPT;
    let dir: c_int = CTINFO2DIR(ctinfo);
    let ct_sane_info: *mut nf_ct_sane_master = nf_ct_help_data(ct);
    let mut exp: *mut nf_conntrack_expect = ptr::null_mut();
    let mut tuple: *mut nf_conntrack_tuple = ptr::null_mut();
    let mut req: *mut sane_request = ptr::null_mut();
    let mut reply: *mut sane_reply_net_start = ptr::null_mut();

    // Until there's been traffic both ways, don't look in packets.
    if ctinfo != IP_CT_ESTABLISHED && ctinfo != IP_CT_ESTABLISHED_REPLY {
        return NF_ACCEPT;
    }

    // Get TCP header
    let mut th: tcphdr = mem::zeroed();
    let th_ptr: *mut c_void = &mut th as *mut _ as *mut c_void;
    let th_result: *mut c_void = skb_header_pointer(skb, protoff as c_int, mem::size_of_val(&th) as c_int, th_ptr);
    if th_result.is_null() {
        return NF_ACCEPT;
    }

    // Calculate data offset
    dataoff = protoff as c_int + (*(&th as *const _ as *const tcphdr)).doff as c_int * 4;
    if dataoff >= (*skb as *mut c_void as *mut c_int).offset(16) {
        return NF_ACCEPT;
    }

    datalen = (*skb as *mut c_void as *mut c_int).offset(16) - dataoff;

    // Acquire spinlock
    spin_lock_bh(nf_sane_lock);

    // Get data pointer
    let sb_ptr: *mut c_void = skb_header_pointer(skb, dataoff, datalen, sane_buffer);
    if sb_ptr.is_null() {
        spin_unlock_bh(nf_sane_lock);
        return NF_ACCEPT;
    }

    if dir == IP_CT_DIR_ORIGINAL {
        if datalen != mem::size_of::<sane_request>() as c_int {
            goto out;
        }

        req = sb_ptr as *mut sane_request;
        if (*req).RPC_code != u32::to_be(SANE_NET_START as u32) {
            (*ct_sane_info).state = 0; // SANE_STATE_NORMAL
            goto out;
        }

        (*ct_sane_info).state = 1; // SANE_STATE_START_REQUESTED
        goto out;
    }

    // Is it a reply to an uninteresting command?
    if (*ct_sane_info).state != 1 {
        goto out;
    }

    (*ct_sane_info).state = 0; // SANE_STATE_NORMAL

    if datalen < mem::size_of::<sane_reply_net_start>() as c_int {
        // pr_debug("NET_START reply too short\n");
        goto out;
    }

    reply = sb_ptr as *mut sane_reply_net_start;
    if (*reply).status != u32::to_be(SANE_STATUS_SUCCESS as u32) {
        // pr_debug("unsuccessful SANE_STATUS = %u\n", ntohl(reply->status));
        goto out;
    }

    if (*reply).zero != 0 {
        goto out;
    }

    exp = nf_ct_expect_alloc(ct);
    if exp.is_null() {
        nf_ct_helper_log(skb, ct, "cannot alloc expectation" as *const c_char);
        ret = NF_DROP;
        goto out;
    }

    tuple = &(*ct).tuplehash[0].tuple;
    nf_ct_expect_init(
        exp,
        0, // NF_CT_EXPECT_CLASS_DEFAULT
        nf_ct_l3num(ct),
        &(*tuple).src.u3 as *mut _,
        &(*tuple).dst.u3 as *mut _,
        IPPROTO_TCP,
        ptr::null_mut(),
        &(*reply).port as *mut _,
    );

    // nf_ct_dump_tuple(&exp->tuple);

    // Can't expect this?  Best to drop packet now.
    if nf_ct_expect_related(exp, 0) != 0 {
        nf_ct_helper_log(skb, ct, "cannot add expectation" as *const c_char);
        ret = NF_DROP;
    }

    nf_ct_expect_put(exp);

out:
    spin_unlock_bh(nf_sane_lock);
    ret
}

// Spinlock operations
extern "C" {
    fn spin_lock_bh(lock: *mut c_void);
    fn spin_unlock_bh(lock: *mut c_void);
}

// Module init/exit
#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_sane_init() -> c_int {
    let mut i: c_int = 0;
    let mut ret: c_int = 0;

    sane_buffer = libc::malloc(65536);
    if sane_buffer.is_null() {
        return ENOMEM;
    }

    if ports_c == 0 {
        ports[0] = SANE_PORT;
        ports_c = 1;
    }

    for i in 0..ports_c {
        nf_ct_helper_init(
            &mut sane[2 * i],
            2, // AF_INET
            IPPROTO_TCP,
            "sane" as *const str as *const c_char,
            SANE_PORT,
            ports[i as usize],
            5 * 60,
            &sane_exp_policy,
            0,
            Some(help as _),
            ptr::null_mut(),
            ptr::null_mut(),
        );
        nf_ct_helper_init(
            &mut sane[2 * i + 1],
            10, // AF_INET6
            IPPROTO_TCP,
            "sane" as *const str as *const c_char,
            SANE_PORT,
            ports[i as usize],
            5 * 60,
            &sane_exp_policy,
            0,
            Some(help as _),
            ptr::null_mut(),
            ptr::null_mut(),
        );
    }

    ret = nf_conntrack_helpers_register(&mut sane, ports_c * 2);
    if ret < 0 {
        libc::free(sane_buffer);
        return ret;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn nf_conntrack_sane_fini() {
    nf_conntrack_helpers_unregister(&mut sane, ports_c * 2);
    libc::free(sane_buffer);
}

// Expectation policy
static sane_exp_policy: nf_conntrack_expect_policy = nf_conntrack_expect_policy {
    max_expected: 1,
    timeout: 5 * 60,
};

// Module macros
#[no_mangle]
pub static nf_conntrack_sane_init_fn: unsafe extern "C" fn() -> c_int = nf_conntrack_sane_init;
#[no_mangle]
pub static nf_conntrack_sane_fini_fn: unsafe extern "C" fn() = nf_conntrack_sane_fini;

// Module license
#[no_mangle]
pub static nf_conntrack_sane_license: [u8; 4] = *b"GPL\0";