use super::def_api_handler;

pub use mm::translate_vaddr;
pub use task::sched_yield;

mod mm;
mod task;

/// The `sched_yield` function yields the processor to another thread.
/// Use `register_api_handler` to register a handler for this function.
#[def_api_handler]
pub static SCHED_YIELD: [fn()];

/// The `translate_vaddr` function translate a vaddr into a phyaddr.
#[def_api_handler]
pub static TRANSLATE_VADDR: [fn(usize) -> Option<usize>];
