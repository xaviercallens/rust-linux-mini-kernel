#![no_std]
#![no_main]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use core::ffi::{c_int, c_uint, c_void};
use core::panic::PanicInfo;
use core::ptr;
use kernel_types::*;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const ENOTSUPP: c_int = -95;
pub const EEXIST: c_int = -17;
pub const EACCES: c_int = -13;
pub const NOT_INIT: c_int = -1;

pub const TCP_CONG_MASK: c_uint = 0;

// Opaque FFI types
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Btf {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BtfType {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BpfProg {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BpfInsnAccessAux {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BpfVerifierLog {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BpfFuncProto {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BtfMember {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_sock {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TcpCongestionOps {
    pub init: Option<extern "C" fn(*mut c_void)>,
    pub release: Option<extern "C" fn(*mut c_void)>,
    pub set_state: Option<extern "C" fn(*mut c_void, c_int)>,
    pub cwnd_event: Option<extern "C" fn(*mut c_void, c_int)>,
    pub in_ack_event: Option<extern "C" fn(*mut c_void, c_int)>,
    pub pkts_acked: Option<extern "C" fn(*mut c_void, *const c_void)>,
    pub min_tso_segs: Option<extern "C" fn(*mut c_void) -> c_uint>,
    pub sndbuf_expand: Option<extern "C" fn(*mut c_void) -> c_uint>,
    pub cong_control: Option<extern "C" fn(*mut c_void, *const c_void, c_uint, c_int)>,
    pub get_info: Option<extern "C" fn(*mut c_void, c_uint, *mut c_void) -> c_int>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BpfVerifierOps {
    pub get_func_proto: extern "C" fn(c_int, *const BpfProg) -> *const BpfFuncProto,
    pub is_valid_access:
        extern "C" fn(c_int, c_int, c_int, *const BpfProg, *mut BpfInsnAccessAux) -> bool,
    pub btf_struct_access: extern "C" fn(
        *mut BpfVerifierLog,
        *mut Btf,
        *mut BtfType,
        c_int,
        c_int,
        c_int,
        *mut c_uint,
    ) -> c_int,
    pub check_kfunc_call: extern "C" fn(c_int) -> bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BpfStructOps {
    pub verifier_ops: *const BpfVerifierOps,
    pub reg: extern "C" fn(*mut c_void) -> c_int,
    pub unreg: extern "C" fn(*mut c_void),
    pub check_member: extern "C" fn(*const BtfType, *const BtfMember) -> c_int,
    pub init_member:
        extern "C" fn(*const BtfType, *const BtfMember, *mut c_void, *const c_void) -> c_int,
    pub init: extern "C" fn(*mut Btf) -> c_int,
    pub name: *const u8,
}

unsafe impl Sync for BpfStructOps {}
unsafe impl Sync for BpfVerifierOps {}

// Offset tables
static mut OPTIONAL_OPS: [c_uint; 9] = [0; 9];
static mut UNSUPPORTED_OPS: [c_uint; 1] = [0; 1];

// External kernel symbols
unsafe extern "C" {
    fn btf_find_by_name_kind(btf: *mut Btf, name: *const u8, kind: c_int) -> c_int;
    fn btf_type_by_id(btf: *mut Btf, id: c_int) -> *mut BtfType;
    fn btf_ctx_access(
        off: c_int,
        size: c_int,
        type_: c_int,
        prog: *const BpfProg,
        info: *mut BpfInsnAccessAux,
    ) -> bool;
    fn bpf_log(log: *mut BpfVerifierLog, fmt: *const u8, ...) -> c_int;
    fn btf_struct_access(
        log: *mut BpfVerifierLog,
        btf: *mut Btf,
        t: *mut BtfType,
        off: c_int,
        size: c_int,
        atype: c_int,
        next_btf_id: *mut c_uint,
    ) -> c_int;
    fn __tcp_send_ack(sock: *mut c_void, rcv_nxt: u32);
    fn tcp_register_congestion_control(kdata: *mut c_void) -> c_int;
    fn tcp_unregister_congestion_control(kdata: *mut c_void);
    fn bpf_obj_name_cpy(dst: *mut u8, src: *const u8, size: c_int) -> c_int;
    fn tcp_find(name: *const u8) -> *mut c_void;

    static bpf_sk_storage_get_proto: BpfFuncProto;
    static bpf_sk_storage_delete_proto: BpfFuncProto;
    static mut btf_vmlinux: *mut Btf;
}

// Globals
static mut TCP_SOCK_TYPE: *mut BtfType = ptr::null_mut();
static mut TCP_SOCK_ID: c_uint = 0;
static mut SOCK_ID: c_uint = 0;

#[no_mangle]
pub unsafe extern "C" fn bpf_tcp_send_ack(tp: *mut tcp_sock, rcv_nxt: u32) -> c_int {
    __tcp_send_ack(tp as *mut c_void, rcv_nxt);
    0
}

extern "C" fn bpf_tcp_ca_get_func_proto(_func_id: c_int, _prog: *const BpfProg) -> *const BpfFuncProto {
    ptr::null()
}

extern "C" fn bpf_tcp_ca_is_valid_access(
    off: c_int,
    size: c_int,
    type_: c_int,
    prog: *const BpfProg,
    info: *mut BpfInsnAccessAux,
) -> bool {
    unsafe { btf_ctx_access(off, size, type_, prog, info) }
}

extern "C" fn bpf_tcp_ca_btf_struct_access(
    log: *mut BpfVerifierLog,
    btf: *mut Btf,
    t: *mut BtfType,
    off: c_int,
    size: c_int,
    atype: c_int,
    next_btf_id: *mut c_uint,
) -> c_int {
    unsafe { btf_struct_access(log, btf, t, off, size, atype, next_btf_id) }
}

extern "C" fn bpf_tcp_ca_check_kfunc_call(_id: c_int) -> bool {
    false
}

extern "C" fn bpf_tcp_ca_reg(_kdata: *mut c_void) -> c_int {
    0
}

extern "C" fn bpf_tcp_ca_unreg(_kdata: *mut c_void) {}

extern "C" fn bpf_tcp_ca_check_member(_t: *const BtfType, _m: *const BtfMember) -> c_int {
    0
}

extern "C" fn bpf_tcp_ca_init_member(
    _t: *const BtfType,
    _m: *const BtfMember,
    _kdata: *mut c_void,
    _value: *const c_void,
) -> c_int {
    0
}

extern "C" fn bpf_tcp_ca_init(_btf: *mut Btf) -> c_int {
    0
}

static BPF_TCP_CA_VERIFIER_OPS: BpfVerifierOps = BpfVerifierOps {
    get_func_proto: bpf_tcp_ca_get_func_proto,
    is_valid_access: bpf_tcp_ca_is_valid_access,
    btf_struct_access: bpf_tcp_ca_btf_struct_access,
    check_kfunc_call: bpf_tcp_ca_check_kfunc_call,
};

static BPF_TCP_CA_NAME: &[u8] = b"tcp_congestion_ops\0";

#[no_mangle]
pub static BPF_TCP_CA_OPS: BpfStructOps = BpfStructOps {
    verifier_ops: &BPF_TCP_CA_VERIFIER_OPS as *const BpfVerifierOps,
    reg: bpf_tcp_ca_reg,
    unreg: bpf_tcp_ca_unreg,
    check_member: bpf_tcp_ca_check_member,
    init_member: bpf_tcp_ca_init_member,
    init: bpf_tcp_ca_init,
    name: BPF_TCP_CA_NAME.as_ptr(),
};

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}