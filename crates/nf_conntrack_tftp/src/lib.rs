#![no_std]
#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;
use kernel_types::*;

pub const TFTP_PORT: u16 = 69;
pub const TFTP_OPCODE_READ: u16 = 1;
pub const TFTP_OPCODE_WRITE: u16 = 2;
pub const NF_ACCEPT: c_int = 1;
pub const AF_INET: c_int = 2;
pub const AF_INET6: c_int = 10;
pub const IPPROTO_UDP: c_int = 17;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tftphdr {
    pub opcode: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udphdr {
    _private: [u8; 8],
}

#[repr(C)]
pub struct sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nf_conn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nf_conntrack_expect {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple_udp {
    pub port: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_conntrack_tuple_union {
    pub u3: [u8; 16],
    pub udp: nf_conntrack_tuple_udp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_tuple {
    pub src: nf_conntrack_tuple_union,
    pub dst: nf_conntrack_tuple_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_expect_policy {
    pub max_expected: c_uint,
    pub timeout: c_uint,
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_conntrack_helper {
    _private: [u8; 0],
}

pub type nf_nat_tftp_hook_t =
    extern "C" fn(*mut sk_buff, c_int, *mut nf_conntrack_expect) -> c_uint;

unsafe extern "C" {
    fn nf_ct_helper_init(
        helper: *mut nf_conntrack_helper,
        l3num: c_int,
        protonum: c_int,
        name: *const c_char,
        src_port: u16,
        dst_port: u16,
        id: c_uint,
        policy: *const nf_conntrack_expect_policy,
        expect_class_max: c_uint,
        help: Option<extern "C" fn(*mut sk_buff, c_uint, *mut nf_conn, c_int) -> c_int>,
        from_nlattr: *mut c_void,
        me: *mut module,
    );

    fn nf_conntrack_helpers_register(helpers: *mut nf_conntrack_helper, count: c_uint) -> c_int;
    fn nf_conntrack_helpers_unregister(helpers: *mut nf_conntrack_helper, count: c_uint);

    fn skb_header_pointer(
        skb: *mut sk_buff,
        offset: c_uint,
        len: c_uint,
        buffer: *mut c_void,
    ) -> *mut c_void;

    fn ntohs(x: u16) -> u16;
    fn pr_err(fmt: *const c_char, ...);
}

static HELPER_NAME: &[u8] = b"tftp\0";
static ERR_MSG: &[u8] = b"failed to register helpers\n\0";

#[unsafe(no_mangle)]
pub static mut nf_nat_tftp_hook: Option<nf_nat_tftp_hook_t> = None;

static mut PORTS: [u16; 8] = [0; 8];
static mut PORTS_C: c_uint = 0;

static mut TFTP_HELPERS: [nf_conntrack_helper; 16] = [nf_conntrack_helper { _private: [] }; 16];

static TFTP_EXP_POLICY: nf_conntrack_expect_policy = nf_conntrack_expect_policy {
    max_expected: 1,
    timeout: 5 * 60,
};

const THIS_MODULE: *mut module = ptr::null_mut();

#[unsafe(no_mangle)]
pub extern "C" fn nf_conntrack_tftp_init() -> c_int {
    let ret: c_int;

    unsafe {
        if PORTS_C == 0 {
            PORTS[0] = TFTP_PORT;
            PORTS_C = 1;
        }

        let mut i: c_uint = 0;
        while i < PORTS_C {
            let idx4 = (2 * i) as usize;
            let idx6 = (2 * i + 1) as usize;

            nf_ct_helper_init(
                &raw mut TFTP_HELPERS[idx4],
                AF_INET,
                IPPROTO_UDP,
                HELPER_NAME.as_ptr() as *const c_char,
                TFTP_PORT,
                PORTS[i as usize],
                i,
                &TFTP_EXP_POLICY,
                0,
                Some(tftp_help),
                ptr::null_mut(),
                THIS_MODULE,
            );

            nf_ct_helper_init(
                &raw mut TFTP_HELPERS[idx6],
                AF_INET6,
                IPPROTO_UDP,
                HELPER_NAME.as_ptr() as *const c_char,
                TFTP_PORT,
                PORTS[i as usize],
                i,
                &TFTP_EXP_POLICY,
                0,
                Some(tftp_help),
                ptr::null_mut(),
                THIS_MODULE,
            );

            i += 1;
        }

        ret = nf_conntrack_helpers_register(core::ptr::addr_of_mut!(TFTP_HELPERS).cast(), 2 * PORTS_C);
        if ret < 0 {
            pr_err(ERR_MSG.as_ptr() as *const c_char);
        }
    }

    ret
}

#[unsafe(no_mangle)]
pub extern "C" fn nf_conntrack_tftp_fini() {
    unsafe {
        nf_conntrack_helpers_unregister(core::ptr::addr_of_mut!(TFTP_HELPERS).cast(), 2 * PORTS_C);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn tftp_help(
    skb: *mut sk_buff,
    protoff: c_uint,
    _ct: *mut nf_conn,
    _ctinfo: c_int,
) -> c_int {
    let mut local_hdr = tftphdr { opcode: [0; 2] };

    let tfh = unsafe {
        skb_header_pointer(
            skb,
            protoff + core::mem::size_of::<udphdr>() as c_uint,
            core::mem::size_of::<tftphdr>() as c_uint,
            (&raw mut local_hdr).cast::<c_void>(),
        ) as *mut tftphdr
    };

    if tfh.is_null() {
        return NF_ACCEPT;
    }

    let opcode = unsafe {
        let raw = [(*tfh).opcode[0], (*tfh).opcode[1]];
        ntohs(u16::from_ne_bytes(raw))
    };

    if opcode != TFTP_OPCODE_READ && opcode != TFTP_OPCODE_WRITE {
        return NF_ACCEPT;
    }

    unsafe {
        if let Some(hook) = nf_nat_tftp_hook {
            let _ = hook(skb, protoff as c_int, ptr::null_mut());
        }
    }

    NF_ACCEPT
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}