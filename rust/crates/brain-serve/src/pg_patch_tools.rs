use brain_contracts::{
    tools::{BuildPlanRequest, CanonicalPatchHistoryRequest, GetHeroRequest, HeroSection},
    Accounted, AuthorizedContext, Evidence, EvidenceKind, PinnedGameContext, PortError,
    PortFailure, Query, SourceVisibility, ToolCall, ToolDefinition, ToolEvidenceDependency,
    ToolExecution, ToolExecutionPort, ToolName, ToolRequest, ToolResult, ToolSubrequest,
    ToolValidationPurpose, Usage, UsageAccounting,
};
use serde_json::{json, Value};
use sqlx::{Connection, PgConnection, PgPool, Row};
use std::{collections::BTreeSet, time::Duration};

const MAX_BYTES: usize = 16_384;
const MAX_EVIDENCE: usize = 100;
const CURRENT: &str = "SELECT to_jsonb(p) || jsonb_build_object('head_citation',o.receipt_id::text)
    FROM serve.current_patch p JOIN serve.published_patch_origin o ON o.patch_id=p.id LIMIT 1";
const RESOLVE: &str = "SELECT to_jsonb(p) || jsonb_build_object('head_citation',o.receipt_id::text)
    FROM canon.patch p JOIN serve.published_patch_origin o ON o.patch_id=p.id
    WHERE p.label=$1 OR CASE WHEN $1 ~ '^[0-9]{4}-[0-9]{2}-[0-9]{2}[T ][0-9]{2}:[0-9]{2}:[0-9]{2}([.][0-9]+)?(Z|[+-][0-9]{2}:[0-9]{2})$'
        AND pg_input_is_valid($1,'timestamp with time zone')
        THEN p.released_at=$1::timestamptz ELSE false END
    ORDER BY p.released_at DESC,p.id DESC LIMIT 2";
const LATEST_CHANGES: &str = "SELECT jsonb_build_object(
    'id',c.id,'entity_id',c.entity_id,'stat_key',c.stat_key,'change_key',c.change_key,
    'old',c.\"old\",'new',c.\"new\",'change_kind',c.change_kind,'line_text',c.line_text,
    'patch',p.label,'released_at',p.released_at,'citation',c.receipt_id::text,
    'head_citation',origin.receipt_id::text,'source_revision',v.source_revision) AS change
    FROM serve.published_patch_change c JOIN canon.patch p ON p.id=c.patch_id
    JOIN serve.published_patch_origin origin ON origin.patch_id=p.id
    JOIN canon.receipt r ON r.id=c.receipt_id JOIN canon.revision v ON v.id=r.revision_id
    WHERE c.entity_id=$1 AND c.patch_id=$2 ORDER BY c.id DESC LIMIT 20";
const RANGE_CHANGES: &str = "SELECT jsonb_build_object(
    'id',c.id,'entity_id',c.entity_id,'stat_key',c.stat_key,'change_key',c.change_key,
    'old',c.\"old\",'new',c.\"new\",'change_kind',c.change_kind,'line_text',c.line_text,
    'patch',p.label,'released_at',p.released_at,'citation',c.receipt_id::text,
    'head_citation',origin.receipt_id::text,'source_revision',v.source_revision) AS change
    FROM serve.published_patch_change c JOIN canon.patch p ON p.id=c.patch_id
    JOIN serve.published_patch_origin origin ON origin.patch_id=p.id
    JOIN canon.receipt r ON r.id=c.receipt_id JOIN canon.revision v ON v.id=r.revision_id
    WHERE c.entity_id=$1 AND ($2::text IS NULL OR p.released_at >= $2::timestamptz)
        AND ($3::text IS NULL OR p.released_at <= $3::timestamptz)
    ORDER BY p.released_at DESC,p.id DESC,c.id DESC LIMIT 20";
const RECEIPTS: &str = "SELECT r.id,r.raw_object_id,r.source_id,r.revision_id,
    r.locator_kind,r.locator,encode(r.value_sha256,'hex') AS value_sha256,
    r.extractor,r.extractor_version,v.source_revision
    FROM canon.receipt r JOIN canon.revision v ON v.id=r.revision_id
    JOIN canon.source_head h ON h.source_id=r.source_id AND h.published_revision_id>=r.revision_id
    WHERE r.id=ANY($1::bigint[]) ORDER BY r.id LIMIT 100";

