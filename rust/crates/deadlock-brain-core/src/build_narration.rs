use std::collections::BTreeSet;

use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};

pub use dbrain_builds::{BuildContext, BuildPath, BuildPathSummary, BuildPhase, ItemDossier};

use crate::minimax::{
    extract_minimax_text, ChatCompletionRequest, ChatMessage, MiniMaxClient, MiniMaxConfig,
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
    let client = MiniMaxClient::from_env().context("load MiniMax client from environment")?;
    let request = build_narration_request(ctx, client.config())?;
    let response = client
        .chat(&request)
        .context("call MiniMax build narration")?;
    let text = extract_minimax_text(&response);
    if text.trim().is_empty() {
        return Err(anyhow!("empty MiniMax response"));
    }
    Ok(text)
}

pub fn build_narration_request(
    ctx: &BuildContext,
    config: &MiniMaxConfig,
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

pub fn validate_narration(text: &str, ctx: &BuildContext) -> ValidationResult {
    let allowed_items = collect_item_names(ctx);
    let ignored_terms = collect_ignored_terms(ctx);
    let violations = detect_item_like_violations(text, &allowed_items, &ignored_terms)
        .into_iter()
        .map(|term| ValidationViolation { term })
        .collect::<Vec<_>>();
    let cleaned = remove_violation_terms(text, &violations);
    ValidationResult {
        text: cleaned,
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

fn collect_ignored_terms(ctx: &BuildContext) -> BTreeSet<String> {
    let mut terms = BTreeSet::new();
    push_nonempty(&mut terms, &ctx.hero_name);
    push_nonempty(&mut terms, &ctx.hero_archetype);
    push_nonempty(&mut terms, &ctx.primary_path.label);
    if let Some(playstyle) = &ctx.playstyle {
        push_nonempty(&mut terms, playstyle);
    }
    for path in &ctx.alternative_paths {
        push_nonempty(&mut terms, &path.label);
    }
    for phase in &ctx.primary_path.phases {
        push_nonempty(&mut terms, &phase.phase);
        for item in &phase.items {
            push_nonempty(&mut terms, &item.buy_phase);
            for synergy in &item.synergy_with {
                push_nonempty(&mut terms, synergy);
            }
        }
    }
    terms
}

fn push_nonempty(terms: &mut BTreeSet<String>, value: &str) {
    let trimmed = value.trim();
    if !trimmed.is_empty() {
        terms.insert(trimmed.to_string());
    }
}

fn detect_item_like_violations(
    text: &str,
    allowed_items: &BTreeSet<String>,
    ignored_terms: &BTreeSet<String>,
) -> BTreeSet<String> {
    title_case_candidates(text)
        .into_iter()
        .filter(|candidate| candidate_is_violation(candidate, allowed_items, ignored_terms))
        .collect()
}

fn candidate_is_violation(
    candidate: &str,
    allowed_items: &BTreeSet<String>,
    ignored_terms: &BTreeSet<String>,
) -> bool {
    let candidate = candidate.trim();
    if candidate.is_empty()
        || allowed_items.contains(candidate)
        || ignored_terms.contains(candidate)
        || candidate_starts_with_stopword(candidate)
    {
        return false;
    }
    !allowed_items
        .iter()
        .chain(ignored_terms.iter())
        .any(|term| {
            phrase_contains_ascii(term, candidate) || phrase_contains_ascii(candidate, term)
        })
}

fn title_case_candidates(text: &str) -> BTreeSet<String> {
    let tokens = ascii_word_tokens(text);
    let mut candidates = BTreeSet::new();
    let mut index = 0;
    while index < tokens.len() {
        if !is_title_item_token(&tokens[index].text) {
            index += 1;
            continue;
        }
        let start = index;
        index += 1;
        while index < tokens.len()
            && is_title_item_token(&tokens[index].text)
            && tokens_are_whitespace_separated(text, &tokens[index - 1], &tokens[index])
        {
            index += 1;
        }
        collect_candidate_ngrams(&tokens[start..index], &mut candidates);
    }
    candidates
}

#[derive(Debug, Clone)]
struct WordToken {
    text: String,
    start: usize,
    end: usize,
}

fn collect_candidate_ngrams(tokens: &[WordToken], candidates: &mut BTreeSet<String>) {
    const MAX_ITEM_NAME_TOKENS: usize = 5;
    for start in 0..tokens.len() {
        for end in (start + 2)..=tokens.len().min(start + MAX_ITEM_NAME_TOKENS) {
            candidates.insert(
                tokens[start..end]
                    .iter()
                    .map(|token| token.text.as_str())
                    .collect::<Vec<_>>()
                    .join(" "),
            );
        }
    }
}

fn ascii_word_tokens(text: &str) -> Vec<WordToken> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut current_start = 0;
    for (index, character) in text.char_indices() {
        if character.is_ascii_alphanumeric() || matches!(character, '\'' | '-') {
            if current.is_empty() {
                current_start = index;
            }
            current.push(character);
        } else if !current.is_empty() {
            tokens.push(WordToken {
                text: std::mem::take(&mut current),
                start: current_start,
                end: index,
            });
        }
    }
    if !current.is_empty() {
        tokens.push(WordToken {
            text: current,
            start: current_start,
            end: text.len(),
        });
    }
    tokens
}

fn tokens_are_whitespace_separated(text: &str, left: &WordToken, right: &WordToken) -> bool {
    text[left.end..right.start].chars().all(char::is_whitespace)
}

fn is_title_item_token(token: &str) -> bool {
    let mut chars = token.chars();
    chars.next().is_some_and(|first| first.is_ascii_uppercase())
        && chars
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '\'' | '-'))
}

