use kernel_types::*;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const EEXIST: c_int = -17;
pub const ENOENT: c_int = -2;
pub const EAGAIN: c_int = -11;

pub const AF_INET6: c_int = 10;
pub const AF_INET: c_int = 2;
pub const AF_MPLS: c_int = 25;

pub const INET6_PROTO_NOPOLICY: c_int = 1 << 0;
pub const INET6_PROTO_FINAL: c_int = 1 << 1;

pub const IPPROTO_IPV6: c_int = 41;
pub const IPPROTO_IPIP: c_int = 4;
pub const IPPROTO_MPLS: c_int = 137;

#[repr(C)]
pub struct sk_buff {
    _priv: [u8; 0],
}

#[repr(C)]
pub struct inet6_skb_parm {
    _priv: [u8; 0],
}

pub type handler_func = unsafe extern "C" fn(*mut sk_buff) -> c_int;
pub type cb_handler_func = unsafe extern "C" fn(*mut sk_buff, c_int) -> c_int;
pub type err_handler_func =
    unsafe extern "C" fn(*mut sk_buff, *mut inet6_skb_parm, u8, u8, c_int, u32) -> c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm6_tunnel {
    pub priority: c_int,
    pub handler: handler_func,
    pub cb_handler: Option<cb_handler_func>,
    pub err_handler: Option<err_handler_func>,
    pub next: *mut xfrm6_tunnel,
}

static mut tunnel6_handlers: *mut xfrm6_tunnel = core::ptr::null_mut();
static mut tunnel46_handlers: *mut xfrm6_tunnel = core::ptr::null_mut();
static mut tunnelmpls6_handlers: *mut xfrm6_tunnel = core::ptr::null_mut();

#[repr(C)]
pub struct mutex {
    _priv: [u8; 0],
}

