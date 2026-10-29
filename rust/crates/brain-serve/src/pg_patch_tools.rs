use brain_contracts::{
    canonical_receipt::{format_receipt, CanonicalReceipt},
    tools::{canonical_patch_history_definition, CanonicalPatchHistoryRequest},
    AuthorizedContext, Evidence, PortError, Query, ToolCall, ToolEvidenceDependency, ToolExecution,
    ToolName, ToolRequest, ToolResult, ToolSubrequest, Usage,
};
use serde_json::{json, Value};
use sqlx::{Connection, PgConnection, PgPool};
use std::{
    collections::{BTreeMap, BTreeSet},
    time::Duration,
};

const MAX_BYTES: usize = 16_384;
const MAX_EVIDENCE: usize = 100;
const CURRENT: &str = "SELECT jsonb_build_object('id',patch_id,'label',label,
    'released_at',released_at,'official_url',official_url,'head_citation',official_receipt_id::text,
    'label_citation',label_receipt_id::text,'released_at_citation',released_at_receipt_id::text,
    'url_citation',url_receipt_id::text)
    FROM serve.p4_patch_catalog c
    WHERE EXISTS(SELECT 1 FROM serve.current_fact f WHERE f.entity_id=c.patch_id
        AND f.raw_object_id=c.raw_object_id AND f.valid_from_run=c.ingest_run_id
        AND f.stat_key COLLATE \"C\" >= 'patch_change_r' AND f.stat_key COLLATE \"C\" < 'patch_change_s')
    ORDER BY released_at DESC,patch_id DESC LIMIT 1";
const CURRENT_ALL: &str = "WITH changed AS MATERIALIZED (
        SELECT DISTINCT patch_id,entity_id FROM serve.p4_patch_change
    ), playable AS MATERIALIZED (
        SELECT e.entity_id FROM (SELECT DISTINCT entity_id FROM changed) e
        WHERE NOT EXISTS(SELECT 1 FROM serve.current_entity h WHERE h.id=e.entity_id AND h.kind='hero')
            OR serve.hero_is_playable(e.entity_id)
    ) SELECT jsonb_build_object('id',patch_id,'label',label,
    'released_at',released_at,'official_url',official_url,'head_citation',official_receipt_id::text,
    'label_citation',label_receipt_id::text,'released_at_citation',released_at_receipt_id::text,
    'url_citation',url_receipt_id::text)
    FROM serve.p4_patch_catalog c
    WHERE c.patch_id IN(SELECT patch_id FROM changed WHERE entity_id IN(SELECT entity_id FROM playable))
    ORDER BY released_at DESC,patch_id DESC LIMIT 1";
const CURRENT_CATALOG: &str = "SELECT jsonb_build_object('id',patch_id,'label',label,
    'released_at',released_at,'official_url',official_url,'head_citation',official_receipt_id::text,
    'label_citation',label_receipt_id::text,'released_at_citation',released_at_receipt_id::text,
    'url_citation',url_receipt_id::text)
    FROM serve.p4_patch_catalog ORDER BY released_at DESC,patch_id DESC LIMIT 1";
const RESOLVE: &str = "SELECT jsonb_build_object('id',patch_id,'label',label,
    'released_at',released_at,'official_url',official_url,'head_citation',official_receipt_id::text,
    'label_citation',label_receipt_id::text,'released_at_citation',released_at_receipt_id::text,
    'url_citation',url_receipt_id::text)
    FROM serve.p4_patch_catalog
    WHERE label=$1 OR CASE WHEN $1 ~ '^[0-9]{4}-[0-9]{2}-[0-9]{2}[T ][0-9]{2}:[0-9]{2}:[0-9]{2}([.][0-9]+)?(Z|[+-][0-9]{2}:[0-9]{2})$'
        AND pg_input_is_valid($1,'timestamp with time zone')
        THEN released_at=$1::timestamptz ELSE false END
    ORDER BY released_at DESC,patch_id DESC LIMIT 2";
const CHANGES: &str = "WITH selected AS MATERIALIZED (
        SELECT * FROM serve.p4_patch_change
        WHERE entity_id=$1 AND ($2::bigint IS NULL OR patch_id=$2)
            AND (NOT EXISTS(SELECT 1 FROM serve.current_entity h WHERE h.id=$1 AND h.kind='hero')
                OR serve.hero_is_playable($1))
    ) SELECT jsonb_build_object('id',c.change_id,'entity_id',c.entity_id,
    'stat_key',c.stat_key,'old',c.old_value,'new',c.new_value,'change_kind',c.change_kind,
    'line_text',c.line_text,'subject',c.subject,'statement_index',c.statement_index,
    'context_index',c.context_index,'stat_label',c.stat_label,'patch',p.label,'released_at',p.released_at,
    'citation',c.receipt_id::text,'head_citation',p.official_receipt_id::text,
    'label_citation',p.label_receipt_id::text,'released_at_citation',p.released_at_receipt_id::text,
    'url_citation',p.url_receipt_id::text,
    'binding_citation',c.binding_receipt_id::text,'entity_citation',c.entity_receipt_id::text,
    'stat_citation',c.stat_receipt_id::text,'source_revision',c.source_revision)
    FROM selected c JOIN LATERAL (
        SELECT e.id AS patch_id,e.receipt_id AS official_receipt_id,m.label,
            CASE WHEN pg_input_is_valid(m.released_at,'timestamp with time zone')
                THEN m.released_at::timestamptz END AS released_at,
            m.label_receipt_id,m.released_at_receipt_id,m.url_receipt_id
        FROM serve.current_entity e CROSS JOIN LATERAL (
            SELECT max(f.value_text::text) FILTER(WHERE f.stat_key='label') AS label,
                max(f.value_text::text) FILTER(WHERE f.stat_key='released_at') AS released_at,
                max(f.receipt_id) FILTER(WHERE f.stat_key='label') AS label_receipt_id,
                max(f.receipt_id) FILTER(WHERE f.stat_key='released_at') AS released_at_receipt_id,
                max(f.receipt_id) FILTER(WHERE f.stat_key='official_url') AS url_receipt_id
            FROM serve.current_fact f
            WHERE f.entity_id=c.patch_id
                AND f.stat_key COLLATE \"C\" IN('label','released_at','official_url')
            OFFSET 0
        ) m
        WHERE e.id=c.patch_id
        OFFSET 0
    ) p ON true
    WHERE ($3::text IS NULL OR p.released_at >= $3::timestamptz)
        AND ($4::text IS NULL OR p.released_at <= $4::timestamptz)
    ORDER BY p.released_at DESC,p.patch_id DESC,c.change_id DESC LIMIT 20";