const HISTORICAL_IDENTITY: &str = "SELECT jsonb_build_object(
    'entity_id',e.id,'kind',e.kind,'key',e.key,'status','unknown',
    'citation',r.id::text,'source_revision',v.source_revision,'patch',requested.label,
    'patch_mapping',jsonb_build_object('citation',opening.receipt_id::text,
        'official_citation',opening.patch_receipt_id::text,'patch',introduced.label,
        'head_citation',(SELECT o.receipt_id::text FROM serve.published_patch_origin o WHERE o.patch_id=introduced.id)),
    'snapshot',jsonb_build_object('citation',snapshot.receipt_id::text,
        'official_citation',snapshot.patch_receipt_id::text,'patch',requested.label,
        'head_citation',(SELECT o.receipt_id::text FROM serve.published_patch_origin o WHERE o.patch_id=requested.id),
        'source_revision',snapshot.source_revision))
    FROM canon.entity e JOIN canon.receipt r ON r.id=e.identity_receipt_id
    JOIN canon.revision v ON v.id=r.revision_id
    JOIN serve.published_source_revision_patch opening
        ON opening.source_id=r.source_id AND opening.source_revision=v.source_revision
    JOIN canon.patch introduced ON introduced.id=opening.patch_id
    JOIN canon.patch requested ON requested.id=$2
    JOIN LATERAL (SELECT s.* FROM serve.published_source_revision_patch s
        WHERE s.source_id=r.source_id AND s.patch_id=requested.id
        ORDER BY s.evidence_revision_id DESC LIMIT 1) snapshot ON true
    WHERE e.id=$1 AND r.revision_id<=snapshot.evidence_revision_id
        AND introduced.released_at<=requested.released_at LIMIT 1";
const HISTORICAL_FACTS: &str = "SELECT jsonb_build_object(
    'entity_id',f.entity_id,'stat_key',f.stat_key,'section',f.section,
    'value',coalesce(to_jsonb(f.value_num),to_jsonb(f.value_text),f.value_json),
    'unit',f.unit,'condition',f.condition,'citation',f.receipt_id::text,
    'patch',requested.label,'source_revision',f.source_revision,
    'patch_mapping',jsonb_build_object('citation',opening.receipt_id::text,
        'official_citation',opening.patch_receipt_id::text,'patch',introduced.label,
        'head_citation',(SELECT o.receipt_id::text FROM serve.published_patch_origin o WHERE o.patch_id=introduced.id)),
    'closing_mapping',CASE WHEN closing.receipt_id IS NULL THEN NULL ELSE jsonb_build_object(
        'citation',closing.receipt_id::text,'official_citation',closing.patch_receipt_id::text,
        'patch',ending.label,'head_citation',
        (SELECT o.receipt_id::text FROM serve.published_patch_origin o WHERE o.patch_id=ending.id)) END,
    'snapshot',jsonb_build_object('citation',f.snapshot_receipt_id::text,
        'official_citation',f.snapshot_official_receipt_id::text,'patch',requested.label,
        'head_citation',(SELECT o.receipt_id::text FROM serve.published_patch_origin o WHERE o.patch_id=requested.id)))
    FROM serve.fact_patch_snapshot f JOIN canon.patch requested ON requested.id=f.snapshot_patch_id
    JOIN canon.patch introduced ON introduced.id=f.proven_from_patch
    JOIN serve.published_source_revision_patch opening ON opening.source_id=f.source_id
        AND opening.source_revision=f.source_revision
    LEFT JOIN canon.revision ending_revision ON ending_revision.id=f.valid_to_revision_id
    LEFT JOIN serve.published_source_revision_patch closing ON closing.source_id=f.source_id
        AND closing.source_revision=ending_revision.source_revision
    LEFT JOIN canon.patch ending ON ending.id=closing.patch_id
    WHERE f.entity_id=$1 AND f.snapshot_patch_id=$2 AND f.section=ANY($3::text[])
        AND introduced.released_at<=requested.released_at
        AND (f.valid_to_revision_id IS NULL OR ending.released_at>requested.released_at)
        AND f.snapshot_revision_id=(SELECT max(s.evidence_revision_id)
            FROM serve.published_source_revision_patch s WHERE s.source_id=f.source_id AND s.patch_id=$2)
    ORDER BY f.display_order,f.stat_key,f.source_id,f.id LIMIT 20";
