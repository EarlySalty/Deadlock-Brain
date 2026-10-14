use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::Path;

pub const GOLD_SCHEMA: &str = "brain.e1-gold.v1";
const CATEGORIES: [&str; 7] = [
    "hero",
    "item",
    "timer",
    "patch",
    "docs",
    "guide",
    "unanswerable",
];
const SOURCE_KINDS: [&str; 3] = ["deadlock_api", "patch_post", "docs"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Expect {
    Answered,
    InsufficientEvidence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub kind: String,
    pub url: String,
    pub field: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fact {
    pub any_of: Vec<String>,
    pub source: Source,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoldCase {
    pub id: String,
    pub category: String,
    pub lang: String,
    pub question: String,
    pub profile: brain_client::AnswerProfile,
    pub expect: Expect,
    pub facts: Vec<Fact>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoldSources {
    pub deadlock_api: Vec<String>,
    pub patch_posts: Vec<String>,
    pub docs_base: String,
    pub fetched: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GoldFile {
    pub schema: String,
    pub version: u32,
    pub created: String,
    pub sources: GoldSources,
    pub cases: Vec<GoldCase>,
}

pub struct LoadedGold {
    pub gold: GoldFile,
    pub sha256: String,
}

pub fn load(path: &Path) -> Result<LoadedGold> {
    let bytes = std::fs::read(path).with_context(|| format!("read {}", path.display()))?;
    let gold: GoldFile = serde_json::from_slice(&bytes).context("parse gold file")?;
    validate(&gold)?;
    let sha256 = hex::encode(<sha2::Sha256 as sha2::Digest>::digest(&bytes));
    Ok(LoadedGold { gold, sha256 })
}

pub fn validate(gold: &GoldFile) -> Result<()> {
    if gold.schema != GOLD_SCHEMA {
        bail!("unexpected gold schema {}", gold.schema);
    }
    if gold.cases.is_empty() {
        bail!("gold file has no cases");
    }
    let mut ids = BTreeSet::new();
    for case in &gold.cases {
        if !ids.insert(case.id.as_str()) {
            bail!("duplicate case id {}", case.id);
        }
        validate_case(case).with_context(|| format!("case {}", case.id))?;
    }
    Ok(())
}

fn validate_case(case: &GoldCase) -> Result<()> {
    if !CATEGORIES.contains(&case.category.as_str()) {
        bail!("unknown category {}", case.category);
    }
    if !matches!(case.lang.as_str(), "de" | "en") {
        bail!("unknown language {}", case.lang);
    }
    if case.question.trim().is_empty() {
        bail!("empty question");
    }
    match case.expect {
        Expect::Answered if case.facts.is_empty() => bail!("answerable case without facts"),
        Expect::InsufficientEvidence if !case.facts.is_empty() => {
            bail!("non-answerable case with facts")
        }
        _ => {}
    }
    if case.category == "unanswerable" && case.expect != Expect::InsufficientEvidence {
        bail!("unanswerable category must expect insufficient_evidence");
    }
    case.facts.iter().try_for_each(validate_fact)
}

fn validate_fact(fact: &Fact) -> Result<()> {
    if fact.any_of.is_empty() || fact.any_of.iter().any(|t| t.trim().is_empty()) {
        bail!("fact without match terms");
    }
    if !SOURCE_KINDS.contains(&fact.source.kind.as_str()) {
        bail!("unknown source kind {}", fact.source.kind);
    }
    if !fact.source.url.starts_with("https://") || fact.source.value.trim().is_empty() {
        bail!("fact source needs an https url and a value");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn committed_gold_file_is_valid() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("gold/e1-gold-v1.json");
        let loaded = load(&path).expect("gold file loads");
        let unanswerable = loaded
            .gold
            .cases
            .iter()
            .filter(|c| c.expect == Expect::InsufficientEvidence && c.category == "unanswerable")
            .count();
        assert!(unanswerable >= 20);
        assert!(loaded.gold.cases.len() >= 120);
    }
}
