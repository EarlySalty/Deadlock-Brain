use tracing_subscriber::{filter::filter_fn, layer::SubscriberExt, util::SubscriberInitExt, Layer};

fn layer<W>(writer: W) -> impl Layer<tracing_subscriber::Registry>
where
    W: for<'a> tracing_subscriber::fmt::MakeWriter<'a> + Send + Sync + 'static,
{
    tracing_subscriber::fmt::layer()
        .json()
        .flatten_event(true)
        .with_current_span(false)
        .with_span_list(false)
        .with_target(false)
        .with_level(false)
        .without_time()
        .with_ansi(false)
        .with_writer(writer)
        .with_filter(filter_fn(|metadata| {
            metadata.is_event()
                && metadata.target() == brain_api::audit::TARGET
                && *metadata.level() == tracing::Level::INFO
        }))
}

pub fn init_request_audit() -> Result<(), tracing_subscriber::util::TryInitError> {
    tracing_subscriber::registry()
        .with(layer(std::io::stderr))
        .try_init()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::Write,
        sync::{Arc, Mutex},
    };

    #[derive(Clone, Default)]
    struct Buffer(Arc<Mutex<Vec<u8>>>);

    impl Write for Buffer {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl<'a> tracing_subscriber::fmt::MakeWriter<'a> for Buffer {
        type Writer = Self;
        fn make_writer(&'a self) -> Self::Writer {
            self.clone()
        }
    }

    #[test]
    fn only_redacted_request_target_reaches_json_writer() {
        let output = Buffer::default();
        let subscriber = tracing_subscriber::registry().with(layer(output.clone()));
        tracing::subscriber::with_default(subscriber, || {
            tracing::info!(target: "sqlx::query", sql = "sensitive-fixture");
            tracing::warn!(target: "reqwest", header = "secret-fixture");
            tracing::info!(target: "brain_api::redacted_request", event = "authenticated_request", consumer_id = "docs-client", request_id = "server-sha256:fixture", route = "/v1/answer", status = 400u16, outcome = "invalid_request", duration_ms = 2u64);
        });
        let bytes = output.0.lock().unwrap();
        let lines: Vec<_> = bytes
            .split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
            .collect();
        assert_eq!(lines.len(), 1);
        let value: serde_json::Value = serde_json::from_slice(lines[0]).unwrap();
        assert_eq!(value.as_object().unwrap().len(), 7);
        assert_eq!(value["event"], "authenticated_request");
        assert_eq!(value["status"], 400);
        assert_eq!(value["duration_ms"], 2);
        let text = std::str::from_utf8(&bytes).unwrap();
        assert!(!text.contains("sensitive-fixture"));
        assert!(!text.contains("secret-fixture"));
    }
}
