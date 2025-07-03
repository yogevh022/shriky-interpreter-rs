use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Counter {
    counter: AtomicUsize,
}

impl Counter {
    pub const fn new() -> Self {
        Self {
            counter: AtomicUsize::new(0),
        }
    }

    pub fn next(&self) -> usize {
        self.counter.fetch_add(1, Ordering::Relaxed)
    }
}
