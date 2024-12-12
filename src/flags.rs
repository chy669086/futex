// Futex options
pub const FUTEX_PRIVATE_FLAG: i32 = 128;
pub const FUTEX_CLOCK_REALTIME: i32 = 256;
pub const FUTEX_CMD_MASK: i32 = !(FUTEX_PRIVATE_FLAG | FUTEX_CLOCK_REALTIME);

// Futex operations
pub const FUTEX_WAIT: i32 = 0;
pub const FUTEX_WAKE: i32 = 1;
pub const FUTEX_FD: i32 = 2;
pub const FUTEX_REQUEUE: i32 = 3;
pub const FUTEX_CMP_REQUEUE: i32 = 4;
pub const FUTEX_WAKE_OP: i32 = 5;
pub const FUTEX_LOCK_PI: i32 = 6;
pub const FUTEX_UNLOCK_PI: i32 = 7;
pub const FUTEX_TRYLOCK_PI: i32 = 8;
pub const FUTEX_WAIT_BITSET: i32 = 9;
pub const FUTEX_WAKE_BITSET: i32 = 10;
pub const FUTEX_WAIT_REQUEUE_PI: i32 = 11;
pub const FUTEX_CMP_REQUEUE_PI: i32 = 12;
pub const FUTEX_LOCK_PI2: i32 = 13;

// Futex private operations
pub const FUTEX_WAIT_PRIVATE: i32 = FUTEX_WAIT | FUTEX_PRIVATE_FLAG;
pub const FUTEX_WAKE_PRIVATE: i32 = FUTEX_WAKE | FUTEX_PRIVATE_FLAG;
pub const FUTEX_WAIT_BITSET_PRIVATE: i32 = FUTEX_WAIT_BITSET | FUTEX_PRIVATE_FLAG;
pub const FUTEX_WAKE_BITSET_PRIVATE: i32 = FUTEX_WAKE_BITSET | FUTEX_PRIVATE_FLAG;

// Futex flags
pub const FLAGS_SHARED: i32 = 0x10;
pub const FLAGS_CLOCKRT: i32 = 0x20;

// Futex bitset
pub const FUTEX_BITSET_MATCH_ANY: u32 = u32::MAX;
