use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

use crate::BuildContext;

pub const DEFAULT_CORPUS_DIR: &str = "/home/naniadm/Documents/deadlock-build-corpus";
pub const BUILD_SPEC_SYSTEM_PROMPT: &str = "Du bist Experte fuer Deadlock-Builds. Waehle NUR Items aus der Kandidatenliste und erfinde niemals Items. Gib strikt ein JSON-Objekt ohne Markdown oder Fliesstext drumherum aus. Schreibe kurze deutsche Annotationen, aus dem Verstaendnis begruendet. Der Build soll wie ein echter Spieler-Build strukturiert und anfaengerfreundlich sein.";

const UNDERSTANDING_CHAR_LIMIT: usize = 6000;
const DOSSIER_CANDIDATE_LIMIT: usize = 30;

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct CorpusBuildFile {
    pub hero: String,
    pub hero_id: i64,
    #[serde(default)]
    pub builds: Vec<CorpusBuild>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct CorpusBuild {
    #[serde(default)]
    pub categories: Vec<CorpusCategory>,
    #[serde(default)]
    pub skill_order: Vec<CorpusSkillOrderEntry>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct CorpusCategory {
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub items: Vec<CorpusItem>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct CorpusItem {
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub slot: String,
    #[serde(default)]
    pub tier: i64,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct CorpusSkillOrderEntry {
    pub id: i64,
    pub delta: i64,
    pub currency: i64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CandidateItem {
    pub id: i64,
    pub name: String,
    pub slot: String,
    pub tier: i64,
    pub signal: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateSet {
    pub items: Vec<CandidateItem>,
    pub name_to_id: BTreeMap<String, i64>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct LlmBuildSpec {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub categories: Vec<LlmCategory>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct LlmCategory {
    pub name: String,
    pub optional: bool,
    #[serde(default)]
    pub items: Vec<LlmItem>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct LlmItem {
    pub name: String,
    pub annotation: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct BuildSpecPayload {
    pub hero_id: i64,
    pub name: String,
    pub description: String,
    pub language: i64,
    pub mod_categories: Vec<BuildSpecCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ability_order: Option<Vec<AbilityOrderEntry>>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct BuildSpecCategory {
    pub name: String,
    pub optional: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    pub mods: Vec<BuildSpecMod>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct BuildSpecMod {
    pub ability_id: i64,
    pub annotation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imbue: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sell_priority: Option<u32>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct AbilityOrderEntry {
    pub ability_id: i64,
    pub currency_type: i64,
    pub delta: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssemblyResult {
    pub payload: BuildSpecPayload,
    pub warnings: Vec<String>,
}

pub fn load_corpus_build_by_hero_id(corpus_dir: &Path, hero_id: i64) -> Result<CorpusBuildFile> {
    let builds_dir = corpus_dir.join("builds");
    for entry in fs::read_dir(&builds_dir)
        .with_context(|| format!("cannot read corpus builds dir: {}", builds_dir.display()))?
    {
        let path = entry?.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = fs::read_to_string(&path)
            .with_context(|| format!("cannot read corpus build JSON: {}", path.display()))?;
        let build: CorpusBuildFile = serde_json::from_str(&raw)
            .with_context(|| format!("invalid corpus build JSON: {}", path.display()))?;
        if build.hero_id == hero_id {
            return Ok(build);
        }
    }
    Err(anyhow!(
        "no corpus build JSON for hero_id {hero_id} in {}",
        builds_dir.display()
    ))
}

pub fn build_candidate_set(
    context: &BuildContext,
    corpus: &CorpusBuildFile,
    known_item_names: &BTreeSet<String>,
) -> CandidateSet {
    let known_lower = known_item_names
        .iter()
        .map(|name| normalize_item_name(name))
        .collect::<BTreeSet<_>>();
    let mut by_id = BTreeMap::new();

    for category in corpus.builds.iter().flat_map(|build| &build.categories) {
        for item in &category.items {
            if known_lower.contains(&normalize_item_name(&item.name)) {
                insert_candidate(
                    &mut by_id,
                    CandidateItem {
                        id: item.id,
                        name: item.name.clone(),
                        slot: item.slot.clone(),
                        tier: item.tier,
                        signal: format!("corpus {}", category.category),
                    },
                );
            }
        }
    }

    let mut dossier_items = context
        .primary_path
        .phases
        .iter()
        .flat_map(|phase| {
            phase
                .items
                .iter()
                .map(move |item| (phase.phase.as_str(), item))
        })
        .collect::<Vec<_>>();
    dossier_items.sort_by(|left, right| {
        right
            .1
            .prevalence_builds
            .cmp(&left.1.prevalence_builds)
            .then_with(|| {
                right
                    .1
                    .lift_pp
                    .unwrap_or(f64::MIN)
                    .partial_cmp(&left.1.lift_pp.unwrap_or(f64::MIN))
                    .unwrap_or(Ordering::Equal)
            })
    });

    for (phase, item) in dossier_items.into_iter().take(DOSSIER_CANDIDATE_LIMIT) {
        if known_lower.contains(&normalize_item_name(&item.name)) {
            insert_candidate(
                &mut by_id,
                CandidateItem {
                    id: item.item_id,
                    name: item.name.clone(),
                    slot: item.slot_type.clone(),
                    tier: item.tier,
                    signal: format!(
                        "dossier phase={phase} prevalence={} lift={}",
                        item.prevalence_builds,
                        item.lift_pp
                            .map(|value| format!("{value:.1}pp"))
                            .unwrap_or_else(|| "n/a".to_string())
                    ),
                },
            );
        }
    }

    let mut items = by_id.into_values().collect::<Vec<_>>();
    items.sort_by(|left, right| left.name.cmp(&right.name));
    let name_to_id = items
        .iter()
        .map(|item| (normalize_item_name(&item.name), item.id))
        .collect();
    CandidateSet { items, name_to_id }
}

pub fn build_spec_user_prompt(
    hero_name: &str,
    understanding: Option<&str>,
    candidates: &[CandidateItem],
    context: &BuildContext,
) -> String {
    let understanding = understanding
        .map(|text| truncate_chars(text, UNDERSTANDING_CHAR_LIMIT))
        .unwrap_or_else(|| "(missing)".to_string());
    let candidate_lines = candidates
        .iter()
        .map(|item| {
            format!(
                "{}|{}|{}|{}|{}",
                item.name, item.id, item.slot, item.tier, item.signal
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "Hero: {hero_name} (id {})\n\nUnderstanding:\n{understanding}\n\nKandidatenliste name|id|slot|tier|signal:\n{candidate_lines}\n\nDossier-Zusammenfassung:\n{}\n\nGib genau dieses JSON-Schema aus:\n{{\"name\": str, \"description\": str, \"categories\":[{{\"name\": str, \"optional\": bool, \"items\":[{{\"name\": str, \"annotation\": str}}]}}]}}\n\nKategorien exakt in dieser Reihenfolge und Benennung: \"Laning\" optional=false, \"#Citadel_HeroBuilds_MidGame\" optional=false, \"#Citadel_HeroBuilds_LateGame\" optional=true, \"Counters\" optional=true. Laning soll 4-6 Items enthalten.",
        context.hero_id,
        dossier_summary(context)
    )
}

pub fn parse_llm_spec_text(text: &str) -> Result<LlmBuildSpec> {
    let start = text.find('{').ok_or_else(|| {
        anyhow!(
            "LLM response contains no JSON object: {}",
            truncate_chars(text, 500)
        )
    })?;
    let end = text.rfind('}').filter(|end| *end >= start).ok_or_else(|| {
        anyhow!(
            "LLM response contains no JSON object: {}",
            truncate_chars(text, 500)
        )
    })?;
    serde_json::from_str(&text[start..=end])
        .with_context(|| format!("invalid LLM JSON: {}", truncate_chars(text, 1000)))
}

pub fn ability_order_from_corpus(corpus: &CorpusBuildFile) -> Option<Vec<AbilityOrderEntry>> {
    corpus
        .builds
        .iter()
        .find(|build| !build.skill_order.is_empty())
        .map(|build| {
            build
                .skill_order
                .iter()
                .map(|entry| AbilityOrderEntry {
                    ability_id: entry.id,
                    currency_type: entry.currency,
                    delta: entry.delta,
                })
                .collect()
        })
}

pub fn assemble_payload(
    hero_id: i64,
    llm: LlmBuildSpec,
    name_to_id: &BTreeMap<String, i64>,
    ability_order: Option<&[AbilityOrderEntry]>,
) -> AssemblyResult {
    let mut warnings = Vec::new();
    let mod_categories = llm
        .categories
        .into_iter()
        .map(|category| {
            let mut mods = Vec::new();
            for item in category.items {
                if let Some(ability_id) = name_to_id.get(&normalize_item_name(&item.name)) {
                    mods.push(BuildSpecMod {
                        ability_id: *ability_id,
                        annotation: item.annotation,
                        imbue: None,
                        sell_priority: None,
                    });
                } else {
                    warnings.push(format!("unknown item ignored: {}", item.name));
                }
            }
            BuildSpecCategory {
                name: category.name,
                optional: category.optional,
                description: None,
                width: None,
                height: None,
                mods,
            }
        })
        .collect();

    AssemblyResult {
        payload: BuildSpecPayload {
            hero_id,
            name: llm.name,
            description: llm.description,
            language: 1,
            mod_categories,
            ability_order: ability_order.map(<[AbilityOrderEntry]>::to_vec),
        },
        warnings,
    }
}

pub fn normalize_item_name(name: &str) -> String {
    name.trim().to_lowercase()
}

pub fn truncate_chars(text: &str, max_chars: usize) -> String {
    let mut truncated = text.chars().take(max_chars).collect::<String>();
    if text.chars().count() > max_chars {
        truncated.push_str("...");
    }
    truncated
}

fn insert_candidate(by_id: &mut BTreeMap<i64, CandidateItem>, candidate: CandidateItem) {
    by_id
        .entry(candidate.id)
        .and_modify(|existing| {
            if !existing.signal.contains(&candidate.signal) {
                existing.signal.push_str("; ");
                existing.signal.push_str(&candidate.signal);
            }
        })
        .or_insert(candidate);
}

fn dossier_summary(context: &BuildContext) -> String {
    let mut lines = vec![format!(
        "primary_path={} matches={} winrate={}",
        context.primary_path.label,
        context.primary_path.sample_matches,
        context
            .primary_path
            .winrate
            .map(|value| format!("{value:.1}%"))
            .unwrap_or_else(|| "n/a".to_string())
    )];
    for phase in &context.primary_path.phases {
        let items = phase
            .items
            .iter()
            .take(8)
            .map(|item| {
                format!(
                    "{} prev={} lift={}",
                    item.name,
                    item.prevalence_builds,
                    item.lift_pp
                        .map(|value| format!("{value:.1}pp"))
                        .unwrap_or_else(|| "n/a".to_string())
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        lines.push(format!("{}: {items}", phase.phase));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn name_mapping_skips_unknown_items_and_warns() {
        let mut name_to_id = BTreeMap::new();
        name_to_id.insert(normalize_item_name("Known Item"), 42);
        let llm = LlmBuildSpec {
            name: "Test Build".to_string(),
            description: "desc".to_string(),
            categories: vec![LlmCategory {
                name: "Laning".to_string(),
                optional: false,
                items: vec![
                    LlmItem {
                        name: "Known Item".to_string(),
                        annotation: "keep".to_string(),
                    },
                    LlmItem {
                        name: "Missing Item".to_string(),
                        annotation: "drop".to_string(),
                    },
                ],
            }],
        };

        let result = assemble_payload(7, llm, &name_to_id, None);

        assert_eq!(result.payload.mod_categories[0].mods.len(), 1);
        assert_eq!(result.payload.mod_categories[0].mods[0].ability_id, 42);
        assert_eq!(
            result.warnings,
            vec!["unknown item ignored: Missing Item".to_string()]
        );
    }

    #[test]
    fn skill_order_fixture_maps_currency_and_delta() {
        let corpus: CorpusBuildFile = serde_json::from_str(
            r#"{
                "hero": "Hero",
                "hero_id": 1,
                "builds": [
                    {"skill_order": []},
                    {"skill_order": [
                        {"ability": "One", "id": 10, "delta": -1, "currency": 2},
                        {"ability": "Two", "id": 11, "delta": -5, "currency": 1}
                    ]}
                ]
            }"#,
        )
        .unwrap();

        let order = ability_order_from_corpus(&corpus).unwrap();

        assert_eq!(
            order,
            vec![
                AbilityOrderEntry {
                    ability_id: 10,
                    currency_type: 2,
                    delta: -1,
                },
                AbilityOrderEntry {
                    ability_id: 11,
                    currency_type: 1,
                    delta: -5,
                },
            ]
        );
    }

    #[test]
    fn assemble_payload_from_llm_json_keeps_annotations_and_ability_order() {
        let llm = parse_llm_spec_text(
            r#"text before
            {"name":"Hero Build","description":"desc","categories":[
                {"name":"Laning","optional":false,"items":[{"name":"Item A","annotation":"ann a"}]},
                {"name":"Counters","optional":true,"items":[{"name":"Item B","annotation":"ann b"}]}
            ]}
            text after"#,
        )
        .unwrap();
        let mut name_to_id = BTreeMap::new();
        name_to_id.insert(normalize_item_name("Item A"), 100);
        name_to_id.insert(normalize_item_name("Item B"), 101);
        let ability_order = vec![AbilityOrderEntry {
            ability_id: 10,
            currency_type: 2,
            delta: -1,
        }];

        let result = assemble_payload(9, llm, &name_to_id, Some(&ability_order));

        assert!(result.warnings.is_empty());
        assert_eq!(result.payload.hero_id, 9);
        assert_eq!(result.payload.language, 1);
        assert_eq!(result.payload.mod_categories[0].name, "Laning");
        assert!(!result.payload.mod_categories[0].optional);
        assert_eq!(result.payload.mod_categories[0].mods[0].ability_id, 100);
        assert_eq!(result.payload.mod_categories[0].mods[0].annotation, "ann a");
        assert!(result.payload.mod_categories[1].optional);
        assert_eq!(result.payload.mod_categories[1].mods[0].ability_id, 101);
        assert_eq!(result.payload.ability_order, Some(ability_order));
    }
}
