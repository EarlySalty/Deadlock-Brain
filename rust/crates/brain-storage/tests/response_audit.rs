#[path = "support/scratch_pg.rs"]
mod scratch_pg;

use brain_contracts::{
    response_audit::{ResponseAuditPort, ResponseDisposition},
    AnswerProfile, AnswerProviderPort, AuthorizedContext, Budget, Evidence, EvidenceKind,
    Principal, Query, SourceVisibility,
};
use brain_providers::{OpenAiCompatibleProvider, ProviderConfig};
use brain_storage::{LocalPgReader, PgStore};
use serde_json::{json, Value};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use std::{
    collections::BTreeSet,
    io::{BufRead, BufReader, Read, Write},
    net::TcpListener,
    sync::Arc,
};

type AuditRow = (String, String, String, Value, Value, Value, String, bool);

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn actual_http_validation_deviations_are_durable_private_and_do_not_trip_the_circuit() {
    let pg = scratch_pg::ScratchPg::start();
    let socket = pg.directory.join("socket");
    let owner = PgPoolOptions::new()
        .max_connections(2)
        .connect_with(
            PgConnectOptions::new()
                .host(socket.to_str().unwrap())
                .port(55439)
                .username("brain_core_test")
                .database("postgres"),
        )
        .await
        .unwrap();
    sqlx::raw_sql("CREATE ROLE brain_service LOGIN; CREATE ROLE brain_readonly LOGIN; CREATE ROLE brain_ingest LOGIN").execute(&owner).await.unwrap();
    let store = PgStore::new(owner.clone());
    store.migrate_core().await.unwrap();
    store.migrate_response_audit().await.unwrap();
    store.migrate_response_audit().await.unwrap();
    sqlx::raw_sql("ALTER TABLE brain.response_deviations_v1 ADD CONSTRAINT fail_second_audit_fixture CHECK (request_id <> 'audit-atomic-failure' OR check_json->>'check' <> 'grounded_envelope')").execute(&owner).await.unwrap();
    sqlx::raw_sql("GRANT USAGE ON SCHEMA brain TO brain_service,brain_readonly,brain_ingest")
        .execute(&owner)
        .await
        .unwrap();
    let reader = LocalPgReader::new(&socket, 55439, "brain_service", "postgres").unwrap();
    let cases = vec![
        (
            "grounded_envelope",
            json!({"text":"not-an-envelope <@123456789012345678> item=42"}),
        ),
        (
            "citation_invalid",
            json!({"text":json!({"text":"fixture", "cited_evidence_ids":["missing"]}).to_string()}),
        ),
        ("chat_schema", json!({"remove":"finish_reason"})),
        ("model_identity", json!({"model":"wrong-model"})),
        ("chat_schema", json!({"remove":"usage"})),
        ("finish_reason", json!({"finish_reason":"length"})),
        ("grounded_envelope", json!({"text":"still-not-an-envelope"})),
        (
            "grounded_envelope",
            json!({"quality_filters":false,"text":"unchecked fixture <@123456789012345678> item=42"}),
        ),
        (
            "finish_reason",
            json!({"quality_filters":false,"finish_reason":"length","text":json!({"text":"truncated fixture", "cited_evidence_ids":["evidence-a"]}).to_string()}),
        ),
        (
            "grounded_envelope",
            json!({"quality_filters":false,"audit_failure":true,"finish_reason":"length","text":"truncated ungrounded fixture"}),
        ),
    ];
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let response_cases = cases.clone();
    let server = std::thread::spawn(move || {
        for (_, case) in &response_cases {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(std::time::Duration::from_secs(5)))
                .unwrap();
            let mut input = BufReader::new(stream.try_clone().unwrap());
            let mut length = 0;
            loop {
                let mut line = String::new();
                input.read_line(&mut line).unwrap();
                if line == "\r\n" {
                    break;
                }
                if let Some((name, value)) = line.split_once(':') {
                    if name.eq_ignore_ascii_case("content-length") {
                        length = value.trim().parse().unwrap();
                    }
                }
            }
            let mut body = vec![0; length];
            input.read_exact(&mut body).unwrap();
            let request: Value = serde_json::from_slice(&body).unwrap();
            assert_eq!(request["model"], "fixture-model");
            let mut response = json!({"model":"fixture-model","choices":[{"finish_reason":"stop","message":{"role":"assistant","content":"answer"}}],"usage":{"prompt_tokens":8,"completion_tokens":4}});
            if let Some(text) = case.get("text") {
                response["choices"][0]["message"]["content"] = text.clone();
            }
            if let Some(model) = case.get("model") {
                response["model"] = model.clone();
            }
            if let Some(reason) = case.get("finish_reason") {
                response["choices"][0]["finish_reason"] = reason.clone();
            }
            if case["remove"] == "finish_reason" {
                response["choices"][0]
                    .as_object_mut()
                    .unwrap()
                    .remove("finish_reason");
            }
            if case["remove"] == "usage" {
                response.as_object_mut().unwrap().remove("usage");
            }
            let body = response.to_string();
            write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), body).unwrap();
        }
    });
    let worker_cases = cases.clone();
    tokio::task::spawn_blocking(move || {
        reader.check_response_audit().unwrap();
        let mut config =
            ProviderConfig::new("fixture", format!("http://{address}"), "fixture-model");
        config.retry_attempts = 1;
        let provider = OpenAiCompatibleProvider::new(config.clone())
            .unwrap()
            .with_response_audit(Arc::new(reader.clone()));
        config.quality_filters = false;
        let relaxed = OpenAiCompatibleProvider::new(config)
            .unwrap()
            .with_response_audit(Arc::new(reader.clone()));
        let evidence = vec![Evidence {
            evidence_id: "evidence-a".into(),
            source_id: "fixture-source".into(),
            logical_id: "fixture-document".into(),
            revision: 1,
            kind: EvidenceKind::Fact,
            content: "fixture knowledge".into(),
            citation: "fixture citation".into(),
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::new(),
            score: 1.0,
            provenance: None,
            patch: None,
        }];
        for (index, (_, case)) in worker_cases.iter().enumerate() {
            let query = Query {
                request_id: if case["audit_failure"] == true {
                    "audit-atomic-failure".into()
                } else {
                    format!("audit-fixture-{index}")
                },
                conversation_id: "fixture-conversation".into(),
                text: "fixture question".into(),
                answer_context: None,
                domain: None,
                requested_scopes: BTreeSet::new(),
                profile: AnswerProfile::Explain,
                patch: None,
                mode: None,
            };
            let context = AuthorizedContext {
                discord: None,
                principal: Principal {
                    actor_id: "fixture-actor".into(),
                    channel: "fixture".into(),
                    scopes: BTreeSet::new(),
                    provider_egress: BTreeSet::from(["public".into()]),
                },
                conversation_id: query.conversation_id.clone(),
                knowledge_release: "fixture-release".into(),
                deadline_ms: 5000,
                budget: Budget::default(),
                request_deadline: None,
            };
            if case["audit_failure"] == true {
                let failure = relaxed
                    .answer_accounted(&query, &context, &evidence)
                    .unwrap_err();
                assert!(matches!(
                    failure.error,
                    brain_contracts::PortError::Unavailable(_)
                ));
                assert_eq!(failure.accounting.unwrap().observed.network_rounds, 1);
            } else if case["quality_filters"] == false {
                let answer = relaxed
                    .answer_accounted(&query, &context, &evidence)
                    .unwrap();
                assert_eq!(answer.accounting.observed.network_rounds, 1);
                assert!(answer.value.cited_evidence_ids.is_empty());
                assert!(!answer.value.text.contains("123456789012345678"));
                assert!(answer.value.text.contains("fixture"));
            } else {
                let failure = provider
                    .answer_accounted(&query, &context, &evidence)
                    .unwrap_err();
                assert!(
                    matches!(
                        failure.error,
                        brain_contracts::PortError::InvalidResponse(_)
                    ),
                    "{:?}",
                    failure.error
                );
                assert_eq!(failure.accounting.unwrap().observed.network_rounds, 1);
            }
        }
        let mut invalid = brain_contracts::response_audit::ResponseDeviation {
            request_id: "unsafe".into(),
            model: "fixture-model".into(),
            check: brain_contracts::response_audit::ResponseCheck {
                check: "fixture".into(),
                field: "content".into(),
                expected: "fixture".into(),
            },
            raw_output: b"123456789012345678".to_vec(),
            raw_output_complete: true,
            source_ids: vec![],
            evidence_ids: vec![],
            disposition: ResponseDisposition::Rejected,
            identifiers_redacted: false,
        };
        assert!(reader.append(&invalid).is_err());
        invalid.raw_output = b"explicit unchecked fixture".to_vec();
        invalid.disposition = ResponseDisposition::UncheckedReturned;
        reader.append(&invalid).unwrap();
    })
    .await
    .unwrap();
    server.join().unwrap();
    let rows: Vec<AuditRow> = sqlx::query_as("SELECT request_id,model,convert_from(raw_output,'UTF8'),check_json,source_ids,evidence_ids,disposition,identifiers_redacted FROM brain.response_deviations_v1 ORDER BY audit_id").fetch_all(&owner).await.unwrap();
    let partial: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM brain.response_deviations_v1 WHERE request_id='audit-atomic-failure'",
    )
    .fetch_one(&owner)
    .await
    .unwrap();
    assert_eq!(partial, 0);
    assert_eq!(rows.len(), cases.len());
    for (index, ((class, case), row)) in cases.iter().take(cases.len() - 1).zip(&rows).enumerate() {
        assert_eq!(row.0, format!("audit-fixture-{index}"));
        assert_eq!(row.1, "fixture-model");
        assert_eq!(row.3["check"], *class);
        assert_eq!(row.4, json!(["fixture-source"]));
        assert_eq!(row.5, json!(["evidence-a"]));
        assert_eq!(
            row.6,
            if case["quality_filters"] == false {
                "unchecked_returned"
            } else {
                "rejected"
            }
        );
        assert!(!row.2.contains("123456789012345678"));
        assert!(!row.2.contains("fixture-actor"));
    }
    assert!(rows[0].7);
    assert!(rows[0].2.contains("item=42"));
    assert!(rows[2].3["expected"]
        .as_str()
        .unwrap()
        .contains("finish_reason"));
    assert_eq!(rows.last().unwrap().6, "unchecked_returned");
    for role in ["brain_service", "brain_readonly", "brain_ingest"] {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect_with(
                PgConnectOptions::new()
                    .host(socket.to_str().unwrap())
                    .port(55439)
                    .username(role)
                    .database("postgres"),
            )
            .await
            .unwrap();
        assert!(
            sqlx::query("SELECT raw_output FROM brain.response_deviations_v1")
                .fetch_all(&pool)
                .await
                .is_err()
        );
        assert!(sqlx::query("DELETE FROM brain.response_deviations_v1")
            .execute(&pool)
            .await
            .is_err());
        assert!(
            sqlx::query("UPDATE brain.response_deviations_v1 SET raw_output='x'")
                .execute(&pool)
                .await
                .is_err()
        );
        pool.close().await;
    }
    let timestamps: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM brain.response_deviations_v1 WHERE recorded_at IS NOT NULL",
    )
    .fetch_one(&owner)
    .await
    .unwrap();
    assert_eq!(timestamps, rows.len() as i64);
    owner.close().await;
}
