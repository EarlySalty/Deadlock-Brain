use crate::docs_tools::{
    bounded_evidence, bounded_execution, scopes, validate_evidence_access, Section, SectionView,
    MAX_SECTIONS,
};
use brain_contracts::{
    canonical_receipt::{format_receipt, CanonicalReceipt},
    tools::SearchDocsRequest,
    AuthorizedContext, Evidence, EvidenceKind, PortError, Query, SourceVisibility, ToolExecution,
    ToolRequest, ToolValidationPurpose,
};
use sqlx::{postgres::PgRow, PgPool, Row};
use std::{borrow::Cow, collections::BTreeSet, future::Future, time::Duration};

mod ports {
    use super::{invalid, DocsReader};
    use crate::docs_tools::scopes;
    use brain_contracts::{
        tools::BuildPlanRequest, Accounted, AuthorizedContext, Evidence, PinnedGameContext,
        PortError, PortFailure, Query, RetrievalPort, ToolCall, ToolDefinition,
        ToolEvidenceDependency, ToolExecution, ToolExecutionPort, ToolRequest,
        ToolValidationPurpose, UsageAccounting,
    };
    use std::sync::Arc;

    pub struct DocsTools {
        delegate: Option<Arc<dyn ToolExecutionPort>>,
        reader: DocsReader,
        definition: ToolDefinition,
    }

    impl DocsTools {
        pub fn new(
            delegate: Option<Arc<dyn ToolExecutionPort>>,
            reader: DocsReader,
            definition: ToolDefinition,
        ) -> Result<Self, PortError> {
            definition.validate()?;
            if definition.name.as_str() != "search_docs"
                || definition.input_schema
                    != brain_contracts::tools::SearchDocsRequest::input_schema()
            {
                return Err(invalid());
            }
            Ok(Self {
                delegate,
                reader,
                definition,
            })
        }
    }

    impl ToolExecutionPort for DocsTools {
        fn required_calls(
            &self,
            query: &Query,
            context: &AuthorizedContext,
            pin: Option<&PinnedGameContext>,
        ) -> Result<Vec<ToolCall>, PortError> {
            self.delegate.as_ref().map_or_else(
                || Ok(Vec::new()),
                |delegate| delegate.required_calls(query, context, pin),
            )
        }

        fn definitions(
            &self,
            query: &Query,
            context: &AuthorizedContext,
            pin: Option<&PinnedGameContext>,
        ) -> Result<Vec<ToolDefinition>, PortError> {
            let mut definitions = self.delegate.as_ref().map_or_else(
                || Ok(Vec::new()),
                |delegate| delegate.definitions(query, context, pin),
            )?;
            if scopes(query, context) == Ok(brain_contracts::SourceVisibility::Public) {
                definitions.push(self.definition.clone());
            }
            Ok(definitions)
        }

        fn execute(
            &self,
            query: &Query,
            context: &AuthorizedContext,
            pin: Option<&PinnedGameContext>,
            id: &str,
            request: &ToolRequest,
        ) -> Result<ToolExecution, PortError> {
            if request.name() == self.definition.name {
                if scopes(query, context)? != brain_contracts::SourceVisibility::Public {
                    return Err(PortError::PermissionDenied(
                        "Öffentliche Dokumentensuche benötigt einen öffentlichen Zugriff.".into(),
                    ));
                }
                let call = ToolCall {
                    id: id.into(),
                    name: request.name(),
                    arguments: request.arguments().clone(),
                };
                if call.validate(std::slice::from_ref(&self.definition))? != *request {
                    return Err(invalid());
                }
                let execution = self.reader.search(query, context, id, request)?;
                execution.validate_for(&call, request, None)?;
                return Ok(execution);
            }
            self.delegate
                .as_ref()
                .ok_or_else(invalid)?
                .execute(query, context, pin, id, request)
        }

        fn execute_accounted(
            &self,
            query: &Query,
            context: &AuthorizedContext,
            pin: Option<&PinnedGameContext>,
            id: &str,
            request: &ToolRequest,
        ) -> Result<Accounted<ToolExecution>, PortFailure> {
            if request.name() != self.definition.name {
                return self
                    .delegate
                    .as_ref()
                    .ok_or_else(|| PortFailure::from(invalid()))?
                    .execute_accounted(query, context, pin, id, request);
            }
            self.execute(query, context, pin, id, request)
                .map(|value| Accounted {
                    value,
                    accounting: UsageAccounting::default(),
                })
                .map_err(|error| PortFailure::accounted(error, UsageAccounting::default()))
        }