unsafe extern "C" {
    static mut tunnel6_mutex: mutex;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet6_protocol {
    pub handler: handler_func,
    pub err_handler: Option<err_handler_func>,
    pub flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_input_afinfo {
    pub family: c_int,
    pub is_ipip: c_int,
    pub callback: unsafe extern "C" fn(*mut sk_buff, u8, c_int) -> c_int,
}

unsafe extern "C" {
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn pskb_may_pull(skb: *mut sk_buff, size: c_int) -> c_int;
    fn icmpv6_send(skb: *mut sk_buff, type_: c_int, code: c_int, info: u32);
    fn kfree_skb(skb: *mut sk_buff);
    fn inet6_add_protocol(proto: *const inet6_protocol, protocol: c_int) -> c_int;
    fn inet6_del_protocol(proto: *const inet6_protocol, protocol: c_int) -> c_int;
    fn xfrm_input_register_afinfo(afinfo: *const xfrm_input_afinfo) -> c_int;
    fn xfrm_input_unregister_afinfo(afinfo: *const xfrm_input_afinfo) -> c_int;
}

#[no_mangle]
pub unsafe extern "C" fn tunnel6_rcv(_skb: *mut sk_buff) -> c_int {
    ENOENT
}

#[no_mangle]
pub unsafe extern "C" fn tunnel46_rcv(_skb: *mut sk_buff) -> c_int {
    ENOENT
}

#[no_mangle]
pub unsafe extern "C" fn tunnelmpls6_rcv(_skb: *mut sk_buff) -> c_int {
    ENOENT
}

#[no_mangle]
pub unsafe extern "C" fn tunnel6_err(
    _skb: *mut sk_buff,
    _opt: *mut inet6_skb_parm,
    _type: u8,
    _code: u8,
    _offset: c_int,
    _info: u32,
) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn tunnel46_err(
    _skb: *mut sk_buff,
    _opt: *mut inet6_skb_parm,
    _type: u8,
    _code: u8,
    _offset: c_int,
    _info: u32,
) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn tunnelmpls6_err(
    _skb: *mut sk_buff,
    _opt: *mut inet6_skb_parm,
    _type: u8,
    _code: u8,
    _offset: c_int,
    _info: u32,
) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn tunnel6_rcv_cb(_skb: *mut sk_buff, _nexthdr: u8, _err: c_int) -> c_int {
    0
}

static tunnel6_protocol: inet6_protocol = inet6_protocol {
    handler: tunnel6_rcv,
    err_handler: Some(tunnel6_err),
    flags: INET6_PROTO_NOPOLICY | INET6_PROTO_FINAL,
};

static tunnel46_protocol: inet6_protocol = inet6_protocol {
    handler: tunnel46_rcv,
    err_handler: Some(tunnel46_err),
    flags: INET6_PROTO_NOPOLICY | INET6_PROTO_FINAL,
};

static tunnelmpls6_protocol: inet6_protocol = inet6_protocol {
    handler: tunnelmpls6_rcv,
    err_handler: Some(tunnelmpls6_err),
    flags: INET6_PROTO_NOPOLICY | INET6_PROTO_FINAL,
};

static tunnel6_input_afinfo: xfrm_input_afinfo = xfrm_input_afinfo {
    family: AF_INET6,
    is_ipip: 1,
    callback: tunnel6_rcv_cb,
};

#[no_mangle]
pub unsafe extern "C" fn xfrm6_tunnel_mpls_supported() -> c_int {
    1
}

#[no_mangle]
pub unsafe extern "C" fn xfrm6_tunnel_register(handler: *mut xfrm6_tunnel, family: c_int) -> c_int {
    if handler.is_null() {
        return EINVAL;
    }

    let priority = (*handler).priority;

    mutex_lock(core::ptr::addr_of_mut!(tunnel6_mutex));

    let mut pprev: *mut *mut xfrm6_tunnel = match family {
        AF_INET6 => core::ptr::addr_of_mut!(tunnel6_handlers),
        AF_INET => core::ptr::addr_of_mut!(tunnel46_handlers),
        AF_MPLS => core::ptr::addr_of_mut!(tunnelmpls6_handlers),
        _ => {
            mutex_unlock(core::ptr::addr_of_mut!(tunnel6_mutex));
            return EINVAL;
        }
    };

    while !(*pprev).is_null() {
        let current = *pprev;
        let current_priority = (*current).priority;
        if current_priority > priority {
            break;
        }
        if current_priority == priority {
            mutex_unlock(core::ptr::addr_of_mut!(tunnel6_mutex));
            return EEXIST;
        }
        pprev = core::ptr::addr_of_mut!((*current).next);
    }

    (*handler).next = *pprev;
    *pprev = handler;

    mutex_unlock(core::ptr::addr_of_mut!(tunnel6_mutex));
    0
}

#[no_mangle]
pub unsafe extern "C" fn xfrm6_tunnel_deregister(handler: *mut xfrm6_tunnel, family: c_int) -> c_int {
    if handler.is_null() {
        return EINVAL;
    }

    mutex_lock(core::ptr::addr_of_mut!(tunnel6_mutex));

    let mut pprev: *mut *mut xfrm6_tunnel = match family {
        AF_INET6 => core::ptr::addr_of_mut!(tunnel6_handlers),
        AF_INET => core::ptr::addr_of_mut!(tunnel46_handlers),
        AF_MPLS => core::ptr::addr_of_mut!(tunnelmpls6_handlers),
        _ => {
            mutex_unlock(core::ptr::addr_of_mut!(tunnel6_mutex));
            return EINVAL;
        }
    };

    while !(*pprev).is_null() {
        if *pprev == handler {
            *pprev = (*handler).next;
            (*handler).next = core::ptr::null_mut();
            mutex_unlock(core::ptr::addr_of_mut!(tunnel6_mutex));
            return 0;
        }
        pprev = core::ptr::addr_of_mut!((**pprev).next);
    }

    mutex_unlock(core::ptr::addr_of_mut!(tunnel6_mutex));
    ENOENT
}