use super::*;
use brain_contracts::{Budget, Principal};

#[path = "pg_asset_tools_postgres_tests.rs"]
mod postgres;

fn request(name: ToolName, arguments: Value) -> Result<ToolRequest, PortError> {
    ToolCall {
        id: "asset-test".into(),
        name,
        arguments,
    }
    .validate(&definitions())
}

#[test]
fn requests_enforce_canonical_ids_exclusive_ability_selection_and_comparison_bounds() {
    for (name, arguments) in [
        (ToolName::GetItem, json!({"item":1})),
        (ToolName::GetAbility, json!({"ability":i64::MAX})),
        (ToolName::GetAbility, json!({"hero":1,"slot":1})),
        (
            ToolName::GetAbility,
            json!({"hero":1,"slot":4,"patch":"current"}),
        ),
        (
            ToolName::Compare,
            json!({"entities":[1,2],"stats":["weapon.damage"]}),
        ),
        (
            ToolName::Compare,
            json!({"entities":[1,2,3,4],"stats":(0..12).map(|n| format!("stats.value_{n}")).collect::<Vec<_>>()}),
        ),
    ] {
        let parsed = request(name, arguments).unwrap();
        assert_eq!(serde_json::to_value(name).unwrap(), name.as_str());
        assert_eq!(parsed.name(), name);
        assert!(!historical(&parsed));
    }
    for (name, arguments) in [
        (ToolName::GetItem, json!({})),
        (ToolName::GetItem, json!({"item":0})),
        (ToolName::GetItem, json!({"item":i64::MAX as u64+1})),
        (ToolName::GetItem, json!({"item":"Toxic Bullets"})),
        (ToolName::GetItem, json!({"item":1,"patch":" "})),
        (ToolName::GetItem, json!({"item":1,"patch":"ä".repeat(65)})),
        (ToolName::GetAbility, json!({})),
        (ToolName::GetAbility, json!({"ability":1,"hero":2,"slot":1})),
        (ToolName::GetAbility, json!({"ability":1,"slot":1})),
        (ToolName::GetAbility, json!({"hero":1})),
        (ToolName::GetAbility, json!({"slot":1})),
        (ToolName::GetAbility, json!({"hero":1,"slot":0})),
        (ToolName::GetAbility, json!({"hero":1,"slot":5})),
        (ToolName::GetAbility, json!({"ability":0})),
        (ToolName::GetAbility, json!({"hero":0,"slot":1})),
        (ToolName::GetAbility, json!({"hero":1,"slot":1.5})),
        (ToolName::GetAbility, json!({"ability":1,"patch":"a\n"})),
        (
            ToolName::Compare,
            json!({"entities":[1],"stats":["stats.health"]}),
        ),
        (
            ToolName::Compare,
            json!({"entities":[1,2,3,4,5],"stats":["stats.health"]}),
        ),
        (
            ToolName::Compare,
            json!({"entities":[1,1],"stats":["stats.health"]}),
        ),
        (
            ToolName::Compare,
            json!({"entities":[0,2],"stats":["stats.health"]}),
        ),
        (ToolName::Compare, json!({"entities":[1,2],"stats":[]})),
        (
            ToolName::Compare,
            json!({"entities":[1,2],"stats":(0..13).map(|n|format!("stats.a{n}")).collect::<Vec<_>>()}),
        ),
        (
            ToolName::Compare,
            json!({"entities":[1,2],"stats":["stats.health","stats.health"]}),
        ),
        (
            ToolName::Compare,
            json!({"entities":[1,2],"stats":["stats.health;"]}),
        ),
        (
            ToolName::Compare,
            json!({"entities":[1,2],"stats":["stats health"]}),
        ),
        (
            ToolName::Compare,
            json!({"entities":[1,2],"stats":["1health"]}),
        ),
        (
            ToolName::Compare,
            json!({"entities":[1,2],"stats":["x".repeat(129)]}),
        ),
        (
            ToolName::Compare,
            json!({"entities":[1,2],"stats":["stats.health"],"sql":"SELECT 1"}),
        ),
    ] {
        assert!(
            request(name, arguments.clone()).is_err(),
            "{name:?} {arguments}"
        );
    }
}