const CHANGES_ALL: &str = "WITH base AS MATERIALIZED (
        SELECT * FROM serve.p4_patch_change WHERE $1::bigint IS NULL AND ($2::bigint IS NULL OR patch_id=$2)
    ), playable AS MATERIALIZED (
        SELECT e.entity_id FROM (SELECT DISTINCT entity_id FROM base) e
        WHERE NOT EXISTS(SELECT 1 FROM serve.current_entity h WHERE h.id=e.entity_id AND h.kind='hero')
            OR serve.hero_is_playable(e.entity_id)
    ), selected AS MATERIALIZED (
        SELECT * FROM base WHERE entity_id IN(SELECT entity_id FROM playable)
    ) SELECT jsonb_build_object('id',c.change_id,'entity_id',c.entity_id,
    'stat_key',c.stat_key,'old',c.old_value,'new',c.new_value,'change_kind',c.change_kind,
    'line_text',c.line_text,'subject',c.subject,'statement_index',c.statement_index,
    'context_index',c.context_index,'stat_label',c.stat_label,'patch',p.label,'released_at',p.released_at,
    'citation',c.receipt_id::text,'head_citation',p.official_receipt_id::text,
    'label_citation',p.label_receipt_id::text,'released_at_citation',p.released_at_receipt_id::text,
    'url_citation',p.url_receipt_id::text,
    'binding_citation',c.binding_receipt_id::text,'entity_citation',c.entity_receipt_id::text,
    'stat_citation',c.stat_receipt_id::text,'source_revision',c.source_revision)
    FROM selected c JOIN LATERAL (
        SELECT e.id AS patch_id,e.receipt_id AS official_receipt_id,m.label,
            CASE WHEN pg_input_is_valid(m.released_at,'timestamp with time zone')
                THEN m.released_at::timestamptz END AS released_at,
            m.label_receipt_id,m.released_at_receipt_id,m.url_receipt_id
        FROM serve.current_entity e CROSS JOIN LATERAL (
            SELECT max(f.value_text::text) FILTER(WHERE f.stat_key='label') AS label,
                max(f.value_text::text) FILTER(WHERE f.stat_key='released_at') AS released_at,
                max(f.receipt_id) FILTER(WHERE f.stat_key='label') AS label_receipt_id,
                max(f.receipt_id) FILTER(WHERE f.stat_key='released_at') AS released_at_receipt_id,
                max(f.receipt_id) FILTER(WHERE f.stat_key='official_url') AS url_receipt_id
            FROM serve.current_fact f
            WHERE f.entity_id=c.patch_id
                AND f.stat_key COLLATE \"C\" IN('label','released_at','official_url')
            OFFSET 0
        ) m
        WHERE e.id=c.patch_id
        OFFSET 0
    ) p ON true
    WHERE ($3::text IS NULL OR p.released_at >= $3::timestamptz)
        AND ($4::text IS NULL OR p.released_at <= $4::timestamptz)
    ORDER BY p.released_at DESC,p.patch_id DESC,c.change_id DESC LIMIT 20";
