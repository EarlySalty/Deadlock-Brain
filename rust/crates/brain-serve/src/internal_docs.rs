use super::docs_tools::DocsReader;
use brain_contracts::{AuthorizedContext, Evidence, PortError, Query, RetrievalPort};
use std::collections::BTreeSet;

pub(super) struct InternalDocs<R> {
    fallback: R,
    reader: Option<DocsReader>,
}

impl<R> InternalDocs<R> {
    pub(super) fn new(fallback: R, reader: Option<DocsReader>) -> Self {
        Self { fallback, reader }
    }

    fn authorize(query: &Query, context: &AuthorizedContext) -> Result<(), PortError> {
        context.check_deadline()?;
        if query.conversation_id != context.conversation_id
            || context.principal.actor_id != "second-brain"
            || context.principal.channel != "internal"
            || context.principal.scopes != BTreeSet::from(["second_brain.internal".into()])
            || !context.principal.provider_egress.is_empty()
        {
            return Err(PortError::PermissionDenied(
                "Interne Dokumentenanfrage ist nicht freigegeben.".into(),
            ));
        }
        Ok(())
    }
}

impl<R: RetrievalPort> RetrievalPort for InternalDocs<R> {
    fn retrieve(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Result<Vec<Evidence>, PortError> {
        match &self.reader {
            Some(reader) => {
                Self::authorize(query, context)?;
                reader.retrieve(query, context)
            }
            None => self.fallback.retrieve(query, context),
        }
    }

    fn retrieve_with_usage(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Result<(Vec<Evidence>, brain_contracts::Usage), PortError> {
        if self.reader.is_none() {
            return self.fallback.retrieve_with_usage(query, context);
        }
        self.retrieve(query, context)
            .map(|evidence| (evidence, brain_contracts::Usage::default()))
    }

    fn validate_evidence(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
        for_provider: bool,
    ) -> Result<(), PortError> {
        let Some(reader) = &self.reader else {
            return self
                .fallback
                .validate_evidence(query, context, evidence, for_provider);
        };
        Self::authorize(query, context)?;
        if for_provider {
            return Err(PortError::PermissionDenied(
                "Interne Dokumente dürfen nicht an einen Modellanbieter gesendet werden.".into(),
            ));
        }
        if reader.retrieve(query, context)? != evidence {
            return Err(PortError::Unavailable(
                "Interne Dokumentenbelege haben sich geändert.".into(),
            ));
        }
        Ok(())
    }

    fn validate_publication(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> Result<(), PortError> {
        if self.reader.is_none() {
            return self.fallback.validate_publication(query, context, evidence);
        }
        Err(PortError::PermissionDenied(
            "Interne Dokumentenbelege dürfen nicht veröffentlicht werden.".into(),
        ))
    }
}