const HISTORICAL_LINKS: &str = "SELECT jsonb_build_object(
    'entity_id',relation.to_entity_id,'slot',relation.slot,'citation',relation.receipt_id::text,
    'patch',requested.label,'source_revision',v.source_revision,
    'patch_mapping',jsonb_build_object('citation',opening.receipt_id::text,
        'official_citation',opening.patch_receipt_id::text,'patch',introduced.label,
        'head_citation',(SELECT o.receipt_id::text FROM serve.published_patch_origin o WHERE o.patch_id=introduced.id)),
    'closing_mapping',CASE WHEN closing.receipt_id IS NULL THEN NULL ELSE jsonb_build_object(
        'citation',closing.receipt_id::text,'official_citation',closing.patch_receipt_id::text,
        'patch',ending.label,'head_citation',
        (SELECT o.receipt_id::text FROM serve.published_patch_origin o WHERE o.patch_id=ending.id)) END,
    'snapshot',jsonb_build_object('citation',snapshot.receipt_id::text,
        'official_citation',snapshot.patch_receipt_id::text,'patch',requested.label,
        'head_citation',(SELECT o.receipt_id::text FROM serve.published_patch_origin o WHERE o.patch_id=requested.id)))
    FROM canon.relation relation JOIN canon.revision v ON v.id=relation.valid_from_revision_id
    JOIN serve.published_source_revision_patch opening ON opening.source_id=relation.source_id
        AND opening.source_revision=v.source_revision
    JOIN canon.patch introduced ON introduced.id=opening.patch_id JOIN canon.patch requested ON requested.id=$2
    JOIN LATERAL (SELECT s.* FROM serve.published_source_revision_patch s
        WHERE s.source_id=relation.source_id AND s.patch_id=$2
        ORDER BY s.evidence_revision_id DESC LIMIT 1) snapshot ON true
    LEFT JOIN canon.revision ending_revision ON ending_revision.id=relation.valid_to_revision_id
    LEFT JOIN serve.published_source_revision_patch closing ON closing.source_id=relation.source_id
        AND closing.source_revision=ending_revision.source_revision
    LEFT JOIN canon.patch ending ON ending.id=closing.patch_id
    WHERE relation.from_entity_id=$1 AND relation.kind='has_ability'
        AND relation.valid_from_revision_id<=snapshot.evidence_revision_id
        AND introduced.released_at<=requested.released_at
        AND (relation.valid_to_revision_id IS NULL OR
            (closing.evidence_revision_id IS NOT NULL AND relation.valid_to_revision_id>snapshot.evidence_revision_id
                AND ending.released_at>requested.released_at))
    ORDER BY relation.slot,relation.source_id,relation.id LIMIT 20";

pub struct PgPatchTools<T> {
    delegate: T,
    pool: PgPool,
    runtime: tokio::runtime::Handle,
}

impl<T> PgPatchTools<T> {
    pub(super) fn from_traced(
        delegate: super::answer_trace::TracedTools<T>,
        pool: PgPool,
        runtime: tokio::runtime::Handle,
    ) -> Self {
        Self::new(delegate.0, pool, runtime)
    }

    pub fn new(delegate: T, pool: PgPool, runtime: tokio::runtime::Handle) -> Self {
        Self {
            delegate,
            pool,
            runtime,
        }
    }

    fn check(query: &Query, context: &AuthorizedContext) -> Result<(), PortError> {
        context.check_deadline()?;
        if query.conversation_id != context.conversation_id
            || !context.principal.provider_egress.contains("public")
        {
            return Err(PortError::PermissionDenied(
                "Spielabfrage ist nicht freigegeben.".into(),
            ));
        }
        Ok(())
    }

    fn canonical_execution(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        id: &str,
        request: &ToolRequest,
    ) -> Result<ToolExecution, PortError> {
        Self::check(query, context)?;
        let call = ToolCall {
            id: id.into(),
            name: request.name(),
            arguments: request.arguments().clone(),
        };
        let definitions = match request.subrequest() {
            ToolSubrequest::CanonicalPatchHistory(_) => vec![definition()],
            ToolSubrequest::GetHero(_) => vec![historical_definition()],
            _ => return Err(invalid()),
        };
        if call.validate(&definitions)? != *request {
            return Err(invalid());
        }
        let budget = context.remaining_time()?.min(Duration::from_millis(500));
        let execution = super::pg_tools::blocking(&self.runtime, async {
            tokio::time::timeout(budget, async {
                let mut connection = self.pool.acquire().await.map_err(database_error)?;
                let mut transaction = connection.begin().await.map_err(database_error)?;
                sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
                    .execute(&mut *transaction)
                    .await
                    .map_err(database_error)?;
                let execution = match request.subrequest() {
                    ToolSubrequest::CanonicalPatchHistory(history) => {
                        history_execution(&mut transaction, id, request, history).await?
                    }
                    ToolSubrequest::GetHero(hero) => {
                        let patch = hero
                            .patch
                            .as_deref()
                            .filter(|patch| *patch != "current")
                            .or_else(|| query.patch.as_deref().filter(|patch| *patch != "current"))
                            .ok_or_else(invalid)?;
                        match resolve_patch(&mut transaction, patch).await? {
                            PatchResolution::Found(patch) => {
                                historical_execution(&mut transaction, id, request, hero, patch)
                                    .await?
                            }
                            resolution => error_execution(
                                id,
                                request.name(),
                                resolution.error_code().ok_or_else(invalid)?,
                            ),
                        }
                    }
                    _ => return Err(invalid()),
                };
                transaction.commit().await.map_err(database_error)?;
                Ok::<_, PortError>(execution)
            })
            .await
        })
        .map_err(|_| PortError::Unavailable("Zeitgrenze für Patchwerte erreicht.".into()))??;
        context.check_deadline()?;
        execution.validate_for(&call, request, None)?;
        Ok(execution)
    }
}

