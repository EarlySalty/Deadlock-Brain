#[path = "timer_tools.rs"]
mod timer_tools;

use brain_contracts::{
    canonical_receipt::{format_receipt, CanonicalReceipt},
    tools::{BuildPlanRequest, CanonicalEntityKind, GetHeroRequest, HeroSection},
    Accounted, AuthorizedContext, Evidence, PinnedGameContext, PortError, PortFailure, Query,
    ToolCall, ToolDefinition, ToolEvidenceDependency, ToolExecution, ToolExecutionPort, ToolName,
    ToolRequest, ToolResult, ToolSubrequest, ToolValidationPurpose, Usage, UsageAccounting,
};
use serde_json::{json, Value};
use sqlx::{postgres::PgConnectOptions, PgConnection, PgPool, Row};
use std::{collections::BTreeMap, future::Future, time::Duration};

const MAX_BYTES: usize = 16_384;
const MAX_EVIDENCE: usize = 100;
const EXACT: &str = "SELECT * FROM (
    SELECT DISTINCT ON (a.entity_id) a.entity_id, i.kind, i.key, a.alias_norm, a.lang,
        a.receipt_id::text AS citation, a.source_revision, i.identity,
        1.0::real AS similarity
    FROM serve.current_alias a JOIN serve.entity_identity i ON i.entity_id=a.entity_id
    JOIN serve.published_receipt identity_receipt ON identity_receipt.id=i.receipt_id
    JOIN serve.published_receipt alias_receipt ON alias_receipt.id=a.receipt_id
        AND alias_receipt.raw_object_id=identity_receipt.raw_object_id
        AND alias_receipt.ingest_run_id=identity_receipt.ingest_run_id
        AND alias_receipt.extractor=identity_receipt.extractor
    WHERE a.alias_norm=$1 AND ($2::text IS NULL OR i.kind=$2)
    ORDER BY a.entity_id, a.receipt_id, a.lang
) candidates ORDER BY entity_id LIMIT $3";
const FUZZY: &str = "SELECT * FROM (
    SELECT DISTINCT ON (a.entity_id) a.entity_id, i.kind, i.key, a.alias_norm, a.lang,
        a.receipt_id::text AS citation, a.source_revision, i.identity,
        public.similarity(a.alias_norm,$1) AS similarity
    FROM serve.current_alias a JOIN serve.entity_identity i ON i.entity_id=a.entity_id
    JOIN serve.published_receipt identity_receipt ON identity_receipt.id=i.receipt_id
    JOIN serve.published_receipt alias_receipt ON alias_receipt.id=a.receipt_id
        AND alias_receipt.raw_object_id=identity_receipt.raw_object_id
        AND alias_receipt.ingest_run_id=identity_receipt.ingest_run_id
        AND alias_receipt.extractor=identity_receipt.extractor
    WHERE a.alias_norm OPERATOR(public.%) $1
        AND public.similarity(a.alias_norm,$1)>=0.3
        AND ($2::text IS NULL OR i.kind=$2)
    ORDER BY a.entity_id, public.similarity(a.alias_norm,$1) DESC, a.receipt_id, a.lang
) candidates ORDER BY similarity DESC, entity_id LIMIT $3";
const RECEIPTS: &str = "SELECT p.id, p.raw_object_id, p.source, p.source_revision, p.path,
    p.locator_kind, p.locator, encode(p.value_sha256,'hex') AS value_sha256,
    p.extractor, p.ingest_run_id
    FROM serve.published_receipt p
    WHERE p.id=ANY($1::bigint[]) ORDER BY p.id LIMIT 100";

fn invalid() -> PortError {
    PortError::InvalidResponse("Kanonischer Spielbeleg ist ungültig.".into())
}

fn database_error(_: sqlx::Error) -> PortError {
    PortError::Unavailable("Spielwerte können derzeit nicht sicher gelesen werden.".into())
}

