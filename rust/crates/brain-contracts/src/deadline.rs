use crate::{AuthorizedContext, PortError};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

struct State {
    expires_at: Instant,
    cancelled: AtomicBool,
    clock: Box<dyn Fn() -> Instant + Send + Sync>,
}

impl std::fmt::Debug for State {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("State")
            .field("expires_at", &self.expires_at)
            .field("cancelled", &self.cancelled)
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Debug)]
pub struct RequestDeadline(Arc<State>);
impl PartialEq for RequestDeadline {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for RequestDeadline {}
impl RequestDeadline {
    pub fn after(duration: Duration) -> Self {
        Self::after_with_clock(duration, Instant::now)
    }
    pub fn after_with_clock(
        duration: Duration,
        clock: impl Fn() -> Instant + Send + Sync + 'static,
    ) -> Self {
        let now = clock();
        Self(Arc::new(State {
            expires_at: now.checked_add(duration).unwrap_or(now),
            cancelled: AtomicBool::new(false),
            clock: Box::new(clock),
        }))
    }
    pub fn expires_at(&self) -> Instant {
        self.0.expires_at
    }
    pub fn cancel(&self) {
        self.0.cancelled.store(true, Ordering::Release);
    }
    pub fn remaining(&self) -> std::result::Result<Duration, PortError> {
        let remaining = self
            .0
            .expires_at
            .saturating_duration_since((self.0.clock)());
        if self.0.cancelled.load(Ordering::Acquire) || remaining.is_zero() {
            Err(PortError::BudgetExceeded)
        } else {
            Ok(remaining)
        }
    }
    pub fn check(&self) -> std::result::Result<(), PortError> {
        self.remaining().map(|_| ())
    }
    pub fn wait(&self, duration: Duration) -> std::result::Result<(), PortError> {
        let until = (self.0.clock)()
            .checked_add(duration)
            .ok_or(PortError::BudgetExceeded)?;
        loop {
            let remaining = self.remaining()?;
            let delay = until.saturating_duration_since((self.0.clock)());
            if delay.is_zero() {
                return Ok(());
            }
            std::thread::sleep(delay.min(remaining).min(Duration::from_millis(10)));
        }
    }
}

impl AuthorizedContext {
    pub fn with_request_deadline(&self) -> std::borrow::Cow<'_, Self> {
        if self.request_deadline.is_some() {
            return std::borrow::Cow::Borrowed(self);
        }
        let mut context = self.clone();
        context.request_deadline = Some(RequestDeadline::after(Duration::from_millis(
            self.deadline_ms,
        )));
        std::borrow::Cow::Owned(context)
    }
    pub fn remaining_time(&self) -> std::result::Result<Duration, PortError> {
        let relative = Duration::from_millis(self.deadline_ms);
        let remaining = match &self.request_deadline {
            Some(deadline) => relative.min(deadline.remaining()?),
            None => relative,
        };
        if remaining.is_zero() {
            Err(PortError::BudgetExceeded)
        } else {
            Ok(remaining)
        }
    }
    pub fn check_deadline(&self) -> std::result::Result<(), PortError> {
        self.remaining_time().map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{atomic::AtomicUsize, Mutex};

    fn controlled_deadline(duration: Duration) -> (RequestDeadline, Arc<Mutex<Instant>>) {
        let clock = Arc::new(Mutex::new(Instant::now()));
        let source = clock.clone();
        let deadline = RequestDeadline::after_with_clock(duration, move || *source.lock().unwrap());
        (deadline, clock)
    }

    #[test]
    fn clones_keep_expiry_and_observe_cancellation() {
        let (original, _) = controlled_deadline(Duration::from_secs(1));
        let worker = original.clone();
        assert_eq!(original, worker);
        assert_eq!(original.expires_at(), worker.expires_at());
        assert_eq!(worker.remaining(), Ok(Duration::from_secs(1)));
        original.cancel();
        assert_eq!(worker.check(), Err(PortError::BudgetExceeded));
    }

    #[test]
    fn zero_or_overflowing_lifetimes_fail_closed() {
        for duration in [Duration::ZERO, Duration::MAX] {
            let (deadline, _) = controlled_deadline(duration);
            assert_eq!(deadline.check(), Err(PortError::BudgetExceeded));
        }
    }

    #[test]
    fn controlled_clock_preserves_original_expiry_across_clones() {
        let duration = Duration::from_secs(60);
        let (original, clock) = controlled_deadline(duration);
        let anchor = *clock.lock().unwrap();
        let worker = original.clone();
        assert_eq!(original.expires_at(), anchor + duration);
        *clock.lock().unwrap() = anchor + Duration::from_secs(45);
        assert_eq!(original.remaining(), Ok(Duration::from_secs(15)));
        assert_eq!(worker.remaining(), Ok(Duration::from_secs(15)));
        assert_eq!(worker.clone(), original);
        *clock.lock().unwrap() = anchor + duration;
        assert_eq!(original.check(), Err(PortError::BudgetExceeded));
        assert_eq!(worker.remaining(), Err(PortError::BudgetExceeded));
        *clock.lock().unwrap() = anchor + duration + Duration::from_secs(1);
        assert_eq!(worker.check(), Err(PortError::BudgetExceeded));
        assert_eq!(worker.expires_at(), anchor + duration);
    }

    #[test]
    fn separately_constructed_deadlines_do_not_share_identity_or_cancellation() {
        let anchor = Instant::now();
        let first = RequestDeadline::after_with_clock(Duration::from_secs(60), move || anchor);
        let second = RequestDeadline::after_with_clock(Duration::from_secs(60), move || anchor);
        assert_eq!(first.expires_at(), second.expires_at());
        assert_ne!(first, second);
        first.cancel();
        assert_eq!(first.check(), Err(PortError::BudgetExceeded));
        assert_eq!(second.remaining(), Ok(Duration::from_secs(60)));
    }

    #[test]
    fn wait_uses_the_injected_clock_for_success_and_expiry() {
        let anchor = Instant::now();
        let reads = Arc::new(AtomicUsize::new(0));
        let source = reads.clone();
        let deadline = RequestDeadline::after_with_clock(Duration::from_secs(1), move || {
            let read = source.fetch_add(1, Ordering::SeqCst);
            anchor + Duration::from_millis(read.saturating_sub(1) as u64 * 250)
        });
        assert_eq!(deadline.wait(Duration::from_millis(500)), Ok(()));
        assert_eq!(reads.load(Ordering::SeqCst), 4);
        assert_eq!(deadline.expires_at(), anchor + Duration::from_secs(1));
        assert_eq!(
            deadline.wait(Duration::from_secs(2)),
            Err(PortError::BudgetExceeded)
        );
        assert_eq!(reads.load(Ordering::SeqCst), 6);
    }

    #[test]
    fn zero_and_overflowing_waits_observe_cancellation_and_bounds() {
        let (deadline, _) = controlled_deadline(Duration::from_secs(60));
        assert_eq!(deadline.wait(Duration::ZERO), Ok(()));
        assert_eq!(deadline.wait(Duration::MAX), Err(PortError::BudgetExceeded));
        deadline.cancel();
        assert_eq!(
            deadline.wait(Duration::ZERO),
            Err(PortError::BudgetExceeded)
        );
    }
}
