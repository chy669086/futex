use core::hash::{Hash, Hasher};

const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

struct SimpleHasher {
    state: u64,
}

impl SimpleHasher {
    fn new() -> Self {
        SimpleHasher {
            state: FNV_OFFSET_BASIS,
        }
    }
}

impl Hasher for SimpleHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state ^= byte as u64;
            self.state = self.state.wrapping_mul(FNV_PRIME);
            self.state ^= self.state.rotate_right(31) ^ self.state.rotate_left(11);
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
