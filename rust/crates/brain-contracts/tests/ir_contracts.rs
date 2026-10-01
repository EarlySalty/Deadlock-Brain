use brain_contracts::{
    external::{ExternalSourceIr, Provenance, Validation},
    replay::*,
    source::{self, *},
    wiki::{Alias, Dependency, IrField, IrValue, SourceLocator, Unit, WikiIr},
    DocumentRevision, SourceRecordV2, SourceVisibility,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
};

fn roundtrip<T: Serialize + DeserializeOwned + PartialEq + Debug>(value: &T) {
    let decoded: T = serde_json::from_slice(&serde_json::to_vec(value).unwrap()).unwrap();
    assert_eq!(&decoded, value);
}
fn policy() -> SourcePolicy {
    SourcePolicy {
        visibility: SourceVisibility::Private,
        allowed_scopes: BTreeSet::from(["wiki.review".into()]),
        authorization_ref: Observed::known("operator:decision-7".into()),
        license: Observed::unknown(UnknownReason::NotPresent),
        publication_allowed: false,
        provider_egress_allowed: false,
        raw_retention_allowed: true,
    }
}
fn origin() -> OriginArtifact {
    OriginArtifact {
        identity: SourceIdentity {
            source_id: "wiki".into(),
            logical_id: "page:42".into(),
        },
        source_revision: SourceRevision::Wiki {
            page_id: 42,
            revision_id: 77,
        },
        raw_sha256: "a".repeat(64),
        locator: "wiki://page/42?oldid=77".into(),
        parser_revision: "parser:9".into(),
        parser_family: "wiki".into(),
        schema_version: Observed::known("capture:1".into()),
        schema_sha256: Observed::unknown(UnknownReason::NotPresent),
        retrieved_at: Observed::known(SourceTimestamp::UnixSeconds(123)),
        source_time: Observed::unknown(UnknownReason::NotPresent),
        language: Observed::known("de".into()),
        origin_artifacts: BTreeSet::from(["upstream:game-data".into()]),
        derivation_family: Observed::known("wiki".into()),
        policy: policy(),
        validity: GameValidity::unknown(),
    }
}
fn record() -> SourceRecordV2 {
    SourceRecordV2 {
        source_id: "wiki".into(),
        logical_id: "page:42".into(),
        revision: 77,
        content_hash: "a".repeat(64),
        content: "retained source".into(),
        visibility: SourceVisibility::Private,
        allowed_scopes: policy().allowed_scopes,
        tombstone: false,
        valid_from: None,
        valid_to: None,
        metadata: BTreeMap::new(),
    }
}
fn field() -> IrField {
    IrField {
        id: "field:42:damage".into(),
        subject_id: "hero:42".into(),
        predicate: "damage".into(),
        value: IrValue::Quantity {
            decimal: "0".into(),
            unit: Unit::Damage,
        },
        condition: Some("while_airborne".into()),
        variant: Some("charged".into()),
        source_revision: DocumentRevision {
            source_id: "wiki".into(),
            logical_id: "page:42".into(),
            revision: 77,
            content_hash: "a".repeat(64),
        },
        locators: vec![SourceLocator {
            page_id: 42,
            revision_id: 77,
            content_hash: "a".repeat(64),
            locator: "json:/damage@utf8:10:14".into(),
        }],
        unknowns: BTreeSet::new(),
    }
}
fn external(payload: Value) -> ExternalSourceIr {
    let mut ir = ExternalSourceIr {
        payload: Observed::known(payload),
        provenance: Provenance {
            source: "assets".into(),
            locator: "https://fixture.invalid/v1/assets".into(),
            source_revision: SourceRevision::Git {
                commit: "b".repeat(40),
            },
            parser_revision: "parser:11".into(),
            parser_family: "assets".into(),
            raw_sha256: "a".repeat(64),
            schema_sha256: Some("c".repeat(64)),
            observed_at: 200,
            origin_artifacts: BTreeSet::from(["upstream:game-data".into()]),
            derivation_family: Some("assets".into()),
            publication_authorized: false,
            provider_egress_authorized: false,
        },
        validation: Validation::Validated {
            extra_fields: vec!["/future".into()],
        },
        transport: json!({"status":200}),
        schema_version: Observed::known("API:0.1.0".into()),
        field_provenance: BTreeMap::new(),
        visibility: SourceVisibility::Internal,
        allowed_scopes: BTreeSet::from(["source.review:assets".into()]),
        license: Observed::unknown(UnknownReason::NotPresent),
    };
    ir.refresh_field_provenance();
    ir
}
fn replay() -> ReplayReport {
    ReplayReport {
        contract_version: brain_contracts::replay::CONTRACT_VERSION.into(),
        artifact: ReplayArtifact {
            source: ReplaySource {
                source_id: "local-replay".into(),
                source_revision: "capture:original-7".into(),
                raw_object_ref: "opaque:replay-7".into(),
                retrieved_at_unix_ms: None,
                expected_sha256: Some("d".repeat(64)),
                match_reference: None,
                rights: ReplayRights {
                    authorization_ref: "operator:7".into(),
                    scope: "replay.private".into(),
                    local_processing_allowed: true,
                    raw_retention_allowed: true,
                    external_egress_allowed: false,
                },
            },
            sha256: "d".repeat(64),
            byte_length: 1024,
        },
        parser_revision: "parser:22".into(),
        schema_revision: "schema:9".into(),
        extraction_revision: "extract:1".into(),
        validity: GameValidity::unknown(),
        entity_mapping: BTreeMap::new(),
        selection: ReplaySelection::default(),
        generation_id: "generation:9".into(),
        commands: 1,
        packets: 1,
        total_decoded_bytes: 128,
        observations: vec![ReplayObservation {
            observation_id: "observation:1".into(),
            time: ReplayTime {
                tick: Observed::unknown(UnknownReason::InitializationTick),
                tick_interval_seconds: Observed::unknown(UnknownReason::NotPresent),
                game_time_seconds: Observed::unknown(UnknownReason::TickOriginNotEstablished),
            },
            entity: Observed::unknown(UnknownReason::NotPresent),
            raw: RawLocator {
                command_index: 0,
                file_byte_offset: 16,
                stored_byte_length: 100,
                compressed: true,
                payload_sha256: "e".repeat(64),
                packet_index: Some(0),
                entity_callback_index: None,
                requires_state_prefix: false,
            },
            event: ObservationKind::UnknownPacket { message_type: 997 },
        }],
        capabilities: Vec::new(),
        real_replay_verified: false,
        coaching_eligible: false,
    }
}
fn legacy(tick: Value) -> Value {
    let mut value = serde_json::to_value(replay()).unwrap();
    value["contract_version"] = json!("brain.replay.v1");
    let map = value.as_object_mut().unwrap();
    map.remove("validity");
    map.remove("entity_mapping");
    value["observations"][0]["time"]["tick"] = tick;
    value
}