        fn validate_build_plan(
            &self,
            query: &Query,
            context: &AuthorizedContext,
            pin: &PinnedGameContext,
            request: &BuildPlanRequest,
            execution: &ToolExecution,
            purpose: brain_contracts::store::AnswerPurpose,
        ) -> Result<(), PortError> {
            self.delegate
                .as_ref()
                .ok_or_else(invalid)?
                .validate_build_plan(query, context, pin, request, execution, purpose)
        }

        fn validate_dependencies(
            &self,
            query: &Query,
            context: &AuthorizedContext,
            pin: Option<&PinnedGameContext>,
            dependencies: &[ToolEvidenceDependency],
            purpose: ToolValidationPurpose,
        ) -> Result<(), PortError> {
            let mut delegated = Vec::new();
            for dependency in dependencies {
                if dependency.request.name() == self.definition.name {
                    self.reader
                        .validate_dependency(query, context, dependency, purpose)?;
                } else {
                    delegated.push(dependency.clone());
                }
            }
            if delegated.is_empty() && !dependencies.is_empty() {
                return Ok(());
            }
            if let Some(delegate) = &self.delegate {
                return delegate.validate_dependencies(query, context, pin, &delegated, purpose);
            }
            if delegated.is_empty() {
                Ok(())
            } else {
                Err(invalid())
            }
        }
    }

    pub struct InternalDocsReader {
        reader: DocsReader,
    }

    impl InternalDocsReader {
        pub fn new(reader: DocsReader) -> Self {
            Self { reader }
        }

        fn authorize(query: &Query, context: &AuthorizedContext) -> Result<(), PortError> {
            crate::internal_docs::authorize(query, context)
        }
    }

    impl RetrievalPort for InternalDocsReader {
        fn retrieve(
            &self,
            query: &Query,
            context: &AuthorizedContext,
        ) -> Result<Vec<Evidence>, PortError> {
            Self::authorize(query, context)?;
            self.reader.retrieve(query, context)
        }

        fn validate_evidence(
            &self,
            query: &Query,
            context: &AuthorizedContext,
            evidence: &[Evidence],
            for_provider: bool,
        ) -> Result<(), PortError> {
            let purpose = if for_provider {
                ToolValidationPurpose::Provider
            } else {
                ToolValidationPurpose::Cache
            };
            crate::internal_docs::validate_internal_evidence(query, context, evidence, purpose)?;
            self.reader
                .validate_evidence(query, context, evidence, purpose)
        }

        fn validate_publication(
            &self,
            query: &Query,
            context: &AuthorizedContext,
            _: &[Evidence],
        ) -> Result<(), PortError> {
            Self::authorize(query, context)?;
            Err(PortError::PermissionDenied(
                "Interne Dokumentenbelege dürfen nicht veröffentlicht werden.".into(),
            ))
        }
    }
}
pub use ports::{DocsTools, InternalDocsReader};

pub(crate) struct OperatorDocsRetrieval<R> {
    public: R,
    internal: Option<InternalDocsReader>,
}

impl<R> OperatorDocsRetrieval<R> {
    pub(crate) fn new(public: R, reader: Option<DocsReader>) -> Self {
        Self {
            public,
            internal: reader.map(InternalDocsReader::new),
        }
    }

    fn internal(&self) -> Result<&InternalDocsReader, PortError> {
        self.internal.as_ref().ok_or_else(|| {
            PortError::Unavailable("Interne Dokumentensuche ist nicht eingerichtet.".into())
        })
    }
}

