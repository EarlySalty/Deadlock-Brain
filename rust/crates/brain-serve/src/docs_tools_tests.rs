use super::*;
use brain_contracts::{Budget, Principal, RetrievalPort};

impl RetrievalPort for Delegate {
    fn retrieve(&self, _: &Query, _: &AuthorizedContext) -> Result<Vec<Evidence>, PortError> {
        Err(PortError::Unavailable("legacy-fixture".into()))
    }
}

#[test]
fn missing_pg_adapter_preserves_legacy_retrieval() {
    let port = super::super::internal_docs::InternalDocs::new(Delegate, None);
    let expected = Err(PortError::Unavailable("legacy-fixture".into()));
    assert_eq!(port.retrieve(&query(), &internal_context()), expected);
    assert_eq!(port.retrieve(&query(), &context("docs.public")), expected);
    assert!(port
        .validate_evidence(&query(), &internal_context(), &[], false)
        .is_err());
    assert!(port
        .validate_publication(&query(), &internal_context(), &[])
        .is_err());
}

struct ScratchPg(tempfile::TempDir);

impl ScratchPg {
    fn start() -> Self {
        let root = tempfile::Builder::new()
            .prefix("brain-p5-search-")
            .tempdir()
            .unwrap();
        std::fs::create_dir(root.path().join("socket")).unwrap();
        for (program, arguments) in [
            (
                "initdb",
                vec![
                    "-D".into(),
                    root.path().join("data").into_os_string(),
                    "-A".into(),
                    "trust".into(),
                    "-U".into(),
                    "brain_core_test".into(),
                    "--no-locale".into(),
                    "--encoding=UTF8".into(),
                ],
            ),
            (
                "pg_ctl",
                vec![
                    "-D".into(),
                    root.path().join("data").into_os_string(),
                    "-l".into(),
                    root.path().join("postgres.log").into_os_string(),
                    "-o".into(),
                    format!(
                        "-k {} -p 55441 -c listen_addresses=''",
                        root.path().join("socket").display()
                    )
                    .into(),
                    "-w".into(),
                    "start".into(),
                ],
            ),
        ] {
            let result =
                std::process::Command::new(format!("/usr/lib/postgresql/16/bin/{program}"))
                    .args(arguments)
                    .output()
                    .unwrap();
            assert!(
                result.status.success(),
                "{}",
                String::from_utf8_lossy(&result.stderr)
            );
        }
        Self(root)
    }
}

impl Drop for ScratchPg {
    fn drop(&mut self) {
        let _ = std::process::Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
            .arg("-D")
            .arg(self.0.path().join("data"))
            .args(["-m", "immediate", "-w", "stop"])
            .output();
    }
}

struct Delegate;

impl ToolExecutionPort for Delegate {
    fn definitions(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: Option<&PinnedGameContext>,
    ) -> Result<Vec<ToolDefinition>, PortError> {
        Ok(Vec::new())
    }

    fn execute(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: Option<&PinnedGameContext>,
        _: &str,
        _: &ToolRequest,
    ) -> Result<ToolExecution, PortError> {
        Err(invalid())
    }

    fn validate_dependencies(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: Option<&PinnedGameContext>,
        dependencies: &[ToolEvidenceDependency],
        _: ToolValidationPurpose,
    ) -> Result<(), PortError> {
        assert!(dependencies.is_empty());
        Ok(())
    }
}