#[test]
fn origin_version_and_policy_roundtrip() {
    let mut original = origin();
    original.validity.patch = Observed::known("patch-A".into());
    original.validity.mode = Observed::known("ranked".into());
    roundtrip(&Versioned::new(original));
}
#[test]
fn envelope_versions_missing_null_future_and_extra_fields_fail_closed() {
    let value = serde_json::to_value(Versioned::new(origin())).unwrap();
    for version in [
        Value::Null,
        json!(1),
        json!("brain.ir.v2"),
        json!("wiki-ir-v1"),
    ] {
        let mut bad = value.clone();
        bad["contract_version"] = version;
        assert!(serde_json::from_value::<Versioned<OriginArtifact>>(bad).is_err());
    }
    let mut bad = value.clone();
    bad.as_object_mut().unwrap().remove("contract_version");
    assert!(serde_json::from_value::<Versioned<OriginArtifact>>(bad).is_err());
    let mut bad = value;
    bad["future"] = json!(true);
    assert!(serde_json::from_value::<Versioned<OriginArtifact>>(bad).is_err());
}
#[test]
fn unknown_is_not_zero_and_null_is_not_an_observation() {
    for value in [
        Observed::known(0_u64),
        Observed::unknown(UnknownReason::NotPresent),
        Observed::unknown(UnknownReason::ExplicitNull),
    ] {
        roundtrip(&value);
    }
    for bad in [
        "null",
        "0",
        "-1",
        "{}",
        r#"{"status":"known","value":null}"#,
        r#"{"status":"known"}"#,
        r#"{"status":"unknown"}"#,
    ] {
        assert!(serde_json::from_str::<Observed<u64>>(bad).is_err(), "{bad}");
    }
    assert_ne!(
        serde_json::to_value(Observed::known(0_u64)).unwrap(),
        serde_json::to_value(Observed::<u64>::unknown(UnknownReason::NotPresent)).unwrap()
    );
}
#[test]
fn store_roundtrip_preserves_exact_origin_acl_and_unknowns() {
    let mut source = record();
    origin().bind_record(&mut source).unwrap();
    let decoded: SourceRecordV2 =
        serde_json::from_slice(&serde_json::to_vec(&source).unwrap()).unwrap();
    let restored = origin_from_record(&decoded).unwrap();
    assert_eq!(restored, origin());
    assert!(!restored.policy.provider_egress_allowed);
    assert!(!restored.policy.publication_allowed);
    assert!(matches!(restored.validity.patch, Observed::Unknown { .. }));
}
#[test]
fn storage_bridge_rejects_acl_expansion_revision_hash_and_identity_drift() {
    let mut source = record();
    origin().bind_record(&mut source).unwrap();
    for index in 0..6 {
        let mut bad = source.clone();
        match index {
            0 => bad.visibility = SourceVisibility::Public,
            1 => {
                bad.allowed_scopes.insert("public".into());
            }
            2 => bad.revision = 78,
            3 => bad.content_hash = "b".repeat(64),
            4 => bad.source_id = "another-source".into(),
            _ => bad.logical_id = "another-page".into(),
        }
        assert!(origin_from_record(&bad).is_err());
    }
}
#[test]
fn parser_and_source_revision_are_separate_from_schema_and_game_patch() {
    let mut original = origin();
    let old_revision = original.source_revision.clone();
    original.parser_revision = "parser:10".into();
    original.schema_version = Observed::known("capture:2".into());
    assert_eq!(original.source_revision, old_revision);
    assert!(matches!(original.validity.patch, Observed::Unknown { .. }));
    for revision in [
        SourceRevision::Git {
            commit: "b".repeat(40),
        },
        SourceRevision::Http {
            body_sha256: original.raw_sha256.clone(),
            etag: Some("etag-7".into()),
            last_modified: None,
        },
        SourceRevision::Api {
            api_version: "0.1.0".into(),
            original_revision: Some("deployment:77".into()),
        },
        SourceRevision::Replay {
            original_revision: "capture:42".into(),
        },
    ] {
        original.source_revision = revision;
        original.validate().unwrap();
        roundtrip(&Versioned::new(original.clone()));
    }
}
#[test]
fn invalid_original_wiki_revision_and_http_hash_are_rejected() {
    let mut bad = origin();
    bad.source_revision = SourceRevision::Wiki {
        page_id: 1,
        revision_id: 0,
    };
    assert!(bad.validate().is_err());
    bad.source_revision = SourceRevision::Http {
        body_sha256: "f".repeat(64),
        etag: None,
        last_modified: None,
    };
    assert!(bad.validate().is_err());
}
#[test]
fn wiki_conditions_variants_aliases_units_dependencies_and_locators_roundtrip() {
    let f = field();
    let locator = f.locators[0].clone();
    let wiki = WikiIr {
        source_id: "wiki".into(),
        mapping_version: "mapping:1".into(),
        mapping_review_ref: "review:1".into(),
        sources: vec![record()],
        fields: vec![f],
        aliases: vec![Alias {
            text: "Geist".into(),
            locale: Some("de".into()),
            subject_id: "hero:42".into(),
            source: locator,
        }],
        entities: BTreeMap::from([(42, "hero:42".into())]),
        artifacts: vec![origin()],
        dependencies: vec![Dependency {
            dependent: origin().identity,
            target: Observed::unknown(UnknownReason::Unmapped),
            target_revision: Observed::unknown(UnknownReason::NotPresent),
            raw_reference: "Template:Unknown".into(),
        }],
        dependency_completeness: BTreeMap::from([("page:42".into(), false)]),
    };
    roundtrip(&Versioned::new(wiki));
}
#[test]
fn missing_condition_is_not_an_unconditional_fact() {
    let mut f = field();
    assert!(!f.is_unconditional_known());
    f.condition = None;
    assert!(!f.is_unconditional_known());
    f.variant = None;
    f.unknowns.insert("missing_condition".into());
    assert!(!f.is_unconditional_known());
    f.unknowns.clear();
    assert!(f.is_unconditional_known());
    f.value = IrValue::Unknown {
        reason: "explicit_null".into(),
    };
    assert!(!f.is_unconditional_known());
    roundtrip(&f);
}
#[test]
fn wiki_null_never_deserializes_as_quantity_zero() {
    for bad in [
        json!(null),
        json!({"kind":"quantity","decimal":null,"unit":"damage"}),
        json!({"kind":"quantity","decimal":0,"unit":"damage"}),
        json!({"kind":"quantity","unit":"damage"}),
    ] {
        assert!(serde_json::from_value::<IrValue>(bad).is_err());
    }
}
#[test]
fn external_null_missing_zero_unknown_fields_and_pointers_are_distinct() {
    let ir = external(json!({"zero":0,"null":null,"future":{"a/b~c":17}}));
    assert_eq!(ir.field("/zero"), Observed::known(json!(0)));
    assert_eq!(
        ir.field("/null"),
        Observed::unknown(UnknownReason::ExplicitNull)
    );
    assert_eq!(
        ir.field("/missing"),
        Observed::unknown(UnknownReason::NotPresent)
    );
    assert_eq!(ir.field("/future/a~1b~0c"), Observed::known(json!(17)));
    assert!(ir.field_provenance.contains_key("/future/a~1b~0c"));
    for provenance in ir.field_provenance.values() {
        assert_eq!(provenance.source_revision, ir.provenance.source_revision);
        assert_eq!(provenance.parser_revision, "parser:11");
    }
    roundtrip(&Versioned::new(ir));
}
#[test]
fn external_quarantine_survives_serialization_and_blocks_field_access() {
    let mut ir = external(json!({"value":0}));
    ir.validation = Validation::Quarantined {
        reasons: vec!["schema_drift".into()],
    };
    roundtrip(&Versioned::new(ir.clone()));
    assert_eq!(
        ir.field("/value"),
        Observed::unknown(UnknownReason::Quarantined)
    );
    ir.payload = Observed::unknown(UnknownReason::Quarantined);
    roundtrip(&Versioned::new(ir));
}
#[test]
fn known_raw_null_is_not_lost_to_option_deserialization() {
    let ir = external(Value::Null);
    roundtrip(&Versioned::new(ir.clone()));
    assert_eq!(ir.field(""), Observed::unknown(UnknownReason::ExplicitNull));
}
#[test]
fn external_origins_share_the_common_artifact_contract_without_revision_conflation() {
    let ir = external(json!({"a":1}));
    let original = ir.origin_artifact();
    original.validate().unwrap();
    assert_eq!(original.source_revision, ir.provenance.source_revision);
    assert_eq!(original.schema_version, Observed::known("API:0.1.0".into()));
    assert_eq!(original.origin_artifacts, origin().origin_artifacts);
    assert!(matches!(original.validity.patch, Observed::Unknown { .. }));
    roundtrip(&Versioned::new(original));
}
#[test]
fn replay_unknown_entity_time_and_fields_roundtrip() {
    roundtrip(&replay());
    roundtrip(&ReplayField {
        value: Observed::unknown(UnknownReason::Unsupported),
        unit: Observed::unknown(UnknownReason::NotPresent),
    });
    roundtrip(&ReplayField {
        value: Observed::known(ReplayScalar::Signed(0)),
        unit: Observed::known("count".into()),
    });
    let mut wire = serde_json::to_value(replay()).unwrap();
    wire["observations"][0]["time"]["tick"] = json!(null);
    assert!(serde_json::from_value::<ReplayReport>(wire).is_err());
}
#[test]
fn replay_time_requires_explicit_unknown_not_missing_or_null_default() {
    for key in ["tick", "tick_interval_seconds", "game_time_seconds"] {
        let mut wire = serde_json::to_value(&replay().observations[0].time).unwrap();
        wire.as_object_mut().unwrap().remove(key);
        assert!(serde_json::from_value::<ReplayTime>(wire).is_err());
    }
}
#[test]
fn replay_versions_reject_legacy_and_future_without_explicit_migration() {
    for version in ["brain.replay.v1", "brain.replay.v3", "brain.v1"] {
        let mut value = serde_json::to_value(replay()).unwrap();
        value["contract_version"] = json!(version);
        assert!(serde_json::from_value::<ReplayReport>(value).is_err());
    }
}
#[test]
fn explicit_legacy_tick_migration_never_invents_a_time() {
    for (raw, expected) in [
        (
            json!(-1),
            Observed::unknown(UnknownReason::InitializationTick),
        ),
        (Value::Null, Observed::unknown(UnknownReason::NotPresent)),
        (json!(0), Observed::known(0)),
        (json!(123), Observed::known(123)),
    ] {
        let migrated = migrate_v1_report(&serde_json::to_vec(&legacy(raw)).unwrap()).unwrap();
        assert_eq!(migrated.observations[0].time.tick, expected);
        assert!(matches!(
            migrated.observations[0].time.game_time_seconds,
            Observed::Unknown { .. }
        ));
        assert_eq!(migrated.parser_revision, "parser:22");
        assert_eq!(
            migrated.artifact.source.source_revision,
            "capture:original-7"
        );
        roundtrip(&migrated);
    }
    let mut missing = legacy(json!(0));
    missing["observations"][0]["time"]
        .as_object_mut()
        .unwrap()
        .remove("tick");
    assert_eq!(
        migrate_v1_report(&serde_json::to_vec(&missing).unwrap())
            .unwrap()
            .observations[0]
            .time
            .tick,
        Observed::unknown(UnknownReason::NotPresent)
    );
}
#[test]
fn legacy_invalid_ticks_or_injected_v2_fields_are_rejected() {
    for bad in [
        json!(-2),
        json!(4294967295_u64),
        json!(0.5),
        json!("0"),
        json!({"status":"known","value":0}),
    ] {
        assert!(migrate_v1_report(&serde_json::to_vec(&legacy(bad)).unwrap()).is_err());
    }
    let mut wire = legacy(json!(0));
    wire["validity"] = json!({});
    assert!(migrate_v1_report(&serde_json::to_vec(&wire).unwrap()).is_err());
}
#[test]
fn replay_detached_provenance_preserves_private_scope_raw_locator_and_unknown_timestamp() {
    let report = replay();
    let provenance = report.observation_provenance("observation:1").unwrap();
    assert_eq!(
        provenance.origin.policy.allowed_scopes,
        BTreeSet::from(["replay.private".into()])
    );
    assert_eq!(provenance.raw, report.observations[0].raw);
    assert_eq!(provenance.origin.parser_revision, report.parser_revision);
    assert_eq!(provenance.replay_id, report.artifact.sha256);
    assert!(matches!(
        provenance.origin.retrieved_at,
        Observed::Unknown { .. }
    ));
    assert!(!provenance.origin.policy.provider_egress_allowed);
    roundtrip(&Versioned::new(provenance));
    assert!(report.observation_provenance("fabricated-id").is_none());
}
#[test]
fn replay_entity_mapping_is_explicit_not_network_index_as_hero_id() {
    let mut report = replay();
    report.observations[0].entity = Observed::known(ReplayEntity {
        network_index: 7,
        creation_ordinal: 2,
        network_serial: None,
        class_name: "Pawn".into(),
    });
    assert_eq!(
        report
            .observation_provenance("observation:1")
            .unwrap()
            .canonical_entity,
        Observed::unknown(UnknownReason::Unmapped)
    );
    report
        .entity_mapping
        .insert("7:2".into(), Observed::known("hero:abrams".into()));
    assert_eq!(
        report
            .observation_provenance("observation:1")
            .unwrap()
            .canonical_entity,
        Observed::known("hero:abrams".into())
    );
    roundtrip(&report);
}
#[test]
fn stable_public_contract_is_not_replaced_by_ir_version() {
    assert_eq!(brain_contracts::CONTRACT_VERSION, "brain.v1");
    assert_eq!(source::IR_VERSION, "brain.ir.v1");
    assert_eq!(brain_contracts::replay::CONTRACT_VERSION, "brain.replay.v2");
    assert!(serde_json::from_value::<brain_contracts::ReplayObservation>(json!({"observation_id":"o","replay_id":"r","occurred_at_ms":null,"kind":"event","payload":{}})).is_err());
}

