use brain_contracts::{
    answer_contract::{AnswerKind, AnswerPart, StructuredAnswer},
    AnswerProfile, AnswerProviderPort, AnswerStatus, AuthorizedContext, Budget, ChunkProvenance,
    DocumentRevision, Evidence, EvidenceKind, PortError, Principal, ProviderAnswer, Query,
    RetrievalPort, SourceVisibility, Usage,
};
use brain_kernel::{AnswerKernelPort, Kernel};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone)]
struct Retrieval(Vec<Evidence>);
impl RetrievalPort for Retrieval {
    fn retrieve(&self, _: &Query, _: &AuthorizedContext) -> Result<Vec<Evidence>, PortError> {
        Ok(self.0.clone())
    }
    fn validate_evidence(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: &[Evidence],
        _: bool,
    ) -> Result<(), PortError> {
        Ok(())
    }
    fn validate_publication(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: &[Evidence],
    ) -> Result<(), PortError> {
        Ok(())
    }
}

struct Provider(StructuredAnswer);
impl AnswerProviderPort for Provider {
    fn answer(
        &self,
        _: &Query,
        _: &AuthorizedContext,
        _: &[Evidence],
    ) -> Result<ProviderAnswer, PortError> {
        Ok(ProviderAnswer {
            text: "This unrendered provider text must never be published: 999".into(),
            cited_evidence_ids: self.0.cited_evidence_ids.clone(),
            structured: Some(self.0.clone()),
            usage: Usage {
                network_rounds: 1,
                output_tokens: 7,
                ..Usage::default()
            },
        })
    }
}

fn evidence(source: &str, content: &str) -> Evidence {
    Evidence {
        evidence_id: "e1".into(),
        source_id: source.into(),
        logical_id: "a".into(),
        revision: 1,
        kind: EvidenceKind::Fact,
        content: content.into(),
        citation: "synthetic".into(),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        score: 1.0,
        provenance: Some(ChunkProvenance {
            document: DocumentRevision {
                logical_id: "a".into(),
                revision: 1,
                source_id: source.into(),
                content_hash: "synthetic".into(),
            },
            chunker_version: "synthetic".into(),
            ordinal: 0,
            byte_start: 0,
            byte_end: content.len(),
            source_locator: "synthetic".into(),
            release_id: "r1".into(),
            knowledge_version: "v1".into(),
            valid_from: None,
            valid_to: None,
            metadata: BTreeMap::new(),
        }),
        patch: None,
    }
}

fn run(
    structured: StructuredAnswer,
    evidence: Vec<Evidence>,
    profile: AnswerProfile,
    quality: bool,
) -> brain_contracts::AnswerResponse {
    let query = Query {
        request_id: "synthetic-k2".into(),
        conversation_id: "synthetic-k2".into(),
        text: "Wie hilft mir das?".into(),
        answer_context: None,
        domain: None,
        requested_scopes: BTreeSet::new(),
        profile,
        patch: None,
        mode: None,
    };
    let context = AuthorizedContext {
        discord: None,
        request_deadline: None,
        conversation_id: query.conversation_id.clone(),
        principal: Principal {
            actor_id: "synthetic".into(),
            channel: "test".into(),
            scopes: BTreeSet::new(),
            provider_egress: BTreeSet::from(["public".into()]),
        },
        knowledge_release: "r1".into(),
        deadline_ms: 5000,
        budget: Budget::default(),
    };
    Kernel::new(Retrieval(evidence), Provider(structured))
        .with_quality_filters(quality)
        .answer_for_publication(&query, &context)
}

fn grounded(parts: Vec<AnswerPart>) -> StructuredAnswer {
    StructuredAnswer {
        kind: AnswerKind::Grounded,
        parts,
        cited_evidence_ids: vec!["e1".into()],
    }
}
fn prose(text: &str) -> AnswerPart {
    AnswerPart::Prose { text: text.into() }
}
fn game(field: &str, value: &str) -> AnswerPart {
    AnswerPart::GameValue {
        evidence_id: "e1".into(),
        field: field.into(),
        value: value.into(),
    }
}

#[test]
fn social_answers_without_game_evidence_for_both_live_profiles_and_quality_modes() {
    for text in [
        "Mir geht's gut, danke! Wie geht's dir?",
        "Gerne!",
        "Hey, schön dich zu sehen!",
    ] {
        for profile in [AnswerProfile::Explain, AnswerProfile::Coaching] {
            for quality in [true, false] {
                let answer = run(
                    StructuredAnswer {
                        kind: AnswerKind::Social,
                        parts: vec![prose(text)],
                        cited_evidence_ids: vec![],
                    },
                    vec![],
                    profile.clone(),
                    quality,
                );
                assert_eq!(answer.status, AnswerStatus::Answered);
                assert_eq!(answer.text, text);
                assert!(answer.citations.is_empty());
                assert_eq!(answer.usage.network_rounds, 1);
            }
        }
    }
}