fn candidate_starts_with_stopword(candidate: &str) -> bool {
    let first = candidate.split_whitespace().next().unwrap_or_default();
    matches!(
        first,
        "A" | "An"
            | "Auf"
            | "Build"
            | "Buy"
            | "Core"
            | "Danach"
            | "Das"
            | "Der"
            | "Die"
            | "Early"
            | "Game"
            | "Gegen"
            | "Guide"
            | "Item"
            | "Items"
            | "Kaufe"
            | "Late"
            | "Mid"
            | "Mit"
            | "Nimm"
            | "Ohne"
            | "Path"
            | "Pick"
            | "Start"
            | "Then"
            | "The"
            | "Use"
    )
}

fn phrase_contains_ascii(haystack: &str, needle: &str) -> bool {
    if needle.trim().is_empty() {
        return false;
    }
    haystack
        .match_indices(needle)
        .any(|(start, _)| has_ascii_boundaries(haystack, start, start + needle.len()))
}

fn remove_violation_terms(text: &str, violations: &[ValidationViolation]) -> String {
    let mut cleaned = text.to_string();
    let mut terms = violations
        .iter()
        .map(|violation| violation.term.as_str())
        .collect::<Vec<_>>();
    terms.sort_by_key(|term| std::cmp::Reverse(term.len()));
    for term in terms {
        cleaned = remove_phrase(&cleaned, term);
    }
    cleanup_removed_text(&cleaned)
}

fn remove_phrase(text: &str, phrase: &str) -> String {
    let mut result = String::new();
    let mut cursor = 0;
    for (start, _) in text.match_indices(phrase) {
        let end = start + phrase.len();
        if !has_ascii_boundaries(text, start, end) {
            continue;
        }
        result.push_str(&text[cursor..start]);
        cursor = end;
    }
    result.push_str(&text[cursor..]);
    result
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

fn cleanup_removed_text(text: &str) -> String {
    text.lines()
        .map(|line| {
            line.split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
                .replace(" ,", ",")
                .replace(" .", ".")
                .replace(" ;", ";")
                .replace(" :", ":")
        })
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
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

    #[test]
    fn validate_narration_removes_items_outside_context() {
        let ctx = fixture_context();
        let result = validate_narration(
            "Seven nutzt Mystic Burst und kauft danach Phantom Strike. Improved Spirit Armor bleibt drin.",
            &ctx,
        );

        assert!(result.text.contains("Mystic Burst"));
        assert!(result.text.contains("Improved Spirit Armor"));
        assert!(!result.text.contains("Phantom Strike"));
        assert_eq!(
            result
                .violations
                .iter()
                .map(|violation| violation.term.as_str())
                .collect::<Vec<_>>(),
            vec!["Phantom Strike"]
        );
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