impl<T: ToolExecutionPort> ToolExecutionPort for PgPatchTools<T> {
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
        Self::check(query, context)?;
        if definitions
            .iter()
            .any(|definition| definition.name == ToolName::PatchHistory)
        {
            return Err(PortError::InvalidResponse("Patchwerkzeug ist bereits registriert; die Eingabeverträge müssen vor der Integration geklärt werden.".into()));
        }
        for registered in &mut definitions {
            if registered.name == ToolName::GetHero {
                registered.description = historical_definition().description;
            }
        }
        definitions.push(definition());
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
        if handled(query, request) {
            self.canonical_execution(query, context, id, request)
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
        if !handled(query, request) {
            return self
                .delegate
                .execute_accounted(query, context, pin, id, request);
        }
        self.canonical_execution(query, context, id, request)
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
            if !handled(query, &dependency.request) {
                delegated.push(dependency.clone());
                continue;
            }
            if dependency.game_context.is_some() || dependency.evidence.is_empty() {
                return Err(invalid());
            }
            let current =
                self.canonical_execution(query, context, "revalidation", &dependency.request)?;
            if current.dependencies.len() != 1
                || current.dependencies[0].evidence != dependency.evidence
            {
                return Err(PortError::Unavailable(
                    "Veröffentlichte Patchbelege haben sich geändert.".into(),
                ));
            }
        }
        self.delegate
            .validate_dependencies(query, context, pin, &delegated, purpose)
    }
}

fn handled(query: &Query, request: &ToolRequest) -> bool {
    matches!(
        request.subrequest(),
        ToolSubrequest::CanonicalPatchHistory(_)
    ) || matches!(request.subrequest(), ToolSubrequest::GetHero(hero)
            if hero.patch.as_deref().is_some_and(|patch| patch != "current")
                || query.patch.as_deref().is_some_and(|patch| patch != "current"))
}

fn definition() -> ToolDefinition {
    ToolDefinition {
        name: ToolName::PatchHistory,
        description: "Liest belegte offizielle Patchänderungen für die positive kanonische entity_id aus find_entity, keine ältere Spiel-ID. Ohne Zeitraum gilt nur der chronologisch neueste veröffentlichte offizielle Patch, nicht der letzte Patch mit einer Änderung dieser Entität. since und until sind inklusive Patchlabels oder exakte Veröffentlichungszeitpunkte mit Zeitzone. Mehrdeutige Labels werden abgewiesen. Höchstens 20 vollständige Änderungszeilen; abgeschnittene oder fehlende Zeilen nicht ergänzen. Eine leere Änderungsliste beweist keinen unveränderten Spielwert. Jede citation verweist auf den Beleg im offiziellen Rohobjekt.".into(),
        input_schema: json!({"type":"object","additionalProperties":false,"required":["entity"],"properties":{
            "entity":{"type":"integer","minimum":1,"maximum":i64::MAX},
            "since":{"type":"string","minLength":1,"maxLength":128},
            "until":{"type":"string","minLength":1,"maxLength":128},
            "limit":{"type":"integer","minimum":1,"maximum":20}}}),
    }
}