#[test]
fn real_pg_search_enforces_scopes_receipts_published_head_entity_and_limits() {
    let migration_path = std::env::var_os("BRAIN_P5_DOCS_MIGRATION")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| {
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../../../scripts/migrations/2026-10-12-brain-gamefacts-p5-docs-v1.sql")
        });
    let migration = std::fs::read_to_string(migration_path)
        .expect("Docs-Migration integrieren oder BRAIN_P5_DOCS_MIGRATION setzen");
    let pg = ScratchPg::start();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let options = |user: &str| {
        sqlx::postgres::PgConnectOptions::new_without_pgpass()
            .host(pg.0.path().join("socket").to_str().unwrap())
            .port(55441)
            .username(user)
            .database("postgres")
    };
    let admin = runtime.block_on(async {
        let pool = sqlx::postgres::PgPoolOptions::new().max_connections(1).connect_with(options("brain_core_test")).await.unwrap();
        sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service LOGIN;
            CREATE ROLE brain_readonly LOGIN; CREATE ROLE brain_tool NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS;
            GRANT brain_tool TO brain_readonly WITH ADMIN FALSE, INHERIT FALSE, SET TRUE;
            GRANT CREATE ON DATABASE postgres TO brain_migrate;").execute(&pool).await.unwrap();
        let mut connection = pool.acquire().await.unwrap();
        sqlx::query("SET ROLE brain_migrate").execute(&mut *connection).await.unwrap();
        sqlx::raw_sql(include_str!("../../../../scripts/migrations/2026-10-12-brain-gamefacts-v1.sql")).execute(&mut *connection).await.unwrap();
        sqlx::raw_sql(&migration).execute(&mut *connection).await.unwrap();
        sqlx::query("RESET ROLE").execute(&mut *connection).await.unwrap();
        sqlx::raw_sql("BEGIN;
            INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('fixture','fixture','v1');
            INSERT INTO raw.source(kind,locator) VALUES ('git','synthetic://docs');
            INSERT INTO raw.run(source_id,job_run_id,target_revision,extractor,extractor_version) VALUES (1,1,'7b0f0e645f026214f28b99dddbb18bd477bc2819','fixture','v1');
            INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body)
                SELECT 1,path,'7b0f0e645f026214f28b99dddbb18bd477bc2819',decode(repeat('01',32),'hex'),'text/html',convert_to('synthetic','UTF8')
                FROM unnest(ARRAY['public/fixture.html','internal/fixture.html']) path;
            INSERT INTO raw.run_object(run_id,path,object_id,action,parse_status) SELECT 1,external_id,id,'upsert','succeeded' FROM raw.object;
            INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version) VALUES (1,1,'7b0f0e645f026214f28b99dddbb18bd477bc2819','fixture','v1');
            INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version)
                SELECT 1,1,1,'css_selector','#section-'||n,decode(repeat('02',32),'hex'),'fixture','v1' FROM generate_series(1,6) n;
            INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version)
                VALUES (2,1,1,'css_selector','#private',decode(repeat('03',32),'hex'),'fixture','v1');
            INSERT INTO canon.entity(kind,key,identity_receipt_id) VALUES ('hero','hero_baba',1);
            INSERT INTO docs.page(source_id,path,title,lang,raw_object_id,valid_from_revision_id)
                SELECT 1,external_id,'Fixture','de',id,1 FROM raw.object;
            INSERT INTO docs.section(page_id,ordinal,heading_path,body,entity_ids,url,receipt_id)
                SELECT 1,n,ARRAY['Baba Start'],repeat('Baba 🧠 ',300),CASE WHEN n=1 THEN ARRAY[1::bigint] ELSE '{}'::bigint[] END,
                    'https://github.com/EarlySalty/Deadlock-Docs/blob/7b0f0e645f026214f28b99dddbb18bd477bc2819/public/fixture.html#L'||n||'-L'||n,n FROM generate_series(1,6) n;
            INSERT INTO docs.section(page_id,ordinal,heading_path,body,entity_ids,url,receipt_id)
                VALUES (2,0,ARRAY['Geheim'], 'Baba private-only überleben', '{}','https://internal.example/docs/fixture#private',7);
            UPDATE raw.run SET status='succeeded' WHERE id=1;
            INSERT INTO canon.source_head(source_id,published_revision_id) VALUES (1,1);
            UPDATE raw.source SET last_complete_revision='7b0f0e645f026214f28b99dddbb18bd477bc2819' WHERE id=1;
            COMMIT;").execute(&mut *connection).await.unwrap();
        drop(connection);
        pool
    });
    let pool = {
        let _entered = runtime.enter();
        super::super::pg_tools::tool_pool(options("brain_readonly"), Duration::from_secs(2))
    };
    runtime.block_on(async {
        let settings: (String, String, String) = sqlx::query_as(
            "SELECT current_user::text,current_setting('transaction_read_only'),current_setting('statement_timeout')",
        ).fetch_one(&pool).await.unwrap();
        assert_eq!(settings, ("brain_tool".into(), "on".into(), "500ms".into()));
        assert!(sqlx::query("INSERT INTO docs.section(page_id,ordinal,body,url,receipt_id) VALUES (1,9,'denied','https://example.invalid',1)")
            .execute(&pool).await.is_err());
        for url in [
            "https://github.com/EarlySalty/Deadlock-Docs/blob/0000000000000000000000000000000000000000/public/fixture.html#L1-L1",
            "https://github.com/EarlySalty/Deadlock-Docs/blob/7b0f0e645f026214f28b99dddbb18bd477bc2819/public/other.html#L1-L1",
            "https://github.com/EarlySalty/Deadlock-Docs/blob/7b0f0e645f026214f28b99dddbb18bd477bc2819/public/fixture.html#L0-L1",
        ] {
            let row = sqlx::query("SELECT section_id,source_id,revision_id,receipt_id,path,title,heading_path,
                left(body,1501) AS excerpt,$1::text AS url,source_revision,scope FROM docs.current_section
                WHERE section_id=1 AND scope='public' LIMIT 1").bind(url).fetch_one(&admin).await.unwrap();
            assert!(section(row, &context("docs.public")).is_err());
        }
    });
    let tools = DocsTools::new(Delegate, pool.clone(), runtime.handle().clone());
    let search = request(json!({"query":"Baba"}));
    let public = tools
        .execute(
            &query(),
            &context("docs.public"),
            None,
            &"\"".repeat(512),
            &search,
        )
        .unwrap();
    assert!(serde_json::to_vec(&public).unwrap().len() < MAX_BYTES);
    assert!(public.result.result["sections"].as_array().unwrap().len() <= 5);
    assert_eq!(public.result.result["truncated"], true);
    assert!(!serde_json::to_string(&public)
        .unwrap()
        .contains("private-only"));
    assert!(public.dependencies[0]
        .evidence
        .iter()
        .all(|evidence| evidence.visibility == SourceVisibility::Public
            && evidence.evidence_id != "7"));
    assert!(public.result.result["sections"]
        .as_array()
        .unwrap()
        .iter()
        .all(
            |section| section["excerpt"].as_str().unwrap().chars().count() <= 1500
                && section["excerpt_truncated"] == true
        ));
    for purpose in [
        ToolValidationPurpose::Provider,
        ToolValidationPurpose::Publication,
        ToolValidationPurpose::Cache,
    ] {
        tools
            .validate_dependencies(
                &query(),
                &context("docs.public"),
                None,
                &public.dependencies,
                purpose,
            )
            .unwrap();
    }
    let entity = tools
        .execute(
            &query(),
            &context("bot.public"),
            None,
            "entity",
            &request(json!({"query":"Baba","entity":1})),
        )
        .unwrap();
    assert_eq!(entity.result.evidence_ids, ["1"]);
    let empty = tools
        .execute(
            &query(),
            &context("docs.public"),
            None,
            "empty",
            &request(json!({"query":"private-only"})),
        )
        .unwrap();
    assert!(
        empty.result.is_error
            && empty.dependencies.is_empty()
            && empty.result.evidence_ids.is_empty()
    );
    let internal = tools
        .execute(&query(), &internal_context(), None, "internal", &search)
        .unwrap();
    assert_eq!(internal.result.evidence_ids, ["7"]);
    assert_eq!(
        internal.dependencies[0].evidence[0].visibility,
        SourceVisibility::Internal
    );
    for purpose in [
        ToolValidationPurpose::Provider,
        ToolValidationPurpose::Publication,
    ] {
        assert!(tools
            .validate_dependencies(
                &query(),
                &internal_context(),
                None,
                &internal.dependencies,
                purpose
            )
            .is_err());
    }
    tools
        .validate_dependencies(
            &query(),
            &internal_context(),
            None,
            &internal.dependencies,
            ToolValidationPurpose::Cache,
        )
        .unwrap();
    let internal_port = super::super::internal_docs::InternalDocs::new(
        Delegate,
        Some(DocsReader::new(pool.clone(), runtime.handle().clone())),
    );
    let internal_evidence = internal_port
        .retrieve(&query(), &internal_context())
        .unwrap();
    assert_eq!(internal_evidence, internal.dependencies[0].evidence);
    let mut changed_evidence = internal_evidence.clone();
    changed_evidence[0].content.push_str("changed");
    assert!(internal_port
        .validate_evidence(&query(), &internal_context(), &changed_evidence, false)
        .is_err());
    assert!(internal_port
        .validate_evidence(&query(), &internal_context(), &[], true)
        .is_err());
    internal_port
        .validate_evidence(&query(), &internal_context(), &internal_evidence, false)
        .unwrap();
    assert!(internal_port
        .validate_evidence(&query(), &internal_context(), &internal_evidence, true)
        .is_err());
    assert!(internal_port
        .validate_publication(&query(), &internal_context(), &internal_evidence)
        .is_err());
    let matching_texts = [
        "Baba\nprivate-only".to_owned(),
        "Baba\0private-only\tüberleben".into(),
        format!("{}private-only", "Baba ".repeat(128)),
        format!("{}private-only", "überleben ".repeat(64)),
        format!("Baba{}private-only", " ".repeat(600)),
        format!(
            "Baba{}private-only",
            " ".repeat(32_768 - "Baba".len() - "private-only".len())
        ),
    ];
    let empty_texts = [
        "\0\u{0001}\u{007f}\t\r\n \u{2003}".to_owned(),
        format!("Baba{}p5suffixnichtvorhanden", " ".repeat(600)),
        format!("{}p5suffixnichtvorhanden", "Baba überleben ".repeat(40)),
    ];
    for text in &matching_texts {
        let mut probe = query();
        probe.text = text.clone();
        probe.validate().unwrap();
        let evidence = internal_port.retrieve(&probe, &internal_context()).unwrap();
        assert_eq!(evidence, internal_evidence);
        assert!(evidence.len() <= MAX_SECTIONS);
        assert!(serde_json::to_vec(&evidence).unwrap().len() <= MAX_BYTES);
        internal_port
            .validate_evidence(&probe, &internal_context(), &evidence, false)
            .unwrap();
        for scope in ["docs.public", "bot.public"] {
            assert!(tools
                .reader
                .retrieve(&probe, &context(scope))
                .unwrap()
                .is_empty());
            assert!(internal_port.retrieve(&probe, &context(scope)).is_err());
        }
    }
    for text in &empty_texts {
        let mut probe = query();
        probe.text = text.clone();
        probe.validate().unwrap();
        assert!(internal_port
            .retrieve(&probe, &internal_context())
            .unwrap()
            .is_empty());
        internal_port
            .validate_evidence(&probe, &internal_context(), &[], false)
            .unwrap();
        assert!(internal_port
            .validate_evidence(&probe, &internal_context(), &internal_evidence, false)
            .is_err());
    }
    for scope in ["docs.public", "bot.public"] {
        assert!(internal_port.retrieve(&query(), &context(scope)).is_err());
    }
    let mut widened = internal_context();
    widened.principal.scopes.insert("docs.public".into());
    assert!(internal_port.retrieve(&query(), &widened).is_err());
    let mut egress = internal_context();
    egress.principal.provider_egress.insert("public".into());
    assert!(internal_port.retrieve(&query(), &egress).is_err());
    let mut foreign = internal_context();
    foreign.conversation_id = "foreign".into();
    assert!(internal_port.retrieve(&query(), &foreign).is_err());
    let policy = brain_policy::PolicyEngine::new(brain_policy::CredentialRegistry::new(vec![
        brain_policy::AuthGrant::from_secret(
            "synthetic-operator",
            "second-brain",
            "internal",
            BTreeSet::from(["second_brain.internal".into()]),
            BTreeSet::new(),
        ),
        brain_policy::AuthGrant::from_secret(
            "synthetic-public",
            "docs-client",
            "docs",
            BTreeSet::from(["docs.public".into()]),
            BTreeSet::from(["public".into()]),
        ),
    ]));
    let service = brain_api::internal::InternalApiService::new(
        policy,
        internal_port,
        "fixture".into(),
        10_000,
        Budget::default(),
    );
    let mut operator_query = query();
    operator_query.requested_scopes = BTreeSet::from(["second_brain.internal".into()]);
    let body = serde_json::to_vec(&operator_query).unwrap();
    let response = service.handle_query(
        Some("Bearer synthetic-operator"),
        &body,
        brain_contracts::RequestDeadline::after(Duration::from_secs(10)),
    );
    assert_eq!(response.status, 200);
    let response: brain_contracts::internal_api::InternalAnswerResponse =
        serde_json::from_str(&response.body).unwrap();
    assert_eq!(response.excerpts.len(), 1);
    assert_eq!(response.excerpts[0].logical_id, "receipt/7");
    assert!(response.excerpts[0].content.contains("private-only"));
    assert!(!response.excerpts[0].content.contains("public/fixture.html"));
    assert_eq!(
        service
            .handle_query(
                Some("Bearer synthetic-public"),
                &body,
                brain_contracts::RequestDeadline::after(Duration::from_secs(10))
            )
            .status,
        403
    );
    for (texts, expected_status) in [
        (
            matching_texts.as_slice(),
            brain_contracts::internal_api::InternalStatus::Answered,
        ),
        (
            empty_texts.as_slice(),
            brain_contracts::internal_api::InternalStatus::InsufficientEvidence,
        ),
    ] {
        for text in texts {
            let mut probe = operator_query.clone();
            probe.text = text.clone();
            let response = service.handle_query(
                Some("Bearer synthetic-operator"),
                &serde_json::to_vec(&probe).unwrap(),
                brain_contracts::RequestDeadline::after(Duration::from_secs(10)),
            );
            assert_eq!(response.status, 200);
            let answer: brain_contracts::internal_api::InternalAnswerResponse =
                serde_json::from_str(&response.body).unwrap();
            assert!(answer.validate(&probe.request_id));
            assert_eq!(answer.status, expected_status);
            if expected_status == brain_contracts::internal_api::InternalStatus::Answered {
                assert_eq!(answer.excerpts.len(), 1);
                assert_eq!(answer.excerpts[0].logical_id, "receipt/7");
                assert_eq!(answer.excerpts[0].content, internal_evidence[0].content);
            } else {
                assert!(answer.excerpts.is_empty());
            }
        }
    }
    runtime.block_on(async {
        sqlx::query("DELETE FROM canon.source_head WHERE source_id=1")
            .execute(&admin)
            .await
            .unwrap();
    });
    assert!(tools
        .validate_dependencies(
            &query(),
            &context("docs.public"),
            None,
            &public.dependencies,
            ToolValidationPurpose::Cache
        )
        .is_err());
    let unpublished = tools
        .execute(
            &query(),
            &context("docs.public"),
            None,
            "unpublished",
            &search,
        )
        .unwrap();
    assert!(unpublished.result.is_error && unpublished.dependencies.is_empty());
    runtime.block_on(async {
        pool.close().await;
        admin.close().await;
    });
    let mut normalized_empty = query();
    normalized_empty.text = "\0\u{0001}\u{007f}".into();
    assert!(tools
        .reader
        .retrieve(&normalized_empty, &internal_context())
        .unwrap()
        .is_empty());
    let mut foreign = internal_context();
    foreign.conversation_id = "foreign".into();
    assert!(tools.reader.retrieve(&normalized_empty, &foreign).is_err());
}