async fn assert_session(connection: &mut PgConnection) -> Result<(), sqlx::Error> {
    let (login, role, readonly, default_readonly, timeout, threshold, jit): (
        String,
        String,
        String,
        String,
        String,
        String,
        String,
    ) = sqlx::query_as(
        "SELECT session_user::text,current_user::text,
        current_setting('transaction_read_only'),current_setting('default_transaction_read_only'),
        current_setting('statement_timeout'),current_setting('pg_trgm.similarity_threshold'),
        current_setting('jit')",
    )
    .fetch_one(connection)
    .await?;
    if login != "brain_readonly"
        || role != "brain_tool"
        || readonly != "on"
        || default_readonly != "on"
        || timeout != "500ms"
        || threshold != "0.3"
        || jit != "off"
    {
        return Err(sqlx::Error::Protocol(
            "Werkzeugverbindung wurde verunreinigt.".into(),
        ));
    }
    Ok(())
}

pub(super) fn tool_pool(options: PgConnectOptions, wait: Duration) -> PgPool {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(wait)
        .after_connect(|connection, _| Box::pin(async move {
            sqlx::query("SET SESSION ROLE brain_tool").execute(&mut *connection).await?;
            sqlx::query("SET SESSION default_transaction_read_only=on").execute(&mut *connection).await?;
            sqlx::query("SET SESSION statement_timeout='500ms'").execute(&mut *connection).await?;
            sqlx::query("SET SESSION pg_trgm.similarity_threshold=0.3").execute(&mut *connection).await?;
            sqlx::query("SET SESSION jit=off").execute(&mut *connection).await?;
            assert_session(connection).await?;
            let permitted: bool = sqlx::query_scalar("SELECT EXISTS (
                SELECT 1 FROM pg_auth_members m JOIN pg_roles r ON r.oid=m.roleid
                JOIN pg_roles u ON u.oid=m.member WHERE r.rolname='brain_tool'
                AND u.rolname=session_user AND m.set_option AND NOT m.inherit_option AND NOT m.admin_option)
                AND NOT has_schema_privilege(current_user,'raw','USAGE')
                AND NOT has_schema_privilege(current_user,'ops','USAGE')
                AND NOT EXISTS (SELECT 1 FROM pg_class c JOIN pg_namespace n ON n.oid=c.relnamespace
                    WHERE n.nspname IN ('canon','raw','ops') AND c.relkind IN ('r','p','v','m')
                    AND has_table_privilege(current_user,c.oid,'SELECT,INSERT,UPDATE,DELETE,TRUNCATE'))")
                .fetch_one(connection).await?;
            if !permitted {
                return Err(sqlx::Error::Protocol("Werkzeugrechte sind nicht isoliert.".into()));
            }
            Ok(())
        }))
        .before_acquire(|connection, _| Box::pin(async move {
            assert_session(connection).await?;
            Ok(true)
        }))
        .connect_lazy_with(options)
}

pub(super) fn blocking<F: Future>(runtime: &tokio::runtime::Handle, future: F) -> F::Output {
    if tokio::runtime::Handle::try_current().is_ok() {
        tokio::task::block_in_place(|| runtime.block_on(future))
    } else {
        runtime.block_on(future)
    }
}

pub(super) struct PgTools<T> {
    delegate: T,
    pool: PgPool,
    runtime: tokio::runtime::Handle,
}

