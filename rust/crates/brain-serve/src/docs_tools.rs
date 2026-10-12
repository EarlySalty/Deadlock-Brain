use super::pg_tools::blocking;
use brain_contracts::{
    public_api::valid_public_docs_section_url, tools::BuildPlanRequest, Accounted,
    AuthorizedContext, Evidence, EvidenceKind, PinnedGameContext, PortError, PortFailure, Query,
    SourceVisibility, ToolCall, ToolDefinition, ToolEvidenceDependency, ToolExecution,
    ToolExecutionPort, ToolName, ToolRequest, ToolResult, ToolSubrequest, ToolValidationPurpose,
    Usage, UsageAccounting,
};
use serde_json::{json, Value};
use sqlx::{PgPool, Row};
use std::{collections::BTreeSet, time::Duration};

const MAX_BYTES: usize = 16_384;
const MAX_SECTIONS: usize = 5;
const MAX_EXCERPT_CHARS: usize = 1500;
const SEARCH: &str = "WITH authorized AS MATERIALIZED (
    SELECT section_id FROM docs.current_section WHERE scope=ANY($1::text[])
), search AS (SELECT websearch_to_tsquery('simple',$2) AS terms)
SELECT d.section_id,d.source_id,d.revision_id,d.receipt_id,
    left(d.path,2049) AS path,left(d.title,513) AS title,
    ARRAY(SELECT left(heading,257) FROM unnest(d.heading_path) heading LIMIT 17) AS heading_path,
    left(d.body,1501) AS excerpt,left(d.url,2049) AS url,
    left(d.source_revision,513) AS source_revision,d.scope
FROM authorized a JOIN docs.current_section d ON d.section_id=a.section_id
JOIN docs.section s ON s.id=d.section_id CROSS JOIN search
JOIN canon.receipt p ON p.id=d.receipt_id AND p.source_id=d.source_id AND p.revision_id=d.revision_id
JOIN canon.revision v ON v.id=p.revision_id AND v.source_revision=d.source_revision
WHERE s.tsv @@ search.terms AND ($3::bigint IS NULL OR s.entity_ids @> ARRAY[$3::bigint])
ORDER BY ts_rank_cd(s.tsv,search.terms) DESC,d.section_id LIMIT $4";

fn invalid() -> PortError {
    PortError::InvalidResponse("Dokumentenbeleg ist ungültig.".into())
}

fn database_error(_: sqlx::Error) -> PortError {
    PortError::Unavailable("Dokumente können derzeit nicht sicher gelesen werden.".into())
}

fn scopes(query: &Query, context: &AuthorizedContext) -> Result<Vec<&'static str>, PortError> {
    context.check_deadline()?;
    if query.conversation_id != context.conversation_id {
        return Err(PortError::PermissionDenied(
            "Dokumentenanfrage ist nicht freigegeben.".into(),
        ));
    }
    let principal = &context.principal;
    if principal.actor_id == "second-brain"
        && principal.channel == "internal"
        && principal.scopes == BTreeSet::from(["second_brain.internal".into()])
        && principal.provider_egress.is_empty()
    {
        return Ok(vec!["internal"]);
    }
    if (principal.scopes.contains("docs.public") || principal.scopes.contains("bot.public"))
        && principal.provider_egress.contains("public")
    {
        return Ok(vec!["public"]);
    }
    Err(PortError::PermissionDenied(
        "Dokumentenanfrage ist nicht freigegeben.".into(),
    ))
}

fn definition() -> ToolDefinition {
    ToolDefinition {
        name: ToolName::SearchDocs,
        description: "Sucht veröffentlichte Dokumentabschnitte zum Suchtext und optional zu einer kanonischen Entitäts-ID aus find_entity. Liefert höchstens fünf Abschnitte mit Auszug, Abschnittsadresse, Beleg-ID und beobachtetem Quellenstand. Der Dokumentzugriff folgt ausschließlich der Freigabe des Aufrufers. Fehlende oder gekürzte Inhalte nicht ergänzen; ein leerer Treffer liefert keinen Beleg. patch null bezeichnet keinen bestätigten offiziellen Patch.".into(),
        input_schema: json!({"type":"object","additionalProperties":false,"required":["query"],"properties":{
            "query":{"type":"string","minLength":1,"maxLength":512},
            "entity":{"type":"integer","minimum":1,"maximum":i64::MAX},
            "limit":{"type":"integer","minimum":1,"maximum":5}}}),
    }
}

