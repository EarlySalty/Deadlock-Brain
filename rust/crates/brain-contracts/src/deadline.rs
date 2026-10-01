//! In-process request lifetime. Never serialized or accepted from an untrusted caller.
use crate::{AuthorizedContext, PortError};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

#[derive(Debug)]
struct State {
    expires_at: Instant,
    cancelled: AtomicBool,
}

/// Clones retain the original expiry and observe the same cancellation flag.
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
        let now = Instant::now();
        Self(Arc::new(State {
            expires_at: now.checked_add(duration).unwrap_or(now),
            cancelled: AtomicBool::new(false),
        }))
    }
    pub fn expires_at(&self) -> Instant {
        self.0.expires_at
    }
    pub fn cancel(&self) {
        self.0.cancelled.store(true, Ordering::Release);
    }
    pub fn remaining(&self) -> std::result::Result<Duration, PortError> {
        let remaining = self.0.expires_at.saturating_duration_since(Instant::now());
        if self.0.cancelled.load(Ordering::Acquire) || remaining.is_zero() {
            Err(PortError::BudgetExceeded)
        } else {
            Ok(remaining)
        }
    }
    pub fn check(&self) -> std::result::Result<(), PortError> {
        self.remaining().map(|_| ())
    }
    /// Cooperatively interrupt bounded synchronous waits, without spawning a helper thread.
    pub fn wait(&self, duration: Duration) -> std::result::Result<(), PortError> {
        let until = Instant::now()
            .checked_add(duration)
            .ok_or(PortError::BudgetExceeded)?;
        loop {
            let remaining = self.remaining()?;
            let delay = until.saturating_duration_since(Instant::now());
            if delay.is_zero() {
                return Ok(());
            }
            std::thread::sleep(delay.min(remaining).min(Duration::from_millis(10)));
        }
    }
}

impl AuthorizedContext {
    /// Bind only at an entry boundary. Nested calls keep the same original deadline.
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
    #[test]
    fn clones_keep_expiry_and_observe_cancellation() {
        let original = RequestDeadline::after(Duration::from_secs(1));
        let worker = original.clone();
        assert_eq!(original.expires_at(), worker.expires_at());
        assert!(worker.check().is_ok());
        original.cancel();
        assert_eq!(worker.check(), Err(PortError::BudgetExceeded));
    }
    #[test]
    fn zero_or_overflowing_lifetimes_fail_closed() {
        assert_eq!(
            RequestDeadline::after(Duration::ZERO).check(),
            Err(PortError::BudgetExceeded)
        );
        assert_eq!(
            RequestDeadline::after(Duration::MAX).check(),
            Err(PortError::BudgetExceeded)
        );
    }
}
