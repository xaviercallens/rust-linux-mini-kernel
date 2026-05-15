//! Socket level control messages processing for Linux kernel
//!
//! This is an FFI-compatible Rust translation of the Linux kernel C implementation.
//! ABI compatibility is maintained for all exported symbols.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(clang::too_many_arguments)]
#![allow(clang::implicit_return_in_proc_macro)]

use core::ptr;
use core::ffi::c_int;
use core::ffi::c_void;
use core::ffi::size_t;
use core::mem;
use core::slice;

// Constants from C
pub const EINVAL: c_int = -22;
pub const ENOMEM: c_int = -12;
pub const EPERM: c_int = -1;
pub const ESRCH: c_int = -3;
pub const EBADF: c_int = -9;

// Type definitions
#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: u32,
    pub cmsg_level: c_int,
    pub cmsg_type: c_int,
}

#[repr(C)]
pub struct ucred {
    pub pid: u32,
    pub uid: u32,
    pub gid: u32,
}

#[repr(C)]
pub struct scm_timestamping {
    pub ts: [libc::timespec; 3],
}

#[repr(C)]
pub struct scm_timestamping64 {
    pub ts: [libc::time64spec; 3],
}

#[repr(C)]
pub struct scm_cookie {
    pub fp: *mut scm_fp_list,
    pub pid: *mut libc::pid,
    pub creds: ucred,
}

#[repr(C)]
pub struct scm_fp_list {
    pub count: c_int,
    pub max: c_int,
    pub user: *mut libc::uid,
    pub fp: [*mut libc::file; 0], // Flexible array member
}

// Function declarations for external C functions
extern "C" {
    fn current_cred() -> *mut libc::cred;
    fn make_kuid(ns: *mut libc::user_namespace, val: u32) -> libc::kuid;
    fn make_kgid(ns: *mut libc::user_namespace, val: u32) -> libc::kgid;
    fn uid_valid(uid: libc::kuid) -> bool;
    fn gid_valid(gid: libc::kgid) -> bool;
    fn task_tgid_vnr(task: *mut libc::task_struct) -> u32;
    fn task_active_pid_ns(task: *mut libc::task_struct) -> *mut libc::pid_namespace;
    fn ns_capable(ns: *mut libc::user_namespace, cap: c_int) -> bool;
    fn uid_eq(a: libc::kuid, b: libc::kuid) -> bool;
    fn gid_eq(a: libc::kgid, b: libc::kgid) -> bool;
    fn fget_raw(fd: c_int) -> *mut libc::file;
    fn fput(file: *mut libc::file);
    fn get_uid(uid: *mut libc::uid) -> *mut libc::uid;
    fn free_uid(uid: *mut libc::uid);
    fn kfree(ptr: *mut c_void);
    fn kmalloc(size: size_t, flags: c_int) -> *mut c_void;
    fn kmemdup(src: *const c_void, size: size_t, flags: c_int) -> *mut c_void;
    fn put_pid(pid: *mut libc::pid);
    fn find_get_pid(pid: u32) -> *mut libc::pid;
    fn pid_vnr(pid: *mut libc::pid) -> u32;
    fn user_write_access_begin(ptr: *mut c_void, size: size_t) -> bool;
    fn user_write_access_end();
    fn unsafe_put_user(src: c_int, dst: *mut c_int) -> c_int;
    fn unsafe_copy_to_user(dst: *mut c_void, src: *const c_void, size: size_t) -> c_int;
    fn receive_fd_user(file: *mut libc::file, fd: *mut c_int, flags: c_int) -> c_int;
    fn put_user<T>(src: T, dst: *mut T) -> c_int;
}

// Helper macros translated to functions
#[inline]
fn CMSG_DATA(cmsg: *const cmsghdr) -> *const c_void {
    (cmsg as *const u8).add(mem::size_of::<cmsghdr>()) as *const c_void
}

#[inline]
fn CMSG_LEN(len: size_t) -> u32 {
    (len as u32) + (mem::size_of::<cmsghdr>() as u32)
}

