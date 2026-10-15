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
        Ok(brain_contracts::discord_task::DiscordContextProjection::empty(questions.len()))
    }
}
