use super::{OpenAiCompatibleProvider, ProviderError, Result};
use std::{
    sync::Mutex,
    time::{Duration, Instant},
};
const FAILURE_THRESHOLD: u32 = 3;
const COOL_DOWN: Duration = Duration::from_secs(5);
#[derive(Debug, Default)]
pub(super) struct Circuit {
    failures: u32,
    opened: Option<Instant>,
    probe: bool,
    generation: u64,
}
#[derive(Clone, Copy)]
struct Ticket {
    probe: bool,
    generation: u64,
}
impl Circuit {
    fn enter(&mut self, now: Instant) -> Result<Ticket> {
        let probe = if let Some(opened) = self.opened {
            if now.saturating_duration_since(opened) < COOL_DOWN || self.probe {
                return Err(ProviderError::CircuitOpen);
            }
            self.probe = true;
            true
        } else {
            false
        };
        Ok(Ticket {
            probe,
            generation: self.generation,
        })
    }
    fn observe(&mut self, now: Instant, failed: Option<bool>, ticket: Ticket) {
        if ticket.generation != self.generation {
            return;
        }
        match failed {
            Some(false) => {
                self.failures = 0;
                if self.opened.take().is_some() {
                    self.generation = self.generation.wrapping_add(1);
                }
            }
            Some(true) => {
                self.failures = self.failures.saturating_add(1);
                if self.failures >= FAILURE_THRESHOLD || ticket.probe {
                    self.opened = Some(now);
                    self.generation = self.generation.wrapping_add(1);
                }
            }
            None => {}
        }
        self.probe = false;
    }
}
struct Permit<'a> {
    circuit: &'a Mutex<Circuit>,
    ticket: Ticket,
    completed: bool,
}
impl Drop for Permit<'_> {
    fn drop(&mut self) {
        if !self.completed && self.ticket.probe {
            let mut circuit = self
                .circuit
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if circuit.generation == self.ticket.generation {
                circuit.probe = false;
            }
        }
    }
}
fn observation<T>(result: &Result<T>) -> Option<bool> {
    match result {
        Err(ProviderError::Http(error)) if error.is_builder() || error.is_decode() => None,
        Err(ProviderError::Http(_)) => Some(true),
        Err(ProviderError::BodyTransport(_)) => Some(true),
        Err(ProviderError::HttpStatus { status }) => Some(status.is_server_error()),
        Err(ProviderError::InvalidResponse(_) | ProviderError::ResponseTooLarge) | Ok(_) => {
            Some(false)
        }
        Err(
            ProviderError::InvalidConfig
            | ProviderError::BudgetExceeded
            | ProviderError::CircuitOpen
            | ProviderError::AuditUnavailable,
        ) => None,
    }
}
impl OpenAiCompatibleProvider {
    pub(super) fn with_circuit<T>(
        &self,
        operation: impl FnOnce(&mut Option<bool>) -> Result<T>,
    ) -> Result<T> {
        self.with_circuit_with_transport_clock(Instant::now, operation)
    }
    #[cfg(test)]
    fn with_circuit_with_clock<T>(
        &self,
        clock: impl Fn() -> Instant,
        operation: impl FnOnce() -> Result<T>,
    ) -> Result<T> {
        self.with_circuit_with_transport_clock(clock, |_| operation())
    }
    fn with_circuit_with_transport_clock<T>(
        &self,
        clock: impl Fn() -> Instant,
        operation: impl FnOnce(&mut Option<bool>) -> Result<T>,
    ) -> Result<T> {
        let now = clock();
        let ticket = self
            .circuit
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .enter(now)?;
        let mut permit = Permit {
            circuit: &self.circuit,
            ticket,
            completed: false,
        };
        let mut transport_failure = None;
        let result = operation(&mut transport_failure);
        self.circuit
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .observe(
                clock(),
                transport_failure.or_else(|| observation(&result)),
                ticket,
            );
        permit.completed = true;
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;
    use std::cell::Cell;

    fn provider() -> OpenAiCompatibleProvider {
        OpenAiCompatibleProvider::new(super::super::ProviderConfig::new(
            "fixture",
            "http://127.0.0.1:1",
            "fixture",
        ))
        .unwrap()
    }

    #[test]
    fn circuit_opens_allows_one_probe_and_recovers() {
        let now = Instant::now();
        let mut circuit = Circuit::default();
        for _ in 0..FAILURE_THRESHOLD {
            let ticket = circuit.enter(now).unwrap();
            assert!(!ticket.probe);
            circuit.observe(now, Some(true), ticket);
        }
        assert!(circuit.enter(now).is_err());
        let later = now + COOL_DOWN;
        let ticket = circuit.enter(later).unwrap();
        assert!(ticket.probe);
        assert!(circuit.enter(later).is_err());
        circuit.observe(later, Some(false), ticket);
        assert!(!circuit.enter(later).unwrap().probe);
    }
    #[test]
    fn failed_probe_restarts_cooldown() {
        let now = Instant::now();
        let mut circuit = Circuit {
            failures: 3,
            opened: Some(now),
            ..Circuit::default()
        };
        let later = now + COOL_DOWN;
        let ticket = circuit.enter(later).unwrap();
        assert!(ticket.probe);
        circuit.observe(later, Some(true), ticket);
        assert!(circuit.enter(later).is_err());
        assert!(circuit.enter(later + COOL_DOWN).unwrap().probe);
    }
    #[test]
    fn answer_format_size_and_client_status_errors_never_open_the_actual_provider_circuit() {
        let provider = provider();
        let now = Instant::now();
        for _ in 0..FAILURE_THRESHOLD * 3 {
            for error in [
                ProviderError::InvalidResponse("grounded answer envelope missing".into()),
                ProviderError::InvalidResponse("unknown, duplicate or missing citation".into()),
                ProviderError::ResponseTooLarge,
                ProviderError::HttpStatus {
                    status: StatusCode::TOO_MANY_REQUESTS,
                },
                ProviderError::HttpStatus {
                    status: StatusCode::UNAUTHORIZED,
                },
            ] {
                let result: Result<()> = provider.with_circuit_with_clock(|| now, || Err(error));
                assert!(!matches!(result, Err(ProviderError::CircuitOpen)));
                let circuit = provider.circuit.lock().unwrap();
                assert_eq!(circuit.failures, 0);
                assert!(circuit.opened.is_none());
                assert!(!circuit.probe);
            }
        }
        assert!(provider.with_circuit_with_clock(|| now, || Ok(())).is_ok());
    }
    #[test]
    fn actual_provider_retries_half_open_after_server_errors_and_closes_on_a_format_error() {
        let provider = provider();
        let now = Cell::new(Instant::now());
        let calls = Cell::new(0);
        for _ in 0..FAILURE_THRESHOLD {
            let result: Result<()> = provider.with_circuit_with_clock(
                || now.get(),
                || {
                    calls.set(calls.get() + 1);
                    Err(ProviderError::HttpStatus {
                        status: StatusCode::SERVICE_UNAVAILABLE,
                    })
                },
            );
            assert!(matches!(result, Err(ProviderError::HttpStatus { .. })));
        }
        assert!(matches!(
            provider.with_circuit_with_clock(
                || now.get(),
                || {
                    calls.set(calls.get() + 1);
                    Ok(())
                }
            ),
            Err(ProviderError::CircuitOpen)
        ));
        assert_eq!(calls.get(), FAILURE_THRESHOLD);
        now.set(now.get() + COOL_DOWN);
        let result: Result<()> = provider.with_circuit_with_clock(
            || now.get(),
            || {
                assert!(matches!(
                    provider.with_circuit_with_clock(|| now.get(), || Ok(())),
                    Err(ProviderError::CircuitOpen)
                ));
                Err(ProviderError::InvalidResponse(
                    "grounded answer envelope missing".into(),
                ))
            },
        );
        assert!(matches!(result, Err(ProviderError::InvalidResponse(_))));
        assert!(provider
            .with_circuit_with_clock(|| now.get(), || Ok(()))
            .is_ok());
    }
    #[test]
    fn real_server_failure_survives_metadata_and_budget_errors_then_probes_again() {
        use brain_contracts::{AuthorizedContext, Budget, Principal};
        use std::{
            collections::BTreeSet,
            io::{BufRead, BufReader, Read, Write},
            net::TcpListener,
        };
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            for index in 0..=FAILURE_THRESHOLD {
                let (mut stream, _) = listener.accept().unwrap();
                stream
                    .set_read_timeout(Some(Duration::from_secs(5)))
                    .unwrap();
                let mut reader = BufReader::new(stream.try_clone().unwrap());
                let mut length = 0;
                loop {
                    let mut line = String::new();
                    assert!(reader.read_line(&mut line).unwrap() > 0);
                    if line == "\r\n" {
                        break;
                    }
                    if let Some((name, value)) = line.split_once(':') {
                        if name.eq_ignore_ascii_case("content-length") {
                            length = value.trim().parse::<usize>().unwrap();
                        }
                    }
                }
                assert!(length < 16384);
                reader.read_exact(&mut vec![0; length]).unwrap();
                let status = if index < FAILURE_THRESHOLD {
                    "503 Service Unavailable"
                } else {
                    "200 OK"
                };
                write!(stream, "HTTP/1.1 {status}\r\nRetry-After: invalid\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{{}}").unwrap();
            }
        });
        let mut config =
            super::super::ProviderConfig::new("fixture", format!("http://{address}"), "fixture");
        config.retry_attempts = 1;
        let provider = OpenAiCompatibleProvider::new(config).unwrap();
        let context = AuthorizedContext {
            principal: Principal {
                actor_id: "fixture".into(),
                channel: "fixture".into(),
                scopes: BTreeSet::new(),
                provider_egress: BTreeSet::from(["public".into()]),
            },
            conversation_id: "fixture".into(),
            knowledge_release: "fixture".into(),
            deadline_ms: 5000,
            budget: Budget::default(),
            discord: None,
            request_deadline: None,
        };
        let now = Cell::new(Instant::now());
        for _ in 0..FAILURE_THRESHOLD {
            let result: Result<()> = provider.with_circuit_with_transport_clock(
                || now.get(),
                |transport_failure| {
                    let mut payload = serde_json::json!({"input":["fixture"]});
                    let response = provider.transport_json(
                        "embeddings",
                        &mut payload,
                        &context,
                        transport_failure,
                    );
                    assert!(matches!(response, Err(ProviderError::InvalidResponse(_))));
                    assert_eq!(*transport_failure, Some(true));
                    Err(ProviderError::BudgetExceeded)
                },
            );
            assert!(matches!(result, Err(ProviderError::BudgetExceeded)));
        }
        assert!(matches!(
            provider.with_circuit_with_transport_clock::<()>(
                || now.get(),
                |_| panic!("open circuit called transport")
            ),
            Err(ProviderError::CircuitOpen)
        ));
        now.set(now.get() + COOL_DOWN);
        let result: Result<()> = provider.with_circuit_with_transport_clock(
            || now.get(),
            |transport_failure| {
                let mut payload = serde_json::json!({"input":["fixture"]});
                provider.transport_json("embeddings", &mut payload, &context, transport_failure)?;
                assert_eq!(*transport_failure, Some(false));
                Err(ProviderError::InvalidResponse(
                    "grounded answer envelope missing".into(),
                ))
            },
        );
        assert!(matches!(result, Err(ProviderError::InvalidResponse(_))));
        assert!(provider
            .with_circuit_with_clock(|| now.get(), || Ok(()))
            .is_ok());
        server.join().unwrap();
    }
    #[test]
    fn stale_success_cannot_release_another_half_open_probe() {
        let now = Instant::now();
        let mut circuit = Circuit::default();
        let stale = circuit.enter(now).unwrap();
        for _ in 0..FAILURE_THRESHOLD {
            let ticket = circuit.enter(now).unwrap();
            circuit.observe(now, Some(true), ticket);
        }
        let later = now + COOL_DOWN;
        let probe = circuit.enter(later).unwrap();
        circuit.observe(later, Some(false), stale);
        assert!(circuit.enter(later).is_err());
        circuit.observe(later, Some(false), probe);
        assert!(circuit.enter(later).is_ok());
    }
    #[test]
    fn panicking_probe_releases_its_permit_for_the_next_request() {
        let provider = provider();
        let now = Instant::now();
        for _ in 0..FAILURE_THRESHOLD {
            let _: Result<()> = provider.with_circuit_with_clock(
                || now,
                || {
                    Err(ProviderError::HttpStatus {
                        status: StatusCode::BAD_GATEWAY,
                    })
                },
            );
        }
        let later = now + COOL_DOWN;
        let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _: Result<()> = provider.with_circuit_with_clock(|| later, || panic!("fixture"));
        }));
        assert!(panicked.is_err());
        assert!(provider
            .with_circuit_with_clock(|| later, || Ok(()))
            .is_ok());
    }
}