fn query() -> Query {
    serde_json::from_value(json!({"request_id":"docs","conversation_id":"docs","text":"Baba"}))
        .unwrap()
}

fn context(scope: &str) -> AuthorizedContext {
    AuthorizedContext {
        principal: Principal {
            actor_id: "fixture".into(),
            channel: "docs".into(),
            scopes: BTreeSet::from([scope.into()]),
            provider_egress: BTreeSet::from(["public".into()]),
        },
        conversation_id: "docs".into(),
        knowledge_release: "fixture".into(),
        deadline_ms: 10_000,
        request_deadline: None,
        discord: None,
        budget: Budget::default(),
    }
}

fn internal_context() -> AuthorizedContext {
    let mut context = context("second_brain.internal");
    context.principal.actor_id = "second-brain".into();
    context.principal.channel = "internal".into();
    context.principal.provider_egress.clear();
    context
}

fn request(arguments: Value) -> ToolRequest {
    ToolCall {
        id: "docs".into(),
        name: ToolName::SearchDocs,
        arguments,
    }
    .validate(&[definition()])
    .unwrap()
}

fn section_fixture(id: i64, text: &str) -> Section {
    let url = "https://github.com/EarlySalty/Deadlock-Docs/blob/7b0f0e645f026214f28b99dddbb18bd477bc2819/public/billy.html#L35-L38";
    let value = json!({"page":"public/billy.html","heading_path":["Start"],"excerpt":text,
        "url":url,"citation":id.to_string(),"patch":null,"source_revision":"fixture"});
    Section {
        evidence: Evidence {
            evidence_id: id.to_string(),
            source_id: "docs.public/1".into(),
            logical_id: format!("receipt/{id}"),
            revision: 1,
            kind: EvidenceKind::Prose,
            content: value.to_string(),
            citation: url.into(),
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::from(["docs.public".into()]),
            score: 1.0,
            provenance: None,
            patch: None,
        },
        value,
    }
}