const LATEST: &str = "WITH base AS MATERIALIZED (
        SELECT * FROM serve.p4_patch_change
    ), playable AS MATERIALIZED (
        SELECT e.entity_id FROM (SELECT DISTINCT entity_id FROM base) e
        WHERE NOT EXISTS(SELECT 1 FROM serve.current_entity h WHERE h.id=e.entity_id AND h.kind='hero')
            OR serve.hero_is_playable(e.entity_id)
    ), head AS MATERIALIZED (
        SELECT c.* FROM serve.p4_patch_catalog c
        WHERE c.patch_id IN(SELECT patch_id FROM base WHERE entity_id IN(SELECT entity_id FROM playable))
        ORDER BY c.released_at DESC,c.patch_id DESC LIMIT 1
    ), bound AS (
        SELECT b.change_id,b.entity_id,b.subject,b.statement_index,b.line_text,b.receipt_id,
            b.binding_receipt_id,b.entity_receipt_id
        FROM base b JOIN head h ON h.patch_id=b.patch_id
        WHERE b.entity_id IN(SELECT entity_id FROM playable)
    ), unbound AS (
        SELECT f.id AS change_id,NULL::bigint AS entity_id,payload.value->>'subject' AS subject,
            payload.value->'statement_index' AS statement_index,payload.value->>'line_text' AS line_text,
            f.receipt_id,NULL::bigint AS binding_receipt_id,NULL::bigint AS entity_receipt_id
        FROM head h JOIN serve.current_fact f ON f.entity_id=h.patch_id
            AND f.raw_object_id=h.raw_object_id AND f.valid_from_run=h.ingest_run_id
        CROSS JOIN LATERAL (
            SELECT CASE WHEN pg_input_is_valid(f.value_text::text,'jsonb')
                THEN f.value_text::jsonb ELSE '{}'::jsonb END AS value OFFSET 0
        ) payload
        WHERE f.stat_key COLLATE \"C\" >= 'patch_change_r' AND f.stat_key COLLATE \"C\" < 'patch_change_s'
            AND f.stat_key ~ '^patch_change_r[0-9a-f]{64}_[0-9]+$'
            AND jsonb_typeof(payload.value)='object'
            AND jsonb_typeof(payload.value->'subject')='string' AND btrim(payload.value->>'subject')<>''
            AND jsonb_typeof(payload.value->'line_text')='string' AND btrim(payload.value->>'line_text')<>''
            AND jsonb_typeof(payload.value->'statement_index')='number'
            AND payload.value->>'statement_index'=substring(f.stat_key::text FROM '[0-9]+$')
            AND EXISTS(SELECT 1 FROM serve.published_receipt r WHERE r.id=f.receipt_id
                AND r.extractor=h.extractor AND r.raw_object_id=h.raw_object_id
                AND r.ingest_run_id=h.ingest_run_id
                AND r.locator_kind='json_pointer' AND r.locator='/raw_content')
            AND NOT EXISTS(SELECT 1 FROM base b WHERE b.change_id=f.id)
    ) SELECT jsonb_build_object('current',jsonb_build_object('id',h.patch_id,'label',h.label,
        'released_at',h.released_at,'official_url',h.official_url,'head_citation',h.official_receipt_id::text,
        'label_citation',h.label_receipt_id::text,'released_at_citation',h.released_at_receipt_id::text,
        'url_citation',h.url_receipt_id::text),
        'changes',COALESCE((SELECT jsonb_agg(jsonb_build_object('subject',c.subject,'entity_id',c.entity_id,
            'line_text',c.line_text,'citation',c.receipt_id::text,
            'binding_citation',c.binding_receipt_id::text,'entity_citation',c.entity_receipt_id::text)
            ORDER BY (c.statement_index)::numeric,c.change_id)
            FROM (SELECT * FROM bound UNION ALL SELECT * FROM unbound) c),'[]'::jsonb))
    FROM head h";
const SEARCH: &str = "WITH catalog AS MATERIALIZED (
        SELECT * FROM serve.p4_patch_catalog c
        WHERE ($2::text IS NULL OR c.released_at >= $2::timestamptz)
            AND ($3::text IS NULL OR c.released_at <= $3::timestamptz)
    ), facts AS MATERIALIZED (
        SELECT c.patch_id,c.label,c.released_at,c.official_receipt_id,c.label_receipt_id,
            c.released_at_receipt_id,c.url_receipt_id,c.extractor,c.raw_object_id,c.ingest_run_id,
            f.id,f.receipt_id,f.value_text::text AS body
        FROM catalog c CROSS JOIN LATERAL (
            SELECT * FROM serve.current_fact f WHERE f.entity_id=c.patch_id
                AND f.stat_key COLLATE \"C\" >= 'patch_change_r' AND f.stat_key COLLATE \"C\" < 'patch_change_s'
                AND f.stat_key ~ '^patch_change_r[0-9a-f]{64}_[0-9]+$'
            UNION ALL
            SELECT * FROM serve.current_fact f WHERE f.entity_id=c.patch_id
                AND f.stat_key COLLATE \"C\" >= 'patch_line_r' AND f.stat_key COLLATE \"C\" < 'patch_line_s'
                AND f.stat_key ~ '^patch_line_r[0-9a-f]{64}_[0-9]+_[0-9]+$'
            UNION ALL
            SELECT * FROM serve.current_fact f WHERE f.entity_id=c.patch_id
                AND f.stat_key COLLATE \"C\" >= 'patch_note_' AND f.stat_key COLLATE \"C\" < 'patch_note`'
                AND f.stat_key ~ '^patch_note_[0-9]+_[0-9]+$'
        ) f
        WHERE f.raw_object_id=c.raw_object_id AND f.valid_from_run=c.ingest_run_id
    ), matched AS MATERIALIZED (
        SELECT f.*,payload.value,(SELECT count(*) FROM unnest($1::text[]) t
            WHERE strpos(lower(payload.value->>'line_text'),t)>0) AS hits
        FROM facts f CROSS JOIN LATERAL (
            SELECT CASE WHEN pg_input_is_valid(f.body,'jsonb')
                THEN f.body::jsonb ELSE '{}'::jsonb END AS value OFFSET 0
        ) payload
        WHERE jsonb_typeof(payload.value)='object'
            AND jsonb_typeof(payload.value->'line_text')='string' AND btrim(payload.value->>'line_text')<>''
    ), distinct_lines AS (
        SELECT DISTINCT ON (m.patch_id,regexp_replace(btrim(m.value->>'line_text'),'^[-*•][[:space:]]*',''))
            m.* FROM matched m WHERE m.hits>0
        ORDER BY m.patch_id,regexp_replace(btrim(m.value->>'line_text'),'^[-*•][[:space:]]*',''),m.id
    ), ranked AS MATERIALIZED (
        SELECT * FROM distinct_lines d
        ORDER BY d.hits DESC,d.released_at DESC,d.patch_id DESC,d.id DESC LIMIT 60
    ), scored AS (
        SELECT * FROM ranked m
        WHERE EXISTS(SELECT 1 FROM serve.published_receipt r WHERE r.id=m.receipt_id
            AND r.extractor=m.extractor AND r.raw_object_id=m.raw_object_id
            AND r.ingest_run_id=m.ingest_run_id
            AND r.locator_kind='json_pointer' AND r.locator='/raw_content')
    ) SELECT jsonb_build_object('id',s.id,'entity_id',NULL::bigint,'section',s.value->'section',
        'subject',s.value->'subject','line_text',s.value->>'line_text','stat_label',s.value->'stat_label',
        'old',s.value->'old','new',s.value->'new','patch',s.label,'released_at',s.released_at,
        'citation',s.receipt_id::text,'head_citation',s.official_receipt_id::text,
        'label_citation',s.label_receipt_id::text,'released_at_citation',s.released_at_receipt_id::text,
        'url_citation',s.url_receipt_id::text,'binding_citation',NULL::text,'entity_citation',NULL::text,
        'stat_citation',NULL::text)
    FROM scored s
    ORDER BY s.hits DESC,s.released_at DESC,s.patch_id DESC,s.id DESC LIMIT 20";
