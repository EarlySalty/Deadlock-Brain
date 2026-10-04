use brain_contracts::{
    store::AnswerPurpose, AuthorizedContext, Evidence, PortError, Query, SnapshotReadPort,
};
use chrono::NaiveDate;
use regex::Regex;
use std::sync::OnceLock;

pub(crate) fn patch_date(text: &str) -> Option<String> {
    static DATE: OnceLock<Regex> = OnceLock::new();
    let expression = DATE.get_or_init(|| {
        Regex::new(r"\b(?:(\d{4})-(\d{2})-(\d{2})|(\d{1,2})\.(\d{1,2})\.(?:(\d{4})\b)?)")
            .expect("Datumsformat")
    });
    let mut dates = Vec::new();
    for capture in expression.captures_iter(text) {
        let (year, month, day) = if let Some(year) = capture.get(1) {
            (Some(year.as_str()), &capture[2], &capture[3])
        } else {
            (
                capture.get(6).map(|value| value.as_str()),
                &capture[5],
                &capture[4],
            )
        };
        let day: u32 = day.parse().ok()?;
        let month: u32 = month.parse().ok()?;
        let year_number = year.unwrap_or("2000").parse().ok()?;
        NaiveDate::from_ymd_opt(year_number, month, day)?;
        dates.push(year.map_or_else(
            || format!("%-{month:02}-{day:02}"),
            |year| format!("{year}-{month:02}-{day:02}"),
        ));
    }
    dates.sort();
    dates.dedup();
    if dates.len() > 1 {
        return None;
    }
    dates.pop()
}

fn invalid() -> PortError {
    PortError::InvalidResponse("Patchdatum ist nicht eindeutig oder ungültig".into())
}

pub(crate) fn retrieve<S: SnapshotReadPort>(
    store: &S,
    query: &Query,
    context: &AuthorizedContext,
    provider: bool,
    purpose: AnswerPurpose,
) -> Result<Option<Vec<Evidence>>, PortError> {
    context.check_deadline()?;
    let date = patch_date(&query.text);
    let evidence =
        store.read_entity_evidence(query, context, date.as_deref(), provider, purpose)?;
    context.check_deadline()?;
    let Some(mut evidence) = evidence else {
        return Ok(None);
    };
    if evidence
        .iter()
        .any(|item| !item.evidence_id.starts_with("entity-profile:"))
    {
        return Err(PortError::InvalidResponse(
            "Ungültiger Entitätsbeleg".into(),
        ));
    }
    for item in &evidence {
        item.validate().map_err(|_| invalid())?;
    }
    if evidence.len() > 100 {
        if query.profile == brain_contracts::AnswerProfile::Fact {
            return Err(PortError::BudgetExceeded);
        }
        let words = brain_contracts::lexical::terms(&query.text);
        let relevance = |item: &Evidence| {
            item.provenance
                .as_ref()
                .and_then(|provenance| provenance.metadata.get("fact_key"))
                .map(|key| {
                    brain_contracts::lexical::terms(key)
                        .iter()
                        .filter(|term| words.contains(term))
                        .count()
                })
                .unwrap_or(0)
        };
        evidence.sort_by(|left, right| {
            relevance(right)
                .cmp(&relevance(left))
                .then_with(|| left.evidence_id.cmp(&right.evidence_id))
        });
        evidence.truncate(100);
    }
    crate::pack(query, context, evidence).map(Some)
}

#[cfg(test)]
mod tests {
    use super::*;
    use brain_contracts::{
        AnswerProfile, Budget, CorpusSnapshot, EvidenceKind, Principal, RetrievalPort,
        SourceVisibility,
    };
    use std::{
        collections::BTreeSet,
        sync::{Arc, Mutex},
    };

    #[derive(Clone)]
    struct Reader(Arc<Mutex<Vec<Evidence>>>);

