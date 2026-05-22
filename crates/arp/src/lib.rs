#![warn(clippy::pedantic)]
#![deny(clippy::all)]
#![allow(clippy::missing_safety_doc)] // Disabled only for concise demonstration of FFI wrappers
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use kernel_types::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arp_tbl {
    pub family: c_int,
    pub key_len: c_int,
    pub hash: *mut c_void,
    pub key_ctor: *mut c_void,
    pub destructor: *mut c_void,
    pub seq_ops: *mut c_void,
    pub arp_parms: *mut c_void,
    pub gc: *mut c_void,
    pub gc_interval: c_int,
    pub gc_thresh1: c_int,
    pub gc_thresh2: c_int,
    pub gc_thresh3: c_int,
    pub last_flush: c_long,
    pub last_rand: c_long,
    pub last_seq: c_long,
    pub entries: c_int,
    pub last_walk: c_long,
    pub rtnl: *mut c_void,
    pub dev: *mut c_void,
    pub stats: *mut c_void,
    pub id: c_char,
    pub parms: *mut c_void,
    pub tb_id: c_char,
    pub owner: *mut c_void,
}

pub const ARPHRD_ETHER: c_int = 1;
pub const ETH_P_IP: c_int = 0x0800;
pub const ETH_HLEN: usize = 14;
pub const ARPOP_REQUEST: c_int = 1;

#[repr(C)]
pub struct net_device { pub type_: c_int }

#[repr(C)]
pub struct arphdr {
    pub ar_hrd: u16,
    pub ar_pro: u16,
    pub ar_hln: u8,
    pub ar_pln: u8,
    pub ar_op: u16,
    pub ar_sip: *mut in_addr,
    pub ar_tip: *mut in_addr,
}

#[repr(C)]
pub struct neighbour { pub dev: *mut net_device, pub ops: *mut ndisc_ops }

#[repr(C)]
pub struct ndisc_ops {
    pub output: Option<unsafe extern "C" fn(*mut sk_buff, *mut neighbour) -> c_int>,
}

#[inline(always)]
#[must_use]
pub fn htons(x: c_int) -> u16 { (x as u16).to_be() }

// Zero-Cost Abstraction Wrapper (Newtype pattern)
pub struct SafeSkb<'a> {
    ptr: *mut sk_buff,
    _marker: core::marker::PhantomData<&'a mut sk_buff>,
}

impl<'a> SafeSkb<'a> {
    pub unsafe fn new(ptr: *mut sk_buff) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self {
                ptr,
                _marker: core::marker::PhantomData,
            })
        }
    }

    #[must_use]
    pub fn dev(&self) -> Option<*mut net_device> {
        // SAFETY: Self cannot be constructed with a null skb pointer.
        let dev_ptr = unsafe { (*self.ptr).dev } as *mut net_device;
        if dev_ptr.is_null() {
            None
        } else {
            Some(dev_ptr)
        }
    }

    #[must_use]
    pub fn data_as<T>(&self, offset: usize) -> Option<*mut T> {
        // SAFETY: Self cannot be constructed with a null skb pointer.
        unsafe {
            let data_ptr = (*self.ptr).data as *mut u8;
            if data_ptr.is_null() {
                None
            } else {
                Some(data_ptr.add(offset) as *mut T)
            }
        }
    }
    #[must_use]
    pub fn dst(&self) -> Option<*mut neighbour> {
        // SAFETY: Self cannot be constructed with a null skb pointer.
        unsafe {
            let dst_ptr = (*self.ptr).dst as *mut neighbour;
            if dst_ptr.is_null() {
                None
            } else {
                Some(dst_ptr)
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn arp_send(
    skb: *mut sk_buff,
    ip: *mut c_void,
) -> c_int {
    // 🛡️ FORMAL VERIFICATION BOUNDARY
    requires!(!skb.is_null(), "arp_send_safety: skb pointer invariant violated");
    requires!(!ip.is_null(), "arp_send_safety: ip pointer invariant violated");

    // Zero-cost abstraction conversion
    let safe_skb = match unsafe { SafeSkb::new(skb) } {
        Some(s) => s,
        None => return -EINVAL,
    };
    
    let dev = match safe_skb.dev() {
        Some(d) => d,
        None => return -EINVAL,
    };

    // SAFETY: Wrapper ensures non-null pointer
    unsafe {
        if (*dev).type_ != ARPHRD_ETHER {
            return -EINVAL;
        }
    }

    let eth = match safe_skb.data_as::<ethhdr>(0) {
        Some(e) => e,
        None => return -EINVAL,
    };

    // SAFETY: Wrapper ensures non-null data pointer, alignment assumed correct for network start
    unsafe {
        if core::ptr::read_unaligned(core::ptr::addr_of!((*eth).h_proto)) != htons(ETH_P_IP) {
            return -EINVAL;
        }
    }

    let arp = match safe_skb.data_as::<arphdr>(ETH_HLEN) {
        Some(a) => a,
        None => return -EINVAL,
    };

    // SAFETY: Unaligned reads for network packet payload fields to prevent ARM panics
    unsafe {
        if core::ptr::read_unaligned(core::ptr::addr_of!((*arp).ar_op)) != htons(ARPOP_REQUEST) {
            return -EINVAL;
        }

        let saddr = core::ptr::read_unaligned(core::ptr::addr_of!((*arp).ar_sip));
        let daddr = core::ptr::read_unaligned(core::ptr::addr_of!((*arp).ar_tip));

        if saddr.is_null() || daddr.is_null() {
            return -EINVAL;
        }

        if core::ptr::read_unaligned(core::ptr::addr_of!((*saddr).s_addr)) == core::ptr::read_unaligned(core::ptr::addr_of!((*daddr).s_addr)) {
            return -EINVAL;
        }

        let dst = match safe_skb.dst() {
            Some(d) => d,
            None => return -EINVAL,
        };

        if (*dst).dev.is_null() || (*dst).dev != dev {
            return -EINVAL;
        }

        if (*dst).ops.is_null() {
            return -EINVAL;
        }

        let ops = (*dst).ops;
        if (*ops).output.is_none() {
            return -EINVAL;
        }

        let result = ((*ops).output.unwrap())(skb, dst);
        ensures!(result <= 0, "arp_send_safety: return code must be 0 or negative error code");
        result
    }
}
