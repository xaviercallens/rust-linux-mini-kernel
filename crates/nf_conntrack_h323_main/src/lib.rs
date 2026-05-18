use kernel_types::*;
use core::ptr;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct h323_call_id {
    pub call_id: [c_char; 128],
    pub call_id_len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct h323_call_signal {
    pub call_id: h323_call_id,
    pub setup: bool,
    pub connect: bool,
    pub release_complete: bool,
    pub call_type: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct h323_call {
    pub call_signal: h323_call_signal,
    pub src_addr: nf_inet_addr,
    pub dst_addr: nf_inet_addr,
    pub src_port: __be16,
    pub dst_port: __be16,
    pub protocol: c_int,
    pub af: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct h323_expect {
    pub call: h323_call,
    pub timeout: c_uint,
    pub flags: c_uint,
    pub master: *mut c_void,
    pub expectfn: Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> c_int>,
    pub expect_data: *mut c_void,
}

#[no_mangle]
pub extern "C" fn h323_expect_create(
    call: *mut h323_call,
    timeout: c_uint,
    flags: c_uint,
    master: *mut c_void,
    expectfn: Option<extern "C" fn(*mut c_void, *mut c_void, *mut c_void) -> c_int>,
    expect_data: *mut c_void,
) -> *mut h323_expect {
    if call.is_null() {
        return ptr::null_mut();
    }

    let layout = core::alloc::Layout::new::<h323_expect>();
    let expect_ptr = unsafe { core::alloc::alloc(layout) };

    if expect_ptr.is_null() {
        return ptr::null_mut();
    }

    unsafe {
        let expect = expect_ptr as *mut h323_expect;
        ptr::write(expect, h323_expect {
            call: *call,
            timeout,
            flags,
            master,
            expectfn,
            expect_data,
        });
        expect
    }
}

#[no_mangle]
pub extern "C" fn h323_expect_destroy(expect: *mut h323_expect) {
    if !expect.is_null() {
        unsafe {
            ptr::drop_in_place(expect);
            let layout = core::alloc::Layout::new::<h323_expect>();
            core::alloc::dealloc(expect as *mut u8, layout);
        }
    }
}

#[no_mangle]
pub extern "C" fn h323_expect_match(
    expect: *mut h323_expect,
    call: *mut h323_call,
) -> bool {
    if expect.is_null() || call.is_null() {
        return false;
    }

    unsafe {
        let expect_call = &(*expect).call;
        let match_call = &*call;

        if expect_call.call_signal.call_id.call_id_len != match_call.call_signal.call_id.call_id_len {
            return false;
        }

        for i in 0..expect_call.call_signal.call_id.call_id_len {
            if expect_call.call_signal.call_id.call_id[i as usize] != match_call.call_signal.call_id.call_id[i as usize] {
                return false;
            }
        }

        if expect_call.src_addr.all != match_call.src_addr.all {
            return false;
        }

        if expect_call.dst_addr.all != match_call.dst_addr.all {
            return false;
        }

        if expect_call.src_port != match_call.src_port {
            return false;
        }

        if expect_call.dst_port != match_call.dst_port {
            return false;
        }

        if expect_call.protocol != match_call.protocol {
            return false;
        }

        if expect_call.af != match_call.af {
            return false;
        }

        true
    }
}

#[no_mangle]
pub extern "C" fn h323_expect_attach(
    expect: *mut h323_expect,
    skb: *mut sk_buff,
) -> c_int {
    if expect.is_null() || skb.is_null() {
        return -1;
    }

    unsafe {
        if (*expect).expectfn.is_none() {
            return 0;
        }

        let expectfn = (*expect).expectfn.unwrap();
        let result = expectfn((*expect).master, skb as *mut c_void, (*expect).expect_data);

        result
    }
}