const SEARCH_TERMS: usize = 8;
const LATEST_WINDOW: usize = 56;
const PATCH_CITATIONS: [&str; 4] = [
    "head_citation",
    "label_citation",
    "released_at_citation",
    "url_citation",
];
const LATEST_CAPACITY: usize = 64;
const RECEIPTS: &str = "SELECT jsonb_build_object('receipt_id',receipt_id,'source_id',source_id,
    'evidence_revision',evidence_revision,'receipt',receipt)
    FROM serve.p4_patch_receipt WHERE receipt_id=ANY($1::bigint[]) ORDER BY receipt_id LIMIT 172";

#[derive(Default)]
struct LatestState {
    generation: String,
    entries: BTreeMap<String, ToolExecution>,
}

#[derive(Default)]
pub struct LatestPatchCache {
    state: tokio::sync::Mutex<LatestState>,
}

pub async fn execute_patch_history(
    pool: &PgPool,
    query: &Query,
    context: &AuthorizedContext,
    id: &str,
    request: &ToolRequest,
) -> Result<ToolExecution, PortError> {
    run_patch_history(pool, None, query, context, id, request).await
}

pub async fn execute_cached_patch_history(
    pool: &PgPool,
    cache: &LatestPatchCache,
    query: &Query,
    context: &AuthorizedContext,
    id: &str,
    request: &ToolRequest,
) -> Result<ToolExecution, PortError> {
    run_patch_history(pool, Some(cache), query, context, id, request).await
}

async fn run_patch_history(
    pool: &PgPool,
    cache: Option<&LatestPatchCache>,
    query: &Query,
    context: &AuthorizedContext,
    id: &str,
    request: &ToolRequest,
) -> Result<ToolExecution, PortError> {
    check(query, context)?;
    let call = ToolCall {
        id: id.into(),
        name: request.name(),
        arguments: request.arguments().clone(),
    };
    if call.validate(&[canonical_patch_history_definition()])? != *request {
        return Err(invalid());
    }
    let ToolSubrequest::CanonicalPatchHistory(history) = request.subrequest() else {
        return Err(invalid());
    };
    let latest = history.entity.is_none()
        && history.since.is_none()
        && history.until.is_none()
        && history.text.is_none();
    let (budget, statement_timeout) = if history.entity.is_some() {
        (
            Duration::from_millis(500),
            "SET LOCAL statement_timeout = '500ms'",
        )
    } else {
        (
            Duration::from_millis(3_500),
            "SET LOCAL statement_timeout = '2000ms'",
        )
    };
    let budget = context.remaining_time()?.min(budget);
    let execution = tokio::time::timeout(budget, async {
        let mut state = match cache.filter(|_| latest) {
            Some(cache) => Some(cache.state.lock().await),
            None => None,
        };
        let mut connection = pool.acquire().await.map_err(database_error)?;
        let mut transaction = connection.begin().await.map_err(database_error)?;
        sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
            .execute(&mut *transaction)
            .await
            .map_err(database_error)?;
        sqlx::query(statement_timeout)
            .execute(&mut *transaction)
            .await
            .map_err(database_error)?;
        let role: String = sqlx::query_scalar("SELECT current_user::text")
            .fetch_one(&mut *transaction)
            .await
            .map_err(database_error)?;
        if role != "brain_tool" {
            return Err(PortError::PermissionDenied(
                "Patchabfrage benötigt den freigegebenen Lesezugang.".into(),
            ));
        }
        let execution = match state.as_deref_mut() {
            Some(state) => cached_execution(&mut transaction, state, id, request, history).await?,
            None => history_execution(&mut transaction, id, request, history).await?,
        };
        transaction.commit().await.map_err(database_error)?;
        Ok::<_, PortError>(execution)
    })
    .await
    .map_err(|_| PortError::Unavailable("Zeitgrenze für Patchwerte erreicht.".into()))??;
    context.check_deadline()?;
    execution.validate_for(&call, request, None)?;
    ensure_size(&execution)?;
    Ok(execution)
}