fn historical_definition() -> ToolDefinition {
    ToolDefinition {
        name: ToolName::GetHero,
        description: "Liest den Heldensteckbrief zur positiven kanonischen entity_id aus find_entity, keine ältere Spiel-ID. Liefert Identität und die gewünschten Abschnitte stats, abilities und weapon, standardmäßig alle drei. Fehlender Patch oder current bedeutet aktuell beobachteter Quellenstand, nicht offizieller Patch. Historische Patches liefern nur veröffentlichte, ausdrücklich zugeordnete Quellensnapshots und belegte Gültigkeitsintervalle. Fehlende Zuordnungen oder unbekannte Schließgrenzen liefern keine Spielwerte. Historische Identitäten und Fähigkeitsbeziehungen benötigen eigene rechtzeitige Belege. Patchzuordnungen haben separate receipt citations; die Spielwertbelege selbst bleiben patchlos. Werte, Einheiten, Bedingungen und Quellenstand unverändert beachten. Fehlende oder abgeschnittene Zeilen nicht ergänzen. unknown bestätigt keine Veröffentlichung des Helden im Spiel.".into(),
        input_schema: json!({"type":"object","additionalProperties":false,"required":["hero"],"properties":{
            "hero":{"type":"integer","minimum":1,"maximum":i64::MAX},
            "patch":{"type":"string","minLength":1,"maxLength":128},
            "sections":{"type":"array","minItems":1,"maxItems":3,"items":{"type":"string","enum":["stats","abilities","weapon"]}}}}),
    }
}

fn invalid() -> PortError {
    PortError::InvalidResponse("Kanonischer Patchbeleg ist ungültig.".into())
}

fn database_error(_: sqlx::Error) -> PortError {
    PortError::Unavailable("Patchwerte können derzeit nicht sicher gelesen werden.".into())
}

enum PatchResolution {
    Found(Value),
    Missing,
    Ambiguous,
}

impl PatchResolution {
    fn error_code(&self) -> Option<&'static str> {
        match self {
            Self::Found(_) => None,
            Self::Missing => Some("patch_not_found"),
            Self::Ambiguous => Some("patch_ambiguous"),
        }
    }
}

fn select_patch(mut rows: Vec<Value>) -> PatchResolution {
    match rows.len() {
        0 => PatchResolution::Missing,
        1 => PatchResolution::Found(rows.remove(0)),
        _ => PatchResolution::Ambiguous,
    }
}

async fn resolve_patch(
    connection: &mut PgConnection,
    label: &str,
) -> Result<PatchResolution, PortError> {
    let rows = sqlx::query_scalar::<_, Value>(RESOLVE)
        .bind(label)
        .fetch_all(connection)
        .await
        .map_err(database_error)?;
    Ok(select_patch(rows))
}

async fn history_execution(
    connection: &mut PgConnection,
    id: &str,
    request: &ToolRequest,
    history: &CanonicalPatchHistoryRequest,
) -> Result<ToolExecution, PortError> {
    let Some(current) = sqlx::query_scalar::<_, Value>(CURRENT)
        .fetch_optional(&mut *connection)
        .await
        .map_err(database_error)?
    else {
        return Ok(error_execution(
            id,
            request.name(),
            "official_patch_unavailable",
        ));
    };
    let mut bounds = Vec::new();
    for label in [&history.since, &history.until] {
        match label {
            None => bounds.push(None),
            Some(label) => match resolve_patch(connection, label).await? {
                PatchResolution::Found(patch) => bounds.push(Some(patch)),
                resolution => {
                    return Ok(error_execution(
                        id,
                        request.name(),
                        resolution.error_code().ok_or_else(invalid)?,
                    ))
                }
            },
        }
    }
    let timestamps = bounds
        .iter()
        .map(|patch| {
            patch
                .as_ref()
                .and_then(|patch| patch["released_at"].as_str())
        })
        .collect::<Vec<_>>();
    let ranged = history.since.is_some() || history.until.is_some();
    if ranged {
        let reversed: bool = sqlx::query_scalar("SELECT $1::text IS NOT NULL AND $2::text IS NOT NULL AND $1::timestamptz > $2::timestamptz")
            .bind(timestamps[0]).bind(timestamps[1]).fetch_one(&mut *connection).await.map_err(database_error)?;
        if reversed {
            return Ok(error_execution(id, request.name(), "patch_range_reversed"));
        }
    }
    let rows = if ranged {
        sqlx::query_scalar::<_, Value>(RANGE_CHANGES)
            .bind(history.entity as i64)
            .bind(timestamps[0])
            .bind(timestamps[1])
            .fetch_all(&mut *connection)
            .await
            .map_err(database_error)?
    } else {
        let patch_id = current["id"]
            .as_i64()
            .filter(|id| *id > 0)
            .ok_or_else(invalid)?;
        sqlx::query_scalar::<_, Value>(LATEST_CHANGES)
            .bind(history.entity as i64)
            .bind(patch_id)
            .fetch_all(&mut *connection)
            .await
            .map_err(database_error)?
    };
    let cap_reached = rows.len() == 20;
    let limit = history.limit.unwrap_or(20);
    let omitted = rows.len().saturating_sub(limit);
    let mut value = json!({"entity_id":history.entity,"current_official_patch":current,"range":{"since":bounds[0],"until":bounds[1]},
        "changes":rows.into_iter().take(limit).collect::<Vec<_>>(),"truncated":omitted>0 || cap_reached,
        "omitted_count":omitted,"omitted_count_is_lower_bound":cap_reached,"completeness":"unknown"});
    if value["changes"].as_array().is_none_or(Vec::is_empty) {
        value["error"] = json!("no_patch_changes");
        return Ok(empty_execution(id, request.name(), value));
    }
    let result = bound_result(id, request.name(), value)?;
    execution_with_receipts(connection, request, result).await
}

