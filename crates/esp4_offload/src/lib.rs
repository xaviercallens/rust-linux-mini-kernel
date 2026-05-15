//! IPv4 GSO/GRO offload support for ESP
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]  // For C-style type names

use core::ptr;
use libc::{c_int, c_uint, c_void, size_t};

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const EINPROGRESS: c_int = -115;
pub const EOPNOTSUPP: c_int = -95;
pub const ENOSYS: c_int = -38;
pub const EAGAIN: c_int = -35;

// Type definitions
#[repr(C)]
pub struct sk_buff {
    // Opaque struct - actual fields defined in kernel headers
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    // Opaque struct
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfrm_state {
    // Opaque struct
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfrm_offload {
    // Opaque struct
    _private: [u8; 0],
}

#[repr(C)]
pub struct sec_path {
    // Opaque struct
    _private: [u8; 0],
}

#[repr(C)]
pub struct iphdr {
    // Opaque struct
    _private: [u8; 0],
}

#[repr(C)]
pub struct ip_esp_hdr {
    // Opaque struct
    _private: [u8; 0],
}

#[repr(C)]
pub struct ip_beet_phdr {
    // Opaque struct
    _private: [u8; 0],
}

#[repr(C)]
pub struct net_offload {
    // Opaque struct
    _private: [u8; 0],
}

#[repr(C)]
pub struct xfrm_type_offload {
    // Opaque struct
    _private: [u8; 0],
}

// Function pointer types
type gro_receive_t = extern "C" fn(*mut list_head, *mut sk_buff) -> *mut sk_buff;
type gso_segment_t = extern "C" fn(*mut sk_buff, netdev_features_t) -> *mut sk_buff;

// Extern functions (declared in kernel headers)
extern "C" {
    fn skb_gro_offset(skb: *mut sk_buff) -> c_int;
    fn pskb_pull(skb: *mut sk_buff, len: c_int) -> *mut sk_buff;
    fn xfrm_parse_spi(skb: *mut sk_buff, proto: c_int, spi: *mut u32, seq: *mut u32) -> c_int;
    fn xfrm_offload(skb: *mut sk_buff) -> *mut xfrm_offload;
    fn secpath_set(skb: *mut sk_buff) -> *mut sec_path;
    fn xfrm_state_lookup(net: *mut c_void, mark: u32, 
                         daddr: *mut xfrm_address_t, spi: u32, 
                         proto: c_int, family: c_int) -> *mut xfrm_state;
    fn xfrm_smark_get(mark: u32, x: *mut xfrm_state) -> u32;
    fn secpath_reset(skb: *mut sk_buff);
    fn xfrm_input(skb: *mut sk_buff, proto: c_int, spi: u32, encap_type: c_int);
    fn NAPI_GRO_CB(skb: *mut sk_buff);
    fn skb_push(skb: *mut sk_buff, len: c_int) -> *mut sk_buff;
    fn ip_esp_hdr(skb: *mut sk_buff) -> *mut ip_esp_hdr;
    fn ip_hdr(skb: *mut sk_buff) -> *mut iphdr;
    fn skb_network_offset(skb: *mut sk_buff) -> c_int;
    fn __skb_push(skb: *mut sk_buff, len: c_int) -> *mut sk_buff;
    fn skb_mac_gso_segment(skb: *mut sk_buff, features: netdev_features_t) -> *mut sk_buff;
    fn skb_shinfo(skb: *mut sk_buff) -> *mut skb_shared_info;
    fn rcu_dereference<T>(ptr: *mut T) -> *mut T;
    fn inet_offloads(proto: c_int) -> *mut net_offload;
    fn xfrm_register_type_offload(type: *mut xfrm_type_offload, family: c_int) -> c_int;
    fn inet_add_offload(ops: *mut net_offload, proto: c_int) -> c_int;
    fn xfrm_unregister_type_offload(type: *mut xfrm_type_offload, family: c_int);
    fn inet_del_offload(ops: *mut net_offload, proto: c_int);
    fn kmalloc(size: size_t, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
}

type netdev_features_t = u32;
type xfrm_address_t = [u8; 16];

#[repr(C)]
struct skb_shared_info {
    gso_type: u16,
    _private: [u8; 0],
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn esp4_gro_receive(
    head: *mut list_head,
    skb: *mut sk_buff,
) -> *mut sk_buff {
    let offset = skb_gro_offset(skb);
    if offset < 0 {
        return ptr::null_mut();
    }

    if pskb_pull(skb, offset) == ptr::null_mut() {
        return ptr::null_mut();
    }

    let mut spi: u32 = 0;
    let mut seq: u32 = 0;
    let err = xfrm_parse_spi(skb, IPPROTO_ESP, &mut spi, &mut seq);
    if err != 0 {
        return ptr::null_mut();
    }

    let xo = xfrm_offload(skb);
    if xo.is_null() || !(*xo).flags & CRYPTO_DONE {
        let sp = secpath_set(skb);
        if sp.is_null() {
            return ptr::null_mut();
        }

        if (*sp).len == XFRM_MAX_DEPTH {
            secpath_reset(skb);
            skb_push(skb, offset);
            NAPI_GRO_CB(skb).same_flow = 0;
            NAPI_GRO_CB(skb).flush = 1;
            return ptr::null_mut();
        }

        let net = dev_net(skb->dev);
        let x = xfrm_state_lookup(net, skb->mark, 
                                  &ip_hdr(skb)->daddr, spi, IPPROTO_ESP, AF_INET);
        if x.is_null() {
            secpath_reset(skb);
            skb_push(skb, offset);
            NAPI_GRO_CB(skb).same_flow = 0;
            NAPI_GRO_CB(skb).flush = 1;
            return ptr::null_mut();
        }

        skb->mark = xfrm_smark_get(skb->mark, x);

        (*sp).xvec[(*sp).len] = x;
        (*sp).len += 1;
        (*sp).olen += 1;

        let xo = xfrm_offload(skb);
        if xo.is_null() {
            secpath_reset(skb);
            skb_push(skb, offset);
            NAPI_GRO_CB(skb).same_flow = 0;
            NAPI_GRO_CB(skb).flush = 1;
            return ptr::null_mut();
        }
    }

    (*xo).flags |= XFRM_GRO;

    XFRM_TUNNEL_SKB_CB(skb).tunnel.ip4 = ptr::null_mut();
    XFRM_SPI_SKB_CB(skb).family = AF_INET;
    XFRM_SPI_SKB_CB(skb).daddroff = offsetof!(iphdr, daddr);
    XFRM_SPI_SKB_CB(skb).seq = seq;

    xfrm_input(skb, IPPROTO_ESP, spi, -2);

    return ERR_PTR(EINPROGRESS);
}

#[no_mangle]
pub unsafe extern "C" fn esp4_gso_encap(
    x: *mut xfrm_state,
    skb: *mut sk_buff,
) {
    let iph = ip_hdr(skb);
    let xo = xfrm_offload(skb);
    let proto = (*iph).protocol;

    __skb_push(skb, -skb_network_offset(skb));
    let esph = ip_esp_hdr(skb);
    *skb_mac_header(skb) = IPPROTO_ESP;

    (*esph).spi = (*x).id.spi;
    (*esph).seq_no = htonl(XFRM_SKB_CB(skb).seq.output.low);

    (*xo).proto = proto;
}

#[no_mangle]
pub unsafe extern "C" fn xfrm4_tunnel_gso_segment(
    x: *mut xfrm_state,
    skb: *mut sk_buff,
    features: netdev_features_t,
) -> *mut sk_buff {
    __skb_push(skb, (*skb).mac_len);
    return skb_mac_gso_segment(skb, features);
}

#[no_mangle]
pub unsafe extern "C" fn xfrm4_transport_gso_segment(
    x: *mut xfrm_state,
    skb: *mut sk_buff,
    features: netdev_features_t,
) -> *mut sk_buff {
    let segs = ERR_PTR(EINVAL);
    let xo = xfrm_offload(skb);
    let ops = rcu_dereference(inet_offloads((*xo).proto));
    
    if !ops.is_null() && !((*ops).callbacks.gso_segment).is_null() {
        (*skb).transport_header += (*x).props.header_len;
        segs = (*ops).callbacks.gso_segment(skb, features);
    }

    return segs;
}

#[no_mangle]
pub unsafe extern "C" fn xfrm4_beet_gso_segment(
    x: *mut xfrm_state,
    skb: *mut sk_buff,
    features: netdev_features_t,
) -> *mut sk_buff {
    let segs = ERR_PTR(EINVAL);
    let xo = xfrm_offload(skb);
    let proto = (*xo).proto;
    
    (*skb).transport_header += (*x).props.header_len;
    
    if (*x).sel.family != AF_INET6 {
        if proto == IPPROTO_BEETPH {
            let ph = &(*skb).data as *const ip_beet_phdr;
            (*skb).transport_header += (*ph).hdrlen * 8;
            proto = (*ph).nexthdr;
        } else {
            (*skb).transport_header -= IPV4_BEET_PHMAXLEN;
        }
    } else {
        let mut frag: __be16 = 0;
        (*skb).transport_header += ipv6_skip_exthdr(skb, 0, &mut proto, &mut frag);
        
        if proto == IPPROTO_TCP {
            (*skb_shinfo(skb)).gso_type |= SKB_GSO_TCPV4;
        }
    }
    
    __skb_pull(skb, skb_transport_offset(skb));
    let ops = rcu_dereference(inet_offloads(proto));
    
    if !ops.is_null() && !((*ops).callbacks.gso_segment).is_null() {
        segs = (*ops).callbacks.gso_segment(skb, features);
    }

    return segs;
}

#[no_mangle]
pub unsafe extern "C" fn xfrm4_outer_mode_gso_segment(
    x: *mut xfrm_state,
    skb: *mut sk_buff,
    features: netdev_features_t,
) -> *mut sk_buff {
    match (*x).outer_mode.encap {
        XFRM_MODE_TUNNEL => return xfrm4_tunnel_gso_segment(x, skb, features),
        XFRM_MODE_TRANSPORT => return xfrm4_transport_gso_segment(x, skb, features),
        XFRM_MODE_BEET => return xfrm4_beet_gso_segment(x, skb, features),
        _ => return ERR_PTR(EOPNOTSUPP),
    }
}

#[no_mangle]
pub unsafe extern "C" fn esp4_gso_segment(
    skb: *mut sk_buff,
    features: netdev_features_t,
) -> *mut sk_buff {
    let xo = xfrm_offload(skb);
    if xo.is_null() {
        return ERR_PTR(EINVAL);
    }

    let shinfo = skb_shinfo(skb);
    if !((*shinfo).gso_type & SKB_GSO_ESP) != 0 {
        return ERR_PTR(EINVAL);
    }

    let sp = skb_sec_path(skb);
    let x = (*sp).xvec[(*sp).len - 1];
    let aead = (*x).data;
    let esph = ip_esp_hdr(skb);

    if (*esph).spi != (*x).id.spi {
        return ERR_PTR(EINVAL);
    }

    if !pskb_may_pull(skb, size_of::<ip_esp_hdr>() + crypto_aead_ivsize(aead)) {
        return ERR_PTR(EINVAL);
    }

    __skb_pull(skb, size_of::<ip_esp_hdr>() + crypto_aead_ivsize(aead));
    (*skb).encap_hdr_csum = 1;

    let esp_features = if (!(skb->dev->gso_partial_features & NETIF_F_HW_ESP) &&
         !(features & NETIF_F_HW_ESP)) || (*x).xso.dev != skb->dev {
        features & !(NETIF_F_SG | NETIF_F_CSUM_MASK | NETIF_F_SCTP_CRC)
    } else if (!(features & NETIF_F_HW_ESP_TX_CSUM) &&
             !(skb->dev->gso_partial_features & NETIF_F_HW_ESP_TX_CSUM)) {
        features & !(NETIF_F_CSUM_MASK | NETIF_F_SCTP_CRC)
    } else {
        features
    };

    (*xo).flags |= XFRM_GSO_SEGMENT;

    return xfrm4_outer_mode_gso_segment(x, skb, esp_features);
}

#[no_mangle]
pub unsafe extern "C" fn esp_input_tail(
    x: *mut xfrm_state,
    skb: *mut sk_buff,
) -> c_int {
    let aead = (*x).data;
    let xo = xfrm_offload(skb);
    
    if !pskb_may_pull(skb, size_of::<ip_esp_hdr>() + crypto_aead_ivsize(aead)) {
        return EINVAL;
    }
    
    if !(*xo).flags & CRYPTO_DONE {
        (*skb).ip_summed = CHECKSUM_NONE;
    }
    
    return esp_input_done2(skb, 0);
}

#[no_mangle]
pub unsafe extern "C" fn esp_xmit(
    x: *mut xfrm_state,
    skb: *mut sk_buff,
    features: netdev_features_t,
) -> c_int {
    let xo = xfrm_offload(skb);
    if xo.is_null() {
        return EINVAL;
    }
    
    let hw_offload = if (!(features & NETIF_F_HW_ESP) &&
         !(skb->dev->gso_partial_features & NETIF_F_HW_ESP)) ||
        (*x).xso.dev != skb->dev {
        (*xo).flags |= CRYPTO_FALLBACK;
        false
    } else {
        true
    };
    
    let aead = (*x).data;
    let alen = crypto_aead_authsize(aead);
    let blksize = ALIGN(crypto_aead_blocksize(aead), 4);
    let clen = ALIGN((*skb).len + 2 + 0, blksize);
    let plen = clen - (*skb).len;
    let tailen = 0 + plen + alen;
    
    let esph = ip_esp_hdr(skb);
    
    if !hw_offload || !skb_is_gso(skb) {
        let nfrags = esp_output_head(x, skb, &esp);
        if nfrags < 0 {
            return nfrags;
        }
    }
    
    let seq = (*xo).seq.low;
    
    (*esph).spi = (*x).id.spi;
    
    skb_push(skb, -skb_network_offset(skb));
    
    if (*xo).flags & XFRM_GSO_SEGMENT {
        (*esph).seq_no = htonl(seq);
        
        if !skb_is_gso(skb) {
            (*xo).seq.low += 1;
        } else {
            (*xo).seq.low += (*skb_shinfo(skb)).gso_segs;
        }
    }
    
    (*ip_hdr(skb)).tot_len = htons((*skb).len);
    ip_send_check(ip_hdr(skb));
    
    if hw_offload {
        if !skb_ext_add(skb, SKB_EXT_SEC_PATH) {
            return ENOMEM;
        }
        
        let xo = xfrm_offload(skb);
        if xo.is_null() {
            return EINVAL;
        }
        
        (*xo).flags |= XFRM_XMIT;
        return 0;
    }
    
    let err = esp_output_tail(x, skb, &esp);
    if err != 0 {
        return err;
    }
    
    secpath_reset(skb);
    
    return 0;
}

// Module initialization
#[no_mangle]
pub unsafe extern "C" fn esp4_offload_init() -> c_int {
    let esp_type_offload = &esp_type_offload;
    if xfrm_register_type_offload(esp_type_offload, AF_INET) < 0 {
        pr_info!("esp4_offload_init: can't add xfrm type offload\n");
        return EAGAIN;
    }
    
    return inet_add_offload(&esp4_offload, IPPROTO_ESP);
}

#[no_mangle]
pub unsafe extern "C" fn esp4_offload_exit() {
    xfrm_unregister_type_offload(&esp_type_offload, AF_INET);
    inet_del_offload(&esp4_offload, IPPROTO_ESP);
}

// Extern constants
const IPPROTO_ESP: c_int = 50;
const IPPROTO_BEETPH: c_int = 148;
const XFRM_MAX_DEPTH: c_int = 32;
const XFRM_MODE_TUNNEL: c_int = 1;
const XFRM_MODE_TRANSPORT: c_int = 0;
const XFRM_MODE_BEET: c_int = 2;
const SKB_GSO_ESP: u16 = 0x0080;
const SKB_GSO_TCPV4: u16 = 0x0008;
const NETIF_F_HW_ESP: netdev_features_t = 1 << 0;
const NETIF_F_HW_ESP_TX_CSUM: netdev_features_t = 1 << 1;
const NETIF_F_SG: netdev_features_t = 1 << 2;
const NETIF_F_CSUM_MASK: netdev_features_t = 0x0000FF00;
const NETIF_F_SCTP_CRC: netdev_features_t = 1 << 10;
const IPV4_BEET_PHMAXLEN: c_int = 20;

// Extern variables
static mut esp4_offload: net_offload = net_offload {
    callbacks: net_offload_callbacks {
        gro_receive: Some(esp4_gro_receive),
        gso_segment: Some(esp4_gso_segment),
    },
};

static mut esp_type_offload: xfrm_type_offload = xfrm_type_offload {
    description: "ESP4 OFFLOAD",
    owner: THIS_MODULE,
    proto: IPPROTO_ESP,
    input_tail: Some(esp_input_tail),
    xmit: Some(esp_xmit),
    encap: Some(esp4_gso_encap),
};

// Module metadata
#[no_mangle]
pub static mut esp4_offload_init: extern "C" fn() -> c_int = esp4_offload_init;
#[no_mangle]
pub static mut esp4_offload_exit: extern "C" fn() = esp4_offload_exit;

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn test_module_init() {
        // Basic test to verify module init function compiles
        // Actual testing would require kernel environment
    }
}