use kernel_types::*;

pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const EEXIST: c_int = -17;
pub const ENOENT: c_int = -2;
pub const EAGAIN: c_int = -11;

// Address family constants
pub const AF_INET6: c_int = 10;
pub const AF_INET: c_int = 2;
pub const AF_MPLS: c_int = 25; // MPLS address family

// inet6_protocol flags
pub const INET6_PROTO_NOPOLICY: c_int = 1 << 0;
pub const INET6_PROTO_FINAL: c_int = 1 << 1;

// IPPROTO constants
pub const IPPROTO_IPV6: c_int = 41;
pub const IPPROTO_IPIP: c_int = 4;
pub const IPPROTO_MPLS: c_int = 137;

// Define function pointer types for handler callbacks
pub type handler_func = unsafe extern "C" fn(*mut sk_buff) -> c_int;
pub type cb_handler_func = unsafe extern "C" fn(*mut sk_buff, c_int) -> c_int;
pub type err_handler_func =
    unsafe extern "C" fn(*mut sk_buff, *mut c_void, u8, u8, c_int, u32) -> c_int;

// Define the xfrm6_tunnel struct with #[repr(C)] for ABI compatibility
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm6_tunnel {
    pub priority: c_int,
    pub handler: handler_func,
    pub cb_handler: Option<cb_handler_func>,
    pub err_handler: Option<err_handler_func>,
    pub next: *mut xfrm6_tunnel,
}

// Static variables - represented as static mut with unsafe access
static mut tunnel6_handlers: *mut xfrm6_tunnel = core::ptr::null_mut();
static mut tunnel46_handlers: *mut xfrm6_tunnel = core::ptr::null_mut();
static mut tunnelmpls6_handlers: *mut xfrm6_tunnel = core::ptr::null_mut();

// Mutex for synchronization - represented as a raw mutex handle
static mut tunnel6_mutex: *mut c_void = core::ptr::null_mut();

// Define the inet6_protocol struct with #[repr(C)] for ABI compatibility
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet6_protocol {
    handler: handler_func,
    err_handler: Option<err_handler_func>,
    flags: c_int,
}

// Define the tunnel protocol instances
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

// Define the xfrm_input_afinfo struct with #[repr(C)] for ABI compatibility
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfrm_input_afinfo {
    pub family: c_int,
    pub is_ipip: c_int,
    pub callback: unsafe extern "C" fn(*mut sk_buff, u8, c_int) -> c_int,
}

// Define the xfrm_input_afinfo instance
static tunnel6_input_afinfo: xfrm_input_afinfo = xfrm_input_afinfo {
    family: AF_INET6,
    is_ipip: 1,
    callback: tunnel6_rcv_cb,
};

// Function declarations for external kernel functions
extern "C" {
    fn mutex_lock(mutex: *mut c_void);
    fn mutex_unlock(mutex: *mut c_void);
    fn pskb_may_pull(skb: *mut sk_buff, size: c_int) -> c_int;
    fn icmpv6_send(skb: *mut sk_buff, type_: c_int, code: c_int, info: u32);
    fn kfree_skb(skb: *mut sk_buff);
    fn inet6_add_protocol(proto: *const inet6_protocol, protocol: c_int) -> c_int;
    fn inet6_del_protocol(proto: *const inet6_protocol, protocol: c_int) -> c_int;
    fn xfrm_input_register_afinfo(afinfo: *const xfrm_input_afinfo) -> c_int;
    fn xfrm_input_unregister_afinfo(afinfo: *const xfrm_input_afinfo) -> c_int;
}

// Helper function to check if MPLS is supported
#[no_mangle]
pub unsafe extern "C" fn xfrm6_tunnel_mpls_supported() -> c_int {
    // In real kernel code, this would check CONFIG_MPLS
    // For this translation, we'll assume it's enabled
    1
}

// Implementation of xfrm6_tunnel_register
#[no_mangle]
pub unsafe extern "C" fn xfrm6_tunnel_register(handler: *mut xfrm6_tunnel, family: c_int) -> c_int {
    let mut ret: c_int = EEXIST;
    let priority = (*handler).priority;

    // Lock the mutex before modifying the list
    mutex_lock(tunnel6_mutex);

    let mut pprev: *mut *mut xfrm6_tunnel;
    match family {
        AF_INET6 => pprev = &mut tunnel6_handlers,
        AF_INET => pprev = &mut tunnel46_handlers,
        AF_MPLS => pprev = &mut tunnelmpls6_handlers,
        _ => {
            mutex_unlock(tunnel6_mutex);
            return EINVAL;
        }
    }

    // Traverse the list to find insertion point
    let mut current: *mut xfrm6_tunnel = *pprev;
    while !current.is_null() {
        let current_priority = (*current).priority;
        if current_priority > priority {
            break;
        }
        if current_priority == priority {
            // Priority already exists
            mutex_unlock(tunnel6_mutex);
            return EEXIST;
        }
        pprev = &mut (*current).next;
        current = *pprev;
    }

    // Insert the new handler
    (*handler).next = *pprev;
    *pprev = handler;

    ret = 0;

    mutex_unlock(tunnel6_mutex);
    ret
}

