use super::*;
use fireworks_model_selection::{read_selection, SelectionError, TRUSTED_UID};
use std::{
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::PathBuf,
};

struct SelectionFixture {
    dir: PathBuf,
}

impl SelectionFixture {
    fn new() -> Self {
        let cwd = std::env::current_dir().unwrap();
        let base = cwd
            .ancestors()
            .find(|path| {
                path.ancestors().all(|ancestor| {
                    std::fs::symlink_metadata(ancestor).is_ok_and(|metadata| {
                        metadata.is_dir()
                            && [0, TRUSTED_UID].contains(&metadata.uid())
                            && metadata.mode() & 0o022 == 0
                    })
                })
            })
            .expect("Die Prüfung benötigt ein geschütztes Verzeichnis");
        let dir = base.join(format!(".provider-selection-test-{}", std::process::id()));
        std::fs::create_dir(&dir).unwrap();
        std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700)).unwrap();
        Self { dir }
    }

    fn path(&self) -> PathBuf {
        self.dir.join("selection.json")
    }

    fn store(&self, model: &str) {
        let next = self.dir.join("next.json");
        let selection = serde_json::json!({
            "schema_version": 1,
            "provider": "fireworks",
            "model": model,
            "release": null,
            "probed_at": "2026-01-01T00:00:00Z",
            "checked_at": "2026-01-01T00:00:00Z"
        });
        std::fs::write(&next, serde_json::to_vec(&selection).unwrap()).unwrap();
        std::fs::set_permissions(&next, std::fs::Permissions::from_mode(0o600)).unwrap();
        std::fs::rename(next, self.path()).unwrap();
    }

    fn selected_model(&self) -> std::result::Result<String, SelectionError> {
        read_selection(&self.path(), TRUSTED_UID).map(|selection| selection.model)
    }
}

impl Drop for SelectionFixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

#[test]
fn official_request_reloads_selection_disables_thinking_and_accounts_for_requested_model() {
    let fixture = SelectionFixture::new();
    let models = [
        "accounts/fireworks/models/deepseek-v4p1-flash",
        "accounts/fireworks/models/deepseek-v4p2-flash",
    ];
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = thread::spawn(move || {
        for model in models {
            let (mut stream, _) = listener.accept().unwrap();
            let request = read_request(&mut stream);
            let (_, body) = request.split_once("\r\n\r\n").unwrap();
            let payload: serde_json::Value = serde_json::from_str(body).unwrap();
            assert_eq!(payload["model"], model);
            assert_eq!(payload["reasoning_effort"], "none");
            assert_eq!(payload["max_tokens"], 17);
            assert_eq!(payload["stream"], false);
            let body = serde_json::json!({
                "model": model,
                "choices": [{"message": {"content": "Abrams Antwort"}}],
                "usage": {"prompt_tokens": 12, "completion_tokens": 3}
            })
            .to_string();
            write!(
                stream,
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            )
            .unwrap();
        }
    });
    let official = ProviderConfig::new(
        "fixture-token",
        "https://api.fireworks.ai/inference/v1",
        "obsolete-configured-model",
    );
    let transport = provider(format!("http://{address}"));
    let mut source_context = context();
    source_context.budget.max_network_rounds = 1;
    source_context.budget.max_output_tokens = 17;
    let context = source_context.with_request_deadline();
    for model in models {
        fixture.store(model);
        let payload = official
            .chat_request(&query(), &context, &[], || fixture.selected_model())
            .unwrap();
        let answer = transport.send_chat(payload, &context, &[]).unwrap();
        assert_eq!(answer.text, "Abrams Antwort");
        assert_eq!(answer.usage.model.as_deref(), Some(model));
        assert_eq!(answer.usage.network_rounds, 1);
    }
    server.join().unwrap();
    std::fs::write(fixture.path(), b"{").unwrap();
    assert!(matches!(
        official.chat_request(&query(), &context, &[], || fixture.selected_model()),
        Err(ProviderError::ModelSelection(SelectionError::InvalidState))
    ));
    std::fs::remove_file(fixture.path()).unwrap();
    assert!(matches!(
        official.chat_request(&query(), &context, &[], || fixture.selected_model()),
        Err(ProviderError::ModelSelection(SelectionError::Unavailable))
    ));
}

#[test]
fn explicit_fixture_model_never_reads_central_selection() {
    let config = ProviderConfig::new("fixture-token", "http://127.0.0.1:1", "fixture-model");
    let payload = config
        .chat_request(&query(), &context(), &[], || {
            panic!("Lokale Prüfungen dürfen die zentrale Auswahl nicht lesen")
        })
        .unwrap();
    let payload = serde_json::to_value(payload).unwrap();
    assert_eq!(payload["model"], "fixture-model");
    assert!(payload.get("reasoning_effort").is_none());
}

#[test]
fn official_endpoint_recognition_uses_exact_host() {
    for (url, official) in [
        ("https://api.fireworks.ai/inference/v1", true),
        ("https://API.FIREWORKS.AI:443/inference/v1/", true),
        ("https://api.fireworks.ai:8443/inference/v1", true),
        ("https://api.fireworks.ai.example.com/inference/v1", false),
        ("http://127.0.0.1/inference/v1", false),
    ] {
        assert_eq!(
            ProviderConfig::new("fixture-token", url, "fixture-model").official_fireworks(),
            official
        );
    }
}

#[test]
fn provider_default_matches_configurable_generous_runtime_timeout() {
    let config = ProviderConfig::new("fixture-token", "http://127.0.0.1", "fixture-model");
    assert_eq!(config.timeout, Duration::from_secs(55));
}
