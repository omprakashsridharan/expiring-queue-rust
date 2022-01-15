use std::{
    collections::BinaryHeap,
    ops::Add,
    time::{Duration, Instant},
};

struct QueueElement<T> {
    time: Instant,
    value: T,
}

pub struct QueueStats<T: Ord + Add<Output = T>> {
    pub min: Option<T>,
    pub max: Option<T>,
    pub sum: Option<T>,
    pub len: usize,
}

impl<T> PartialEq for QueueElement<T> {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl<T> Eq for QueueElement<T> {}

impl<T> Ord for QueueElement<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.time.cmp(&self.time)
    }
}

impl<T> PartialOrd for QueueElement<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn now() -> Instant {
    Instant::now()
}

pub struct ExpiringQueue<T> {
    heap: BinaryHeap<QueueElement<T>>,
    max_age: Duration,
}

impl<T> ExpiringQueue<T> {
    pub fn new(max_age_duration: Duration) -> ExpiringQueue<T> {
        ExpiringQueue {
            heap: BinaryHeap::<QueueElement<T>>::new(),
            max_age: max_age_duration,
        }
    }

    pub fn with_capacity(max_age_duration: Duration, capacity: usize) -> ExpiringQueue<T> {
        ExpiringQueue {
            heap: BinaryHeap::<QueueElement<T>>::with_capacity(capacity),
            max_age: max_age_duration,
        }
    }

    fn clear_oldest(&mut self, now: Instant) {
        while let Some(el) = self.heap.peek() {
            let peek_age = now - el.time;
            if peek_age > self.max_age {
                self.heap.pop();
            } else {
                break;
            }
        }
    }

    pub fn push(&mut self, value: T) -> usize {
        let now = now();
        self.clear_oldest(now);
        self.heap.push(QueueElement { time: now, value });
        self.heap.len()
    }

    pub fn clear(&mut self) {
        self.heap.clear();
    }

    pub fn len(&mut self) -> usize {
        self.clear_oldest(now());
        self.heap.len()
    }

    pub fn is_empty(&mut self) -> bool {
        self.len() == 0
    }

    pub fn capacity(&self) -> usize {
        self.heap.capacity()
    }

    pub fn max_age(&self) -> Duration {
        self.max_age
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.clear_oldest(now());
        self.heap.peek().map(|q_element| &q_element.value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.clear_oldest(now());
        self.heap.pop().map(|q_element| q_element.value)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::ExpiringQueue;

    #[test]
    fn is_empty_test() {
        let mut queue: ExpiringQueue<i32> = ExpiringQueue::new(Duration::from_secs(5));
        queue.push(2);
        assert_eq!(queue.len(), 1);
        sleep_secs(5);
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.is_empty(), true);
    }

    #[cfg(test)]
    fn sleep_secs(dur_secs: u64) {
        println!("\nSleeping {} secs ...", dur_secs);
        std::thread::sleep(Duration::from_secs(dur_secs));
    }
}