#[test]
fn internal_search_normalizes_permitted_controls_and_whitespace() {
    let mut query = query();
    query.text = " \tBaba\r\nprivate-only\u{2003}überleben\u{00a0} ".into();
    query.validate().unwrap();
    assert_eq!(
        internal_search_text(&query).unwrap(),
        "Baba private-only überleben"
    );
    for code in (0..=0x1f).chain(0x7f..=0x9f) {
        let separator = char::from_u32(code).unwrap();
        assert!(separator.is_control());
        query.text = format!("Baba{separator}private-only");
        query.validate().unwrap();
        assert_eq!(internal_search_text(&query).unwrap(), "Baba private-only");
    }
    query.text = "\0\u{0001}\u{007f}\t\r\n \u{2003}".into();
    query.validate().unwrap();
    assert!(internal_search_text(&query).unwrap().is_empty());
}

#[test]
fn internal_search_preserves_complete_long_ascii_and_unicode_inputs() {
    let mut query = query();
    for text in [
        format!("{}private-only", "Baba ".repeat(128)),
        format!("{}private-only", "überleben ".repeat(64)),
    ] {
        assert!(text.len() > 512);
        query.text = text.clone();
        query.validate().unwrap();
        assert_eq!(internal_search_text(&query).unwrap(), text);
    }
    query.text = format!("Baba{}p5suffixnichtvorhanden", " ".repeat(600));
    assert_eq!(
        internal_search_text(&query).unwrap(),
        "Baba p5suffixnichtvorhanden"
    );
    query.text = format!(
        "Baba{}private-only",
        " ".repeat(32_768 - "Baba".len() - "private-only".len())
    );
    assert_eq!(query.text.len(), 32_768);
    query.validate().unwrap();
    assert_eq!(internal_search_text(&query).unwrap(), "Baba private-only");
}

