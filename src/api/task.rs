use core::any::Any;

use alloc::sync::Arc;
use lazy_static::lazy_static;

use crate::futex::FutexQ;

use super::{CURRENT_PROSESS_ID, CURRENT_TASK, SCHED_YIELD, WEAK};

lazy_static! {
    pub static ref sched_yield: fn() = sched_yield_getter();
}

lazy_static! {
    pub static ref weak: fn(&FutexQ) -> Option<()> = weak_getter();
}

lazy_static! {
    pub static ref current_task: fn() -> Option<Arc<dyn Any + Send + Sync>> = current_task_getter();
}

lazy_static! {
    pub static ref current_prosess_id: fn() -> Option<usize> = current_prosess_id_getter();
}

fn current_prosess_id_getter() -> fn() -> Option<usize> {
    let mut iter = CURRENT_PROSESS_ID.iter();
    let Some(handler) = iter.next() else {
        panic!("No handler for CURRENT_PROSESS_ID");
    };

    assert!(
        iter.next().is_none(),
        "Multiple handlers for CURRENT_PROSESS_ID"
    );

    drop(iter);

    handler.clone()
}

fn current_task_getter() -> fn() -> Option<Arc<dyn Any + Send + Sync>> {
    let mut iter = CURRENT_TASK.iter();
    let Some(handler) = iter.next() else {
        panic!("No handler for CURRENT_TASK");
    };

    assert!(iter.next().is_none(), "Multiple handlers for CURRENT_TASK");

    drop(iter);

    handler.clone()
}

fn weak_getter() -> fn(&FutexQ) -> Option<()> {
    let mut iter = WEAK.iter();
    let Some(handler) = iter.next() else {
        panic!("No handler for WEAK");
    };

    assert!(iter.next().is_none(), "Multiple handlers for WEAK");

    drop(iter);

    handler.clone()
}

fn sched_yield_getter() -> fn() {
    let mut iter = SCHED_YIELD.iter();
    let Some(handler) = iter.next() else {
        panic!("No handler for SCHED_YIELD");
    };

    assert!(iter.next().is_none(), "Multiple handlers for SCHED_YIELD");

    drop(iter);

    handler.clone()
}
