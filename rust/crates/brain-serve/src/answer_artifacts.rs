use brain_contracts::{
    public_api::MAX_PUBLIC_ARTIFACTS, AnswerArtifactPort, Evidence, PortError, PublicArtifact,
    RequestDeadline, SourceVisibility,
};
use serde_json::Value;
use std::{
    collections::BTreeSet,
    time::{Duration, Instant},
};

const LOOKUP_TIMEOUT: Duration = Duration::from_secs(3);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct HeroPair {
    heroes: [i64; 2],
    client_version: i64,
    title: String,
}

fn hero_pair(evidence: &Evidence) -> Option<HeroPair> {
    if evidence.source_id != crate::deadlock_data::GAME_SOURCE
        || evidence.visibility != SourceVisibility::Public
        || !evidence
            .logical_id
            .starts_with("assets-mirror/hero-compare/")
    {
        return None;
    }
    let content: Value = serde_json::from_str(&evidence.content).ok()?;
    if content["operation"] != "hero_compare" || content["metric"] != "health" {
        return None;
    }
    let side = |key: &str| -> Option<(i64, String)> {
        let entry = &content[key];
        let name = entry["name"].as_str()?.trim();
        (!name.is_empty() && name.len() <= 64 && !name.chars().any(char::is_control))
            .then(|| (entry["hero_id"].as_i64(), name.to_owned()))
            .and_then(|(id, name)| Some((id.filter(|id| *id > 0)?, name)))
    };
    let (left, left_name) = side("hero")?;
    let (right, right_name) = side("opponent")?;
    let client_version = content["client_version"].as_i64().filter(|v| *v > 0)?;
    (left != right).then(|| HeroPair {
        heroes: [left, right],
        client_version,
        title: format!("{left_name} gegen {right_name}: Lebenspunkte nach Boons"),
    })
}

pub(crate) struct CompareArtifacts {
    pool: sqlx::PgPool,
    runtime: tokio::runtime::Handle,
}

impl CompareArtifacts {
    pub(crate) fn new(pool: sqlx::PgPool, runtime: tokio::runtime::Handle) -> Self {
        Self { pool, runtime }
    }

    fn lookup(
        &self,
        release: &str,
        pair: &HeroPair,
        budget: Duration,
    ) -> Result<Option<String>, PortError> {
        if tokio::runtime::Handle::try_current().is_ok_and(|handle| {
            handle.runtime_flavor() == tokio::runtime::RuntimeFlavor::CurrentThread
        }) {
            return Err(PortError::Unavailable(
                "Grafiksuche benötigt den blockierenden Servicepfad".into(),
            ));
        }
        let store = brain_storage::PgStore::new(self.pool.clone());
        let read = store.find_published_hero_compare(release, pair.heroes, pair.client_version);
        let bounded = async {
            tokio::time::timeout(budget, read)
                .await
                .map_err(|_| PortError::Unavailable("Grafiksuche hat zu lange gedauert".into()))?
        };
        if tokio::runtime::Handle::try_current().is_ok() {
            tokio::task::block_in_place(|| self.runtime.block_on(bounded))
        } else {
            self.runtime.block_on(bounded)
        }
    }
}

impl AnswerArtifactPort for CompareArtifacts {
    fn artifacts(
        &self,
        knowledge_release: &str,
        citations: &[Evidence],
        deadline: &RequestDeadline,
    ) -> Result<Vec<PublicArtifact>, PortError> {
        let pairs: BTreeSet<_> = citations.iter().filter_map(hero_pair).collect();
        if pairs.is_empty() {
            return Ok(Vec::new());
        }
        let stop = Instant::now() + deadline.remaining()?.min(LOOKUP_TIMEOUT);
        let mut artifacts = Vec::new();
        let mut seen = BTreeSet::new();
        for pair in pairs.iter().take(MAX_PUBLIC_ARTIFACTS) {
            let budget = stop.saturating_duration_since(Instant::now());
            if budget.is_zero() {
                return Err(PortError::Unavailable(
                    "Grafiksuche hat zu lange gedauert".into(),
                ));
            }
            if let Some(id) = self.lookup(knowledge_release, pair, budget)? {
                if seen.insert(id.clone()) {
                    artifacts.push(PublicArtifact::hero_compare(&id, &pair.title)?);
                }
            }
        }
        Ok(artifacts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn evidence(content: Value) -> Evidence {
        Evidence {
            evidence_id: "deadlock-api:fixture".into(),
            source_id: crate::deadlock_data::GAME_SOURCE.into(),
            logical_id: "assets-mirror/hero-compare/13-1/fixture".into(),
            revision: 6766,
            kind: brain_contracts::EvidenceKind::Prose,
            content: content.to_string(),
            citation: "Assets-Spiegel Client 6766: Lebenspunkte nach Boons".into(),
            visibility: SourceVisibility::Public,
            allowed_scopes: Default::default(),
            score: 1.0,
            patch: None,
            provenance: None,
        }
    }

    #[test]
    fn only_service_hero_compare_results_name_a_pair() {
        let valid = serde_json::json!({"operation":"hero_compare","metric":"health",
            "client_version":6766,"hero":{"hero_id":13,"name":"Haze"},
            "opponent":{"hero_id":1,"name":"Infernus"}});
        let pair = hero_pair(&evidence(valid.clone())).expect("pair");
        assert_eq!(pair.heroes, [13, 1]);
        assert_eq!(pair.title, "Haze gegen Infernus: Lebenspunkte nach Boons");
        let mut other_source = evidence(valid.clone());
        other_source.source_id = "deadlock-api-live-v1".into();
        assert!(hero_pair(&other_source).is_none());
        let mut same = valid.clone();
        same["opponent"]["hero_id"] = serde_json::json!(13);
        assert!(hero_pair(&evidence(same)).is_none());
        let mut counters = valid;
        counters["operation"] = serde_json::json!("counters");
        assert!(hero_pair(&evidence(counters)).is_none());
    }
}