#[test]
fn internal_search_validates_original_query_before_normalization() {
    let mut query = query();
    for text in [
        String::new(),
        " \t\n".into(),
        "x".repeat(32_769),
        "ä".repeat(16_385),
        format!("Baba{}private-only", " ".repeat(32_768)),
    ] {
        query.text = text;
        assert!(query.validate().is_err());
        assert!(internal_search_text(&query).is_err());
    }
    query.text = "Baba".into();
    query.request_id = "invalid\nidentifier".into();
    assert!(query.validate().is_err());
    assert!(internal_search_text(&query).is_err());
}

#[test]
fn search_contract_accepts_only_bounded_typed_arguments() {
    let minimal = request(json!({"query":"Baba"}));
    let ToolSubrequest::SearchDocs(search) = minimal.subrequest() else {
        panic!();
    };
    assert_eq!(search.entity, None);
    assert_eq!(search.limit, None);
    for limit in [1, 5] {
        let parsed = request(json!({"query":"ä".repeat(256),"entity":i64::MAX,"limit":limit}));
        assert_eq!(parsed.name(), ToolName::SearchDocs);
    }
    for arguments in [
        json!({}),
        json!({"query":""}),
        json!({"query":"  "}),
        json!({"query":"x".repeat(513)}),
        json!({"query":"ä".repeat(257)}),
        json!({"query":"Baba\n"}),
        json!({"query":7}),
        json!({"query":"Baba","entity":0}),
        json!({"query":"Baba","entity":-1}),
        json!({"query":"Baba","entity":i64::MAX as u64+1}),
        json!({"query":"Baba","entity":"1"}),
        json!({"query":"Baba","limit":0}),
        json!({"query":"Baba","limit":6}),
        json!({"query":"Baba","limit":1.5}),
        json!({"query":"Baba","limit":"5"}),
    ] {
        assert!(ToolCall {
            id: "docs".into(),
            name: ToolName::SearchDocs,
            arguments
        }
        .validate(&[definition()])
        .is_err());
    }
    for field in [
        "scope",
        "scopes",
        "principal",
        "provider_egress",
        "url",
        "sql",
        "actor_id",
    ] {
        let mut arguments = json!({"query":"Baba"});
        arguments[field] = json!("internal");
        assert!(ToolCall {
            id: "docs".into(),
            name: ToolName::SearchDocs,
            arguments
        }
        .validate(&[definition()])
        .is_err());
    }
}