#[test]
fn matching_values_render_from_the_exact_field_not_from_provider_text() {
    for profile in [AnswerProfile::Explain, AnswerProfile::Coaching] {
        for quality in [true, false] {
            let answer = run(
                grounded(vec![
                    prose("Abrams hat "),
                    game("health", "650"),
                    prose(" Lebenspunkte."),
                ]),
                vec![evidence(
                    "deadlock-assets-heroes",
                    "health: 650\ndamage: 42",
                )],
                profile.clone(),
                quality,
            );
            assert_eq!(answer.status, AnswerStatus::Answered);
            assert_eq!(answer.text, "Abrams hat 650 Lebenspunkte.");
            assert_eq!(answer.citations.len(), 1);
        }
    }
}

#[test]
fn mismatched_undeclared_and_wrong_field_numbers_are_silent_even_without_quality_filters() {
    for parts in [
        vec![],
        vec![game("health", "999")],
        vec![game("health", "42")],
        vec![prose("Abrams hat 650 Lebenspunkte.")],
        vec![prose("Abrams hat ６５０ Lebenspunkte.")],
    ] {
        for profile in [AnswerProfile::Explain, AnswerProfile::Coaching] {
            for quality in [true, false] {
                let answer = run(
                    grounded(parts.clone()),
                    vec![evidence(
                        "deadlock-assets-heroes",
                        "health: 650\ndamage: 42",
                    )],
                    profile.clone(),
                    quality,
                );
                assert_eq!(answer.status, AnswerStatus::InsufficientEvidence);
                assert!(answer.citations.is_empty());
                assert_eq!(answer.usage.network_rounds, 1);
            }
        }
    }
}

#[test]
fn ambient_discord_and_community_sources_cannot_back_game_slots() {
    for source in [
        "discord.public-live.v1",
        "community-coaching",
        "docs.public",
    ] {
        let answer = run(
            grounded(vec![game("health", "650")]),
            vec![evidence(source, "health: 650")],
            AnswerProfile::Explain,
            false,
        );
        assert_eq!(answer.status, AnswerStatus::InsufficientEvidence);
    }
    let mut ambient = evidence("deadlock-assets-heroes", "health: 650");
    ambient.visibility = SourceVisibility::RequestScoped;
    assert!(brain_contracts::answer_contract::citable_values(&ambient).is_empty());
}

#[test]
fn uncertain_community_coaching_remains_answered() {
    let answer = run(
        grounded(vec![prose(
            "Ich bin mir nicht sicher, aber probier mehr Deckung und schau auf die Minimap.",
        )]),
        vec![evidence(
            "community-coaching",
            "Tip: Nutze Deckung und achte auf die Minimap.",
        )],
        AnswerProfile::Coaching,
        true,
    );
    assert_eq!(answer.status, AnswerStatus::Answered);
    assert_eq!(answer.citations.len(), 1);
}

#[test]
fn social_kind_cannot_smuggle_game_value_claims_or_numbers() {
    for parts in [
        vec![game("health", "650")],
        vec![prose("Mir geht es gut. Abrams hat 650 Lebenspunkte.")],
    ] {
        let answer = run(
            StructuredAnswer {
                kind: AnswerKind::Social,
                parts,
                cited_evidence_ids: vec![],
            },
            vec![evidence("deadlock-assets-heroes", "health: 650")],
            AnswerProfile::Explain,
            true,
        );
        assert_eq!(answer.status, AnswerStatus::InsufficientEvidence);
    }
}

#[test]
fn other_slots_are_field_bound_and_cannot_relabel_health_from_discord_as_a_count() {
    for (field, value, status) in [
        ("date", "2026-10-11", AnswerStatus::Answered),
        ("health", "650", AnswerStatus::InsufficientEvidence),
        ("count", "650", AnswerStatus::InsufficientEvidence),
        ("date", "2026-10-10", AnswerStatus::InsufficientEvidence),
    ] {
        let part = AnswerPart::Other {
            evidence_id: "e1".into(),
            field: field.into(),
            value: value.into(),
        };
        let answer = run(
            grounded(vec![part]),
            vec![evidence(
                "discord.public-live.v1",
                "date: 2026-10-11\nhealth: 650\ncount: 2",
            )],
            AnswerProfile::Explain,
            true,
        );
        assert_eq!(answer.status, status);
    }
}

#[test]
fn json_pointers_preserve_decimal_strings_and_duplicate_fields_fail_closed() {
    for (content, field, value, status) in [
        (
            r#"{"starting_stats":{"speed":{"value":6.000000000000001}}}"#,
            "/starting_stats/speed/value",
            "6.000000000000001",
            AnswerStatus::Answered,
        ),
        (
            "health: 650\nhealth: 650",
            "health",
            "650",
            AnswerStatus::InsufficientEvidence,
        ),
        (
            r#"{"health":650,"health":700}"#,
            r#"{"health""#,
            r#"650,"health":700}"#,
            AnswerStatus::InsufficientEvidence,
        ),
        (
            r#"{"health":650"#,
            r#"{"health""#,
            "650",
            AnswerStatus::InsufficientEvidence,
        ),
    ] {
        let answer = run(
            grounded(vec![game(field, value)]),
            vec![evidence("deadlock-assets-heroes", content)],
            AnswerProfile::Explain,
            true,
        );
        assert_eq!(answer.status, status);
    }
}