impl<T> PgTools<T> {
    pub(super) fn new(delegate: T, pool: PgPool, runtime: tokio::runtime::Handle) -> Self {
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
        call_id: &str,
        request: &ToolRequest,
    ) -> Result<ToolExecution, PortError> {
        Self::check(query, context)?;
        let call = ToolCall {
            id: call_id.into(),
            name: request.name(),
            arguments: request.arguments().clone(),
        };
        if call.validate(&definitions())? != *request {
            return Err(invalid());
        }
        if query
            .patch
            .as_deref()
            .is_some_and(|patch| patch != "current")
            || crate::pg_asset_tools::historical(request)
            || matches!(request.subrequest(), ToolSubrequest::GetHero(hero)
                if hero.patch.as_deref().is_some_and(|patch| patch != "current"))
            || matches!(request.subrequest(), ToolSubrequest::GetTimer(timer)
                if timer.patch.as_deref().is_some_and(|patch| patch != "current"))
        {
            return Ok(error_execution(
                call_id,
                request.name(),
                "historical_patch_unavailable",
                "Für historische Patches sind keine veröffentlichten Spielwerte verfügbar.",
            ));
        }
        let budget = context.remaining_time()?;
        let value = blocking(&self.runtime, async { tokio::time::timeout(budget, async {
            let mut connection = self.pool.begin().await.map_err(database_error)?;
            sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
                .execute(&mut *connection).await.map_err(database_error)?;
            let result = match request.subrequest() {
                ToolSubrequest::FindEntity(find) => {
                    let text = brain_storage::gamefacts::limits::normalize_alias(&find.text);
                    let kind = find.kind.map(kind_name);
                    let limit = find.limit.unwrap_or(5);
                    let mut rows = sqlx::query(EXACT).bind(&text).bind(kind).bind((limit + 1) as i64)
                        .fetch_all(&mut *connection).await.map_err(database_error)?;
                    let exact = !rows.is_empty();
                    if !exact {
                        rows = sqlx::query(FUZZY).bind(&text).bind(kind).bind((limit + 1) as i64)
                            .fetch_all(&mut *connection).await.map_err(database_error)?;
                    }
                    if rows.is_empty() {
                        return Ok(error_execution(call_id, request.name(), "entity_not_found",
                            "Keine passende veröffentlichte Entität gefunden."));
                    }
                    let ambiguous = rows.len() > 1;
                    let omitted = rows.len().saturating_sub(limit);
                    let entities = rows.into_iter().take(limit).map(|row| {
                        let entity_id = row.try_get::<i64,_>("entity_id").map_err(database_error)?;
                        let identity = row.try_get::<Value,_>("identity").map_err(database_error)?;
                        let similarity = row.try_get::<f32,_>("similarity").map_err(database_error)?;
                        if entity_id <= 0 || !similarity.is_finite() { return Err(invalid()); }
                        Ok(json!({"entity_id":entity_id,
                            "kind":row.try_get::<String,_>("kind").map_err(database_error)?,
                            "key":row.try_get::<String,_>("key").map_err(database_error)?,
                            "name_en":identity["name_en"], "name_de":identity["name_de"],
                            "score":similarity,
                            "alias":row.try_get::<String,_>("alias_norm").map_err(database_error)?,
                            "language":row.try_get::<String,_>("lang").map_err(database_error)?,
                            "citation":row.try_get::<String,_>("citation").map_err(database_error)?,
                            "patch":null,"source_revision":row.try_get::<String,_>("source_revision").map_err(database_error)?,
                            "identity":identity}))
                    }).collect::<Result<Vec<Value>,PortError>>()?;
                    json!({"entities":entities,"match":if exact {"exact"} else {"suggestion"},
                        "ambiguous":ambiguous,"resolved":exact && !ambiguous,
                        "truncated":omitted>0,"omitted_count":omitted,"omitted_count_is_lower_bound":omitted>0})
                }
                ToolSubrequest::GetHero(hero) => {
                    let profile = sqlx::query_scalar::<_,Value>(hero_sql(hero))
                        .bind(hero.hero as i64).fetch_optional(&mut *connection).await.map_err(database_error)?;
                    let Some(profile) = profile else {
                        return Ok(error_execution(call_id, request.name(), "hero_not_found",
                            "Kein veröffentlichter Heldensteckbrief für diese Entitäts-ID gefunden."));
                    };
                    profile
                }
                ToolSubrequest::GetTimer(timer) => {
                    let Some(value) = timer_tools::read(&mut connection, timer).await? else {
                        return Ok(error_execution(call_id, request.name(), "timer_not_found",
                            "Für diesen Timer sind keine belegten Werte veröffentlicht."));
                    };
                    value
                }
                ToolSubrequest::GetItem(_) | ToolSubrequest::GetAbility(_) | ToolSubrequest::Compare(_) => {
                    let Some(value) = crate::pg_asset_tools::read(&mut connection, call_id, request).await? else {
                        return Ok(error_execution(call_id, request.name(), "asset_selection_unavailable",
                            "Für diese Auswahl sind keine vollständigen veröffentlichten Spielbelege verfügbar."));
                    };
                    value
                }
                _ => return Err(invalid()),
            };
            let execution = bounded_execution(&mut connection, call_id, request, result).await?;
            connection.commit().await.map_err(database_error)?;
            Ok(execution)
        }).await }).map_err(|_| PortError::Unavailable("Zeitgrenze für Spielwerte erreicht.".into()))??;
        context.check_deadline()?;
        value.validate_for(&call, request, None)?;
        Ok(value)
    }
}