#[test]
fn scope_mapping_uses_only_authorized_principal_without_egress_widening() {
    let mut query = query();
    query
        .requested_scopes
        .insert("second_brain.internal".into());
    for scope in ["docs.public", "bot.public"] {
        assert_eq!(scopes(&query, &context(scope)).unwrap(), vec!["public"]);
    }
    assert_eq!(
        scopes(&query, &internal_context()).unwrap(),
        vec!["internal"]
    );
    for scope in ["docs.internal", "second_brain.internal", "unrelated"] {
        assert!(scopes(&query, &context(scope)).is_err());
    }
    let mut internal = internal_context();
    internal.principal.provider_egress.insert("internal".into());
    assert!(scopes(&query, &internal).is_err());
    let mut internal = internal_context();
    internal.principal.actor_id = "docs-client".into();
    assert!(scopes(&query, &internal).is_err());
    let mut internal = internal_context();
    internal.principal.channel = "docs".into();
    assert!(scopes(&query, &internal).is_err());
    let mut public = context("docs.public");
    public.conversation_id = "foreign".into();
    assert!(scopes(&query, &public).is_err());
}

#[test]
fn internal_dependencies_are_never_published_or_sent_to_provider() {
    let request = request(json!({"query":"Baba"}));
    let mut evidence = section_fixture(1, "private").evidence;
    evidence.visibility = SourceVisibility::Internal;
    evidence.source_id = "docs.internal/1".into();
    evidence.allowed_scopes = BTreeSet::from(["second_brain.internal".into()]);
    let dependency = ToolEvidenceDependency {
        request,
        game_context: None,
        evidence: vec![evidence],
    };
    for purpose in [
        ToolValidationPurpose::Provider,
        ToolValidationPurpose::Publication,
    ] {
        assert!(validate_access(&query(), &internal_context(), &dependency, purpose).is_err());
    }
    assert!(validate_access(
        &query(),
        &internal_context(),
        &dependency,
        ToolValidationPurpose::Cache
    )
    .is_ok());
    assert!(validate_access(
        &query(),
        &context("docs.public"),
        &dependency,
        ToolValidationPurpose::Cache
    )
    .is_err());
}

