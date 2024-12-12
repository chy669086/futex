use core::any::Any;

use alloc::sync::Arc;

///
pub struct FutexQ {
    pub key: FutexKey,
    pub bitset: u32,
    task: Arc<dyn Any + Send + Sync>,
}

/// **Now only support private key!**
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum FutexKey {
    Private(PrivateKey),
    Shared,
}

#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct PrivateKey {
    pub(crate) pid: u64,
    pub(crate) aligned: u64,
    pub(crate) offset: u64,
}

impl FutexQ {
    pub fn new(futex: FutexKey, bitset: u32, task: Arc<dyn Any + Send + Sync>) -> Self {
        Self {
            key: futex,
            bitset,
            task,
        }
    }

    pub fn get_task<T>(&self) -> Option<Arc<T>>
    where
        T: Any + Send + Sync,
    {
        Arc::clone(&self.task).downcast().ok()
    }
}

impl PrivateKey {
    pub fn new(pid: u64, aligned: u64, offset: u64) -> Self {
        Self {
            pid,
            aligned,
            offset,
        }
    }
}