// Implementation of xfrm6_tunnel_deregister
#[no_mangle]
pub unsafe extern "C" fn xfrm6_tunnel_deregister(
    handler: *mut xfrm6_tunnel,
    family: c_int,
) -> c_int {
    let mut ret: c_int = ENOENT;

    mutex_lock(tunnel6_mutex);

    let mut pprev: *mut *mut xfrm6_tunnel;
    match family {
        AF_INET6 => pprev = &mut tunnel6_handlers,
        AF_INET => pprev = &mut tunnel46_handlers,
        AF_MPLS => pprev = &mut tunnelmpls6_handlers,
        _ => {
            mutex_unlock(tunnel6_mutex);
            return EINVAL;
        }
    }

    let mut current: *mut xfrm6_tunnel = *pprev;
    while !current.is_null() {
        if current == handler {
            *pprev = (*current).next;
            ret = 0;
            break;
        }
        pprev = &mut (*current).next;
        current = *pprev;
    }

    mutex_unlock(tunnel6_mutex);

    // Synchronize with network
    synchronize_net();

    ret
}

// Helper function for synchronization
#[no_mangle]
pub unsafe extern "C" fn synchronize_net() {
    // In real kernel code, this would synchronize with network operations
    // For this translation, we'll just return
}

// Implementation of tunnelmpls6_rcv
#[no_mangle]
pub unsafe extern "C" fn tunnelmpls6_rcv(skb: *mut sk_buff) -> c_int {
    if pskb_may_pull(skb, core::mem::size_of::<ipv6hdr>() as c_int) == 0 {
        kfree_skb(skb);
        return 0;
    }

    let mut handler: *mut xfrm6_tunnel = tunnelmpls6_handlers;
    while !handler.is_null() {
        if ((*handler).handler)(skb) == 0 {
            return 0;
        }
        handler = (*handler).next;
    }

    icmpv6_send(skb, ICMPV6_DEST_UNREACH, ICMPV6_PORT_UNREACH, 0);
    kfree_skb(skb);
    0
}

// Implementation of tunnel6_rcv
#[no_mangle]
pub unsafe extern "C" fn tunnel6_rcv(skb: *mut sk_buff) -> c_int {
    if pskb_may_pull(skb, core::mem::size_of::<ipv6hdr>() as c_int) == 0 {
        kfree_skb(skb);
        return 0;
    }

    let mut handler: *mut xfrm6_tunnel = tunnel6_handlers;
    while !handler.is_null() {
        if ((*handler).handler)(skb) == 0 {
            return 0;
        }
        handler = (*handler).next;
    }

    icmpv6_send(skb, ICMPV6_DEST_UNREACH, ICMPV6_PORT_UNREACH, 0);
    kfree_skb(skb);
    0
}

// Implementation of tunnel6_rcv_cb
#[no_mangle]
pub unsafe extern "C" fn tunnel6_rcv_cb(skb: *mut sk_buff, proto: u8, err: c_int) -> c_int {
    let head: *mut xfrm6_tunnel = if proto == IPPROTO_IPV6 {
        tunnel6_handlers
    } else {
        tunnel46_handlers
    };

    let mut handler: *mut xfrm6_tunnel = head;
    while !handler.is_null() {
        if let Some(cb_handler) = (*handler).cb_handler {
            let ret = cb_handler(skb, err);
            if ret <= 0 {
                return ret;
            }
        }
        handler = (*handler).next;
    }

    0
}

// Implementation of tunnel46_rcv
#[no_mangle]
pub unsafe extern "C" fn tunnel46_rcv(skb: *mut sk_buff) -> c_int {
    if pskb_may_pull(skb, core::mem::size_of::<iphdr>() as c_int) == 0 {
        kfree_skb(skb);
        return 0;
    }

    let mut handler: *mut xfrm6_tunnel = tunnel46_handlers;
    while !handler.is_null() {
        if ((*handler).handler)(skb) == 0 {
            return 0;
        }
        handler = (*handler).next;
    }

    icmpv6_send(skb, ICMPV6_DEST_UNREACH, ICMPV6_PORT_UNREACH, 0);
    kfree_skb(skb);
    0
}