fn identity(id: u64) -> Value {
    json!({"entity_id":id,"kind":"hero","key":format!("hero_{id}"),"citation":id.to_string(),"patch":null,"source_revision":"commit-a"})
}

fn fact(entity: u64, stat: &str, receipt: u64) -> Value {
    json!({"entity_id":entity,"stat_key":stat,"value":"x".repeat(512),"unit":"damage",
        "condition":{"rank":3,"citation":"not-a-receipt"},"citation":receipt.to_string(),
        "patch":null,"source_revision":"commit-a"})
}

#[test]
fn receipt_completeness_requires_every_displayed_fact_relation_and_identity() {
    let value = json!({"identity":identity(1),
        "facts":{"facts":[fact(1,"item.cost",5)],"omitted_count":0,"truncated":false},
        "links":{"links":[{"entity_id":2,"kind":"component_of","citation":"6","patch":null,"source_revision":"assets-a","identity":identity(2)}],"omitted_count":0,"truncated":false}});
    let bounded = bounded_value(ToolName::GetItem, value.clone())
        .unwrap()
        .unwrap();
    let mut ids = BTreeSet::new();
    citations(&bounded, &mut ids).unwrap();
    assert_eq!(ids, BTreeSet::from([1, 2, 5, 6]));
    for pointer in [
        "/identity",
        "/facts/facts/0",
        "/links/links/0",
        "/links/links/0/identity",
    ] {
        for field in ["citation", "patch", "source_revision"] {
            let mut invalid = value.clone();
            invalid
                .pointer_mut(pointer)
                .unwrap()
                .as_object_mut()
                .unwrap()
                .remove(field);
            assert!(bounded_value(ToolName::GetItem, invalid).unwrap().is_none());
        }
    }
    let mut invalid = value.clone();
    invalid["facts"]["facts"][0]["condition"] = Value::Null;
    assert!(bounded_value(ToolName::GetItem, invalid).unwrap().is_none());
    let mut invalid = value.clone();
    invalid["links"]["links"][0] = json!({});
    assert!(bounded_value(ToolName::GetItem, invalid).unwrap().is_none());
    let mut invalid = value;
    invalid["facts"]["facts"][0]["patch"] = json!("unconfirmed");
    assert!(bounded_value(ToolName::GetItem, invalid).unwrap().is_none());
}