impl<T: ToolExecutionPort> ToolExecutionPort for PgTools<T> {
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
        let mut result = self.delegate.definitions(query, context, pin)?;
        if Self::check(query, context).is_ok() {
            result.extend(definitions());
        }
        Ok(result)
    }

    fn execute(
        &self,
        query: &Query,
        context: &AuthorizedContext,
        pin: Option<&PinnedGameContext>,
        id: &str,
        request: &ToolRequest,
    ) -> Result<ToolExecution, PortError> {
        if canonical(request.name()) {
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
        if !canonical(request.name()) {
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
        let mut old = Vec::new();
        for dependency in dependencies {
            if !canonical(dependency.request.name()) {
                old.push(dependency.clone());
                continue;
            }
            if dependency.game_context.is_some() || dependency.evidence.is_empty() {
                return Err(invalid());
            }
            let current = self.canonical_execution(
                query,
                context,
                "revalidation",
                &dependency.request.without_canonical_result(),
            )?;
            if current.dependencies.as_slice() != std::slice::from_ref(dependency) {
                return Err(PortError::Unavailable(
                    "Veröffentlichte Spielwerte haben sich geändert.".into(),
                ));
            }
        }
        self.delegate
            .validate_dependencies(query, context, pin, &old, purpose)
    }
}

pub(super) fn canonical(name: ToolName) -> bool {
    matches!(
        name,
        ToolName::FindEntity | ToolName::GetHero | ToolName::GetTimer
    ) || crate::pg_asset_tools::canonical(name)
}

fn kind_name(kind: CanonicalEntityKind) -> &'static str {
    match kind {
        CanonicalEntityKind::Hero => "hero",
        CanonicalEntityKind::Ability => "ability",
        CanonicalEntityKind::Item => "item",
        CanonicalEntityKind::Weapon => "weapon",
        CanonicalEntityKind::Timer => "timer",
        CanonicalEntityKind::Rule => "rule",
    }
}

fn definitions() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {name:ToolName::FindEntity,
            description:"Sucht veröffentlichte Spielentitäten nach Namen oder Alias. Für einen Heldensteckbrief zuerst find_entity mit kind hero verwenden und anschließend get_hero mit der zurückgegebenen entity_id. Diese positive Datenbank-ID ist keine ältere Spiel-ID. Exakte eindeutige Treffer sind aufgelöst; unscharfe Treffer sind nur Vorschläge, keine sichere Zuordnung. Bei Mehrdeutigkeit nachfragen. Belege enthalten den beobachteten Quellenstand, keinen bestätigten offiziellen Patch.".into(),
            input_schema:json!({"type":"object","additionalProperties":false,"required":["text"],"properties":{
                "text":{"type":"string","minLength":1,"maxLength":128},
                "kind":{"type":"string","enum":["hero","ability","item","weapon","timer","rule"]},
                "limit":{"type":"integer","minimum":1,"maximum":5}}})},
        ToolDefinition {name:ToolName::GetHero,
            description:"Liest den veröffentlichten Heldensteckbrief zur kanonischen entity_id aus find_entity. Liefert Identität und die gewünschten Abschnitte stats, abilities und weapon, standardmäßig alle drei. Fehlender Patch oder current bedeutet aktuell beobachteter Quellenstand, nicht offizieller Patch. Andere Patches werden ohne Belege abgewiesen. Werte samt Quellenstand und Beleg unverändert beachten. Fehlende und abgeschnittene Zeilen nicht ergänzen. Leere oder nicht verfügbare Fähigkeiten sind keine bestätigte Aussage über das Spiel.".into(),
            input_schema:json!({"type":"object","additionalProperties":false,"required":["hero"],"properties":{
                "hero":{"type":"integer","minimum":1,"maximum":i64::MAX},
                "patch":{"type":"string","minLength":1,"maxLength":128},
                "sections":{"type":"array","minItems":1,"maxItems":3,"items":{"type":"string","enum":["stats","abilities","weapon"]}}}})},
        timer_tools::definition(),
    ].into_iter().chain(crate::pg_asset_tools::definitions()).collect()
}

