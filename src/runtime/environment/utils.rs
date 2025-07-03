use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Counter;
impl Counter {
    pub fn new() -> Self {
        Counter
    }

    pub fn next(&self) -> usize {
        static GLOBAL_COUNTER: AtomicUsize = AtomicUsize::new(0);
        GLOBAL_COUNTER.fetch_add(1, Ordering::Relaxed)
    }
}
