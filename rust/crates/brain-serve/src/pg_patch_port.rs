use crate::pg_patch_tools::LatestPatchCache;
use brain_contracts::{
    tools::{canonical_patch_history_definition, BuildPlanRequest, ToolSubrequest},
    Accounted, AuthorizedContext, PinnedGameContext, PortError, PortFailure, Query, ToolCall,
    ToolDefinition, ToolEvidenceDependency, ToolExecution, ToolExecutionPort, ToolName,
    ToolRequest, ToolValidationPurpose, UsageAccounting,
};
use sqlx::PgPool;
use tokio::runtime::{Handle, RuntimeFlavor};

pub struct PgPatchToolPort<T> {
    delegate: T,
    pool: PgPool,
    runtime: Handle,
    cache: LatestPatchCache,
}

impl<T> PgPatchToolPort<T> {
    pub fn new(delegate: T, pool: PgPool, runtime: Handle) -> Result<Self, PortError> {
        if runtime.runtime_flavor() != RuntimeFlavor::MultiThread {
            return Err(runtime_error());
        }
        Ok(Self {
            delegate,
            pool,
            runtime,
            cache: LatestPatchCache::default(),
        })
    }

    fn drive<R>(
        &self,
        operation: impl std::future::Future<Output = Result<R, PortError>>,
    ) -> Result<R, PortError> {
        if Handle::try_current()
            .is_ok_and(|handle| handle.runtime_flavor() == RuntimeFlavor::CurrentThread)
        {
            return Err(runtime_error());
        }
        tokio::task::block_in_place(|| self.runtime.block_on(operation))
    }
}

fn runtime_error() -> PortError {
    PortError::Unavailable(
        "Synchroner Patchleser benötigt die vorhandene Mehrthread-Runtime.".into(),
    )
}

fn canonical(request: &ToolRequest) -> bool {
    matches!(
        request.subrequest(),
        ToolSubrequest::CanonicalPatchHistory(_)
    )
}

impl<T: ToolExecutionPort> ToolExecutionPort for PgPatchToolPort<T> {
    fn required_calls(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        game_context: Option<&PinnedGameContext>,
    ) -> Result<Vec<ToolCall>, PortError> {
        self.delegate.required_calls(query, context, game_context)
    }

    fn definitions(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        game_context: Option<&PinnedGameContext>,
    ) -> Result<Vec<ToolDefinition>, PortError> {
        let mut definitions = self.delegate.definitions(query, context, game_context)?;
        context.check_deadline()?;
        if query.conversation_id == context.conversation_id
            && context.principal.provider_egress.contains("public")
        {
            if let Some(definition) = definitions
                .iter_mut()
                .find(|definition| definition.name == ToolName::PatchHistory)
            {
                *definition = canonical_patch_history_definition();
            } else {
                definitions.push(canonical_patch_history_definition());
            }
        }
        Ok(definitions)
    }

    fn execute(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        game_context: Option<&PinnedGameContext>,
        call_id: &str,
        request: &ToolRequest,
    ) -> Result<ToolExecution, PortError> {
        if canonical(request) {
            self.drive(crate::pg_patch_tools::execute_cached_patch_history(
                &self.pool,
                &self.cache,
                query,
                context,
                call_id,
                request,
            ))
        } else {
            self.delegate
                .execute(query, context, game_context, call_id, request)
        }
    }

    fn execute_accounted(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        game_context: Option<&PinnedGameContext>,
        call_id: &str,
        request: &ToolRequest,
    ) -> Result<Accounted<ToolExecution>, PortFailure> {
        if !canonical(request) {
            return self
                .delegate
                .execute_accounted(query, context, game_context, call_id, request);
        }
        self.execute(query, context, game_context, call_id, request)
            .map(|value| Accounted {
                accounting: UsageAccounting::observed(value.usage.clone()),
                value,
            })
            .map_err(PortFailure::from)
    }

    fn validate_build_plan(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        game_context: &PinnedGameContext,
        request: &BuildPlanRequest,
        execution: &ToolExecution,
        purpose: brain_contracts::store::AnswerPurpose,
    ) -> Result<(), PortError> {
        self.delegate
            .validate_build_plan(query, context, game_context, request, execution, purpose)
    }

    fn validate_dependencies(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        game_context: Option<&PinnedGameContext>,
        dependencies: &[ToolEvidenceDependency],
        purpose: ToolValidationPurpose,
    ) -> Result<(), PortError> {
        if !dependencies
            .iter()
            .any(|dependency| canonical(&dependency.request))
        {
            return self.delegate.validate_dependencies(
                query,
                context,
                game_context,
                dependencies,
                purpose,
            );
        }
        let mut delegated = Vec::new();
        for dependency in dependencies {
            if canonical(&dependency.request) {
                self.drive(crate::pg_patch_tools::validate_cached_patch_dependency(
                    &self.pool,
                    &self.cache,
                    query,
                    context,
                    dependency,
                ))?;
            } else {
                delegated.push(dependency.clone());
            }
        }
        if delegated.is_empty() {
            Ok(())
        } else {
            self.delegate
                .validate_dependencies(query, context, game_context, &delegated, purpose)
        }
    }
}
