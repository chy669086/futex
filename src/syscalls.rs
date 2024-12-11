use core::sync::atomic::{AtomicU32, Ordering};

use crate::api::{current_prosess_id, current_task, sched_yield, weak};
use crate::flags::{
    FUTEX_BITSET_MATCH_ANY, FUTEX_WAIT, FUTEX_WAIT_BITSET, FUTEX_WAKE, FUTEX_WAKE_BITSET,
};
use crate::futex::{FutexKey, FutexQ};
use crate::queue::FUTEX_QUEUES;

pub fn sys_futex(
    uaddr: *mut u32,
    futex_op: i32,
    val: u32,
    timeout: *const core::ffi::c_void,
    _uaddr2: *mut u32,
    val3: u32,
) -> i32 {
    match futex_op {
        FUTEX_WAIT => futex_wait(uaddr, val, timeout, FUTEX_BITSET_MATCH_ANY),
        FUTEX_WAIT_BITSET => futex_wait(uaddr, val, timeout, val3),
        FUTEX_WAKE => futex_wake(uaddr, val, FUTEX_BITSET_MATCH_ANY),
        FUTEX_WAKE_BITSET => futex_wake(uaddr, val, val3),
        _ => {
            // Invalid operation
            -1
        }
    }
}

fn futex_wait(uaddr: *mut u32, val: u32, _timeout: *const core::ffi::c_void, bitset: u32) -> i32 {
    if bitset == 0 {
        return -libc::EINVAL;
    }

    let Some(addr) = translate_uaddr(uaddr) else {
        return -libc::EINVAL;
    };

    let ptr = addr as *mut AtomicU32;
    let exp_val = unsafe { (*ptr).load(Ordering::SeqCst) };
    if exp_val != val {
        return -libc::EAGAIN;
    }

    let task = current_task().unwrap();
    let key = get_futex_key(uaddr);
    let futex = FutexQ::new(key, bitset, task);

    FUTEX_QUEUES.push(futex);

    sched_yield();
    0
}

fn futex_wake(uaddr: *mut u32, val: u32, bitset: u32) -> i32 {
    let Some(addr) = translate_uaddr(uaddr) else {
        return -1;
    };

    let ptr = addr as *mut AtomicU32;
    let exp_val = unsafe { (*ptr).load(Ordering::SeqCst) };
    if exp_val != val {
        return -libc::EAGAIN;
    }

    let key = get_futex_key(uaddr);

    for _ in 0..val {
        if let Some(futex) = FUTEX_QUEUES.weak(&key, bitset) {
            weak(&futex);
        }
    }

    0
}

#[inline]
fn translate_uaddr(uaddr: *mut u32) -> Option<usize> {
    let uaddr = uaddr as usize;
    crate::api::translate_vaddr(uaddr)
}

fn get_futex_key(uaddr: *mut u32) -> FutexKey {
    let uaddr = uaddr as usize;
    let pid = current_prosess_id().unwrap() as u64;
    let aligned = (uaddr / 4096 * 4096) as u64;
    let offset = uaddr as u64 - aligned;
    FutexKey::new(pid, aligned, offset)
}
