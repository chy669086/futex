use core::any::Any;

use crate::futex::FutexQ;

use super::def_api_handler;

use alloc::sync::Arc;
pub(crate) use mm::translate_vaddr;
pub(crate) use task::{current_prosess_id, current_task, sched_yield, wake};

mod mm;
mod task;

/// The `sched_yield` function yields the processor to another thread.
/// Use `register_api_handler` to register a handler for this function.
#[def_api_handler]
pub static SCHED_YIELD: [fn()];

/// The `translate_vaddr` function translate a vaddr into a phyaddr.
/// return None if the vaddr is invaild.
#[def_api_handler]
pub static TRANSLATE_VADDR: [fn(usize) -> Option<usize>];

/// The `wake` function recv a task from the futex queue and push it into task_queue.
/// return None if failed.
#[def_api_handler]
pub static WAKE: [fn(&FutexQ) -> Option<()>];

/// The `current_task` function return the current task.
#[def_api_handler]
pub static CURRENT_TASK: [fn() -> Option<Arc<dyn Any + Send + Sync>>];

#[def_api_handler]
pub static CURRENT_PROSESS_ID: [fn() -> Option<usize>];
