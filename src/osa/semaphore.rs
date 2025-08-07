use core::sync::atomic::{AtomicIsize, Ordering};

use log::warn;

pub struct Semaphore {
    count: AtomicIsize,
}

impl Semaphore {
    pub fn new(count: isize) -> Self {
        warn!("=== Semaphore::new START with count = {} ===", count);
        let semaphore = Self {
            count: AtomicIsize::new(count),
        };
        warn!("=== Semaphore::new END, actual count = {} ===", 
              semaphore.count.load(Ordering::Relaxed));
        semaphore
    }

    pub fn down(&self) {
        loop {
            let current = self.count.load(Ordering::Relaxed);
            warn!("Semaphore down: current count = {}", current);
            if current > 0 {
                if self
                    .count
                    .compare_exchange(current, current - 1, Ordering::Acquire, Ordering::Relaxed)
                    .is_ok()
                {
                    break;
                }
            }
            core::hint::spin_loop();
        }
    }

    pub fn up(&self) {
        self.count.fetch_add(1, Ordering::Release);
    }

    pub fn count(&self) -> isize {
        self.count.load(Ordering::Relaxed)
    }
}