pub(super) struct DocsTools<T> {
    delegate: T,
    reader: DocsReader,
}

impl<T> DocsTools<T> {
    pub(super) fn new(delegate: T, pool: PgPool, runtime: tokio::runtime::Handle) -> Self {
        Self {
            delegate,
            reader: DocsReader::new(pool, runtime),
        }
    }
}

#[derive(Clone)]
pub(super) struct DocsReader {
    pool: PgPool,
    runtime: tokio::runtime::Handle,
}

impl DocsReader {
    pub(super) fn new(pool: PgPool, runtime: tokio::runtime::Handle) -> Self {
        Self { pool, runtime }
    }

    pub(super) fn retrieve(
        &self,
        query: &Query,
        context: &AuthorizedContext,
    ) -> Result<Vec<Evidence>, PortError> {
        let search_text = internal_search_text(query)?;
        let (sections, _) = self.select_sections(query, context, &search_text, None, MAX_SECTIONS)?;
        bounded_evidence(sections)
    }

    fn search(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        id: &str,
        request: &ToolRequest,
    ) -> Result<ToolExecution, PortError> {
        scopes(query, context)?;
        let call = ToolCall {
            id: id.into(),
            name: request.name(),
            arguments: request.arguments().clone(),
        };
        if call.validate(&[definition()])? != *request {
            return Err(invalid());
        }
        let ToolSubrequest::SearchDocs(search) = request.subrequest() else {
            return Err(invalid());
        };
        let (sections, truncated) = self.select_sections(
            query,
            context,
            &search.query,
            search.entity,
            search.limit.unwrap_or(MAX_SECTIONS),
        )?;
        let execution = bounded_execution(id, request, sections, truncated)?;
        execution.validate_for(&call, request, None)?;
        Ok(execution)
    }

    fn select_sections(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        search_text: &str,
        entity: Option<u64>,
        limit: usize,
    ) -> Result<(Vec<Section>, bool), PortError> {
        let allowed = scopes(query, context)?;
        if !(1..=MAX_SECTIONS).contains(&limit) {
            return Err(invalid());
        }
        if search_text.is_empty() {
            return Ok((Vec::new(), false));
        }
        let budget = context.remaining_time()?.min(Duration::from_secs(2));
        let rows = blocking(&self.runtime, async {
            tokio::time::timeout(budget, async {
                sqlx::query(SEARCH)
                    .bind(&allowed)
                    .bind(search_text)
                    .bind(entity.map(|entity| entity as i64))
                    .bind((limit + 1) as i64)
                    .fetch_all(&self.pool)
                    .await
                    .map_err(database_error)
            })
            .await
        })
        .map_err(|_| PortError::Unavailable("Zeitgrenze für Dokumente erreicht.".into()))??;
        context.check_deadline()?;
        let truncated = rows.len() > limit;
        let sections = rows
            .into_iter()
            .take(limit)
            .map(|row| section(row, context))
            .collect::<Result<Vec<_>, _>>()?;
        Ok((sections, truncated))
    }
}

impl<T: ToolExecutionPort> ToolExecutionPort for DocsTools<T> {
    fn required_calls(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        pin: Option<&PinnedGameContext>,
    ) -> Result<Vec<ToolCall>, PortError> {
        self.delegate.required_calls(query, context, pin)
    }

