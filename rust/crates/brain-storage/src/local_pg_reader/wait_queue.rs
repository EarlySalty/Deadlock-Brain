//! FIFO admission for the existing bounded connection pool.
//! Each waiter has its own signal so returning a connection wakes the head, not an
//! arbitrary thread that can be overtaken by a fresh checkout.
use std::{
    collections::VecDeque,
    sync::{Arc, Condvar},
};

#[derive(Default)]
pub(super) struct WaitQueue {
    queued: VecDeque<Arc<Condvar>>,
}

impl WaitQueue {
    pub(super) fn may_acquire(&self, waiter: Option<&Arc<Condvar>>) -> bool {
        match (self.queued.front(), waiter) {
            (None, None) => true,
            (Some(head), Some(waiter)) => Arc::ptr_eq(head, waiter),
            _ => false,
        }
    }

    pub(super) fn push(&mut self) -> Arc<Condvar> {
        let signal = Arc::new(Condvar::new());
        self.queued.push_back(Arc::clone(&signal));
        signal
    }

    pub(super) fn remove(&mut self, waiter: Option<&Arc<Condvar>>) {
        let Some(waiter) = waiter else {
            return;
        };
        if let Some(index) = self
            .queued
            .iter()
            .position(|entry| Arc::ptr_eq(entry, waiter))
        {
            drop(self.queued.remove(index));
        }
    }

    pub(super) fn notify_front(&self) {
        if let Some(head) = self.queued.front() {
            head.notify_one();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WaitQueue;

    #[test]
    fn new_checkouts_cannot_overtake_queued_waiters() {
        let mut queue = WaitQueue::default();
        assert!(queue.may_acquire(None));
        let first = queue.push();
        let second = queue.push();
        assert!(!queue.may_acquire(None));
        assert!(queue.may_acquire(Some(&first)));
        assert!(!queue.may_acquire(Some(&second)));
        queue.remove(Some(&first));
        assert!(!queue.may_acquire(None));
        assert!(queue.may_acquire(Some(&second)));
        assert!(!queue.may_acquire(Some(&first)));
    }

    #[test]
    fn timed_out_waiters_do_not_block_the_remaining_queue() {
        let mut queue = WaitQueue::default();
        let first = queue.push();
        let expired = queue.push();
        let last = queue.push();
        queue.remove(Some(&expired));
        assert!(queue.may_acquire(Some(&first)));
        assert!(!queue.may_acquire(Some(&last)));
        queue.remove(Some(&first));
        assert!(queue.may_acquire(Some(&last)));
        queue.remove(Some(&last));
        assert!(queue.may_acquire(None));
    }

    #[test]
    fn removing_an_absent_waiter_never_removes_another_request() {
        let mut queue = WaitQueue::default();
        let first = queue.push();
        let unrelated = WaitQueue::default().push();
        queue.remove(None);
        queue.remove(Some(&unrelated));
        assert!(queue.may_acquire(Some(&first)));
        assert!(!queue.may_acquire(None));
    }
}