// Implementation of tunnel6_err
#[no_mangle]
pub unsafe extern "C" fn tunnel6_err(
    skb: *mut sk_buff,
    opt: *mut c_void,
    type_: u8,
    code: u8,
    offset: c_int,
    info: u32,
) -> c_int {
    let mut handler: *mut xfrm6_tunnel = tunnel6_handlers;
    while !handler.is_null() {
        if let Some(err_handler) = (*handler).err_handler {
            if err_handler(skb, opt, type_, code, offset, info) == 0 {
                return 0;
            }
        }
        handler = (*handler).next;
    }

    -ENOENT
}

// Implementation of tunnel46_err
#[no_mangle]
pub unsafe extern "C" fn tunnel46_err(
    skb: *mut sk_buff,
    opt: *mut c_void,
    type_: u8,
    code: u8,
    offset: c_int,
    info: u32,
) -> c_int {
    let mut handler: *mut xfrm6_tunnel = tunnel46_handlers;
    while !handler.is_null() {
        if let Some(err_handler) = (*handler).err_handler {
            if err_handler(skb, opt, type_, code, offset, info) == 0 {
                return 0;
            }
        }
        handler = (*handler).next;
    }

    -ENOENT
}

// Implementation of tunnelmpls6_err
#[no_mangle]
pub unsafe extern "C" fn tunnelmpls6_err(
    skb: *mut sk_buff,
    opt: *mut c_void,
    type_: u8,
    code: u8,
    offset: c_int,
    info: u32,
) -> c_int {
    let mut handler: *mut xfrm6_tunnel = tunnelmpls6_handlers;
    while !handler.is_null() {
        if let Some(err_handler) = (*handler).err_handler {
            if err_handler(skb, opt, type_, code, offset, info) == 0 {
                return 0;
            }
        }
        handler = (*handler).next;
    }

    -ENOENT
}

// Module initialization function
#[no_mangle]
pub unsafe extern "C" fn tunnel6_init() -> c_int {
    if inet6_add_protocol(&tunnel6_protocol, IPPROTO_IPV6) != 0 {
        return -EAGAIN;
    }

    if inet6_add_protocol(&tunnel46_protocol, IPPROTO_IPIP) != 0 {
        inet6_del_protocol(&tunnel6_protocol, IPPROTO_IPV6);
        return -EAGAIN;
    }

    if xfrm6_tunnel_mpls_supported() != 0 {
        if inet6_add_protocol(&tunnelmpls6_protocol, IPPROTO_MPLS) != 0 {
            inet6_del_protocol(&tunnel6_protocol, IPPROTO_IPV6);
            inet6_del_protocol(&tunnel46_protocol, IPPROTO_IPIP);
            return -EAGAIN;
        }
    }

    if xfrm6_tunnel_mpls_supported() != 0 {
        if xfrm_input_register_afinfo(&tunnel6_input_afinfo) != 0 {
            inet6_del_protocol(&tunnel6_protocol, IPPROTO_IPV6);
            inet6_del_protocol(&tunnel46_protocol, IPPROTO_IPIP);
            if xfrm6_tunnel_mpls_supported() != 0 {
                inet6_del_protocol(&tunnelmpls6_protocol, IPPROTO_MPLS);
            }
            return -EAGAIN;
        }
    }

    0
}

// Module cleanup function
#[no_mangle]
pub unsafe extern "C" fn tunnel6_fini() {
    if xfrm6_tunnel_mpls_supported() != 0 {
        if xfrm_input_unregister_afinfo(&tunnel6_input_afinfo) != 0 {
            // Handle error
        }
    }

    if inet6_del_protocol(&tunnel46_protocol, IPPROTO_IPIP) != 0 {
        // Handle error
    }

    if inet6_del_protocol(&tunnel6_protocol, IPPROTO_IPV6) != 0 {
        // Handle error
    }

    if xfrm6_tunnel_mpls_supported() != 0 {
        if inet6_del_protocol(&tunnelmpls6_protocol, IPPROTO_MPLS) != 0 {
            // Handle error
        }
    }
}

// Module macros
#[no_mangle]
pub unsafe extern "C" fn module_init() {
    tunnel6_init();
}

#[no_mangle]
pub unsafe extern "C" fn module_exit() {
    tunnel6_fini();
}

// Constants for ICMPv6
pub const ICMPV6_DEST_UNREACH: c_int = 3;
pub const ICMPV6_PORT_UNREACH: c_int = 4;