fn hero_sql(request: &GetHeroRequest) -> &'static str {
    let sections = request.sections.as_deref().unwrap_or(&[
        HeroSection::Stats,
        HeroSection::Abilities,
        HeroSection::Weapon,
    ]);
    let mask = sections.iter().fold(0, |mask, section| {
        mask | match section {
            HeroSection::Stats => 1,
            HeroSection::Abilities => 2,
            HeroSection::Weapon => 4,
        }
    });
    match mask {
        1=>"SELECT jsonb_build_object('identity',identity,'stats',stats) FROM serve.hero_profile WHERE entity_id=$1 LIMIT 1",
        2=>"SELECT jsonb_build_object('identity',identity,'abilities',abilities) FROM serve.hero_profile WHERE entity_id=$1 LIMIT 1",
        3=>"SELECT jsonb_build_object('identity',identity,'stats',stats,'abilities',abilities) FROM serve.hero_profile WHERE entity_id=$1 LIMIT 1",
        4=>"SELECT jsonb_build_object('identity',identity,'weapon',weapon) FROM serve.hero_profile WHERE entity_id=$1 LIMIT 1",
        5=>"SELECT jsonb_build_object('identity',identity,'stats',stats,'weapon',weapon) FROM serve.hero_profile WHERE entity_id=$1 LIMIT 1",
        6=>"SELECT jsonb_build_object('identity',identity,'abilities',abilities,'weapon',weapon) FROM serve.hero_profile WHERE entity_id=$1 LIMIT 1",
        _=>"SELECT jsonb_build_object('identity',identity,'stats',stats,'abilities',abilities,'weapon',weapon) FROM serve.hero_profile WHERE entity_id=$1 LIMIT 1",
    }
}

fn error_execution(id: &str, name: ToolName, code: &str, message: &str) -> ToolExecution {
    ToolExecution {
        result: ToolResult {
            call_id: id.into(),
            name,
            result: json!({"error":code,"message":message}),
            evidence_ids: Vec::new(),
            is_error: true,
        },
        dependencies: Vec::new(),
        usage: Usage::default(),
    }
}

