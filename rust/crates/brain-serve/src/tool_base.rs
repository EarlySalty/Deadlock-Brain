use brain_contracts::{
    tools::{GameContextResolver, ObjectiveKind},
    AuthorizedContext, Evidence, PinnedGameContext, PortError, Query, RetrievalPort,
    ToolDefinition, ToolEvidenceDependency, ToolExecution, ToolExecutionPort, ToolRequest,
    ToolValidationPurpose,
};
use std::collections::BTreeSet;

#[derive(Clone)]
pub(crate) struct ToolBase;

impl RetrievalPort for ToolBase {
    fn retrieve(&self, _: &Query, context: &AuthorizedContext) -> Result<Vec<Evidence>, PortError> {
        context.check_deadline()?;
        Ok(Vec::new())
    }

    fn validate_evidence(
        &self,
        _: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
        _: bool,
    ) -> Result<(), PortError> {
        context.check_deadline()?;
        if evidence.is_empty() {
            Ok(())
        } else {
            Err(PortError::PermissionDenied(
                "Nur Werkzeugbelege sind freigegeben.".into(),
            ))
        }
    }

    fn validate_publication(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> Result<(), PortError> {
        self.validate_evidence(query, context, evidence, false)
    }
}

impl GameContextResolver for ToolBase {
    fn resolve(
        &self,
        _: &Query,
        context: &AuthorizedContext,
    ) -> Result<Option<PinnedGameContext>, PortError> {
        context.check_deadline()?;
        Ok(None)
    }

    fn validate(
        &self,
        _: &Query,
        context: &AuthorizedContext,
        pin: Option<&PinnedGameContext>,
    ) -> Result<(), PortError> {
        context.check_deadline()?;
        if pin.is_none() {
            Ok(())
        } else {
            Err(PortError::PermissionDenied(
                "Gepinnter Spielkontext ist nicht freigegeben.".into(),
            ))
        }
    }
}

impl ToolExecutionPort for ToolBase {
    fn definitions(
        &self,
        _: &Query,
        context: &AuthorizedContext,
        _: Option<&PinnedGameContext>,
    ) -> Result<Vec<ToolDefinition>, PortError> {
        context.check_deadline()?;
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
        Err(PortError::PermissionDenied(
            "Werkzeug ist nicht freigegeben.".into(),
        ))
    }

    fn validate_dependencies(
        &self,
        _: &Query,
        context: &AuthorizedContext,
        _: Option<&PinnedGameContext>,
        dependencies: &[ToolEvidenceDependency],
        _: ToolValidationPurpose,
    ) -> Result<(), PortError> {
        context.check_deadline()?;
        if dependencies.is_empty() {
            Ok(())
        } else {
            Err(PortError::PermissionDenied(
                "Werkzeugbeleg ist nicht freigegeben.".into(),
            ))
        }
    }
}

pub(crate) trait EntityNames: Send + Sync {
    fn known(
        &self,
        phrases: &[String],
        context: &AuthorizedContext,
    ) -> Result<BTreeSet<String>, PortError>;
}

pub(crate) struct ConversationSubjects {
    names: Option<Box<dyn EntityNames>>,
}

impl ConversationSubjects {
    pub(crate) fn new(names: Option<Box<dyn EntityNames>>) -> Self {
        Self { names }
    }

    fn known(
        &self,
        questions: &[String],
        context: &AuthorizedContext,
    ) -> Result<BTreeSet<String>, PortError> {
        let phrases: BTreeSet<String> = questions
            .iter()
            .flat_map(|question| {
                let words = words(question);
                spans(words.len())
                    .map(|(start, end)| words[start..end].join(" ").to_lowercase())
                    .collect::<Vec<_>>()
            })
            .collect();
        match &self.names {
            Some(names) if !phrases.is_empty() => {
                names.known(&phrases.into_iter().collect::<Vec<_>>(), context)
            }
            _ => Ok(BTreeSet::new()),
        }
    }
}

impl brain_contracts::discord_task::DiscordContextResolver for ConversationSubjects {
    fn resolve(
        &self,
        _: &Query,
        context: &AuthorizedContext,
        questions: &[String],
    ) -> Result<brain_contracts::discord_task::DiscordContextProjection, PortError> {
        context.check_deadline()?;
        let questions = &questions[..questions.len().min(4)];
        let known = self.known(questions, context)?;
        let turns = questions
            .iter()
            .map(|question| subjects(question, &known))
            .collect();
        let projection = brain_contracts::discord_task::DiscordContextProjection { turns };
        Ok(if projection.valid() {
            projection
        } else {
            brain_contracts::discord_task::DiscordContextProjection::empty(questions.len())
        })
    }
}

const GAME_TERMS: [(&str, &str); 34] = [
    ("trooper", "Trooper"),
    ("minion", "Trooper"),
    ("seele", "Seelen"),
    ("soul", "Seelen"),
    ("orb", "Seelenorb"),
    ("kopfgeld", "Kopfgeld"),
    ("bounty", "Kopfgeld"),
    ("denial", "Deny"),
    ("deny", "Deny"),
    ("denie", "Deny"),
    ("urne", "Urne"),
    ("urn", "Urne"),
    ("midboss", "Midboss"),
    ("rejuvenator", "Rejuvenator"),
    ("patron", "Patron"),
    ("walker", "Walker"),
    ("guardian", "Guardian"),
    ("wächter", "Guardian"),
    ("schrein", "Schrein"),
    ("shrine", "Schrein"),
    ("neutral", "Neutrale"),
    ("creep", "Neutrale"),
    ("lane", "Lane"),
    ("zipline", "Zipline"),
    ("seilbahn", "Zipline"),
    ("respawn", "Respawn"),
    ("powerup", "Powerup"),
    ("sinner", "Sinner's Sacrifice"),
    ("vault", "Vault"),
    ("kiste", "Kisten"),
    ("crate", "Kisten"),
    ("statue", "Statuen"),
    ("comeback", "Comeback"),
    ("patch", "Patch"),
];

const MAX_PHRASE_WORDS: usize = 4;
const MAX_SUBJECTS: usize = 8;

fn words(question: &str) -> Vec<&str> {
    question
        .split(|ch: char| !(ch.is_alphabetic() || matches!(ch, '\'' | '-')))
        .map(|word| word.trim_matches(|ch| matches!(ch, '\'' | '-')))
        .filter(|word| !word.is_empty())
        .collect()
}

fn spans(count: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..count).flat_map(move |start| {
        (start + 1..=(start + MAX_PHRASE_WORDS).min(count)).map(move |end| (start, end))
    })
}