async fn cached_execution(
    connection: &mut PgConnection,
    state: &mut LatestState,
    id: &str,
    request: &ToolRequest,
    history: &CanonicalPatchHistoryRequest,
) -> Result<ToolExecution, PortError> {
    let generation: String = sqlx::query_scalar("SELECT serve.site_generation()")
        .fetch_one(&mut *connection)
        .await
        .map_err(database_error)?;
    if state.generation != generation {
        state.generation = generation;
        state.entries.clear();
    }
    let key = request.arguments().to_string();
    if let Some(cached) = state.entries.get(&key) {
        let mut execution = cached.clone();
        execution.result.call_id = id.into();
        return Ok(execution);
    }
    let execution = history_execution(connection, id, request, history).await?;
    if state.entries.len() < LATEST_CAPACITY {
        state.entries.insert(key, execution.clone());
    }
    Ok(execution)
}

pub async fn validate_patch_dependency(
    pool: &PgPool,
    query: &Query,
    context: &AuthorizedContext,
    dependency: &ToolEvidenceDependency,
) -> Result<(), PortError> {
    revalidate(pool, None, query, context, dependency).await
}

pub async fn validate_cached_patch_dependency(
    pool: &PgPool,
    cache: &LatestPatchCache,
    query: &Query,
    context: &AuthorizedContext,
    dependency: &ToolEvidenceDependency,
) -> Result<(), PortError> {
    revalidate(pool, Some(cache), query, context, dependency).await
}

async fn revalidate(
    pool: &PgPool,
    cache: Option<&LatestPatchCache>,
    query: &Query,
    context: &AuthorizedContext,
    dependency: &ToolEvidenceDependency,
) -> Result<(), PortError> {
    if dependency.game_context.is_some() || dependency.evidence.is_empty() {
        return Err(invalid());
    }
    let current = run_patch_history(
        pool,
        cache,
        query,
        context,
        "revalidation",
        &dependency.request.without_canonical_result(),
    )
    .await?;
    if current.dependencies.as_slice() != std::slice::from_ref(dependency) {
        return Err(PortError::Unavailable(
            "Patchbelege haben sich geändert.".into(),
        ));
    }
    Ok(())
}