#[test]
fn unsupported_origin_versions_fail_closed_at_batch_and_egress_boundaries() {
    let mut r = record();
    origin().bind_record(&mut r).unwrap();
    let principal = brain_contracts::Principal {
        actor_id: "reader".into(),
        channel: "test".into(),
        scopes: policy().allowed_scopes,
        provider_egress: BTreeSet::from(["private".into()]),
    };
    assert!(brain_contracts::store::record_allowed(
        &r, &principal, false
    ));
    assert!(!brain_contracts::store::record_allowed(
        &r, &principal, true
    ));
    let mut wire: Value = serde_json::from_str(&r.metadata[source::ORIGIN_METADATA_KEY]).unwrap();
    wire["contract_version"] = json!("brain.ir.v99");
    r.metadata
        .insert(source::ORIGIN_METADATA_KEY.into(), wire.to_string());
    assert!(!brain_contracts::store::record_allowed(
        &r, &principal, false
    ));
    let batch = brain_contracts::SourceBatch {
        expected_generation: 0,
        checkpoint: brain_contracts::SourceCheckpoint {
            source_id: "wiki".into(),
            configuration: "fixture".into(),
            generation: 1,
            state: json!({}),
        },
        records: vec![r],
    };
    assert!(batch.validate().is_err());
}
