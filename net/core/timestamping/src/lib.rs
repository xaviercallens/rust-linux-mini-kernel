//! PTP 1588 clock support - support for timestamping in PHY devices
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]

use core::ptr;
use libc::{c_int, c_uint, c_void, size_t};

// Constants from C
pub const PTP_CLASS_NONE: c_uint = 0;
pub const ETH_HLEN: c_uint = 14; // Ethernet header length

// Type definitions
#[repr(C)]
struct sk_buff {
    dev: *mut net_device,
    sk: *mut c_void,
}

#[repr(C)]
struct net_device {
    phydev: *mut phy_device,
}

#[repr(C)]
struct phy_device {
    mii_ts: *mut mii_timestamper,
}

#[repr(C)]
struct mii_timestamper {
    txtstamp: Option<extern "C" fn(*mut mii_timestamper, *mut sk_buff, c_uint)>,
    rxtstamp: Option<extern "C" fn(*mut mii_timestamper, *mut sk_buff, c_uint) -> bool>,
}

// Extern declarations for kernel functions
extern "C" {
    fn skb_clone_sk(skb: *mut sk_buff) -> *mut sk_buff;
    fn ptp_classify_raw(skb: *const sk_buff) -> c_uint;
    fn skb_headroom(skb: *mut sk_buff) -> size_t;
    fn __skb_push(skb: *mut sk_buff, len: size_t);
    fn __skb_pull(skb: *mut sk_buff, len: size_t);
}

// Internal helper function
fn classify(skb: *const sk_buff) -> c_uint {
    unsafe {
        if !skb.is_null() &&
           !(*skb).dev.is_null() &&
           !(*(*skb).dev).phydev.is_null() &&
           !(*(*(*skb).dev).phydev).mii_ts.is_null() {
            ptp_classify_raw(skb)
        } else {
            PTP_CLASS_NONE
        }
    }
}

// Exported functions
/// Clone skb for transmit timestamping
///
/// # Safety
/// - `skb` must be a valid pointer to a `sk_buff` with a valid `sk` field
/// - The `skb->dev->phydev->mii_ts` must be valid and have a non-null `txtstamp` function
/// - Caller must ensure no data races on `skb`
#[no_mangle]
pub unsafe extern "C" fn skb_clone_tx_timestamp(skb: *mut sk_buff) {
    if skb.is_null() {
        return;
    }

    // SAFETY: skb is non-null (checked above)
    if (*skb).sk.is_null() {
        return;
    }

    let type_ = classify(skb);
    if type_ == PTP_CLASS_NONE {
        return;
    }

    let mii_ts = unsafe {
        let phydev = (*(*skb).dev).phydev;
        (*phydev).mii_ts
    };

    if !mii_ts.is_null() {
        let clone = unsafe { skb_clone_sk(skb) };
        if clone.is_null() {
            return;
        }

        // SAFETY: mii_ts.txtstamp is guaranteed to be non-null by the caller
        unsafe {
            let txtstamp = (*mii_ts).txtstamp.expect("txtstamp must be set");
            txtstamp(mii_ts, clone, type_);
        }
    }
}

/// Defer receive timestamping to hardware
///
/// # Safety
/// - `skb` must be a valid pointer to a `sk_buff`
/// - The `skb->dev->phydev->mii_ts` must be valid and have a non-null `rxtstamp` function
/// - Caller must ensure no data races on `skb`
#[no_mangle]
pub unsafe extern "C" fn skb_defer_rx_timestamp(skb: *mut sk_buff) -> bool {
    if skb.is_null() {
        return false;
    }

    let dev = (*skb).dev;
    if dev.is_null() {
        return false;
    }

    let phydev = (*dev).phydev;
    if phydev.is_null() {
        return false;
    }

    let mii_ts = (*phydev).mii_ts;
    if mii_ts.is_null() {
        return false;
    }

    let headroom = skb_headroom(skb);
    if headroom < ETH_HLEN as size_t {
        return false;
    }

    unsafe {
        __skb_push(skb, ETH_HLEN as size_t);
    }

    let type_ = ptp_classify_raw(skb);

    unsafe {
        __skb_pull(skb, ETH_HLEN as size_t);
    }

    if type_ == PTP_CLASS_NONE {
        return false;
    }

    unsafe {
        let rxtstamp = (*mii_ts).rxtstamp.expect("rxtstamp must be set");
        rxtstamp(mii_ts, skb, type_)
    }
}