#[test]
fn wrapped_cards_preserve_receipt_boundaries_and_complete_omission_counts() {
    let mut first = fact(1, "ability.damage", 5);
    first["value"] = json!({"citation":"source-value","facts":{"citation":"also-source-data"},"links":[{"citation":"source-link"}]});
    first["condition"] =
        json!({"citation":"source-condition","identity":{"citation":"source-identity"}});
    let link = json!({"entity_id":2,"kind":"has_ability","citation":"6","patch":null,"source_revision":"assets-a","identity":identity(2)});
    let value = json!({"identity":identity(1),
        "facts":{"facts":[first.clone()],"omitted_count":7,"truncated":true},
        "links":{"links":[link.clone(),link.clone()],"omitted_count":3,"truncated":true},
        "abilities":{"links":[link.clone()],"omitted_count":4,"truncated":true}});
    for name in [ToolName::GetItem, ToolName::GetAbility] {
        let bounded = bounded_value(name, value.clone()).unwrap().unwrap();
        assert_eq!(bounded, value);
        let mut ids = BTreeSet::new();
        citations(&bounded, &mut ids).unwrap();
        assert_eq!(ids, BTreeSet::from([1, 2, 5, 6]));
    }
    for (pointer, replacement) in [
        ("/facts", json!([])),
        ("/facts/facts", json!({})),
        ("/facts/omitted_count", json!(-1)),
        ("/facts/omitted_count", json!(u64::MAX)),
        ("/facts/truncated", json!(false)),
        ("/facts/facts/0", json!({})),
        ("/links/links/0/identity", json!({})),
        ("/links/omitted_count", Value::Null),
        ("/abilities/links", Value::Null),
    ] {
        let mut malformed = value.clone();
        *malformed.pointer_mut(pointer).unwrap() = replacement;
        assert!(bounded_value(ToolName::GetAbility, malformed)
            .unwrap()
            .is_none());
    }
    let mut trimmed = value.clone();
    assert!(trim_card(&mut trimmed));
    assert_eq!(trimmed["abilities"]["omitted_count"], 5);
    assert!(trim_card(&mut trimmed));
    assert_eq!(trimmed["facts"]["omitted_count"], 8);
    assert!(trim_card(&mut trimmed));
    assert_eq!(trimmed["links"]["omitted_count"], 4);
    assert_eq!(trimmed["links"]["links"], json!([link]));
    assert_eq!(trimmed["identity"], value["identity"]);
    let original = (0..12)
        .map(|n| {
            let mut row = fact(1, &format!("ability.value_{n}"), n + 10);
            row["value"] = json!("x".repeat(2500));
            row
        })
        .collect::<Vec<_>>();
    let oversized = json!({"identity":identity(1),
        "facts":{"facts":original,"omitted_count":7,"truncated":true},
        "links":{"links":[],"omitted_count":3,"truncated":true}});
    let bounded = bounded_value(ToolName::GetAbility, oversized)
        .unwrap()
        .unwrap();
    let rows = bounded["facts"]["facts"].as_array().unwrap();
    assert!(!rows.is_empty() && rows.len() < 12);
    assert!(rows.iter().all(|row| original.contains(row)));
    assert_eq!(bounded["facts"]["omitted_count"], 7 + 12 - rows.len());
    assert_eq!(bounded["links"]["omitted_count"], 3);
    assert_eq!(bounded["identity"], identity(1));
    let mut ids = BTreeSet::new();
    citations(&bounded, &mut ids).unwrap();
    let result = ToolResult {
        call_id: "\"".repeat(512),
        name: ToolName::GetAbility,
        result: bounded,
        evidence_ids: ids.iter().map(ToString::to_string).collect(),
        is_error: false,
    };
    assert!(serde_json::to_vec(&result).unwrap().len() <= MAX_BYTES);
    assert!(fits(&result, ids.len()).unwrap());
}

#[test]
fn bounded_four_by_twelve_comparison_keeps_identities_and_marks_missing_and_omitted_cells() {
    let keys = (0..12)
        .map(|n| format!("stats.{}_{n}", "k".repeat(110)))
        .collect::<Vec<_>>();
    let mut facts = Vec::new();
    let mut cells = Vec::new();
    for entity in 1..=4 {
        for (index, key) in keys.iter().enumerate() {
            let missing = entity == 4 && index == 11;
            if !missing {
                facts.push(fact(entity, key, entity * 100 + index as u64));
            }
            cells.push(
                json!({"entity_id":entity,"stat_key":key,"total_count":if missing {0} else {1}}),
            );
        }
    }
    let original = facts.clone();
    let mut value = json!({"entities":(1..=4).map(identity).collect::<Vec<_>>(),"stats":{"facts":facts},"cells":cells});
    update_cells(&mut value).unwrap();
    let identities = value["entities"].clone();
    let bounded = bounded_value(ToolName::Compare, value).unwrap().unwrap();
    assert_eq!(bounded["entities"], identities);
    assert_eq!(bounded["cells"].as_array().unwrap().len(), 48);
    assert_eq!(bounded["cells"][47]["status"], "missing");
    let rows = bounded["stats"]["facts"].as_array().unwrap();
    assert!(rows.len() < 47);
    assert!(rows.iter().all(|row| original.contains(row)));
    assert_eq!(bounded["stats"]["omitted_count"], 47 - rows.len());
    assert_eq!(bounded["stats"]["truncated"], true);
    assert!(bounded["cells"]
        .as_array()
        .unwrap()
        .iter()
        .any(|cell| cell["status"] == "omitted"));
    let mut ids = BTreeSet::new();
    citations(&bounded, &mut ids).unwrap();
    assert!((1..=4).all(|id| ids.contains(&id)));
    let result = ToolResult {
        call_id: "\"".repeat(512),
        name: ToolName::Compare,
        result: bounded,
        evidence_ids: ids.iter().map(ToString::to_string).collect(),
        is_error: false,
    };
    assert!(serde_json::to_vec(&result).unwrap().len() <= MAX_BYTES);
    assert!(fits(&result, ids.len()).unwrap());
}

