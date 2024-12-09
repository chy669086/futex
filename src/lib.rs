#![no_std]

extern crate alloc;

pub mod api;
pub mod flags;
pub mod futex;
pub mod queue;
pub mod syscalls;

use linkme::distributed_slice as def_api_handler;
pub use linkme::distributed_slice as register_api_handler;
