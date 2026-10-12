use brain_contracts::{
    answer_contract::{AnswerKind, AnswerPart, AnswerViolation, StructuredAnswer},
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

fn other(field: &str, value: &str) -> AnswerPart {
    AnswerPart::Other {
        evidence_id: "e1".into(),
        field: field.into(),
        value: value.into(),
    }
}

const NUMERIC_LITERALS: &[&str] = &[
    "65042",
    "650,42",
    "650.42",
    "650٫42",
    "650，42",
    "650．42",
    "650e42",
    "650E+42",
    "650e-42",
    "650e−42",
    "650.e42",
    "650.E+42",
    "650.e-42",
    "650.e−42",
    "650_e42",
    "650e_42",
    "650_e_42",
    "650__e__42",
    "650.e4_2",
    "650.E+4'2",
    "650e4__2",
    "650e42_",
    "650_",
    "650__42",
    "650.E＋42",
    "650.E－42",
    "+650.e42",
    "−650.e-42",
    "650,e42",
    "650٫E+42",
    "650．e-42",
    "650，e−42",
    "650_042.e42",
    "650'042.E+42",
    "650’042.e-42",
    "650ʼ042.e−42",
    "650٬042.E＋42",
    "1 234 567.e-42",
    "650\u{00a0}042.e42",
    "650\u{2007}042.e42",
    "650\u{2009}042.e42",
    "650\u{202f}042.e42",
    "６５０．e−４２",
    "0x650",
    "0XdeadBEEF",
    "0b101",
    "0B101",
    "0o650",
    "0O650",
    "+0x650",
    "-0b101",
    "−0o650",
    "0x_650",
    "0x__650",
    "0x650_",
    "0b1_01",
    "0o6_50",
    "0x6'50",
    "0x6’50",
    "0x6ʼ50",
    "0x650p42",
    "0x650.p42",
    "0x650.p+42",
    "0x650.p-42",
    "0x650p4_2",
    "0x650.p4'2",
    "0x1.8p−42",
    "0x.8p42",
    "0Xf.P＋42",
    "+650",
    "-650",
    "−650",
    "＋650",
    "－650",
    ".42",
    ",42",
    "-.42",
    "650_042",
    "650'042",
    "650’042",
    "650ʼ042",
    "650٬042",
    "650 042",
    "650\u{00a0}042",
    "650\u{2007}042",
    "650\u{2009}042",
    "650\u{202f}042",
    "1 234 567,89e-42",
    "６５０,４２",
];

fn numeric_evidence(left: &str, right: &str) -> Evidence {
    evidence(
        "deadlock-assets-heroes",
        &serde_json::json!({
            "health": left,
            "damage": right,
            "count": left,
            "capacity": right,
        })
        .to_string(),
    )
}

#[test]
fn numeric_literals_cannot_cross_exact_value_slots() {
    for literal in NUMERIC_LITERALS {
        for (split, _) in literal.char_indices().skip(1) {
            let (left, right) = literal.split_at(split);
            for first in [game("/health", left), other("/count", left)] {
                for second in [game("/damage", right), other("/capacity", right)] {
                    let source = numeric_evidence(left, right);
                    assert_eq!(
                        grounded(vec![first.clone()]).render(std::slice::from_ref(&source)),
                        Ok(left.to_owned())
                    );
                    assert_eq!(
                        grounded(vec![second.clone()]).render(std::slice::from_ref(&source)),
                        Ok(right.to_owned())
                    );
                    assert_eq!(
                        grounded(vec![first.clone(), second.clone()])
                            .render(std::slice::from_ref(&source)),
                        Err(AnswerViolation::Value)
                    );
                    for profile in [AnswerProfile::Explain, AnswerProfile::Coaching] {
                        for quality in [true, false] {
                            let answer = run(
                                grounded(vec![first.clone(), prose(""), second.clone()]),
                                vec![numeric_evidence(left, right)],
                                profile.clone(),
                                quality,
                            );
                            assert_eq!(
                                answer.status,
                                AnswerStatus::InsufficientEvidence,
                                "{literal:?}, split {split}, {first:?}, {second:?}"
                            );
                            assert!(answer.citations.is_empty());
                            assert_eq!(answer.usage.network_rounds, 1);
                            assert_eq!(answer.usage.output_tokens, 7);
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn numeric_free_prose_cannot_join_values_into_a_new_literal() {
    for separator in [
        "", ".", ",", "٫", "．", "，", "e", "E+", "e-", "E−", "_", "__", "_e", "e_", "_e_",
        "__e__", "'", "’", "ʼ", "٬", " ", "\u{00a0}", "\u{2007}", "\u{2009}", "\u{202f}",
    ] {
        let right = if !separator.is_empty() && separator.chars().all(char::is_whitespace) {
            "042"
        } else {
            "42"
        };
        for first in [game("/health", "650"), other("/count", "650")] {
            for second in [game("/damage", right), other("/capacity", right)] {
                for fragmented in [true, false] {
                    let mut parts = vec![first.clone()];
                    if fragmented {
                        parts.extend(separator.chars().map(|value| prose(&value.to_string())));
                    } else {
                        parts.push(prose(separator));
                    }
                    parts.push(second.clone());
                    for profile in [AnswerProfile::Explain, AnswerProfile::Coaching] {
                        for quality in [true, false] {
                            let answer = run(
                                grounded(parts.clone()),
                                vec![numeric_evidence("650", right)],
                                profile.clone(),
                                quality,
                            );
                            assert_eq!(
                                answer.status,
                                AnswerStatus::InsufficientEvidence,
                                "{separator:?}, {first:?}, {second:?}, fragmented {fragmented}"
                            );
                            assert!(answer.citations.is_empty());
                            assert_eq!(answer.usage.network_rounds, 1);
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn trailing_decimal_exponents_cannot_be_composed_with_prose() {
    for decimal in [".", ",", "٫", "．", "，"] {
        for exponent in ["e", "E"] {
            for sign in ["", "+", "-", "−", "＋", "－"] {
                let separator = format!("{decimal}{exponent}{sign}");
                let literal = format!("650{separator}42");
                for value in [game("/health", &literal), other("/count", &literal)] {
                    assert_eq!(
                        grounded(vec![value]).render(&[numeric_evidence(&literal, "42")]),
                        Ok(literal.clone())
                    );
                }
                for first in [game("/health", "650"), other("/count", "650")] {
                    for second in [game("/damage", "42"), other("/capacity", "42")] {
                        for fragmented in [false, true] {
                            let mut parts = vec![first.clone()];
                            if fragmented {
                                parts.extend(
                                    separator.chars().map(|value| prose(&value.to_string())),
                                );
                            } else {
                                parts.push(prose(&separator));
                            }
                            parts.push(second.clone());
                            assert_eq!(
                                grounded(parts.clone()).render(&[numeric_evidence("650", "42")]),
                                Err(AnswerViolation::Value),
                                "{literal:?}, {first:?}, {second:?}, fragmented {fragmented}"
                            );
                            for profile in [AnswerProfile::Explain, AnswerProfile::Coaching] {
                                for quality in [true, false] {
                                    let answer = run(
                                        grounded(parts.clone()),
                                        vec![numeric_evidence("650", "42")],
                                        profile.clone(),
                                        quality,
                                    );
                                    assert_eq!(
                                        answer.status,
                                        AnswerStatus::InsufficientEvidence,
                                        "{literal:?}, {first:?}, {second:?}, fragmented {fragmented}"
                                    );
                                    assert!(answer.citations.is_empty());
                                    assert_eq!(answer.usage.network_rounds, 1);
                                    assert_eq!(answer.usage.output_tokens, 7);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn radix_literals_and_separated_exponents_cannot_be_composed_with_prose() {
    for (left, separator, right) in [
        ("0", "x", "650"),
        ("0", "X", "deadBEEF"),
        ("0", "b", "101"),
        ("0", "B", "101"),
        ("0", "o", "650"),
        ("0", "O", "650"),
        ("0", "x_", "650"),
        ("0", "X__", "650"),
        ("0x650", "p", "42"),
        ("0x650", ".p", "42"),
        ("0x650", ".p+", "42"),
        ("0x650", ".P−", "42"),
        ("0x", ".fp-", "42"),
        ("650e4", "_", "2"),
        ("650e4", "'", "2"),
        ("650.e4", "_", "2"),
        ("650", "e_", "42"),
        ("650", "_e", "42"),
        ("650__", "e_", "42"),
        ("0x650p4", "_", "2"),
        ("0x650.p4", "'", "2"),
    ] {
        let literal = format!("{left}{separator}{right}");
        for value in [game("/health", &literal), other("/count", &literal)] {
            assert_eq!(
                grounded(vec![value]).render(&[numeric_evidence(&literal, right)]),
                Ok(literal.clone())
            );
        }
        for first in [game("/health", left), other("/count", left)] {
            for second in [game("/damage", right), other("/capacity", right)] {
                for fragmented in [false, true] {
                    let mut parts = vec![first.clone()];
                    if fragmented {
                        parts.extend(separator.chars().map(|value| prose(&value.to_string())));
                    } else {
                        parts.push(prose(separator));
                    }
                    parts.push(second.clone());
                    assert_eq!(
                        grounded(parts.clone()).render(&[numeric_evidence(left, right)]),
                        Err(AnswerViolation::Value),
                        "{literal:?}, {first:?}, {second:?}, fragmented {fragmented}"
                    );
                    for profile in [AnswerProfile::Explain, AnswerProfile::Coaching] {
                        for quality in [true, false] {
                            let answer = run(
                                grounded(parts.clone()),
                                vec![numeric_evidence(left, right)],
                                profile.clone(),
                                quality,
                            );
                            assert_eq!(
                                answer.status,
                                AnswerStatus::InsufficientEvidence,
                                "{literal:?}, {first:?}, {second:?}, fragmented {fragmented}"
                            );
                            assert!(answer.citations.is_empty());
                            assert_eq!(answer.usage.network_rounds, 1);
                            assert_eq!(answer.usage.output_tokens, 7);
                        }
                    }
                }
            }
        }
    }
    for (literal, suffix) in [
        ("0x650", "deadBEEF"),
        ("0x650", "_"),
        ("0x650", "__"),
        ("650", "_"),
        ("650", "__"),
        ("650e42", "_"),
        ("650e42", "__"),
    ] {
        for value in [game("/health", literal), other("/count", literal)] {
            let answer = run(
                grounded(vec![value, prose(suffix)]),
                vec![numeric_evidence(literal, "42")],
                AnswerProfile::Explain,
                false,
            );
            assert_eq!(
                answer.status,
                AnswerStatus::InsufficientEvidence,
                "{suffix:?}"
            );
            assert!(answer.citations.is_empty());
        }
    }
    for separator in [
        ". Ende. Danach ",
        ", ",
        ".p ohne Exponent, dann ",
        ".P+ ohne Exponent, dann ",
    ] {
        for first in [game("/health", "0x650"), other("/count", "0x650")] {
            for second in [game("/damage", "42"), other("/capacity", "42")] {
                let answer = run(
                    grounded(vec![first.clone(), prose(separator), second]),
                    vec![numeric_evidence("0x650", "42")],
                    AnswerProfile::Explain,
                    false,
                );
                assert_eq!(answer.status, AnswerStatus::Answered, "{separator:?}");
                assert_eq!(answer.text, format!("0x650{separator}42"));
                assert_eq!(answer.citations.len(), 1);
            }
        }
    }
}

#[test]
fn sentence_punctuation_and_incomplete_exponents_remain_separate_from_values() {
    for separator in [
        ".",
        ",",
        ". Ende. Danach ",
        ".Ende. Danach ",
        ",Einordnung: ",
        ".e ohne Exponent, dann ",
        ".E+ ohne Exponent, dann ",
        ".e- ohne Exponent, dann ",
        ".e− ohne Exponent, dann ",
        "e ohne Exponent, dann ",
        "E+ ohne Exponent, dann ",
        "e- ohne Exponent, dann ",
    ] {
        for first in [game("/health", "650"), other("/count", "650")] {
            assert_eq!(
                grounded(vec![first.clone(), prose(separator)])
                    .render(&[numeric_evidence("650", "42")]),
                Ok(format!("650{separator}")),
                "{separator:?}, {first:?}"
            );
            if !separator.ends_with(' ') {
                continue;
            }
            for second in [game("/damage", "42"), other("/capacity", "42")] {
                let answer = run(
                    grounded(vec![first.clone(), prose(separator), second]),
                    vec![numeric_evidence("650", "42")],
                    AnswerProfile::Explain,
                    false,
                );
                assert_eq!(answer.status, AnswerStatus::Answered, "{separator:?}");
                assert_eq!(answer.text, format!("650{separator}42"));
                assert_eq!(answer.citations.len(), 1);
            }
        }
    }
}

#[test]
fn numeric_prefixes_from_prose_are_not_evidence_bound() {
    for prefix in ["+", "-", "−", "＋", "－", ".", ",", "٫", "．", "，", "-."] {
        for value in [game("/health", "650"), other("/count", "650")] {
            let answer = run(
                grounded(vec![prose(prefix), value]),
                vec![numeric_evidence("650", "042")],
                AnswerProfile::Explain,
                false,
            );
            assert_eq!(
                answer.status,
                AnswerStatus::InsufficientEvidence,
                "{prefix:?}"
            );
            assert!(answer.citations.is_empty());
        }
    }
}

#[test]
fn complete_literals_in_one_field_and_clearly_separated_values_remain_answerable() {
    for literal in NUMERIC_LITERALS {
        for value in [game("/health", literal), other("/count", literal)] {
            let answer = run(
                grounded(vec![prose("Belegt ist "), value, prose(" als Wert.")]),
                vec![numeric_evidence(literal, "42")],
                AnswerProfile::Explain,
                false,
            );
            assert_eq!(answer.status, AnswerStatus::Answered, "{literal:?}");
            assert_eq!(answer.text, format!("Belegt ist {literal} als Wert."));
            assert_eq!(answer.citations.len(), 1);
        }
    }
    for separator in [
        " und ",
        ", ",
        ". ",
        ";",
        " / ",
        "\n",
        " ",
        " Lebenspunkte und ",
    ] {
        for first in [game("/health", "650"), other("/count", "650")] {
            for second in [game("/damage", "42"), other("/capacity", "42")] {
                let answer = run(
                    grounded(vec![first.clone(), prose(separator), second]),
                    vec![numeric_evidence("650", "42")],
                    AnswerProfile::Coaching,
                    true,
                );
                assert_eq!(answer.status, AnswerStatus::Answered, "{separator:?}");
                assert_eq!(answer.text, format!("650{separator}42"));
                assert_eq!(answer.citations.len(), 1);
            }
        }
    }
}

#[test]
fn values_from_different_authorities_cannot_compose_a_literal() {
    for separator in [",", "e-", ""] {
        let mut metadata = evidence("discord.public-live.v1", "count: 42");
        metadata.evidence_id = "e2".into();
        let value = AnswerPart::Other {
            evidence_id: "e2".into(),
            field: "count".into(),
            value: "42".into(),
        };
        for parts in [
            vec![game("health", "650"), prose(separator), value.clone()],
            vec![value.clone(), prose(separator), game("health", "650")],
        ] {
            let mut structured = grounded(parts);
            structured.cited_evidence_ids.push("e2".into());
            let answer = run(
                structured,
                vec![
                    evidence("deadlock-assets-heroes", "health: 650"),
                    metadata.clone(),
                ],
                AnswerProfile::Explain,
                false,
            );
            assert_eq!(answer.status, AnswerStatus::InsufficientEvidence);
            assert!(answer.citations.is_empty());
            assert_eq!(answer.usage.network_rounds, 1);
        }
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