#[test]
fn p2_receipt_results_validate_without_a_legacy_game_pin() {
    for (name, arguments) in [
        (ToolName::GetItem, json!({"item":1})),
        (ToolName::GetAbility, json!({"ability":1})),
        (
            ToolName::Compare,
            json!({"entities":[1,2],"stats":["stats.health"]}),
        ),
    ] {
        let call = ToolCall {
            id: "pin-independent".into(),
            name,
            arguments,
        };
        let request = call.validate(&definitions()).unwrap();
        let execution = ToolExecution {
            result: ToolResult {
                call_id: call.id.clone(),
                name,
                result: identity(1),
                evidence_ids: vec!["1".into()],
                is_error: false,
            },
            dependencies: vec![ToolEvidenceDependency {
                request: request.clone(),
                game_context: None,
                evidence: vec![Evidence {
                    evidence_id: "1".into(),
                    source_id: "canon.source/1".into(),
                    logical_id: "receipt/1".into(),
                    revision: 1,
                    kind: EvidenceKind::Fact,
                    content: identity(1).to_string(),
                    citation: "1".into(),
                    visibility: SourceVisibility::Public,
                    allowed_scopes: BTreeSet::new(),
                    score: 1.0,
                    provenance: None,
                    patch: None,
                }],
            }],
            usage: Usage::default(),
        };
        execution.validate_for(&call, &request, None).unwrap();
        let mut invalid = execution;
        invalid.dependencies[0].evidence.clear();
        assert!(invalid.validate_for(&call, &request, None).is_err());
    }
}

struct NoLegacy;

impl ToolExecutionPort for NoLegacy {
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
        panic!("P2 tools must not delegate to legacy tools")
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
fn p2_scope_egress_and_historical_errors_are_checked_before_database_access() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let pool = {
        let _guard = runtime.enter();
        tool_pool(
            PgConnectOptions::new_without_pgpass()
                .host("/nonexistent-p2-test-socket")
                .username("brain_readonly"),
            Duration::from_secs(1),
        )
    };
    let tools = PgTools::new(NoLegacy, pool, runtime.handle().clone());
    let query: Query = serde_json::from_value(
        json!({"request_id":"p2","conversation_id":"p2","text":"Vergleich","profile":"explain"}),
    )
    .unwrap();
    let context = AuthorizedContext {
        principal: Principal {
            actor_id: "fixture".into(),
            channel: "test".into(),
            scopes: BTreeSet::new(),
            provider_egress: BTreeSet::from(["public".into()]),
        },
        conversation_id: "p2".into(),
        knowledge_release: "fixture".into(),
        deadline_ms: 10000,
        request_deadline: None,
        discord: None,
        budget: Budget::default(),
    };
    for (name, arguments) in [
        (ToolName::GetItem, json!({"item":1,"patch":"old"})),
        (
            ToolName::GetAbility,
            json!({"hero":1,"slot":1,"patch":"old"}),
        ),
        (
            ToolName::Compare,
            json!({"entities":[1,2],"stats":["stats.health"],"patch":"old"}),
        ),
    ] {
        let request = request(name, arguments).unwrap();
        let error = tools
            .execute(&query, &context, None, "historical", &request)
            .unwrap();
        assert!(error.result.is_error);
        assert!(error.result.evidence_ids.is_empty() && error.dependencies.is_empty());
        for unauthorized in [
            AuthorizedContext {
                principal: Principal {
                    provider_egress: BTreeSet::new(),
                    ..context.principal.clone()
                },
                ..context.clone()
            },
            AuthorizedContext {
                conversation_id: "other".into(),
                ..context.clone()
            },
        ] {
            assert!(matches!(
                tools.execute(&query, &unauthorized, None, "denied", &request),
                Err(PortError::PermissionDenied(_))
            ));
            assert!(tools.definitions(&query, &unauthorized, None).is_err());
        }
    }
}