    impl SnapshotReadPort for Reader {
        fn read_snapshot(&self, _: &str) -> Result<CorpusSnapshot, PortError> {
            panic!("Entitätsfragen dürfen keinen alten HTML-Snapshot lesen")
        }

        fn read_entity_evidence(
            &self,
            query: &Query,
            _: &AuthorizedContext,
            date: Option<&str>,
            _: bool,
            _: AnswerPurpose,
        ) -> Result<Option<Vec<Evidence>>, PortError> {
            if query.text.contains("16.09.") {
                assert_eq!(date, Some("%-09-16"));
            }
            Ok(Some(self.0.lock().unwrap().clone()))
        }
    }

    fn fixture(text: &str) -> (Query, AuthorizedContext, Reader) {
        let query = Query {
            request_id: "r1".into(),
            conversation_id: "c1".into(),
            text: text.into(),
            domain: None,
            requested_scopes: BTreeSet::new(),
            profile: AnswerProfile::Explain,
            patch: None,
            mode: None,
        };
        let context = AuthorizedContext {
            request_deadline: None,
            principal: Principal {
                actor_id: "fixture".into(),
                channel: "test".into(),
                scopes: BTreeSet::new(),
                provider_egress: BTreeSet::from(["public".into()]),
            },
            conversation_id: "c1".into(),
            knowledge_release: "r1".into(),
            deadline_ms: 1000,
            budget: Budget::default(),
        };
        let evidence = Evidence {
            evidence_id: "entity-profile:fixture".into(),
            source_id: "fixture".into(),
            logical_id: "hero/fixture".into(),
            revision: 1,
            kind: EvidenceKind::Prose,
            content: "Beleg aus der Datenbank".into(),
            citation: "fixture".into(),
            visibility: SourceVisibility::Public,
            allowed_scopes: BTreeSet::new(),
            score: 1.0,
            provenance: None,
            patch: None,
        };
        (query, context, Reader(Arc::new(Mutex::new(vec![evidence]))))
    }

    #[test]
    fn historical_question_preserves_unknown_year() {
        assert_eq!(
            patch_date("Was wurde bei Abrams im Patch vom 16.09. geändert?"),
            Some("%-09-16".into())
        );
        assert_eq!(
            patch_date("Abrams am 16.09.2026"),
            Some("2026-09-16".into())
        );
        assert_eq!(
            patch_date("Item im Patch 2026-09-16"),
            Some("2026-09-16".into())
        );
        assert_eq!(patch_date("Wie viele Lebenspunkte hat Abrams?"), None);
    }

    #[test]
    fn ungueltige_und_mehrdeutige_muster_ergeben_keinen_patchtermin() {
        for text in [
            "Patch 31.02.2026",
            "Patch 16.09. und 17.09.",
            "Version 0.5.1",
            "Patch 6.0.1",
            "Zwischen 01.09. und 16.09.",
        ] {
            assert_eq!(patch_date(text), None, "{text}");
        }
    }

    #[test]
    fn normal_hero_item_and_patch_texts_use_reader_and_revalidate_changes() {
        for text in [
            "Wie viele Lebenspunkte hat Abrams?",
            "Was macht das Item Mystic Burst?",
            "Was änderte sich bei Abrams im Patch vom 16.09.?",
        ] {
            let (query, context, reader) = fixture(text);
            let retriever = crate::ReleaseRetriever::new(reader.clone(), 10);
            let evidence = retriever.retrieve(&query, &context).unwrap();
            assert_eq!(evidence.len(), 1);
            assert!(retriever
                .validate_evidence(&query, &context, &evidence, false)
                .is_ok());
            reader.0.lock().unwrap()[0].content = "Neuer gespeicherter Stand".into();
            assert!(retriever
                .validate_evidence(&query, &context, &evidence, false)
                .is_err());
            reader.0.lock().unwrap().clear();
            assert!(retriever.retrieve(&query, &context).unwrap().is_empty());
            assert!(retriever
                .validate_publication(&query, &context, &evidence)
                .is_err());
        }
    }
}
