use core::any::Any;

use alloc::sync::Arc;

///
pub struct FutexQ {
    pub(crate) key: FutexKey,
    pub(crate) bitset: u32,
    task: Arc<dyn Any + Send + Sync>,
}

/// **Now only support private key!**
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub(crate) enum FutexKey {
    Private(PrivateKey),
    Shared(SharedKey),
}

#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub(crate) struct PrivateKey {
    pub(crate) pid: u64,
    pub(crate) aligned: u64,
    pub(crate) offset: u64,
}

#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub(crate) struct SharedKey {
    pub(crate) aligned: u64,
    pub(crate) offset: u64,
}

impl FutexQ {
    pub(crate) fn new(futex: FutexKey, bitset: u32, task: Arc<dyn Any + Send + Sync>) -> Self {
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

impl SharedKey {
    pub fn new(aligned: u64, offset: u64) -> Self {
        Self { aligned, offset }
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
