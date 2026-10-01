use super::{OpenAiCompatibleProvider, ProviderError, Result};
use std::time::{Duration, Instant};
const FAILURE_THRESHOLD: u32 = 3;
const COOL_DOWN: Duration = Duration::from_secs(5);
#[derive(Debug, Default)]
pub(super) struct Circuit {
    failures: u32,
    opened: Option<Instant>,
    probe: bool,
}
impl Circuit {
    fn enter(&mut self, now: Instant) -> Result<bool> {
        if let Some(opened) = self.opened {
            if now.saturating_duration_since(opened) < COOL_DOWN || self.probe {
                return Err(ProviderError::CircuitOpen);
            }
            self.probe = true;
            return Ok(true);
        }
        Ok(false)
    }
    fn observe(&mut self, now: Instant, failed: bool, probe: bool) {
        if !failed {
            self.failures = 0;
            self.opened = None;
            self.probe = false;
            return;
        }
        self.failures = self.failures.saturating_add(1);
        if self.failures >= FAILURE_THRESHOLD || probe {
            self.opened = Some(now);
        }
        self.probe = false;
    }
}
impl OpenAiCompatibleProvider {
    pub(super) fn with_circuit<T>(&self, operation: impl FnOnce() -> Result<T>) -> Result<T> {
        let probe = self
            .circuit
            .lock()
            .map_err(|_| ProviderError::CircuitOpen)?
            .enter(Instant::now())?;
        let result = operation();
        let failed = matches!(
            &result,
            Err(ProviderError::Http(_)
                | ProviderError::InvalidResponse(_)
                | ProviderError::ResponseTooLarge)
        ) || matches!(&result,Err(ProviderError::HttpStatus {status}) if status.is_server_error() || status.as_u16()==429);
        if let Ok(mut circuit) = self.circuit.lock() {
            if result.is_ok() || failed {
                circuit.observe(Instant::now(), failed, probe);
            } else if probe {
                circuit.probe = false;
            }
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn circuit_opens_allows_one_probe_and_recovers() {
        let now = Instant::now();
        let mut circuit = Circuit::default();
        for _ in 0..FAILURE_THRESHOLD {
            assert!(!circuit.enter(now).unwrap());
            circuit.observe(now, true, false);
        }
        assert!(circuit.enter(now).is_err());
        let later = now + COOL_DOWN;
        assert!(circuit.enter(later).unwrap());
        assert!(circuit.enter(later).is_err());
        circuit.observe(later, false, true);
        assert!(!circuit.enter(later).unwrap());
    }
    #[test]
    fn failed_probe_restarts_cooldown() {
        let now = Instant::now();
        let mut circuit = Circuit {
            failures: 3,
            opened: Some(now),
            probe: false,
        };
        let later = now + COOL_DOWN;
        assert!(circuit.enter(later).unwrap());
        circuit.observe(later, true, true);
        assert!(circuit.enter(later).is_err());
    }
}