    fn definitions(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        pin: Option<&PinnedGameContext>,
    ) -> Result<Vec<ToolDefinition>, PortError> {
        let mut definitions = self.delegate.definitions(query, context, pin)?;
        if scopes(query, context).is_ok() {
            definitions.push(definition());
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
        if request.name() == ToolName::SearchDocs {
            self.reader.search(query, context, id, request)
        } else {
            self.delegate.execute(query, context, pin, id, request)
        }
    }

    fn execute_accounted(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        pin: Option<&PinnedGameContext>,
        id: &str,
        request: &ToolRequest,
    ) -> Result<Accounted<ToolExecution>, PortFailure> {
        if request.name() != ToolName::SearchDocs {
            return self
                .delegate
                .execute_accounted(query, context, pin, id, request);
        }
        self.reader
            .search(query, context, id, request)
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
            if dependency.request.name() != ToolName::SearchDocs {
                delegated.push(dependency.clone());
                continue;
            }
            validate_access(query, context, dependency, purpose)?;
            let current =
                self.reader
                    .search(query, context, "revalidation", &dependency.request)?;
            if current.dependencies.len() != 1
                || current.dependencies[0].evidence != dependency.evidence
            {
                return Err(PortError::Unavailable(
                    "Veröffentlichte Dokumente haben sich geändert.".into(),
                ));
            }
        }
        self.delegate
            .validate_dependencies(query, context, pin, &delegated, purpose)
    }
}

fn validate_access(
    query: &Query,
    context: &AuthorizedContext,
    dependency: &ToolEvidenceDependency,
    purpose: ToolValidationPurpose,
) -> Result<(), PortError> {
    let allowed = scopes(query, context)?;
    if dependency.game_context.is_some() || dependency.evidence.is_empty() {
        return Err(invalid());
    }
    for evidence in &dependency.evidence {
        if !brain_policy::evidence_allowed(&context.principal, evidence)
            || !match evidence.visibility {
                SourceVisibility::Public => allowed.contains(&"public"),
                SourceVisibility::Internal => {
                    allowed.contains(&"internal") && purpose == ToolValidationPurpose::Cache
                }
                _ => false,
            }
            || (purpose == ToolValidationPurpose::Provider
                && !context.principal.provider_egress.contains("public"))
        {
            return Err(PortError::PermissionDenied(
                "Dokumentenbeleg ist für diesen Zweck nicht freigegeben.".into(),
            ));
        }
    }
    Ok(())
}

struct Section {
    value: Value,
    evidence: Evidence,
}

fn section(row: sqlx::postgres::PgRow, context: &AuthorizedContext) -> Result<Section, PortError> {
    let id: i64 = row.try_get("receipt_id").map_err(database_error)?;
    let section_id: i64 = row.try_get("section_id").map_err(database_error)?;
    let source: i64 = row.try_get("source_id").map_err(database_error)?;
    let revision: i64 = row.try_get("revision_id").map_err(database_error)?;
    let path: String = row.try_get("path").map_err(database_error)?;
    let title: String = row.try_get("title").map_err(database_error)?;
    let headings: Vec<String> = row.try_get("heading_path").map_err(database_error)?;
    let excerpt: String = row.try_get("excerpt").map_err(database_error)?;
    let url: String = row.try_get("url").map_err(database_error)?;
    let source_revision: String = row.try_get("source_revision").map_err(database_error)?;
    let scope: String = row.try_get("scope").map_err(database_error)?;
    if [id, section_id, source, revision].iter().any(|id| *id <= 0)
        || !bounded_text(&path, 2048)
        || !bounded_text(&title, 512)
        || source_revision.len() != 40
        || !source_revision
            .bytes()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
        || headings.len() > 16
        || headings.iter().any(|heading| !bounded_text(heading, 256))
    {
        return Err(invalid());
    }
    let (visibility, required_scope) = match scope.as_str() {
        "public"
            if valid_public_docs_section_url(&url)
                && url.starts_with(&format!(
                    "https://github.com/EarlySalty/Deadlock-Docs/blob/{source_revision}/{path}#L"
                )) =>
        {
            let required = if context.principal.scopes.contains("docs.public") {
                "docs.public"
            } else {
                "bot.public"
            };
            (SourceVisibility::Public, required)
        }
        "internal" if valid_internal_url(&url) => {
            (SourceVisibility::Internal, "second_brain.internal")
        }
        _ => return Err(invalid()),
    };
    let value = json!({"page":path,"title":title,"heading_path":headings,
        "excerpt":excerpt.chars().take(MAX_EXCERPT_CHARS).collect::<String>(),
        "excerpt_truncated":excerpt.chars().count()>MAX_EXCERPT_CHARS,
        "url":url,"citation":id.to_string(),"patch":null,"source_revision":source_revision});
    let evidence = Evidence {
        evidence_id: id.to_string(),
        source_id: format!("docs.{scope}/{source}"),
        logical_id: format!("receipt/{id}"),
        revision: revision as u64,
        kind: EvidenceKind::Prose,
        content: json!({"section_id":section_id,"source_id":source,"revision_id":revision,"section":value}).to_string(),
        citation: url,
        visibility,
        allowed_scopes: BTreeSet::from([required_scope.into()]),
        score: 1.0,
        provenance: None,
        patch: None,
    };
    if !brain_policy::evidence_allowed(&context.principal, &evidence) {
        return Err(PortError::PermissionDenied(
            "Dokumentenbeleg ist nicht freigegeben.".into(),
        ));
    }
    Ok(Section { value, evidence })
}