async fn historical_identity(
    connection: &mut PgConnection,
    entity: u64,
    patch: i64,
) -> Result<Option<Value>, PortError> {
    sqlx::query_scalar(HISTORICAL_IDENTITY)
        .bind(entity as i64)
        .bind(patch)
        .fetch_optional(connection)
        .await
        .map_err(database_error)
}

async fn historical_facts(
    connection: &mut PgConnection,
    entity: u64,
    patch: i64,
    sections: &[&str],
) -> Result<Vec<Value>, PortError> {
    sqlx::query_scalar(HISTORICAL_FACTS)
        .bind(entity as i64)
        .bind(patch)
        .bind(sections)
        .fetch_all(connection)
        .await
        .map_err(database_error)
}

fn fact_section(rows: &[Value], section: &str, cap_reached: bool) -> Value {
    let facts = rows
        .iter()
        .filter(|row| row["section"] == section)
        .cloned()
        .collect::<Vec<_>>();
    json!({"facts":facts,"completeness":"unknown","truncated":cap_reached,
        "omitted_count":0,"omitted_count_is_lower_bound":cap_reached})
}

async fn historical_execution(
    connection: &mut PgConnection,
    id: &str,
    request: &ToolRequest,
    hero: &GetHeroRequest,
    patch: Value,
) -> Result<ToolExecution, PortError> {
    let patch_id = patch["id"]
        .as_i64()
        .filter(|id| *id > 0)
        .ok_or_else(invalid)?;
    let Some(mut identity) = historical_identity(connection, hero.hero, patch_id)
        .await?
        .filter(|identity| identity["kind"] == "hero")
    else {
        return Ok(error_execution(
            id,
            request.name(),
            "historical_patch_unavailable",
        ));
    };
    let sections = hero.sections.as_deref().unwrap_or(&[
        HeroSection::Stats,
        HeroSection::Abilities,
        HeroSection::Weapon,
    ]);
    let mut selected = vec!["identity"];
    if sections.contains(&HeroSection::Stats) {
        selected.push("stats");
    }
    if sections.contains(&HeroSection::Weapon) {
        selected.push("weapon");
    }
    let facts = historical_facts(connection, hero.hero, patch_id, &selected).await?;
    let cap_reached = facts.len() == 20;
    identity["facts"] = fact_section(&facts, "identity", cap_reached)["facts"].clone();
    identity["completeness"] = json!("unknown");
    let mut value = json!({"identity":identity,"requested_patch":patch,"completeness":"unknown"});
    if sections.contains(&HeroSection::Stats) {
        value["stats"] = fact_section(&facts, "stats", cap_reached);
    }
    if sections.contains(&HeroSection::Weapon) {
        value["weapon"] = fact_section(&facts, "weapon", cap_reached);
    }
    if sections.contains(&HeroSection::Abilities) {
        let rows = sqlx::query_scalar::<_, Value>(HISTORICAL_LINKS)
            .bind(hero.hero as i64)
            .bind(patch_id)
            .fetch_all(&mut *connection)
            .await
            .map_err(database_error)?;
        let cap_reached = rows.len() == 20;
        let omitted = rows.len().saturating_sub(4);
        let mut links = Vec::new();
        let mut unavailable = 0;
        for mut link in rows.into_iter().take(4) {
            let entity = link["entity_id"]
                .as_u64()
                .filter(|id| *id > 0)
                .ok_or_else(invalid)?;
            let Some(identity) = historical_identity(connection, entity, patch_id)
                .await?
                .filter(|identity| identity["kind"] == "ability")
            else {
                unavailable += 1;
                continue;
            };
            let ability_facts =
                historical_facts(connection, entity, patch_id, &["ability", "identity"]).await?;
            link["identity"] = identity;
            link["truncated"] = json!(ability_facts.len() == 20);
            link["facts"] = json!(ability_facts);
            link["completeness"] = json!("unknown");
            links.push(link);
        }
        value["abilities"] = json!({"links":links,"completeness":"unknown",
            "truncated":omitted>0 || cap_reached,"omitted_count":omitted,
            "omitted_count_is_lower_bound":cap_reached,"unavailable_count":unavailable});
    }
    if facts.is_empty()
        && value["abilities"]["links"]
            .as_array()
            .is_none_or(Vec::is_empty)
    {
        return Ok(error_execution(
            id,
            request.name(),
            "historical_patch_unavailable",
        ));
    }
    let result = bound_profile_result(id, request.name(), value)?;
    execution_with_receipts(connection, request, result).await
}