pub fn historical_patch_unavailable(id: &str, name: ToolName) -> ToolExecution {
    error_execution(id, name, "historical_patch_unavailable")
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

fn patch_timestamp(patch: &Value) -> Result<&str, PortError> {
    if patch["id"].as_i64().is_none_or(|id| id <= 0)
        || patch["label"].as_str().is_none_or(str::is_empty)
        || patch["head_citation"]
            .as_str()
            .is_none_or(|id| !id.parse::<i64>().is_ok_and(|id| id > 0))
    {
        return Err(invalid());
    }
    patch["released_at"]
        .as_str()
        .filter(|timestamp| !timestamp.is_empty())
        .ok_or_else(invalid)
}

async fn history_execution(
    connection: &mut PgConnection,
    id: &str,
    request: &ToolRequest,
    history: &CanonicalPatchHistoryRequest,
) -> Result<ToolExecution, PortError> {
    let ranged = history.since.is_some() || history.until.is_some();
    if history.entity.is_none() && !ranged && history.text.is_none() {
        return latest_execution(connection, id, request, history).await;
    }
    if history.offset.is_some() {
        return Ok(error_execution(
            id,
            request.name(),
            "patch_offset_unsupported",
        ));
    }
    let current_query = if history.text.is_some() {
        CURRENT_CATALOG
    } else if history.entity.is_some() {
        CURRENT
    } else {
        CURRENT_ALL
    };
    let Some(current) = sqlx::query_scalar::<_, Value>(current_query)
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
    patch_timestamp(&current)?;
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
        .map(|patch| patch.as_ref().map(patch_timestamp).transpose())
        .collect::<Result<Vec<_>, _>>()?;
    if ranged {
        let reversed: bool = sqlx::query_scalar("SELECT $1::text IS NOT NULL AND $2::text IS NOT NULL AND $1::timestamptz > $2::timestamptz")
            .bind(timestamps[0]).bind(timestamps[1]).fetch_one(&mut *connection).await.map_err(database_error)?;
        if reversed {
            return Ok(error_execution(id, request.name(), "patch_range_reversed"));
        }
    }
    if let Some(text) = &history.text {
        let rows = sqlx::query_scalar::<_, Value>(SEARCH)
            .bind(search_terms(text))
            .bind(timestamps[0])
            .bind(timestamps[1])
            .fetch_all(&mut *connection)
            .await
            .map_err(database_error)?;
        let [since, until]: [Option<Value>; 2] = bounds.try_into().map_err(|_| invalid())?;
        return finish_history(
            connection,
            id,
            request,
            history,
            current,
            [since, until],
            rows,
        )
        .await;
    }
    let entity = history
        .entity
        .map(|entity| i64::try_from(entity).map_err(|_| invalid()))
        .transpose()?;
    let patch_id = if ranged {
        None
    } else {
        Some(
            current["id"]
                .as_i64()
                .filter(|id| *id > 0)
                .ok_or_else(invalid)?,
        )
    };
    let changes_query = if entity.is_some() {
        CHANGES
    } else {
        CHANGES_ALL
    };
    let rows = sqlx::query_scalar::<_, Value>(changes_query)
        .bind(entity)
        .bind(patch_id)
        .bind(timestamps[0])
        .bind(timestamps[1])
        .fetch_all(&mut *connection)
        .await
        .map_err(database_error)?;
    let [since, until]: [Option<Value>; 2] = bounds.try_into().map_err(|_| invalid())?;
    finish_history(
        connection,
        id,
        request,
        history,
        current,
        [since, until],
        rows,
    )
    .await
}

fn search_terms(text: &str) -> Vec<String> {
    let mut terms: Vec<String> = Vec::new();
    for word in text
        .split(|ch: char| !ch.is_alphanumeric() && ch != '%')
        .map(str::to_lowercase)
        .filter(|word| word.chars().count() >= 3)
    {
        if terms.len() < SEARCH_TERMS && !terms.contains(&word) {
            terms.push(word);
        }
    }
    terms
}

async fn latest_execution(
    connection: &mut PgConnection,
    id: &str,
    request: &ToolRequest,
    history: &CanonicalPatchHistoryRequest,
) -> Result<ToolExecution, PortError> {
    let Some(mut latest) = sqlx::query_scalar::<_, Value>(LATEST)
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
    let current = latest["current"].take();
    patch_timestamp(&current)?;
    let Value::Array(rows) = latest["changes"].take() else {
        return Err(invalid());
    };
    if rows.is_empty() {
        return Ok(error_execution(id, request.name(), "no_patch_changes"));
    }
    let offset = history.offset.unwrap_or(0);
    if offset >= rows.len() {
        return Ok(error_execution(
            id,
            request.name(),
            "patch_offset_out_of_range",
        ));
    }
    let window = history.limit.unwrap_or(LATEST_WINDOW).min(LATEST_WINDOW);
    let rows_window = &rows[offset..rows.len().min(offset + window)];
    let ids = latest_citations(&current, rows_window)?;
    let evidence = load_receipts(connection, &ids).await?;
    let receipts = receipt_map(&evidence)?;
    for count in (1..=rows_window.len()).rev() {
        let page = LatestPage {
            offset,
            count,
            total: rows.len(),
        };
        let value = latest_value(&current, rows_window, &page);
        if let Some(execution) = latest_bounded(id, request, value, &receipts)? {
            return Ok(execution);
        }
    }
    Ok(error_execution(id, request.name(), "result_too_large"))
}

struct LatestPage {
    offset: usize,
    count: usize,
    total: usize,
}

fn latest_citations(current: &Value, rows: &[Value]) -> Result<BTreeSet<String>, PortError> {
    let mut ids = BTreeSet::new();
    for field in PATCH_CITATIONS {
        ids.insert(positive_citation(&current[field])?);
    }
    for row in rows {
        ids.insert(positive_citation(&row["citation"])?);
        let bound = !row["entity_id"].is_null();
        for field in ["binding_citation", "entity_citation"] {
            match (bound, row[field].is_null()) {
                (true, false) => {
                    ids.insert(positive_citation(&row[field])?);
                }
                (false, true) => {}
                _ => return Err(invalid()),
            }
        }
        if bound && row["entity_id"].as_i64().is_none_or(|id| id <= 0) {
            return Err(invalid());
        }
        for field in ["subject", "line_text"] {
            if row[field]
                .as_str()
                .is_none_or(|text| text.trim().is_empty())
            {
                return Err(invalid());
            }
        }
    }
    Ok(ids)
}

fn positive_citation(value: &Value) -> Result<String, PortError> {
    value
        .as_str()
        .filter(|id| id.parse::<i64>().is_ok_and(|id| id > 0))
        .map(str::to_owned)
        .ok_or_else(invalid)
}

fn latest_value(current: &Value, rows: &[Value], page: &LatestPage) -> Value {
    let mut groups: Vec<Value> = Vec::new();
    for row in &rows[..page.count] {
        let change = json!({"citation":row["citation"],"line_text":row["line_text"]});
        let open = groups.last_mut().filter(|group| {
            group["subject"] == row["subject"] && group["entity_id"] == row["entity_id"]
        });
        match open.and_then(|group| group["changes"].as_array_mut()) {
            Some(changes) => changes.push(change),
            None => groups.push(
                json!({"subject":row["subject"],"entity_id":row["entity_id"],
                "binding_citation":row["binding_citation"],"entity_citation":row["entity_citation"],
                "changes":[change]}),
            ),
        }
    }
    let next = page.offset + page.count;
    let complete = page.offset == 0 && next == page.total;
    let mut current = current.clone();
    current["citation"] = current["released_at_citation"].clone();
    json!({"entity_id":null,"current_official_patch":current,
        "range":{"since":null,"until":null},"groups":groups,
        "offset":page.offset,"returned_count":page.count,"total_count":page.total,
        "next_offset":(next < page.total).then_some(next),"truncated":next < page.total,
        "completeness":if complete {"all_extracted_hero_section_lines"} else {"partial"}})
}

fn latest_bounded(
    id: &str,
    request: &ToolRequest,
    mut value: Value,
    receipts: &BTreeMap<String, Evidence>,
) -> Result<Option<ToolExecution>, PortError> {
    let ids = remap_latest(&mut value, receipts)?;
    let selected = ids
        .iter()
        .map(|id| receipts.get(id).cloned().ok_or_else(invalid))
        .collect::<Result<Vec<_>, _>>()?;
    sealed(id, request, value, selected)
}

fn sealed(
    id: &str,
    request: &ToolRequest,
    value: Value,
    selected: Vec<Evidence>,
) -> Result<Option<ToolExecution>, PortError> {
    if selected.len() > MAX_EVIDENCE {
        return Ok(None);
    }
    let result = ToolResult {
        call_id: id.into(),
        name: request.name(),
        result: value,
        evidence_ids: selected
            .iter()
            .map(|evidence| evidence.evidence_id.clone())
            .collect(),
        is_error: false,
    };
    if !result_fits(&result)? {
        return Ok(None);
    }
    let dependency = ToolEvidenceDependency {
        request: request.clone(),
        game_context: None,
        evidence: selected,
    }
    .with_canonical_result(&result)?;
    Ok(Some(ToolExecution {
        result,
        dependencies: vec![dependency],
        usage: Usage::default(),
    }))
}

fn remap_latest(
    value: &mut Value,
    receipts: &BTreeMap<String, Evidence>,
) -> Result<BTreeSet<String>, PortError> {
    let mut ids = BTreeSet::new();
    let mut citation = |value: &mut Value| -> Result<(), PortError> {
        let raw = positive_citation(value)?;
        *value = json!(receipts.get(&raw).ok_or_else(invalid)?.evidence_id);
        ids.insert(raw);
        Ok(())
    };
    let current = &mut value["current_official_patch"];
    for field in PATCH_CITATIONS.into_iter().chain(["citation"]) {
        citation(&mut current[field])?;
    }
    for group in value["groups"].as_array_mut().ok_or_else(invalid)? {
        for field in ["binding_citation", "entity_citation"] {
            if !group[field].is_null() {
                citation(&mut group[field])?;
            }
        }
        for change in group["changes"].as_array_mut().ok_or_else(invalid)? {
            citation(&mut change["citation"])?;
        }
    }
    Ok(ids)
}

fn result_fits(result: &ToolResult) -> Result<bool, PortError> {
    let mut worst = result.clone();
    worst.call_id = "\"".repeat(512);
    Ok(serde_json::to_vec(&worst).map_err(|_| invalid())?.len() < MAX_BYTES)
}

async fn load_receipts(
    connection: &mut PgConnection,
    ids: &BTreeSet<String>,
) -> Result<Vec<Evidence>, PortError> {
    let numeric_ids = ids
        .iter()
        .map(|id| id.parse::<i64>().map_err(|_| invalid()))
        .collect::<Result<Vec<_>, _>>()?;
    let rows = sqlx::query_scalar::<_, Value>(RECEIPTS)
        .bind(numeric_ids)
        .fetch_all(connection)
        .await
        .map_err(database_error)?;
    let evidence = rows
        .into_iter()
        .map(receipt_evidence)
        .collect::<Result<Vec<_>, _>>()?;
    let resolved_ids = evidence
        .iter()
        .map(|evidence| {
            CanonicalReceipt::from_evidence(evidence)
                .map(|receipt| receipt.raw_receipt_id.to_string())
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    if resolved_ids != *ids || evidence.len() != ids.len() {
        return Err(invalid());
    }
    Ok(evidence)
}

fn receipt_map(evidence: &[Evidence]) -> Result<BTreeMap<String, Evidence>, PortError> {
    let mut receipts = BTreeMap::new();
    for evidence in evidence {
        let metadata = CanonicalReceipt::from_evidence(evidence)?;
        if receipts
            .insert(metadata.raw_receipt_id.to_string(), evidence.clone())
            .is_some()
        {
            return Err(invalid());
        }
    }
    Ok(receipts)
}

async fn finish_history(
    connection: &mut PgConnection,
    id: &str,
    request: &ToolRequest,
    history: &CanonicalPatchHistoryRequest,
    current: Value,
    bounds: [Option<Value>; 2],
    rows: Vec<Value>,
) -> Result<ToolExecution, PortError> {
    let cap_reached = rows.len() == 20;
    let limit = history.limit.unwrap_or(20);
    let omitted = rows.len().saturating_sub(limit);
    let value = json!({"entity_id":history.entity,"current_official_patch":current,
        "range":{"since":bounds[0],"until":bounds[1]},
        "changes":rows.into_iter().take(limit).collect::<Vec<_>>(),"truncated":omitted>0 || cap_reached,
        "omitted_count":omitted,"omitted_count_is_lower_bound":cap_reached,"completeness":"unknown"});
    if value["changes"].as_array().is_none_or(Vec::is_empty) {
        return Ok(error_execution(id, request.name(), "no_patch_changes"));
    }
    let ids = receipt_ids(&value)?;
    let evidence = load_receipts(connection, &ids).await?;
    bound_execution(id, request, value, &evidence)
}

fn receipt_ids(value: &Value) -> Result<BTreeSet<String>, PortError> {
    fn citation(value: &Value, ids: &mut BTreeSet<String>) -> Result<(), PortError> {
        let id = value
            .as_str()
            .filter(|id| id.parse::<i64>().is_ok_and(|id| id > 0))
            .ok_or_else(invalid)?;
        ids.insert(id.to_owned());
        Ok(())
    }
    fn patch_citations(value: &Value, ids: &mut BTreeSet<String>) -> Result<(), PortError> {
        for field in [
            "head_citation",
            "label_citation",
            "released_at_citation",
            "url_citation",
        ] {
            citation(&value[field], ids)?;
        }
        Ok(())
    }
    let mut ids = BTreeSet::new();
    patch_citations(&value["current_official_patch"], &mut ids)?;
    for bound in ["since", "until"] {
        if !value["range"][bound].is_null() {
            patch_citations(&value["range"][bound], &mut ids)?;
        }
    }
    for change in value["changes"].as_array().ok_or_else(invalid)? {
        citation(&change["citation"], &mut ids)?;
        patch_citations(change, &mut ids)?;
        let bound = !change["entity_id"].is_null();
        for field in ["binding_citation", "entity_citation"] {
            if bound || !change[field].is_null() {
                citation(&change[field], &mut ids)?;
            }
        }
        if !change["stat_citation"].is_null() {
            citation(&change["stat_citation"], &mut ids)?;
        }
    }
    Ok(ids)
}

fn receipt_evidence(row: Value) -> Result<Evidence, PortError> {
    let id = row["receipt_id"]
        .as_i64()
        .filter(|id| *id > 0)
        .ok_or_else(invalid)?;
    let source = row["source_id"]
        .as_str()
        .filter(|source| !source.is_empty())
        .ok_or_else(invalid)?;
    let revision = row["evidence_revision"]
        .as_u64()
        .filter(|revision| *revision > 0)
        .ok_or_else(invalid)?;
    let citation = id.to_string();
    let content = &row["receipt"];
    if !content.is_object()
        || content.get("patch") != Some(&Value::Null)
        || content["raw_object_id"].as_i64().is_none_or(|id| id <= 0)
        || content["ingest_run_id"].as_u64() != Some(revision)
        || content["source"].as_str() != Some(source)
        || content["citation"].as_str() != Some(citation.as_str())
    {
        return Err(invalid());
    }
    for field in [
        "source",
        "source_revision",
        "path",
        "content_sha256",
        "extractor",
        "locator_kind",
        "locator",
        "value_sha256",
    ] {
        if content[field].as_str().is_none() {
            return Err(invalid());
        }
    }
    for field in ["content_sha256", "value_sha256"] {
        let hash = content[field].as_str().ok_or_else(invalid)?;
        if hash.len() != 64 || !hash.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(invalid());
        }
    }
    format_receipt(&CanonicalReceipt {
        raw_receipt_id: id,
        raw_object_id: content["raw_object_id"].as_i64().ok_or_else(invalid)?,
        ingest_run_id: content["ingest_run_id"].as_i64().ok_or_else(invalid)?,
        source: source.into(),
        source_revision: content["source_revision"]
            .as_str()
            .ok_or_else(invalid)?
            .into(),
        path: content["path"].as_str().ok_or_else(invalid)?.into(),
        locator_kind: content["locator_kind"].as_str().ok_or_else(invalid)?.into(),
        locator: content["locator"].as_str().ok_or_else(invalid)?.into(),
        value_sha256: content["value_sha256"].as_str().ok_or_else(invalid)?.into(),
        extractor: content["extractor"].as_str().ok_or_else(invalid)?.into(),
        patch: None,
    })
}

fn remap_citations(
    value: &mut Value,
    receipts: &BTreeMap<String, Evidence>,
) -> Result<(), PortError> {
    fn citation(value: &mut Value, receipts: &BTreeMap<String, Evidence>) -> Result<(), PortError> {
        let receipt = value
            .as_str()
            .and_then(|id| receipts.get(id))
            .ok_or_else(invalid)?;
        *value = json!(receipt.evidence_id);
        Ok(())
    }
    fn patch_citations(
        value: &mut Value,
        receipts: &BTreeMap<String, Evidence>,
    ) -> Result<(), PortError> {
        for field in [
            "head_citation",
            "label_citation",
            "released_at_citation",
            "url_citation",
        ] {
            citation(&mut value[field], receipts)?;
        }
        Ok(())
    }
    patch_citations(&mut value["current_official_patch"], receipts)?;
    for bound in ["since", "until"] {
        if !value["range"][bound].is_null() {
            patch_citations(&mut value["range"][bound], receipts)?;
        }
    }
    for change in value["changes"].as_array_mut().ok_or_else(invalid)? {
        citation(&mut change["citation"], receipts)?;
        patch_citations(change, receipts)?;
        let bound = !change["entity_id"].is_null();
        for field in ["binding_citation", "entity_citation"] {
            if bound || !change[field].is_null() {
                citation(&mut change[field], receipts)?;
            }
        }
        if !change["stat_citation"].is_null() {
            citation(&mut change["stat_citation"], receipts)?;
        }
    }
    Ok(())
}

fn bound_execution(
    id: &str,
    request: &ToolRequest,
    mut value: Value,
    evidence: &[Evidence],
) -> Result<ToolExecution, PortError> {
    let receipts = receipt_map(evidence)?;
    loop {
        if value["changes"].as_array().ok_or_else(invalid)?.is_empty() {
            return Ok(error_execution(id, request.name(), "result_too_large"));
        }
        let ids = receipt_ids(&value)?;
        let selected = ids
            .iter()
            .map(|id| receipts.get(id).cloned().ok_or_else(invalid))
            .collect::<Result<Vec<_>, _>>()?;
        let mut result = value.clone();
        remap_citations(&mut result, &receipts)?;
        if let Some(execution) = sealed(id, request, result, selected)? {
            return Ok(execution);
        }
        value["changes"].as_array_mut().ok_or_else(invalid)?.pop();
        let omitted = value["omitted_count"].as_u64().unwrap_or(0);
        value["omitted_count"] = json!(omitted.saturating_add(1));
        value["truncated"] = json!(true);
    }
}

fn ensure_size(execution: &ToolExecution) -> Result<(), PortError> {
    if serde_json::to_vec(&execution.result)
        .map_err(|_| invalid())?
        .len()
        >= MAX_BYTES
    {
        return Err(PortError::Unavailable(
            "Patchabfrage überschreitet die Antwortgrenze.".into(),
        ));
    }
    Ok(())
}

fn error_execution(id: &str, name: ToolName, code: &str) -> ToolExecution {
    ToolExecution {
        result: ToolResult {
            call_id: id.into(),
            name,
            result: json!({"error":code,"message":"Für diese Anfrage liegen keine sicher belegten Patchwerte vor."}),
            evidence_ids: Vec::new(),
            is_error: true,
        },
        dependencies: Vec::new(),
        usage: Usage::default(),
    }
}

#[cfg(test)]
#[path = "pg_patch_tools_tests.rs"]
mod tests;
