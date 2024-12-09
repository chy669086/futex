use lazy_static::lazy_static;

use super::SCHED_YIELD;

lazy_static! {
    pub static ref sched_yield: fn() = sched_yield_getter();
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
