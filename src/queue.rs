use alloc::{boxed::Box, collections::vec_deque::VecDeque, vec::Vec};
use lazy_static::lazy_static;
use log::info;
use spin::{Mutex, MutexGuard};

use crate::{
    flags::FUTEX_BITSET_MATCH_ANY,
    futex::{FutexKey, FutexQ},
    hash::simple_hash,
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
        let mut ret = Vec::with_capacity(cnt);
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
        while let Some(mut futex) =
            FutexQueues::get_one(&mut bucket1, *key1, FUTEX_BITSET_MATCH_ANY)
        {
            futex.key = *key2;
            bucket2.push_back(futex);
        }
    }

    fn get_one(
        bucket: &mut MutexGuard<'_, VecDeque<FutexQ>>,
        key: FutexKey,
        bitset: u32,
    ) -> Option<FutexQ> {
        for _ in 0..bucket.len() {
            let Some(futex) = bucket.pop_front() else {
                break;
            };

            if futex.key == key && futex.bitset == bitset {
                return Some(futex);
            } else {
                bucket.push_back(futex);
            }
        }
        None
    }
}

pub fn futex_hash(futex_key: &FutexKey) -> usize {
    simple_hash(futex_key) as usize % FUTEX_HASH_SIZE
}
