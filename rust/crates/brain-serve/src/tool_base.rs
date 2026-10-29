use brain_contracts::{
    tools::GameContextResolver, AuthorizedContext, Evidence, PinnedGameContext, PortError, Query,
    RetrievalPort, ToolDefinition, ToolEvidenceDependency, ToolExecution, ToolExecutionPort,
    ToolRequest, ToolValidationPurpose,
};

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

impl brain_contracts::discord_task::DiscordContextResolver for ToolBase {
    fn resolve(
        &self,
        _: &Query,
        context: &AuthorizedContext,
        questions: &[String],
    ) -> Result<brain_contracts::discord_task::DiscordContextProjection, PortError> {
        context.check_deadline()?;
        let turns = questions
            .iter()
            .take(4)
            .map(|question| game_terms(question))
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

fn game_terms(question: &str) -> Vec<String> {
    let mut terms: Vec<String> = Vec::new();
    for word in question
        .split(|ch: char| !ch.is_alphabetic())
        .map(str::to_lowercase)
    {
        let found = GAME_TERMS
            .iter()
            .find(|(stem, _)| word.starts_with(stem) || (stem.len() >= 4 && word.ends_with(stem)))
            .map(|(_, term)| (*term).to_owned());
        if let Some(term) = found.filter(|term| !terms.contains(term)) {
            if terms.len() < 8 {
                terms.push(term);
            }
        }
    }
    terms
}

#[cfg(test)]
mod tests {
    use super::game_terms;

    #[test]
    fn prior_questions_project_only_public_game_terms() {
        assert_eq!(
            game_terms("Wie viele Seelen gibt ein Trooper in Minute 1, und wie viel davon im Orb?"),
            ["Seelen", "Trooper", "Seelenorb"]
        );
        assert!(game_terms("Mein Name ist Max und ich wohne in Berlin").is_empty());
    }
}
