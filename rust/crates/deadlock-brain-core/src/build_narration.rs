use std::collections::{BTreeMap, BTreeSet};

use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};

pub use dbrain_builds::{BuildContext, BuildPath, BuildPathSummary, BuildPhase, ItemDossier};

use crate::ai::{
    extract_ai_text, ChatCompletionRequest, ChatMessage, AiClient, AiConfig,
};

pub const BUILD_NARRATION_SYSTEM_PROMPT: &str = "Du bist ein Deadlock-Build-Coach für einen deutschen Discord. Dir wird ein Build vorgelegt, der bereits aus echten High-MMR-Spieldaten berechnet wurde: Pfade, Phasen und pro Item harte Fakten (slot_type, tier, defense_kind, damage_axis, prevalence_builds, winrate, lift_pp, sample_matches, confidence, buy_phase, synergy_with) sowie zum Helden archetype und hero_base_health.

Deine Aufgabe: Erkläre genau diesen Build auf Deutsch und leite jede Begründung selbst aus den gelieferten Fakten ab. Behaupte nichts, was nicht aus den Daten folgt.

Harte Regeln:
- Nenne ausschließlich Items, die im Kontext stehen. Erfinde keine Items und tausche keine aus.
- Item-, Helden- und Ability-Namen bleiben exakt Englisch, der Rest ist Deutsch.
- Sei bei niedriger confidence oder kleiner sample_matches ehrlich vorsichtig.

So gehst du beim Begründen vor (Methodik, kein vorgegebenes Ergebnis):
- Verteidigung: berücksichtige, wie defense_kind und hero_base_health zusammenwirken, also ob ein Schutz mit steigender HP an Wert gewinnt oder verliert.
- Schadensachse: prüfe, ob die damage_axis eines Items zur Achse des gewählten Pfades passt; liegt der Hauptwert daneben, bleibt er für diesen Build weitgehend wirkungslos.
- Gewichtung: lies prevalence_builds, winrate und lift_pp zusammen statt einzeln; eine einzelne Zahl trägt keine Aussage. sample_matches und confidence sagen dir, wie sehr du der Zahl traust.
- Ablauf: nutze buy_phase und synergy_with, um Kauf-Reihenfolge und Zusammenspiel zu erklären.";
pub const BUILD_NARRATION_USER_PROMPT: &str = "Erkläre den folgenden, bereits berechneten Build verständlich auf Deutsch. Geh die Phasen early/mid/late durch und begründe pro Kern-Item kurz, warum es hier passt — ausschließlich anhand der gelieferten Fakten. Schließe mit einem kurzen Hinweis auf alternative_paths, falls vorhanden. Knapp und konkret, kein Marketing.";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationResult {
    pub text: String,
    pub violations: Vec<ValidationViolation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationViolation {
    pub term: String,
}

pub fn narrate_build(ctx: &BuildContext) -> anyhow::Result<String> {
    let client = AiClient::from_env().context("load Fireworks client from environment")?;
    let request = build_narration_request(ctx, client.config())?;
    let response = client
        .chat(&request)
        .context("call Fireworks build narration")?;
    let text = extract_ai_text(&response);
    if text.trim().is_empty() {
        return Err(anyhow!("empty Fireworks response"));
    }
    Ok(text)
}

pub fn build_narration_request(
    ctx: &BuildContext,
    config: &AiConfig,
) -> anyhow::Result<ChatCompletionRequest> {
    Ok(ChatCompletionRequest::new(
        vec![
            ChatMessage::system(BUILD_NARRATION_SYSTEM_PROMPT),
            ChatMessage::user(build_narration_user_prompt(ctx)?),
        ],
        config,
    ))
}

pub fn build_narration_user_prompt(ctx: &BuildContext) -> anyhow::Result<String> {
    let context_json = serde_json::to_string(ctx).context("serialize build context")?;
    Ok(format!(
        "{BUILD_NARRATION_USER_PROMPT}\n\nBUILD_CONTEXT_JSON:\n{context_json}"
    ))
}

pub fn validate_narration(
    text: &str,
    ctx: &BuildContext,
    known_item_names: &BTreeSet<String>,
) -> ValidationResult {
    let allowed_items = normalized_names(collect_item_names(ctx).iter().map(String::as_str));
    let known_items = canonical_names_by_normalized_key(known_item_names);
    let normalized_text = text.to_ascii_lowercase();
    let allowed_ranges = allowed_items
        .iter()
        .flat_map(|item_name| phrase_match_ranges(&normalized_text, item_name))
        .collect::<Vec<_>>();
    let violations = known_items
        .into_iter()
        .filter(|(normalized_name, _)| !allowed_items.contains(normalized_name))
        .filter(|(normalized_name, _)| {
            phrase_match_ranges(&normalized_text, normalized_name)
                .into_iter()
                .any(|range| !range_is_covered_by_allowed_item(range, &allowed_ranges))
        })
        .map(|(_, term)| ValidationViolation { term })
        .collect::<Vec<_>>();
    ValidationResult {
        text: text.to_string(),
        violations,
    }
}

fn collect_item_names(ctx: &BuildContext) -> BTreeSet<String> {
    ctx.primary_path
        .phases
        .iter()
        .flat_map(|phase| phase.items.iter())
        .map(|item| item.name.clone())
        .filter(|name| !name.trim().is_empty())
        .collect()
}

fn normalized_names<'a>(names: impl Iterator<Item = &'a str>) -> BTreeSet<String> {
    names
        .map(normalize_item_name)
        .filter(|name| !name.is_empty())
        .collect()
}