fn profile_citations(value: &Value, ids: &mut BTreeSet<String>) -> Result<(), PortError> {
    match value {
        Value::Object(object) => {
            for (key, value) in object {
                if matches!(
                    key.as_str(),
                    "citation" | "official_citation" | "head_citation"
                ) {
                    let id = value
                        .as_str()
                        .filter(|id| id.parse::<i64>().is_ok_and(|id| id > 0))
                        .ok_or_else(invalid)?;
                    ids.insert(id.to_owned());
                } else if matches!(
                    key.as_str(),
                    "identity"
                        | "stats"
                        | "weapon"
                        | "abilities"
                        | "facts"
                        | "links"
                        | "patch_mapping"
                        | "closing_mapping"
                        | "snapshot"
                        | "requested_patch"
                        | "current_official_patch"
                        | "range"
                        | "since"
                        | "until"
                        | "changes"
                ) {
                    profile_citations(value, ids)?;
                }
            }
        }
        Value::Array(rows) => {
            for row in rows {
                profile_citations(row, ids)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn trim_profile_row(value: &mut Value) -> bool {
    let Some(object) = value.as_object_mut() else {
        return false;
    };
    for key in ["weapon", "abilities", "stats", "identity"] {
        if object.get_mut(key).is_some_and(trim_profile_row) {
            return true;
        }
    }
    for key in ["facts", "links"] {
        let Some(rows) = object.get_mut(key).and_then(Value::as_array_mut) else {
            continue;
        };
        if rows.pop().is_some() {
            let omitted = object
                .get("omitted_count")
                .and_then(Value::as_u64)
                .unwrap_or(0);
            object.insert("omitted_count".into(), json!(omitted.saturating_add(1)));
            object.insert("truncated".into(), json!(true));
            return true;
        }
    }
    false
}

fn bound_profile_result(
    id: &str,
    name: ToolName,
    mut value: Value,
) -> Result<ToolResult, PortError> {
    loop {
        let mut ids = BTreeSet::new();
        profile_citations(&value, &mut ids)?;
        let result = ToolResult {
            call_id: id.into(),
            name,
            result: value.clone(),
            evidence_ids: ids.iter().cloned().collect(),
            is_error: false,
        };
        let mut worst = result.clone();
        worst.call_id = "\"".repeat(512);
        if ids.len() <= MAX_EVIDENCE
            && !ids.is_empty()
            && serde_json::to_vec(&worst).map_err(|_| invalid())?.len() <= MAX_BYTES
        {
            return Ok(result);
        }
        if !trim_profile_row(&mut value) {
            return Ok(error_execution(id, name, "result_too_large").result);
        }
    }
}

async fn execution_with_receipts(
    connection: &mut PgConnection,
    request: &ToolRequest,
    result: ToolResult,
) -> Result<ToolExecution, PortError> {
    if result.is_error {
        return Ok(ToolExecution {
            result,
            dependencies: Vec::new(),
            usage: Usage::default(),
        });
    }
    let ids = result
        .evidence_ids
        .iter()
        .map(|id| id.parse::<i64>().map_err(|_| invalid()))
        .collect::<Result<Vec<_>, _>>()?;
    let rows = sqlx::query(RECEIPTS)
        .bind(&ids)
        .fetch_all(connection)
        .await
        .map_err(database_error)?;
    if rows.len() != ids.len() || ids.is_empty() {
        return Err(invalid());
    }
    let evidence = rows.into_iter().map(|receipt| {
        let id = receipt.try_get::<i64,_>("id").map_err(database_error)?;
        receipt_evidence(json!({"citation":id.to_string(),
            "raw_object_id":receipt.try_get::<i64,_>("raw_object_id").map_err(database_error)?,
            "source_id":receipt.try_get::<i64,_>("source_id").map_err(database_error)?,
            "revision_id":receipt.try_get::<i64,_>("revision_id").map_err(database_error)?,
            "source_revision":receipt.try_get::<String,_>("source_revision").map_err(database_error)?,
            "locator_kind":receipt.try_get::<String,_>("locator_kind").map_err(database_error)?,
            "locator":receipt.try_get::<String,_>("locator").map_err(database_error)?,
            "value_sha256":receipt.try_get::<String,_>("value_sha256").map_err(database_error)?,
            "extractor":receipt.try_get::<String,_>("extractor").map_err(database_error)?,
            "extractor_version":receipt.try_get::<String,_>("extractor_version").map_err(database_error)?,
            "patch":null}))
    }).collect::<Result<Vec<_>,_>>()?;
    Ok(ToolExecution {
        result,
        dependencies: vec![ToolEvidenceDependency {
            request: request.clone(),
            game_context: None,
            evidence,
        }],
        usage: Usage::default(),
    })
}

fn receipt_evidence(content: Value) -> Result<Evidence, PortError> {
    let id = content["citation"]
        .as_str()
        .and_then(|id| id.parse::<i64>().ok())
        .filter(|id| *id > 0)
        .ok_or_else(invalid)?;
    let source = content["source_id"]
        .as_i64()
        .filter(|id| *id > 0)
        .ok_or_else(invalid)?;
    let revision = content["revision_id"]
        .as_u64()
        .filter(|id| *id > 0)
        .ok_or_else(invalid)?;
    if content["patch"] != Value::Null {
        return Err(invalid());
    }
    let evidence = Evidence {
        evidence_id: id.to_string(),
        source_id: format!("canon.source/{source}"),
        logical_id: format!("receipt/{id}"),
        revision,
        kind: EvidenceKind::Fact,
        content: content.to_string(),
        citation: id.to_string(),
        visibility: SourceVisibility::Public,
        allowed_scopes: BTreeSet::new(),
        score: 1.0,
        provenance: None,
        patch: None,
    };
    evidence.validate().map_err(|_| invalid())?;
    Ok(evidence)
}

fn bound_result(id: &str, name: ToolName, mut value: Value) -> Result<ToolResult, PortError> {
    loop {
        let rows = value["changes"].as_array().ok_or_else(invalid)?;
        let mut ids = rows
            .iter()
            .map(|row| {
                row["citation"]
                    .as_str()
                    .filter(|id| id.parse::<i64>().is_ok_and(|id| id > 0))
                    .map(ToOwned::to_owned)
                    .ok_or_else(invalid)
            })
            .collect::<Result<BTreeSet<_>, _>>()?;
        profile_citations(&value, &mut ids)?;
        let result = ToolResult {
            call_id: id.into(),
            name,
            result: value.clone(),
            evidence_ids: ids.into_iter().collect(),
            is_error: false,
        };
        let mut worst = result.clone();
        worst.call_id = "\"".repeat(512);
        if result.evidence_ids.len() <= MAX_EVIDENCE
            && serde_json::to_vec(&worst).map_err(|_| invalid())?.len() <= MAX_BYTES
            && !rows.is_empty()
        {
            return Ok(result);
        }
        let rows = value["changes"].as_array_mut().ok_or_else(invalid)?;
        if rows.pop().is_none() {
            return Ok(error_execution(id, name, "result_too_large").result);
        }
        let omitted = value["omitted_count"].as_u64().unwrap_or(0);
        value["omitted_count"] = json!(omitted.saturating_add(1));
        value["truncated"] = json!(true);
    }
}

fn empty_execution(id: &str, name: ToolName, result: Value) -> ToolExecution {
    ToolExecution {
        result: ToolResult {
            call_id: id.into(),
            name,
            result,
            evidence_ids: Vec::new(),
            is_error: true,
        },
        dependencies: Vec::new(),
        usage: Usage::default(),
    }
}

fn error_execution(id: &str, name: ToolName, code: &str) -> ToolExecution {
    empty_execution(
        id,
        name,
        json!({"error":code,"message":"Für diese Anfrage liegen keine sicher belegten Patchwerte vor."}),
    )
}

#[cfg(test)]
#[path = "pg_patch_tools_tests.rs"]
mod tests;
