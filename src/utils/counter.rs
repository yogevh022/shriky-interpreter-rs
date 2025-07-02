use std::sync::atomic::{AtomicU64, Ordering};

pub struct Counter;
impl Counter {
    pub fn new() -> Self {
        Counter
    }

    pub fn next(&self) -> u64 {
        static GLOBAL_COUNTER: AtomicU64 = AtomicU64::new(0);
        GLOBAL_COUNTER.fetch_add(1, Ordering::Relaxed)
    }
}