#[test]
fn public_dependencies_preserve_real_scopes_and_receipt_url() {
    let request = request(json!({"query":"Baba"}));
    let execution =
        bounded_execution("docs", &request, vec![section_fixture(7, "Baba")], false).unwrap();
    let call = ToolCall {
        id: "docs".into(),
        name: ToolName::SearchDocs,
        arguments: request.arguments().clone(),
    };
    execution.validate_for(&call, &request, None).unwrap();
    assert_eq!(execution.result.evidence_ids, ["7"]);
    let dependency = &execution.dependencies[0];
    assert_eq!(dependency.evidence[0].evidence_id, "7");
    assert_eq!(
        execution.result.result["sections"][0]["url"],
        dependency.evidence[0].citation
    );
    assert_eq!(
        brain_contracts::PublicCitation::from_evidence(&dependency.evidence[0], 0).label,
        dependency.evidence[0].citation
    );
    for purpose in [
        ToolValidationPurpose::Provider,
        ToolValidationPurpose::Publication,
        ToolValidationPurpose::Cache,
    ] {
        assert!(validate_access(&query(), &context("docs.public"), dependency, purpose).is_ok());
        assert!(validate_access(&query(), &context("bot.public"), dependency, purpose).is_err());
    }
}

#[test]
fn internal_evidence_bound_accounts_for_the_complete_serialized_list() {
    let make = || {
        (1..=MAX_SECTIONS as i64)
            .map(|id| section_fixture(id, &"\"\\🧠".repeat(375)))
            .collect::<Vec<_>>()
    };
    let complete = make()
        .into_iter()
        .map(|section| section.evidence)
        .collect::<Vec<_>>();
    assert!(serde_json::to_vec(&complete).unwrap().len() > MAX_BYTES);
    let bounded = bounded_evidence(make()).unwrap();
    assert!(!bounded.is_empty());
    assert!(bounded.len() < MAX_SECTIONS);
    assert!(serde_json::to_vec(&bounded).unwrap().len() <= MAX_BYTES);
    assert_eq!(bounded, complete[..bounded.len()]);
    assert_eq!(bounded_evidence(make()).unwrap(), bounded);
    assert!(bounded_evidence(Vec::new()).unwrap().is_empty());
    assert!(bounded_evidence(vec![section_fixture(1, &"x".repeat(MAX_BYTES))])
        .unwrap()
        .is_empty());
    assert!(bounded_evidence(vec![section_fixture(1, "a"), section_fixture(1, "b")]).is_err());
}