fn citations(value: &Value, ids: &mut BTreeMap<i64, String>) -> Result<(), PortError> {
    match value {
        Value::Object(object) => {
            if let Some(citation) = object.get("citation") {
                let id = citation
                    .as_str()
                    .and_then(|id| id.parse::<i64>().ok())
                    .filter(|id| *id > 0)
                    .ok_or_else(invalid)?;
                let revision = object
                    .get("source_revision")
                    .and_then(Value::as_str)
                    .filter(|revision| !revision.is_empty())
                    .ok_or_else(invalid)?;
                if object.get("patch") != Some(&Value::Null)
                    || (object.contains_key("stat_key")
                        && !object.get("condition").is_some_and(Value::is_object))
                    || ids
                        .insert(id, revision.to_owned())
                        .is_some_and(|previous| previous != revision)
                {
                    return Err(invalid());
                }
            }
            for key in [
                "identity",
                "stats",
                "weapon",
                "abilities",
                "facts",
                "links",
                "entities",
                "qualifiers",
            ] {
                if let Some(field) = object.get(key) {
                    citations(field, ids)?;
                }
            }
        }
        Value::Array(values) => {
            for field in values {
                citations(field, ids)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn trim_row(value: &mut Value) -> bool {
    let Some(object) = value.as_object_mut() else {
        return false;
    };
    for key in ["weapon", "abilities", "stats"] {
        if object.get_mut(key).is_some_and(trim_row) {
            return true;
        }
    }
    for key in ["facts", "links", "entities"] {
        let Some(rows) = object.get_mut(key).and_then(Value::as_array_mut) else {
            continue;
        };
        if rows.is_empty() {
            continue;
        }
        rows.pop();
        let omitted = object
            .get("omitted_count")
            .and_then(Value::as_u64)
            .unwrap_or(0);
        object.insert("omitted_count".into(), json!(omitted.saturating_add(1)));
        object.insert("truncated".into(), json!(true));
        return true;
    }
    false
}

fn replace_citations(
    value: &mut Value,
    receipts: &BTreeMap<i64, Evidence>,
) -> Result<(), PortError> {
    match value {
        Value::Object(object) => {
            if let Some(citation) = object.get_mut("citation") {
                let receipt = citation
                    .as_str()
                    .and_then(|id| id.parse::<i64>().ok())
                    .and_then(|id| receipts.get(&id))
                    .ok_or_else(invalid)?;
                *citation = json!(receipt.evidence_id);
            }
            for key in [
                "identity",
                "stats",
                "weapon",
                "abilities",
                "facts",
                "links",
                "entities",
                "qualifiers",
            ] {
                if let Some(field) = object.get_mut(key) {
                    replace_citations(field, receipts)?;
                }
            }
        }
        Value::Array(values) => {
            for field in values {
                replace_citations(field, receipts)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn fits(result: &ToolResult, count: usize) -> Result<bool, PortError> {
    let mut worst = result.clone();
    worst.call_id = "\"".repeat(512);
    Ok(count <= MAX_EVIDENCE
        && serde_json::to_vec(&worst).map_err(|_| invalid())?.len() < MAX_BYTES)
}

pub(super) async fn bounded_execution(
    connection: &mut PgConnection,
    id: &str,
    request: &ToolRequest,
    mut value: Value,
) -> Result<ToolExecution, PortError> {
    let mut receipts = BTreeMap::new();
    loop {
        let mut ids = BTreeMap::new();
        citations(&value, &mut ids)?;
        if ids.is_empty() {
            return Err(invalid());
        }
        if ids.len() <= MAX_EVIDENCE {
            let missing: Vec<_> = ids
                .keys()
                .filter(|id| !receipts.contains_key(*id))
                .copied()
                .collect();
            if !missing.is_empty() {
                let rows = sqlx::query(RECEIPTS)
                    .bind(&missing)
                    .fetch_all(&mut *connection)
                    .await
                    .map_err(database_error)?;
                if rows.len() != missing.len() {
                    return Err(invalid());
                }
                for row in rows {
                    let receipt = receipt_metadata(row)?;
                    if ids.get(&receipt.raw_receipt_id) != Some(&receipt.source_revision) {
                        return Err(invalid());
                    }
                    receipts.insert(receipt.raw_receipt_id, format_receipt(&receipt)?);
                }
            }
            let evidence = ids
                .keys()
                .map(|id| receipts.get(id).cloned().ok_or_else(invalid))
                .collect::<Result<Vec<_>, _>>()?;
            let mut result = ToolResult {
                call_id: id.into(),
                name: request.name(),
                result: value.clone(),
                evidence_ids: evidence
                    .iter()
                    .map(|item| item.evidence_id.clone())
                    .collect(),
                is_error: false,
            };
            replace_citations(&mut result.result, &receipts)?;
            if fits(&result, evidence.len())? {
                let dependency = ToolEvidenceDependency {
                    request: request.clone(),
                    game_context: None,
                    evidence,
                }
                .with_canonical_result(&result)?;
                return Ok(ToolExecution {
                    result,
                    dependencies: vec![dependency],
                    usage: Usage::default(),
                });
            }
        }
        let trimmed = if crate::pg_asset_tools::canonical(request.name()) {
            crate::pg_asset_tools::trim_execution(&mut value, request)?
        } else {
            trim_row(&mut value)
        };
        if !trimmed {
            return Ok(error_execution(
                id,
                request.name(),
                "result_too_large",
                "Der vollständige Spielbeleg überschreitet die Ausgabegrenze.",
            ));
        }
    }
}

fn receipt_metadata(row: sqlx::postgres::PgRow) -> Result<CanonicalReceipt, PortError> {
    Ok(CanonicalReceipt {
        raw_receipt_id: row.try_get("id").map_err(database_error)?,
        raw_object_id: row.try_get("raw_object_id").map_err(database_error)?,
        ingest_run_id: row.try_get("ingest_run_id").map_err(database_error)?,
        source: row.try_get("source").map_err(database_error)?,
        source_revision: row.try_get("source_revision").map_err(database_error)?,
        path: row.try_get("path").map_err(database_error)?,
        locator_kind: row.try_get("locator_kind").map_err(database_error)?,
        locator: row.try_get("locator").map_err(database_error)?,
        value_sha256: row.try_get("value_sha256").map_err(database_error)?,
        extractor: row.try_get("extractor").map_err(database_error)?,
        patch: None,
    })
}

#[cfg(test)]
#[path = "pg_asset_executor_tests.rs"]
mod asset_tests;

#[cfg(test)]
#[path = "pg_tools_tests.rs"]
mod tests;
