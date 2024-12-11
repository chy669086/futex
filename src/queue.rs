use core::hash::Hasher;

use alloc::{boxed::Box, collections::vec_deque::VecDeque, vec::Vec};
use lazy_static::lazy_static;
use log::info;
use spin::Mutex;

use crate::futex::{FutexKey, FutexQ};

const FUTEX_HASH_SIZE: usize = 256;

lazy_static! {
    pub static ref FUTEX_QUEUES: FutexQueues = {
        info!("Initializing futex queues");
        let queue = FutexQueues::new(FUTEX_HASH_SIZE);
        queue
    };
}

pub struct FutexQueues {
    pub buckets: Box<[Mutex<VecDeque<FutexQ>>]>,
}

impl FutexQueues {
    fn new(size: usize) -> Self {
        let mut buckets = Vec::with_capacity(size);
        for _ in 0..size {
            buckets.push(Mutex::new(VecDeque::new()));
        }
        Self {
            buckets: buckets.into_boxed_slice(),
        }
    }

    pub(crate) fn push(&self, futex: FutexQ) {
        let key = futex_hash(&futex.key);
        self.buckets[key].lock().push_back(futex);
    }

    pub(crate) fn weak(&self, key: &FutexKey, bitset: u32) -> Option<FutexQ> {
        let idx = futex_hash(key);
        let mut bucket = self.buckets[idx].lock();
        for i in 0..bucket.len() {
            if bucket[i].key == *key && bucket[i].bitset & bitset != 0 {
                return bucket.remove(i);
            }
        }
        None
    }
}

pub fn futex_hash(futex_key: &FutexKey) -> usize {
    let mut hasher = twox_hash::XxHash3_64::with_seed(0);
    for key in [futex_key.pid, futex_key.aligned, futex_key.offset] {
        hasher.write_u64(key);
    }
    hasher.finish() as usize % FUTEX_HASH_SIZE
}
