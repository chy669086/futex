#![no_std]
//! A library for futex implementation.
//! You should use `register_api_handler` to register the api handlers.
//!

extern crate alloc;

use linkme::distributed_slice as def_api_handler;
pub use linkme::distributed_slice as register_api_handler;

pub mod api;
pub mod flags;
pub mod futex;
pub(crate) mod hash;
pub(crate) mod queue;
pub mod syscall;