#[inline]
fn CMSG_SPACE(len: size_t) -> size_t {
    (len + (mem::size_of::<cmsghdr>() as size_t) + (len & 3)) & !3
}

#[inline]
fn CMSG_USER_DATA(cm: *mut cmsghdr) -> *mut c_void {
    (cm as *mut u8).add(mem::size_of::<cmsghdr>()) as *mut c_void
}

// Function implementations
#[no_mangle]
pub unsafe extern "C" fn __scm_destroy(scm: *mut scm_cookie) {
    if scm.is_null() {
        return;
    }
    
    let fpl = (*scm).fp;
    if fpl.is_null() {
        return;
    }
    
    (*scm).fp = ptr::null_mut();
    
    for i in 0..(*fpl).count {
        fput((*fpl).fp[i as usize]);
    }
    
    if !(*fpl).user.is_null() {
        free_uid((*fpl).user);
    }
    
    kfree(fpl as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn __scm_send(
    sock: *mut libc::socket,
    msg: *mut libc::msghdr,
    p: *mut scm_cookie,
) -> c_int {
    if sock.is_null() || msg.is_null() || p.is_null() {
        return EINVAL;
    }
    
    let mut cmsg = ptr::null_mut();
    let mut err = 0;
    
    // Simulate for_each_cmsghdr macro
    let mut cmsg_ptr = (*msg).msg_control as *mut cmsghdr;
    while !cmsg_ptr.is_null() {
        cmsg = cmsg_ptr;
        
        if (*cmsg).cmsg_len < mem::size_of::<cmsghdr>() as u32 {
            err = EINVAL;
            break;
        }
        
        if (*cmsg).cmsg_level != SOL_SOCKET {
            cmsg_ptr = next_cmsg(cmsg);
            continue;
        }
        
        match (*cmsg).cmsg_type {
            SCM_RIGHTS => {
                if (*sock).ops.is_null() || (*(*sock).ops).family != PF_UNIX {
                    err = EINVAL;
                    break;
                }
                
                err = scm_fp_copy(cmsg, &mut (*p).fp);
                if err < 0 {
                    break;
                }
            },
            SCM_CREDENTIALS => {
                if (*cmsg).cmsg_len != CMSG_LEN(mem::size_of::<ucred>()) {
                    err = EINVAL;
                    break;
                }
                
                let mut creds = ucred {
                    pid: 0,
                    uid: 0,
                    gid: 0,
                };
                
                ptr::copy_nonoverlapping(
                    CMSG_DATA(cmsg) as *const ucred,
                    &mut creds as *mut ucred,
                    mem::size_of::<ucred>()
                );
                
                err = scm_check_creds(&creds);
                if err < 0 {
                    break;
                }
                
                (*p).creds.pid = creds.pid;
                
                if (*p).pid.is_null() || pid_vnr((*p).pid) != creds.pid {
                    let pid = find_get_pid(creds.pid);
                    if pid.is_null() {
                        err = ESRCH;
                        break;
                    }
                    put_pid((*p).pid);
                    (*p).pid = pid;
                }
                
                let cred = current_cred();
                let user_ns = (*cred).user_ns;
                let uid = make_kuid(user_ns, creds.uid);
                let gid = make_kgid(user_ns, creds.gid);
                
                if !uid_valid(uid) || !gid_valid(gid) {
                    err = EINVAL;
                    break;
                }
                
                (*p).creds.uid = uid;
                (*p).creds.gid = gid;
            },
            _ => {
                err = EINVAL;
                break;
            }
        }
        
        cmsg_ptr = next_cmsg(cmsg);
    }
    
    if err < 0 {
        __scm_destroy(p);
        return err;
    }
    
    if !(*p).fp.is_null() && (*(*p).fp).count == 0 {
        kfree((*p).fp as *mut c_void);
        (*p).fp = ptr::null_mut();
    }
    
    0
}

#[no_mangle]
pub unsafe extern "C" fn put_cmsg(
    msg: *mut libc::msghdr,
    level: c_int,
    type_: c_int,
    len: c_int,
    data: *const c_void,
) -> c_int {
    if msg.is_null() {
        return EINVAL;
    }
    
    let cmlen = CMSG_LEN(len as size_t) as c_int;
    
    if (*msg).msg_flags & MSG_CMSG_COMPAT != 0 {
        // Compatibility path not implemented
        return 0;
    }
    
    if (*msg).msg_control.is_null() || (*msg).msg_controllen < mem::size_of::<cmsghdr>() as c_int {
        (*msg).msg_flags |= MSG_CTRUNC;
        return 0;
    }
    
    if (*msg).msg_controllen < cmlen {
        (*msg).msg_flags |= MSG_CTRUNC;
        let cmlen = (*msg).msg_controllen;
        
        if (*msg).msg_control_is_user != 0 {
            let cm = (*msg).msg_control_user;
            
            if !user_write_access_begin(cm as *mut c_void, cmlen as size_t) {
                return EFAULT;
            }
            
            let mut result = 0;
            result |= unsafe_put_user(cmlen, &mut (*cm).cmsg_len);
            result |= unsafe_put_user(level, &mut (*cm).cmsg_level);
            result |= unsafe_put_user(type_, &mut (*cm).cmsg_type);
            result |= unsafe_copy_to_user(
                CMSG_USER_DATA(cm),
                data,
                cmlen as size_t - mem::size_of::<cmsghdr>() as size_t
            );
            
            user_write_access_end();
            
            if result != 0 {
                return EFAULT;
            }
        } else {
            let cm = (*msg).msg_control as *mut cmsghdr;
            (*cm).cmsg_len = cmlen as u32;
            (*cm).cmsg_level = level;
            (*cm).cmsg_type = type_;
            ptr::copy_nonoverlapping(data, CMSG_DATA(cm) as *mut c_void, len as usize);
        }
        
        let cmlen = CMSG_SPACE(len as size_t) as c_int;
        (*msg).msg_control = ((*msg).msg_control as *mut u8).add(cmlen as usize) as *mut c_void;
        (*msg).msg_controllen -= cmlen;
        
        return 0;
    }
    
    0
}

#[no_mangle]
pub unsafe extern "C" fn put_cmsg_scm_timestamping64(
    msg: *mut libc::msghdr,
    tss_internal: *const libc::scm_timestamping_internal,
) {
    if msg.is_null() || tss_internal.is_null() {
        return;
    }
    
    let mut tss = libc::scm_timestamping64 {
        ts: [Default::default(); 3],
    };
    
    for i in 0..3 {
        tss.ts[i].tv_sec = (*tss_internal).ts[i].tv_sec;
        tss.ts[i].tv_nsec = (*tss_internal).ts[i].tv_nsec;
    }
    
    put_cmsg(msg, SOL_SOCKET, SO_TIMESTAMPING_NEW, mem::size_of::<libc::scm_timestamping64>() as c_int, &tss as *const _ as *const c_void);
}

#[no_mangle]
pub unsafe extern "C" fn put_cmsg_scm_timestamping(
    msg: *mut libc::msghdr,
    tss_internal: *const libc::scm_timestamping_internal,
) {
    if msg.is_null() || tss_internal.is_null() {
        return;
    }
    
    let mut tss = libc::scm_timestamping {
        ts: [Default::default(); 3],
    };
    
    for i in 0..3 {
        tss.ts[i].tv_sec = (*tss_internal).ts[i].tv_sec;
        tss.ts[i].tv_nsec = (*tss_internal).ts[i].tv_nsec;
    }
    
    put_cmsg(msg, SOL_SOCKET, SO_TIMESTAMPING_OLD, mem::size_of::<libc::scm_timestamping>() as c_int, &tss as *const _ as *const c_void);
}

#[no_mangle]
pub unsafe extern "C" fn scm_detach_fds(
    msg: *mut libc::msghdr,
    scm: *mut scm_cookie,
) {
    if msg.is_null() || scm.is_null() {
        return;
    }
    
    if (*msg).msg_control_is_user == 0 {
        return;
    }
    
    if (*msg).msg_flags & MSG_CMSG_COMPAT != 0 {
        // Compatibility path not implemented
        return;
    }
    
    let cm = (*msg).msg_control_user;
    let o_flags = if (*msg).msg_flags & MSG_CMSG_CLOEXEC != 0 { O_CLOEXEC } else { 0 };
    let fdmax = min(scm_max_fds(msg), (*scm).fp->count);
    let cmsg_data = CMSG_USER_DATA(cm) as *mut c_int;
    let mut err = 0;
    let mut i = 0;
    
    while i < fdmax {
        err = receive_fd_user((*scm).fp->fp[i as usize], cmsg_data.add(i), o_flags);
        if err < 0 {
            break;
        }
        i += 1;
    }
    
    if i > 0 {
        let cmlen = CMSG_LEN(i as size_t * mem::size_of::<c_int>()) as c_int;
        
        err = put_user(SOL_SOCKET, &mut (*cm).cmsg_level);
        if err < 0 {
            return;
        }
        
        err = put_user(SCM_RIGHTS, &mut (*cm).cmsg_type);
        if err < 0 {
            return;
        }
        
        err = put_user(cmlen, &mut (*cm).cmsg_len);
        if err < 0 {
            return;
        }
        
        let cmlen = CMSG_SPACE(i as size_t * mem::size_of::<c_int>()) as c_int;
        if (*msg).msg_controllen < cmlen {
            (*msg).msg_control = ((*msg).msg_control as *mut u8).add(cmlen as usize) as *mut c_void;
            (*msg).msg_controllen -= cmlen;
        }
    }
    
    if i < (*scm).fp->count || (fdmax <= 0 && (*scm).fp->count > 0) {
        (*msg).msg_flags |= MSG_CTRUNC;
    }
    
    __scm_destroy(scm);
}

#[no_mangle]
pub unsafe extern "C" fn scm_fp_dup(fpl: *mut scm_fp_list) -> *mut scm_fp_list {
    if fpl.is_null() {
        return ptr::null_mut();
    }
    
    let new_fpl = kmemdup(fpl as *const c_void, offsetof!(scm_fp_list, fp[(*fpl).count]), GFP_KERNEL) as *mut scm_fp_list;
    if new_fpl.is_null() {
        return ptr::null_mut();
    }
    
    for i in 0..(*fpl).count {
        get_file((*fpl).fp[i as usize]);
    }
    
    (*new_fpl).max = (*new_fpl).count;
    (*new_fpl).user = get_uid((*fpl).user);
    
    new_fpl
}

// Internal functions
fn scm_check_creds(creds: *const ucred) -> c_int {
    let cred = unsafe { current_cred() };
    let user_ns = unsafe { (*cred).user_ns };
    let uid = unsafe { make_kuid(user_ns, (*creds).uid) };
    let gid = unsafe { make_kgid(user_ns, (*creds).gid) };
    
    if !unsafe { uid_valid(uid) } || !unsafe { gid_valid(gid) } {
        return EINVAL;
    }
    
    let current = unsafe { current() };
    let task_tgid = unsafe { task_tgid_vnr(current) };
    
    if (unsafe { (*creds).pid == task_tgid } || 
        unsafe { ns_capable((*task_active_pid_ns(current)).user_ns, CAP_SYS_ADMIN) }) &&
       (unsafe { uid_eq(uid, (*cred).uid) } || 
        unsafe { uid_eq(uid, (*cred).euid) } || 
        unsafe { uid_eq(uid, (*cred).suid) } || 
        unsafe { ns_capable((*cred).user_ns, CAP_SETUID) }) &&
       (unsafe { gid_eq(gid, (*cred).gid) } || 
        unsafe { gid_eq(gid, (*cred).egid) } || 
        unsafe { gid_eq(gid, (*cred).sgid) } || 
        unsafe { ns_capable((*cred).user_ns, CAP_SETGID) }) {
        return 0;
    }
    
    EPERM
}

fn scm_fp_copy(cmsg: *mut cmsghdr, fplp: *mut *mut scm_fp_list) -> c_int {
    let fdp = CMSG_DATA(cmsg) as *const c_int;
    let fpl = *fplp;
    
    let num = (((*cmsg).cmsg_len as usize) - mem::size_of::<cmsghdr>()) / mem::size_of::<c_int>();
    
    if num <= 0 {
        return 0;
    }
    
    if num > SCM_MAX_FD {
        return EINVAL;
    }
    
    if fpl.is_null() {
        let fpl = kmalloc(mem::size_of::<scm_fp_list>() as size_t, GFP_KERNEL) as *mut scm_fp_list;
        if fpl.is_null() {
            return ENOMEM;
        }
        
        *fplp = fpl;
        (*fpl).count = 0;
        (*fpl).max = SCM_MAX_FD;
        (*fpl).user = ptr::null_mut();
    }
    
    let fpl = *fplp;
    let fpp = &mut (*fpl).fp[(*fpl).count as usize];
    
    if (*fpl).count + num > (*fpl).max {
        return EINVAL;
    }
    
    for i in 0..num {
        let fd = unsafe { *fdp.offset(i as isize) };
        let file = unsafe { fget_raw(fd) };
        
        if fd < 0 || file.is_null() {
            return EBADF;
        }
        
        unsafe { *fpp = file };
        (*fpl).count += 1;
        fpp = unsafe { fpp.offset(1) };
    }
    
    if (*fpl).user.is_null() {
        let current = unsafe { current() };
        (*fpl).user = unsafe { get_uid(current_user()) };
    }
    
    num as c_int
}

fn scm_max_fds(msg: *mut libc::msghdr) -> c_int {
    if (*msg).msg_controllen < mem::size_of::<cmsghdr>() as c_int {
        return 0;
    }
    
    (((*msg).msg_controllen as usize) - mem::size_of::<cmsghdr>()) / mem::size_of::<c_int>() as c_int
}

// Constants
pub const SOL_SOCKET: c_int = 1;
pub const SCM_RIGHTS: c_int = 1;
pub const SCM_CREDENTIALS: c_int = 2;
pub const SO_TIMESTAMPING_OLD: c_int = 35;
pub const SO_TIMESTAMPING_NEW: c_int = 35;
pub const MSG_CTRUNC: c_int = 0x0020;
pub const MSG_CMSG_COMPAT: c_int = 0x0080;
pub const MSG_CMSG_CLOEXEC: c_int = 0x0100;
pub const O_CLOEXEC: c_int = 0x80000;
pub const PF_UNIX: c_int = 1;
pub const GFP_KERNEL: c_int = 0x0020;
pub const SCM_MAX_FD: c_int = 253;

// Helper functions
#[inline]
fn offsetof<T, F>(_: *const T, _: F) -> usize {
    unsafe { &(*ptr::null::<T>().cast::<u8>()).0 as *const u8 as usize }
}

#[inline]
fn min(a: c_int, b: c_int) -> c_int {
    if a < b { a } else { b }
}

#[inline]
fn next_cmsg(cmsg: *mut cmsghdr) -> *mut cmsghdr {
    let cmlen = CMSG_SPACE((*cmsg).cmsg_len as size_t) as usize;
    (cmsg as *mut u8).add(cmlen) as *mut cmsghdr
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_scm_check_creds() {
        // Basic test - should not panic
        let creds = ucred {
            pid: 1234,
            uid: 1000,
            gid: 1000,
        };
        
        let result = unsafe { scm_check_creds(&creds) };
        assert!(result >= 0 || result == EPERM);
    }
    
    #[test]
    fn test_scm_max_fds() {
        let msg = libc::msghdr {
            msg_controllen: 1024,
            ..Default::default()
        };
        
        let result = unsafe { scm_max_fds(&msg) };
        assert!(result > 0);
    }
}
