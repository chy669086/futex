use core::hash::{Hash, Hasher};

struct SimpleHasher {
    state: u64,
}

impl SimpleHasher {
    fn new() -> Self {
        SimpleHasher { state: 0 }
    }
}

impl Hasher for SimpleHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state = self.state.wrapping_mul(31).wrapping_add(byte as u64);
        }
    }

    fn finish(&self) -> u64 {
        self.state
    }
}

pub fn simple_hash<T: Hash>(value: &T) -> u64 {
    let mut hasher = SimpleHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}
