use core::hash::Hasher;

use alloc::{boxed::Box, collections::vec_deque::VecDeque, vec::Vec};
use lazy_static::lazy_static;
use log::info;
use spin::{Mutex, MutexGuard};

use crate::{
    flags::FUTEX_BITSET_MATCH_ANY,
    futex::{FutexKey, FutexQ},
};

const FUTEX_HASH_SIZE: usize = 257;

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

    pub(crate) fn wake_some(&self, key: &FutexKey, bitset: u32, cnt: usize) -> Vec<FutexQ> {
        let idx = futex_hash(key);
        let mut bucket = self.buckets[idx].lock();
        let mut ret = Vec::new();
        for _ in 0..cnt {
            if let Some(futex) = FutexQueues::get_one(&mut bucket, *key, bitset) {
                ret.push(futex);
            } else {
                break;
            }
        }
        ret
    }

    pub(crate) fn requeue(&self, key1: &FutexKey, key2: &FutexKey) {
        let idx1 = futex_hash(key1);
        let idx2 = futex_hash(key2);
        let mut bucket1 = self.buckets[idx1].lock();
        let mut bucket2 = self.buckets[idx2].lock();
        while let Some(futex) = FutexQueues::get_one(&mut bucket1, *key1, FUTEX_BITSET_MATCH_ANY) {
            bucket2.push_back(futex);
        }
    }

    fn get_one(
        bucket: &mut MutexGuard<'_, VecDeque<FutexQ>>,
        key: FutexKey,
        bitset: u32,
    ) -> Option<FutexQ> {
        for _ in 0..bucket.len() {
            if let Some(futex) = bucket.pop_front() {
                if futex.key == key && futex.bitset == bitset {
                    return Some(futex);
                } else {
                    bucket.push_back(futex);
                }
            } else {
                break;
            }
        }
        None
    }
}

pub fn futex_hash(futex_key: &FutexKey) -> usize {
    let mut hasher = twox_hash::XxHash3_64::with_seed(0);
    match futex_key {
        FutexKey::Private(key) => {
            hasher.write_u64(key.pid);
            hasher.write_u64(key.aligned);
            hasher.write_u64(key.offset);
        }
        _ => {
            unimplemented!();
        }
    }
    hasher.finish() as usize % FUTEX_HASH_SIZE
}