fn bounded_text(value: &str, maximum: usize) -> bool {
    !value.trim().is_empty() && value.len() <= maximum && !value.chars().any(char::is_control)
}

fn valid_internal_url(value: &str) -> bool {
    let Ok(url) = reqwest::Url::parse(value) else {
        return false;
    };
    bounded_text(value, 2048)
        && url.scheme() == "https"
        && url.host_str().is_some()
        && url.username().is_empty()
        && url.password().is_none()
        && url.query().is_none()
        && url.fragment().is_some_and(|fragment| !fragment.is_empty())
}

fn internal_search_text(query: &Query) -> Result<String, PortError> {
    query
        .validate()
        .map_err(|_| PortError::InvalidResponse("Ungültige Dokumentenanfrage.".into()))?;
    Ok(query
        .text
        .split(|character: char| character.is_whitespace() || character.is_control())
        .filter(|term| !term.is_empty())
        .collect::<Vec<_>>()
        .join(" "))
}

fn bounded_evidence(sections: Vec<Section>) -> Result<Vec<Evidence>, PortError> {
    let mut evidence = sections
        .into_iter()
        .map(|section| section.evidence)
        .collect::<Vec<_>>();
    let mut ids = BTreeSet::new();
    if evidence
        .iter()
        .any(|item| !ids.insert(item.evidence_id.clone()))
    {
        return Err(invalid());
    }
    while serde_json::to_vec(&evidence).map_err(|_| invalid())?.len() > MAX_BYTES {
        if evidence.pop().is_none() {
            return Err(invalid());
        }
    }
    Ok(evidence)
}

fn bounded_execution(
    id: &str,
    request: &ToolRequest,
    mut sections: Vec<Section>,
    mut truncated: bool,
) -> Result<ToolExecution, PortError> {
    loop {
        let empty = sections.is_empty();
        let mut ids = BTreeSet::new();
        if sections
            .iter()
            .any(|section| !ids.insert(section.evidence.evidence_id.clone()))
        {
            return Err(invalid());
        }
        let mut result = ToolResult {
            call_id: id.into(),
            name: ToolName::SearchDocs,
            result: json!({"sections":sections.iter().map(|section| &section.value).collect::<Vec<_>>(),"truncated":truncated}),
            evidence_ids: sections
                .iter()
                .map(|section| section.evidence.evidence_id.clone())
                .collect(),
            is_error: empty,
        };
        if empty {
            result.result["error"] = json!(if truncated {
                "result_too_large"
            } else {
                "no_matching_sections"
            });
        }
        let dependencies = if empty {
            Vec::new()
        } else {
            vec![ToolEvidenceDependency {
                request: request.clone(),
                game_context: None,
                evidence: sections
                    .iter()
                    .map(|section| section.evidence.clone())
                    .collect(),
            }]
        };
        let execution = ToolExecution {
            result,
            dependencies,
            usage: Usage::default(),
        };
        let mut worst = execution.clone();
        worst.result.call_id = "\"".repeat(512);
        if serde_json::to_vec(&worst).map_err(|_| invalid())?.len() <= MAX_BYTES {
            return Ok(execution);
        }
        if sections.pop().is_none() {
            return Err(invalid());
        }
        truncated = true;
    }
}

#[cfg(test)]
#[path = "docs_tools_tests.rs"]
mod tests;