impl<R: brain_contracts::RetrievalPort> brain_contracts::RetrievalPort
    for OperatorDocsRetrieval<R>
{
    fn retrieve(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Result<Vec<Evidence>, PortError> {
        if context.principal.scopes.contains("second_brain.internal") {
            crate::internal_docs::authorize(query, context)?;
            self.internal()?.retrieve(query, context)
        } else {
            self.public.retrieve(query, context)
        }
    }

    fn validate_evidence(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
        for_provider: bool,
    ) -> Result<(), PortError> {
        if context.principal.scopes.contains("second_brain.internal") {
            crate::internal_docs::authorize(query, context)?;
            self.internal()?
                .validate_evidence(query, context, evidence, for_provider)
        } else {
            self.public
                .validate_evidence(query, context, evidence, for_provider)
        }
    }

    fn validate_publication(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
    ) -> Result<(), PortError> {
        if context.principal.scopes.contains("second_brain.internal") {
            crate::internal_docs::authorize(query, context)?;
            self.internal()?
                .validate_publication(query, context, evidence)
        } else {
            self.public.validate_publication(query, context, evidence)
        }
    }
}

const SEARCH: &str = include_str!("pg_docs/search.sql");

#[derive(Clone)]
pub struct DocsReader {
    pool: PgPool,
    runtime: tokio::runtime::Handle,
}

impl DocsReader {
    pub fn new(pool: PgPool, runtime: tokio::runtime::Handle) -> Self {
        Self { pool, runtime }
    }

    pub fn search(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        call_id: &str,
        request: &ToolRequest,
    ) -> Result<ToolExecution, PortError> {
        if request.name().as_str() != "search_docs" {
            return Err(invalid());
        }
        let search = SearchDocsRequest::parse(request.arguments())?;
        let (sections, truncated) = self.select(
            query,
            context,
            &search.query,
            search.entity,
            search.limit.unwrap_or(MAX_SECTIONS),
        )?;
        bounded_execution(call_id, request, sections, truncated)
    }

    pub fn retrieve(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Result<Vec<Evidence>, PortError> {
        let search = crate::internal_docs::internal_search_text(query)?;
        let (sections, _) = self.select(query, context, &search, None, MAX_SECTIONS)?;
        bounded_evidence(sections)
    }

    pub fn validate_dependency(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        dependency: &brain_contracts::ToolEvidenceDependency,
        purpose: ToolValidationPurpose,
    ) -> Result<(), PortError> {
        validate_evidence_access(query, context, &dependency.evidence, purpose)?;
        if dependency.game_context.is_some() || dependency.evidence.is_empty() {
            return Err(invalid());
        }
        let fresh = self.search(
            query,
            context,
            "revalidation",
            &dependency.request.without_canonical_result(),
        )?;
        if fresh.dependencies.len() != 1 || fresh.dependencies[0] != *dependency {
            return Err(changed());
        }
        Ok(())
    }

    pub fn validate_evidence(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        evidence: &[Evidence],
        purpose: ToolValidationPurpose,
    ) -> Result<(), PortError> {
        validate_evidence_access(query, context, evidence, purpose)?;
        if self.retrieve(query, context)? != evidence {
            return Err(changed());
        }
        Ok(())
    }

    fn select(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        search: &str,
        entity: Option<u64>,
        limit: usize,
    ) -> Result<(Vec<Section>, bool), PortError> {
        let visibility = scopes(query, context)?;
        if !(1..=MAX_SECTIONS).contains(&limit) {
            return Err(invalid());
        }
        let allowed = match visibility {
            SourceVisibility::Public => "public",
            SourceVisibility::Internal => "internal",
            _ => return Err(invalid()),
        };
        let entity = entity
            .map(i64::try_from)
            .transpose()
            .map_err(|_| invalid())?;
        if search.is_empty() {
            return Ok((Vec::new(), false));
        }
        let search = compact_plain_search(search);
        let budget = context.remaining_time()?.min(Duration::from_millis(500));
        let rows = blocking(&self.runtime, async {
            tokio::time::timeout(budget, async {
                let mut tx = self.pool.begin().await.map_err(database_error)?;
                sqlx::raw_sql("SET TRANSACTION READ ONLY; SET LOCAL ROLE brain_tool; SET LOCAL statement_timeout='500ms'")
                    .execute(&mut *tx)
                    .await
                    .map_err(database_error)?;
                let role: String = sqlx::query_scalar("SELECT current_user::text")
                    .fetch_one(&mut *tx)
                    .await
                    .map_err(database_error)?;
                if role != "brain_tool" {
                    return Err(PortError::PermissionDenied(
                        "Dokumente benötigen einen begrenzten Datenbankzugriff.".into(),
                    ));
                }
                let rows = sqlx::query(SEARCH)
                    .bind(allowed)
                    .bind(search.as_ref())
                    .bind(entity)
                    .bind(limit as i64)
                    .fetch_all(&mut *tx)
                    .await
                    .map_err(database_error)?;
                tx.commit().await.map_err(database_error)?;
                Ok::<_, PortError>(rows)
            })
            .await
            .map_err(|_| PortError::Unavailable("Zeitgrenze für Dokumente erreicht.".into()))?
        })?;
        context.check_deadline()?;
        let truncated = rows
            .first()
            .map(|row| row.try_get::<i64, _>("matching_sections"))
            .transpose()
            .map_err(database_error)?
            .is_some_and(|count| count > limit as i64);
        let sections = rows
            .into_iter()
            .take(limit)
            .map(|row| section(row, context))
            .collect::<Result<Vec<_>, _>>()?;
        Ok((sections, truncated))
    }
}

fn compact_plain_search(search: &str) -> Cow<'_, str> {
    if search.split_ascii_whitespace().any(|term| {
        term.eq_ignore_ascii_case("or") || !term.bytes().all(|byte| byte.is_ascii_alphabetic())
    }) {
        return Cow::Borrowed(search);
    }
    let mut seen = BTreeSet::new();
    let mut terms = Vec::new();
    let mut repeated = false;
    for term in search.split_ascii_whitespace() {
        if seen.insert(term) {
            terms.push(term);
        } else {
            repeated = true;
        }
    }
    if repeated {
        Cow::Owned(terms.join(" "))
    } else {
        Cow::Borrowed(search)
    }
}

fn blocking<F, T>(runtime: &tokio::runtime::Handle, future: F) -> Result<T, PortError>
where
    F: Future<Output = Result<T, PortError>>,
{
    match tokio::runtime::Handle::try_current() {
        Ok(current) if current.runtime_flavor() == tokio::runtime::RuntimeFlavor::MultiThread => {
            tokio::task::block_in_place(|| runtime.block_on(future))
        }
        Ok(_) => Err(PortError::Unavailable(
            "Dokumente benötigen einen unabhängigen Ausführungsplatz.".into(),
        )),
        Err(_) => runtime.block_on(future),
    }
}

fn section(row: PgRow, context: &AuthorizedContext) -> Result<Section, PortError> {
    let receipt: i64 = row.try_get("receipt_id").map_err(database_error)?;
    let page: i64 = row.try_get("page_id").map_err(database_error)?;
    let raw: i64 = row.try_get("raw_object_id").map_err(database_error)?;
    let run: i64 = row.try_get("valid_from_run").map_err(database_error)?;
    let path: String = row.try_get("path").map_err(database_error)?;
    let revision: String = row.try_get("source_revision").map_err(database_error)?;
    let scope: String = row.try_get("scope").map_err(database_error)?;
    let start: i32 = row.try_get("line_start").map_err(database_error)?;
    let end: i32 = row.try_get("line_end").map_err(database_error)?;
    if [receipt, page, raw, run].into_iter().any(|id| id <= 0) || start < 1 || end < start {
        return Err(invalid());
    }
    let mut id = receipt.to_string();
    let (visibility, allowed_scope, url) = match scope.as_str() {
        "public" => (
            SourceVisibility::Public,
            if context.principal.scopes.contains("docs.public") { "docs.public" } else { "bot.public" },
            Some(format!("https://github.com/EarlySalty/Deadlock-Docs/blob/{revision}/{path}#L{start}-L{end}")),
        ),
        "internal" => (SourceVisibility::Internal, "second_brain.internal", None),
        _ => return Err(invalid()),
    };
    let mut evidence = Evidence {
        evidence_id: id.clone(),
        source_id: format!("docs.{scope}/{page}"),
        logical_id: format!("receipt/{receipt}"),
        revision: run as u64,
        kind: EvidenceKind::Prose,
        content: String::new(),
        citation: url.clone().unwrap_or_else(|| format!("receipt/{id}")),
        visibility,
        allowed_scopes: BTreeSet::from([allowed_scope.into()]),
        score: 1.0,
        provenance: None,
        patch: None,
    };
    if visibility == SourceVisibility::Public {
        evidence = format_receipt(&CanonicalReceipt {
            raw_receipt_id: receipt,
            raw_object_id: raw,
            ingest_run_id: row.try_get("ingest_run_id").map_err(database_error)?,
            source: "deadlock-docs".into(),
            source_revision: revision.clone(),
            path: path.clone(),
            locator_kind: row.try_get("locator_kind").map_err(database_error)?,
            locator: row.try_get("locator").map_err(database_error)?,
            value_sha256: row.try_get("value_sha256").map_err(database_error)?,
            extractor: row.try_get("extractor").map_err(database_error)?,
            patch: None,
        })?;
        id = evidence.evidence_id.clone();
    }
    Section::new(
        SectionView {
            page: path,
            title: row.try_get("title").map_err(database_error)?,
            heading_path: row.try_get("heading_path").map_err(database_error)?,
            excerpt: row.try_get("excerpt").map_err(database_error)?,
            excerpt_truncated: false,
            url,
            citation: id,
            patch: None,
            source_revision: revision,
        },
        evidence,
    )
}

fn invalid() -> PortError {
    PortError::InvalidResponse("Dokumentenbeleg ist ungültig.".into())
}

fn changed() -> PortError {
    PortError::Unavailable("Veröffentlichte Dokumente haben sich geändert.".into())
}

fn database_error(_: sqlx::Error) -> PortError {
    PortError::Unavailable("Dokumente können derzeit nicht sicher gelesen werden.".into())
}