fn canonical_names_by_normalized_key(
    known_item_names: &BTreeSet<String>,
) -> BTreeMap<String, String> {
    known_item_names
        .iter()
        .filter_map(|name| {
            let normalized = normalize_item_name(name);
            (!normalized.is_empty()).then(|| (normalized, name.trim().to_string()))
        })
        .collect()
}

fn normalize_item_name(name: &str) -> String {
    name.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

fn phrase_match_ranges(text: &str, phrase: &str) -> Vec<(usize, usize)> {
    if phrase.trim().is_empty() {
        return Vec::new();
    }
    text.match_indices(phrase)
        .filter_map(|(start, _)| {
            let end = start + phrase.len();
            has_ascii_boundaries(text, start, end).then_some((start, end))
        })
        .collect()
}

fn range_is_covered_by_allowed_item(
    range: (usize, usize),
    allowed_ranges: &[(usize, usize)],
) -> bool {
    allowed_ranges
        .iter()
        .any(|allowed| allowed.0 <= range.0 && range.1 <= allowed.1)
}

fn has_ascii_boundaries(text: &str, start: usize, end: usize) -> bool {
    let start_ok = start == 0
        || text[..start]
            .chars()
            .next_back()
            .is_none_or(|character| !character.is_ascii_alphanumeric());
    let end_ok = end >= text.len()
        || text[end..]
            .chars()
            .next()
            .is_none_or(|character| !character.is_ascii_alphanumeric());
    start_ok && end_ok
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_context() -> BuildContext {
        BuildContext {
            hero_id: 7,
            hero_name: "Seven".to_string(),
            hero_archetype: "Spirit Carry".to_string(),
            hero_base_health: Some(550.0),
            playstyle: Some("spirit".to_string()),
            primary_path: BuildPath {
                label: "Spirit Burst".to_string(),
                winrate: Some(0.54),
                sample_matches: 1200,
                phases: vec![
                    BuildPhase {
                        phase: "early".to_string(),
                        items: vec![ItemDossier {
                            item_id: 101,
                            name: "Mystic Burst".to_string(),
                            slot_type: "spirit".to_string(),
                            tier: 2,
                            defense_kind: vec![],
                            damage_axis: "spirit".to_string(),
                            prevalence_builds: 42,
                            winrate: Some(0.53),
                            sample_matches: 400,
                            lift_pp: Some(2.1),
                            buy_phase: "early".to_string(),
                            synergy_with: vec!["Storm Cloud".to_string()],
                            confidence: "high".to_string(),
                        }],
                    },
                    BuildPhase {
                        phase: "mid".to_string(),
                        items: vec![ItemDossier {
                            item_id: 102,
                            name: "Improved Spirit Armor".to_string(),
                            slot_type: "vitality".to_string(),
                            tier: 3,
                            defense_kind: vec!["spirit_resist".to_string()],
                            damage_axis: "none".to_string(),
                            prevalence_builds: 18,
                            winrate: Some(0.55),
                            sample_matches: 300,
                            lift_pp: Some(1.4),
                            buy_phase: "mid".to_string(),
                            synergy_with: vec![],
                            confidence: "medium".to_string(),
                        }],
                    },
                ],
            },
            alternative_paths: vec![BuildPathSummary {
                label: "Weapon Tempo".to_string(),
                winrate: Some(0.51),
                sample_matches: 220,
            }],
            ability_order: Some(vec![1, 2, 3, 4]),
            generated_at: 1_772_147_200,
        }
    }

    fn known_items(names: &[&str]) -> BTreeSet<String> {
        names.iter().map(|name| (*name).to_string()).collect()
    }

    #[test]
    fn validate_narration_ignores_german_prose_and_keeps_text() {
        let ctx = fixture_context();
        let text = "Alternative Pfade\nIm Datensatz wirkt Mystic Burst stabil.\nHohe Confidence, aber Verteidigung Der Pfad bleibt vorsichtig.";
        let result = validate_narration(
            text,
            &ctx,
            &known_items(&["Mystic Burst", "Improved Spirit Armor", "Weapon Shielding"]),
        );

        assert_eq!(result.text, text);
        assert!(result.violations.is_empty());
    }

    #[test]
    fn validate_narration_flags_real_item_outside_context_without_changing_text() {
        let ctx = fixture_context();
        let text = "Seven nutzt Mystic Burst und kauft danach Weapon Shielding.";
        let result = validate_narration(
            text,
            &ctx,
            &known_items(&["Mystic Burst", "Improved Spirit Armor", "Weapon Shielding"]),
        );

        assert_eq!(result.text, text);
        assert_eq!(
            result
                .violations
                .iter()
                .map(|violation| violation.term.as_str())
                .collect::<Vec<_>>(),
            vec!["Weapon Shielding"]
        );
    }

    #[test]
    fn validate_narration_allows_build_item_in_text() {
        let ctx = fixture_context();
        let text = "Improved Spirit Armor ist im Build erlaubt.";
        let result = validate_narration(
            text,
            &ctx,
            &known_items(&["Improved Spirit Armor", "Spirit Armor"]),
        );

        assert_eq!(result.text, text);
        assert!(result.violations.is_empty());
    }

    #[test]
    fn build_narration_prompt_contains_context_items() {
        let ctx = fixture_context();
        let prompt = build_narration_user_prompt(&ctx).expect("prompt");

        assert!(prompt.contains(BUILD_NARRATION_USER_PROMPT));
        assert!(prompt.contains("Seven"));
        assert!(prompt.contains("Mystic Burst"));
        assert!(prompt.contains("Improved Spirit Armor"));
    }
}
