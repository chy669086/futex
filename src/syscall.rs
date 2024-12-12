use core::sync::atomic::{AtomicU32, Ordering};

use crate::api::{current_prosess_id, current_task, sched_yield, weak};
use crate::flags::{
    FUTEX_BITSET_MATCH_ANY, FUTEX_CMP_REQUEUE, FUTEX_FD, FUTEX_PRIVATE_FLAG, FUTEX_REQUEUE,
    FUTEX_WAIT, FUTEX_WAIT_BITSET, FUTEX_WAKE, FUTEX_WAKE_BITSET,
};
use crate::futex::{FutexKey, FutexQ, PrivateKey};
use crate::queue::FUTEX_QUEUES;

#[derive(Clone, Copy)]
enum Type {
    Private,
    Shared,
}

pub fn sys_futex(
    uaddr: *mut u32,
    futex_op: i32,
    val: u32,
    timeout: *const core::ffi::c_void,
    uaddr2: *mut u32,
    val3: u32,
) -> i32 {
    let typ = if futex_op & FUTEX_PRIVATE_FLAG != 0 {
        Type::Private
    } else {
        Type::Shared
    };

    let futex_op = futex_op & !FUTEX_PRIVATE_FLAG;

    match futex_op {
        FUTEX_WAIT => futex_wait(uaddr, val, timeout, FUTEX_BITSET_MATCH_ANY, typ),
        FUTEX_WAIT_BITSET => futex_wait(uaddr, val, timeout, val3, typ),
        FUTEX_WAKE => futex_wake(uaddr, val, FUTEX_BITSET_MATCH_ANY, typ),
        FUTEX_WAKE_BITSET => futex_wake(uaddr, val, val3, typ),
        FUTEX_FD => {
            // Because it was inherently racy, FUTEX_FD has been removed
            -libc::EAGAIN
        }
        FUTEX_REQUEUE => futex_requeue(uaddr, val, uaddr2, typ),
        FUTEX_CMP_REQUEUE => futex_cmp_requeue(uaddr, val, uaddr2, val3, typ),
        _ => {
            // Invalid operation
            -libc::EAGAIN
        }
    }
}

fn futex_cmp_requeue(uaddr: *mut u32, val: u32, uaddr2: *mut u32, val3: u32, typ: Type) -> i32 {
    let Some(addr) = translate_uaddr(uaddr) else {
        return -libc::EAGAIN;
    };

    let ptr = addr as *mut AtomicU32;
    let exp_val = unsafe { (*ptr).load(Ordering::SeqCst) };
    if exp_val != val3 {
        return -libc::EAGAIN;
    }

    let Some(addr2) = translate_uaddr(uaddr2) else {
        return -libc::EAGAIN;
    };

    requeue(addr, val, addr2, typ)
}

fn futex_requeue(uaddr: *mut u32, val: u32, uaddr2: *mut u32, typ: Type) -> i32 {
    let Some(addr) = translate_uaddr(uaddr) else {
        return -libc::EAGAIN;
    };

    let Some(addr2) = translate_uaddr(uaddr2) else {
        return -libc::EAGAIN;
    };

    requeue(addr, val, addr2, typ)
}

fn requeue(addr: usize, val: u32, addr2: usize, typ: Type) -> i32 {
    let key = get_futex_key(addr, typ);
    let key2 = get_futex_key(addr2, typ);

    let weak_tasks = FUTEX_QUEUES.weak_some(&key, FUTEX_BITSET_MATCH_ANY, val as usize);
    for futex in weak_tasks {
        weak(&futex);
    }

    FUTEX_QUEUES.requeue(&key, &key2);

    0
}

fn futex_wait(
    uaddr: *mut u32,
    val: u32,
    _timeout: *const core::ffi::c_void,
    bitset: u32,
    typ: Type,
) -> i32 {
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
    let key = get_futex_key(addr, typ);
    let futex = FutexQ::new(key, bitset, task);

    FUTEX_QUEUES.push(futex);

    sched_yield();
    0
}

fn futex_wake(uaddr: *mut u32, val: u32, bitset: u32, typ: Type) -> i32 {
    let Some(addr) = translate_uaddr(uaddr) else {
        return -libc::EAGAIN;
    };

    let key = get_futex_key(addr, typ);

    let weak_tasks = FUTEX_QUEUES.weak_some(&key, bitset, val as usize);
    for futex in weak_tasks {
        weak(&futex);
    }

    0
}

#[inline]
fn translate_uaddr(uaddr: *mut u32) -> Option<usize> {
    let uaddr = uaddr as usize;
    crate::api::translate_vaddr(uaddr)
}

fn get_futex_key(addr: usize, typ: Type) -> FutexKey {
    let addr = addr;

    let aligned = (addr / 4096 * 4096) as u64;
    let offset = addr as u64 - aligned;

    match typ {
        Type::Private => {
            let pid = current_prosess_id().unwrap() as u64;
            FutexKey::Private(PrivateKey::new(pid, aligned, offset))
        }
        Type::Shared => {
            unimplemented!()
        }
    }
}
