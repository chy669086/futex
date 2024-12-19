use core::sync::atomic::{AtomicU32, Ordering};

use crate::api::{current_prosess_id, current_task, sched_yield, wake};
use crate::flags::{
    EACCES, EAGAIN, EINVAL, FUTEX_BITSET_MATCH_ANY, FUTEX_CMP_REQUEUE, FUTEX_FD, FUTEX_OP_ADD,
    FUTEX_OP_ANDN, FUTEX_OP_OR, FUTEX_OP_SET, FUTEX_OP_XOR, FUTEX_PRIVATE_FLAG, FUTEX_REQUEUE,
    FUTEX_WAIT, FUTEX_WAIT_BITSET, FUTEX_WAKE, FUTEX_WAKE_BITSET, FUTEX_WAKE_OP,
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
            -EAGAIN
        }
        FUTEX_REQUEUE => futex_requeue(uaddr, val, uaddr2, typ),
        FUTEX_CMP_REQUEUE => futex_cmp_requeue(uaddr, val, uaddr2, val3, typ),
        FUTEX_WAKE_OP => futex_wake_op(uaddr, val, uaddr2, val3, timeout as u32, typ),
        _ => {
            // Invalid operation
            -EAGAIN
        }
    }
}

fn futex_cmp_requeue(uaddr: *mut u32, val: u32, uaddr2: *mut u32, val3: u32, typ: Type) -> i32 {
    let Some(addr) = translate_uaddr(uaddr) else {
        return -EACCES;
    };

    let ptr = addr as *mut AtomicU32;
    let exp_val = unsafe { (*ptr).load(Ordering::SeqCst) };
    if exp_val != val3 {
        return -EAGAIN;
    }

    let Some(addr2) = translate_uaddr(uaddr2) else {
        return -EACCES;
    };

    requeue(addr, val, addr2, typ)
}

fn futex_requeue(uaddr: *mut u32, val: u32, uaddr2: *mut u32, typ: Type) -> i32 {
    let Some(addr) = translate_uaddr(uaddr) else {
        return -EACCES;
    };

    let Some(addr2) = translate_uaddr(uaddr2) else {
        return -EACCES;
    };

    requeue(addr, val, addr2, typ)
}

fn requeue(addr: usize, val: u32, addr2: usize, typ: Type) -> i32 {
    let key = get_futex_key(addr, typ);
    let key2 = get_futex_key(addr2, typ);

    let weak_tasks = FUTEX_QUEUES.wake_some(&key, FUTEX_BITSET_MATCH_ANY, val as usize);
    for futex in weak_tasks {
        wake(&futex);
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
        return -EAGAIN;
    }

    let Some(addr) = translate_uaddr(uaddr) else {
        return -EACCES;
    };

    let ptr = addr as *mut AtomicU32;
    let exp_val = unsafe { (*ptr).load(Ordering::SeqCst) };
    if exp_val != val {
        return -EAGAIN;
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
        return -EAGAIN;
    };

    let key = get_futex_key(addr, typ);

    let wake_tasks = FUTEX_QUEUES.wake_some(&key, bitset, val as usize);
    for futex in wake_tasks {
        wake(&futex);
    }

    0
}

fn futex_wake_op(
    uaddr1: *mut u32,
    val1: u32,
    uaddr2: *mut u32,
    val2: u32,
    op: u32,
    typ: Type,
) -> i32 {
    let Some(addr1) = translate_uaddr(uaddr1) else {
        return -EAGAIN;
    };

    let Some(addr2) = translate_uaddr(uaddr2) else {
        return -EAGAIN;
    };

    let key1 = get_futex_key(addr1, typ);

    // 唤醒在 uaddr1 上等待的线程
    let wake_tasks = FUTEX_QUEUES.wake_some(&key1, !0, val1 as usize);
    for futex in wake_tasks {
        wake(&futex);
    }

    let addr2 = addr2 as *mut u32 as *mut AtomicU32;

    // 在 uaddr2 上执行操作
    let addr2_val = unsafe { &*addr2 }.load(Ordering::SeqCst);
    let new_val = match op as i32 {
        FUTEX_OP_ADD => addr2_val + val2,
        FUTEX_OP_SET => val2,
        FUTEX_OP_OR => addr2_val | val2,
        FUTEX_OP_ANDN => addr2_val & !val2,
        FUTEX_OP_XOR => addr2_val ^ val2,
        _ => return -EINVAL,
    };

    unsafe { &*addr2 }.store(new_val, Ordering::SeqCst);

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
