use core::any::Any;

use crate::futex::FutexQ;

use super::def_api_handler;

use alloc::sync::Arc;
pub(crate) use mm::{copy_from_user, copy_to_user, translate_vaddr};
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

/// The `current_prosess_id` function return the current prosess id.
#[def_api_handler]
pub static CURRENT_PROSESS_ID: [fn() -> Option<usize>];

/// The `copy_from_user` function copy data from user space to kernel space.
/// return the number of bytes copied.
#[def_api_handler]
pub static COPY_FROM_USER: [fn(usize, *mut u8, usize) -> usize];

/// The `copy_to_user` function copy data from kernel space to user space.
/// return the number of bytes copied.
#[def_api_handler]
pub static COPY_TO_USER: [fn(usize, *mut u8, usize) -> usize];
