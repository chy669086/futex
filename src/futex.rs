pub trait FutexQ: Send + Sync {
    fn get_key(&self) -> &FutexKey;
    fn get_bitset(&self) -> u32;
}

/// **Now only support private key!**
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct FutexKey {
    pub pid: u64,
    pub(crate) aligned: u64,
    pub(crate) offset: u64,
}

impl FutexKey {
    pub fn new(pid: u64, aligned: u64, offset: u64) -> Self {
        Self {
            pid,
            aligned,
            offset,
        }
    }
}
