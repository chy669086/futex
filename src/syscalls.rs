use crate::flags::{FUTEX_WAIT, FUTEX_WAKE};

pub fn futex_syscall(
    uaddr: *mut core::ffi::c_uint,
    futex_op: i32,
    val: core::ffi::c_uint,
    timeout: *const core::ffi::c_void,
    _uaddr2: *mut core::ffi::c_uint,
    _val3: core::ffi::c_uint,
) -> i32 {
    match futex_op {
        FUTEX_WAIT => futex_wait(uaddr, val, timeout),
        FUTEX_WAKE => futex_wake(uaddr, val),
        _ => {
            // Invalid operation
            -1
        }
    }
}

pub(crate) fn futex_wait(
    uaddr: *mut core::ffi::c_uint,
    val: core::ffi::c_uint,
    _timeout: *const core::ffi::c_void,
) -> i32 {
    let addr = translate_uaddr(uaddr);
    if addr.is_none() {
        return -1;
    }
    0
}

pub(crate) fn futex_wake(uaddr: *mut core::ffi::c_uint, val: core::ffi::c_uint) -> i32 {
    let addr = translate_uaddr(uaddr);
    if addr.is_none() {
        return -1;
    }
    0
}

#[inline]
fn translate_uaddr(uaddr: *mut core::ffi::c_uint) -> Option<usize> {
    let uaddr = uaddr as usize;
    crate::api::translate_vaddr(uaddr)
}