fn game_term(word: &str) -> Option<String> {
    let word = word.to_lowercase();
    GAME_TERMS
        .iter()
        .find(|(stem, _)| word.starts_with(stem) || (stem.len() >= 4 && word.ends_with(stem)))
        .map(|(_, term)| (*term).to_owned())
}

fn title_case(phrase: &str) -> String {
    phrase
        .split(' ')
        .map(|word| {
            let mut chars = word.chars();
            chars
                .next()
                .map(|first| first.to_uppercase().chain(chars).collect::<String>())
                .unwrap_or_default()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn objective_name(phrase: &str) -> Option<String> {
    ObjectiveKind::ALL
        .iter()
        .map(|objective| objective.as_str().replace('_', " "))
        .find(|name| phrase == name || phrase.strip_suffix('s') == Some(name.as_str()))
        .map(|name| title_case(&name))
}

fn phrase_subjects(words: &[&str], known: &BTreeSet<String>) -> Vec<(usize, String)> {
    spans(words.len())
        .filter_map(|(start, end)| {
            let phrase = words[start..end].join(" ");
            let lower = phrase.to_lowercase();
            if known.contains(&lower) {
                Some((start, phrase))
            } else {
                objective_name(&lower).map(|name| (start, name))
            }
        })
        .collect()
}

fn covered(term: &str, other: &str) -> bool {
    let term: Vec<String> = term.split(' ').map(str::to_lowercase).collect();
    let other: Vec<String> = other.split(' ').map(str::to_lowercase).collect();
    other.len() > term.len() && other.windows(term.len()).any(|window| window == term)
}

fn subjects(question: &str, known: &BTreeSet<String>) -> Vec<String> {
    let words = words(question);
    let mut found = phrase_subjects(&words, known);
    found.extend(
        words
            .iter()
            .enumerate()
            .filter_map(|(index, word)| game_term(word).map(|term| (index, term))),
    );
    found.sort_by_key(|(start, _)| *start);
    let mut terms: Vec<String> = Vec::new();
    for (_, term) in &found {
        let duplicate = terms
            .iter()
            .any(|kept| kept.to_lowercase() == term.to_lowercase());
        if !duplicate
            && terms.len() < MAX_SUBJECTS
            && !found.iter().any(|(_, other)| covered(term, other))
        {
            terms.push(term.clone());
        }
    }
    terms
}

#[cfg(test)]
mod tests {
    use super::subjects;
    use std::collections::BTreeSet;

    fn game_terms(question: &str) -> Vec<String> {
        subjects(question, &BTreeSet::new())
    }

    #[test]
    fn prior_questions_project_only_public_game_terms() {
        assert_eq!(
            game_terms("Wie viele Seelen gibt ein Trooper in Minute 1, und wie viel davon im Orb?"),
            ["Seelen", "Trooper", "Seelenorb"]
        );
        assert!(game_terms("Mein Name ist Max und ich wohne in Berlin").is_empty());
    }

    #[test]
    fn prior_questions_keep_full_entity_and_objective_names() {
        let known = BTreeSet::from(["echo shard".to_owned(), "shiv".to_owned()]);
        assert_eq!(
            subjects("Wie viel HP hat ein Base Guardian?", &known),
            ["Base Guardian"]
        );
        assert_eq!(subjects("Und Base Guardians?", &known), ["Base Guardian"]);
        assert_eq!(subjects("Was macht Echo Shard?", &known), ["Echo Shard"]);
        assert_eq!(
            subjects("Ist Echo Shard auf Shiv gut gegen Guardians?", &known),
            ["Echo Shard", "Shiv", "Guardian"]
        );
    }
}
