use alloc::{boxed::Box, collections::vec_deque::VecDeque, vec::Vec};
use lazy_static::lazy_static;
use log::info;
use spin::Mutex;

use crate::futex::FutexQ;

const FUTEX_HASH_SIZE: usize = 256;

lazy_static! {
    pub static ref FUTEX_QUEUES: FutexQueues = {
        info!("Initializing futex queues");
        let queue = FutexQueues::new(FUTEX_HASH_SIZE);
        queue
    };
}

pub struct FutexQueues {
    pub buckets: Box<[VecDeque<Mutex<FutexQ>>]>,
}

impl FutexQueues {
    fn new(size: usize) -> Self {
        let mut buckets = Vec::with_capacity(size);
        for _ in 0..size {
            buckets.push(VecDeque::new());
        }
        Self {
            buckets: buckets.into_boxed_slice(),
        }
    }
}