#[test]
fn whole_result_bound_accounts_for_unicode_escaping_call_id_and_evidence_ids() {
    let request = request(json!({"query":"Baba"}));
    let make = || {
        (1..=5)
            .map(|id| section_fixture(id, &"\"\\🧠".repeat(375)))
            .collect()
    };
    let bounded = bounded_execution(&"\"".repeat(512), &request, make(), false).unwrap();
    assert!(serde_json::to_vec(&bounded).unwrap().len() < MAX_BYTES);
    assert_eq!(bounded.result.result["truncated"], true);
    assert!(!bounded.result.is_error);
    let repeated = bounded_execution("revalidation", &request, make(), false).unwrap();
    assert_eq!(bounded.dependencies, repeated.dependencies);
    assert_eq!(
        bounded.result.evidence_ids.len(),
        bounded.result.result["sections"].as_array().unwrap().len()
    );
    assert_eq!(
        bounded.dependencies[0].evidence.len(),
        bounded.result.evidence_ids.len()
    );
}

#[test]
fn empty_and_oversized_results_cannot_fabricate_evidence() {
    let request = request(json!({"query":"Baba"}));
    for (sections, truncated) in [
        (Vec::new(), false),
        (vec![section_fixture(1, &"x".repeat(MAX_BYTES))], false),
    ] {
        let execution = bounded_execution("docs", &request, sections, truncated).unwrap();
        assert!(execution.result.is_error);
        assert!(execution.result.evidence_ids.is_empty());
        assert!(execution.dependencies.is_empty());
        let call = ToolCall {
            id: "docs".into(),
            name: ToolName::SearchDocs,
            arguments: request.arguments().clone(),
        };
        execution.validate_for(&call, &request, None).unwrap();
    }
}

#[test]
fn duplicated_receipts_are_rejected_and_internal_urls_have_no_credentials_or_query() {
    let request = request(json!({"query":"Baba"}));
    assert!(bounded_execution(
        "docs",
        &request,
        vec![section_fixture(1, "a"), section_fixture(1, "b")],
        false
    )
    .is_err());
    assert!(valid_internal_url(
        "https://internal.example/docs/page#section"
    ));
    for url in [
        "http://internal.example/docs#section",
        "https://name:password@internal.example/docs#section",
        "https://internal.example/docs?token=x#section",
        "https://internal.example/docs",
        "https://internal.example/docs#",
    ] {
        assert!(!valid_internal_url(url));
    }
}
