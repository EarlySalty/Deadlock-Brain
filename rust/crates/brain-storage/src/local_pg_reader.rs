//! Native synchronous PostgreSQL adapter backed by one hard-bounded shared pool.
//! Unix sockets only: remote TLS/credential provisioning is deliberately not implicit.
use crate::memory_repository::validate_release;
use brain_contracts::{
    store::{ConversationOwnershipPort, STORE_VERSION},
    CorpusRelease, CorpusSnapshot, PortError, SnapshotReadPort,
};
use postgres::Row;
mod connection;
use connection::Client;
#[cfg(test)]
mod review_deadline;
#[cfg(test)]
mod review_startup;
mod wait_queue;
use std::{
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    sync::{Arc, Condvar, Mutex, MutexGuard},
    time::{Duration, Instant},
};
use wait_queue::WaitQueue;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct LocalPgPoolStats {
    pub max_connections: u32,
    pub open_connections: u32,
    pub connecting_connections: u32,
    pub idle_connections: u32,
    pub checked_out_connections: u32,
    pub peak_connections: u32,
    pub created_connections: u64,
    pub reused_checkouts: u64,
    pub wait_count: u64,
    pub wait_timeout_count: u64,
    pub wait_total_micros: u64,
    pub wait_max_micros: u64,
}

#[derive(Clone)]
struct ConnectionConfig {
    socket: PathBuf,
    port: u16,
    user: String,
    database: String,
    password: Option<String>,
    connect_timeout: Duration,
    statement_timeout: Duration,
    lock_timeout: Duration,
}

#[derive(Default)]
struct PoolState {
    idle: Vec<Client>,
    waiting: WaitQueue,
    open: u32,
    connecting: u32,
    checked_out: u32,
    peak: u32,
    created: u64,
    reused: u64,
    wait_count: u64,
    wait_timeout_count: u64,
    wait_total_micros: u64,
    wait_max_micros: u64,
}

struct ClientPool {
    config: ConnectionConfig,
    max_connections: u32,
    acquire_timeout: Duration,
    state: Mutex<PoolState>,
}

struct PooledClient {
    client: Option<Client>,
    pool: Arc<ClientPool>,
}

impl PooledClient {
    fn prepare(
        &mut self,
        deadline: Option<&brain_contracts::RequestDeadline>,
    ) -> Result<(), PortError> {
        self.client
            .as_mut()
            .expect("pooled PostgreSQL client missing before drop")
            .set_deadline(deadline)
    }
}

fn request_check(deadline: Option<&brain_contracts::RequestDeadline>) -> Result<(), PortError> {
    if let Some(deadline) = deadline {
        deadline.check()?;
    }
    Ok(())
}

impl Deref for PooledClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        self.client
            .as_ref()
            .expect("pooled PostgreSQL client missing before drop")
    }
}

impl DerefMut for PooledClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.client
            .as_mut()
            .expect("pooled PostgreSQL client missing before drop")
    }
}

impl Drop for PooledClient {
    fn drop(&mut self) {
        let Some(client) = self.client.take() else {
            return;
        };
        let closed = client.is_closed();
        let mut state = self
            .pool
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        state.checked_out = state.checked_out.saturating_sub(1);
        if closed {
            state.open = state.open.saturating_sub(1);
        } else {
            state.idle.push(client);
        }
        state.waiting.notify_front();
    }
}

fn error(_: postgres::Error) -> PortError {
    PortError::Unavailable("local PostgreSQL operation failed".into())
}

fn invalid(message: &str) -> PortError {
    PortError::InvalidResponse(message.into())
}

fn unavailable(message: &str) -> PortError {
    PortError::Unavailable(message.into())
}

fn valid_timeout(timeout: Duration) -> bool {
    (Duration::from_millis(1)..=Duration::from_secs(60)).contains(&timeout)
}

fn micros(duration: Duration) -> u64 {
    duration.as_micros().min(u128::from(u64::MAX)) as u64
}

impl ClientPool {
    fn new(
        config: ConnectionConfig,
        max_connections: u32,
        acquire_timeout: Duration,
    ) -> Result<Self, PortError> {
        if !(1..=64).contains(&max_connections) || !valid_timeout(acquire_timeout) {
            return Err(invalid("invalid local PostgreSQL pool options"));
        }
        Ok(Self {
            config,
            max_connections,
            acquire_timeout,
            state: Mutex::new(PoolState::default()),
        })
    }

    fn lock(&self) -> Result<MutexGuard<'_, PoolState>, PortError> {
        self.state
            .lock()
            .map_err(|_| unavailable("local PostgreSQL pool state unavailable"))
    }

    fn record_wait(state: &mut PoolState, elapsed: Duration) {
        let elapsed = micros(elapsed);
        state.wait_total_micros = state.wait_total_micros.saturating_add(elapsed);
        state.wait_max_micros = state.wait_max_micros.max(elapsed);
    }

    fn connect(
        &self,
        deadline: Option<&brain_contracts::RequestDeadline>,
    ) -> Result<Client, PortError> {
        Client::connect(&self.config, deadline)
    }

    fn acquire(self: &Arc<Self>) -> Result<PooledClient, PortError> {
        self.acquire_until(None)
    }

    fn acquire_until(
        self: &Arc<Self>,
        deadline: Option<&brain_contracts::RequestDeadline>,
    ) -> Result<PooledClient, PortError> {
        let started = Instant::now();
        request_check(deadline)?;
        let mut waiter: Option<Arc<Condvar>> = None;
        let mut state = self.lock()?;
        loop {
            if request_check(deadline).is_err() {
                state.waiting.remove(waiter.as_ref());
                if waiter.is_some() {
                    state.wait_timeout_count = state.wait_timeout_count.saturating_add(1);
                    Self::record_wait(&mut state, started.elapsed());
                }
                state.waiting.notify_front();
                return Err(PortError::BudgetExceeded);
            }
            // Once queued, preserve the original deadline even across notifications.
            if waiter.is_some() && started.elapsed() >= self.acquire_timeout {
                state.waiting.remove(waiter.as_ref());
                state.wait_timeout_count = state.wait_timeout_count.saturating_add(1);
                Self::record_wait(&mut state, started.elapsed());
                state.waiting.notify_front();
                return Err(unavailable("local PostgreSQL pool exhausted"));
            }

            // A returned connection belongs to the oldest waiter. New callers must
            // not steal it while the notified thread is waiting to reacquire the lock.
            if state.waiting.may_acquire(waiter.as_ref()) {
                if let Some(client) = state.idle.pop() {
                    state.waiting.remove(waiter.as_ref());
                    state.checked_out += 1;
                    state.reused = state.reused.saturating_add(1);
                    if waiter.is_some() {
                        Self::record_wait(&mut state, started.elapsed());
                    }
                    state.waiting.notify_front();
                    drop(state);
                    let mut client = PooledClient {
                        client: Some(client),
                        pool: self.clone(),
                    };
                    client.prepare(deadline)?;
                    return Ok(client);
                }

                if state.open + state.connecting < self.max_connections {
                    // Reserve capacity before waking the next waiter; parallel
                    // connection attempts remain inside the same hard pool limit.
                    state.connecting += 1;
                    state.waiting.remove(waiter.as_ref());
                    state.waiting.notify_front();
                    drop(state);
                    let connected = self.connect(deadline);
                    let mut state = self.lock()?;
                    state.connecting = state.connecting.saturating_sub(1);
                    match connected {
                        Ok(client) => {
                            state.open += 1;
                            state.checked_out += 1;
                            state.created = state.created.saturating_add(1);
                            state.peak = state.peak.max(state.open);
                            if waiter.is_some() {
                                Self::record_wait(&mut state, started.elapsed());
                            }
                            state.waiting.notify_front();
                            drop(state);
                            let mut client = PooledClient {
                                client: Some(client),
                                pool: self.clone(),
                            };
                            client.prepare(deadline)?;
                            return Ok(client);
                        }
                        Err(error) => {
                            state.waiting.notify_front();
                            return Err(error);
                        }
                    }
                }
            }

            if waiter.is_none() {
                state.wait_count = state.wait_count.saturating_add(1);
                waiter = Some(state.waiting.push());
            }
            let mut remaining = self.acquire_timeout.saturating_sub(started.elapsed());
            if let Some(deadline) = deadline {
                // Observe HTTP cancellation promptly without a new thread or runtime.
                remaining = remaining
                    .min(deadline.remaining().unwrap_or(Duration::ZERO))
                    .min(Duration::from_millis(10));
            }
            let signal = waiter.as_ref().expect("queued PostgreSQL waiter missing");
            let (next, _) = signal
                .wait_timeout(state, remaining)
                .map_err(|_| unavailable("local PostgreSQL pool state unavailable"))?;
            // Keep this guard rather than dropping and competing for the mutex again.
            state = next;
        }
    }

    fn stats(&self) -> LocalPgPoolStats {
        let state = self
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        LocalPgPoolStats {
            max_connections: self.max_connections,
            open_connections: state.open,
            connecting_connections: state.connecting,
            idle_connections: state.idle.len().min(u32::MAX as usize) as u32,
            checked_out_connections: state.checked_out,
            peak_connections: state.peak,
            created_connections: state.created,
            reused_checkouts: state.reused,
            wait_count: state.wait_count,
            wait_timeout_count: state.wait_timeout_count,
            wait_total_micros: state.wait_total_micros,
            wait_max_micros: state.wait_max_micros,
        }
    }
}

#[derive(Clone)]
pub struct LocalPgReader {
    pool: Arc<ClientPool>,
}

impl std::fmt::Debug for LocalPgReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalPgReader")
            .field("transport", &"local Unix socket")
            .field("max_connections", &self.pool.max_connections)
            .finish_non_exhaustive()
    }
}

impl LocalPgReader {
    pub fn new(
        socket: impl AsRef<Path>,
        port: u16,
        user: &str,
        database: &str,
    ) -> Result<Self, PortError> {
        if !socket.as_ref().is_absolute()
            || port == 0
            || [user, database]
                .iter()
                .any(|v| v.trim().is_empty() || v.len() > 128 || v.chars().any(char::is_control))
        {
            return Err(invalid("invalid local PostgreSQL configuration"));
        }
        let config = ConnectionConfig {
            socket: socket.as_ref().into(),
            port,
            user: user.into(),
            database: database.into(),
            password: None,
            connect_timeout: Duration::from_secs(2),
            statement_timeout: Duration::from_secs(2),
            lock_timeout: Duration::from_secs(1),
        };
        Ok(Self {
            pool: Arc::new(ClientPool::new(config, 4, Duration::from_secs(2))?),
        })
    }

    /// Compatibility configuration for non-service callers. Service composition should use
    /// with_pool_options so the hard connection cap and acquire wait are explicit.
    pub fn with_connection_options(
        self,
        password: Option<String>,
        connect_timeout: Duration,
        statement_timeout: Duration,
        lock_timeout: Duration,
    ) -> Result<Self, PortError> {
        self.with_pool_options(
            password,
            connect_timeout,
            statement_timeout,
            lock_timeout,
            4,
            connect_timeout,
        )
    }

    pub fn with_pool_options(
        self,
        password: Option<String>,
        connect_timeout: Duration,
        statement_timeout: Duration,
        lock_timeout: Duration,
        max_connections: u32,
        acquire_timeout: Duration,
    ) -> Result<Self, PortError> {
        if [
            connect_timeout,
            statement_timeout,
            lock_timeout,
            acquire_timeout,
        ]
        .iter()
        .any(|timeout| !valid_timeout(*timeout))
            || lock_timeout > statement_timeout
            || !(1..=64).contains(&max_connections)
            || password
                .as_ref()
                .is_some_and(|value| value.is_empty() || value.contains('\0'))
        {
            return Err(invalid("invalid local PostgreSQL connection options"));
        }
        let mut config = self.pool.config.clone();
        config.password = password;
        config.connect_timeout = connect_timeout;
        config.statement_timeout = statement_timeout;
        config.lock_timeout = lock_timeout;
        Ok(Self {
            pool: Arc::new(ClientPool::new(config, max_connections, acquire_timeout)?),
        })
    }

    pub fn pool_stats(&self) -> LocalPgPoolStats {
        self.pool.stats()
    }

    /// SELECT-only service startup preflight using the same bounded runtime pool.
    pub fn check_core_schema(&self) -> Result<(), PortError> {
        let mut client = self.pool.acquire()?;
        let mut tx = client.transaction(true, false)?;
        let rows = tx
            .query_schema("SELECT schema_version, store_contract FROM brain.core_schema_version")?;
        if rows.len() != 1 {
            return Err(invalid("unsupported core schema/store version"));
        }
        let schema_version: i32 = rows[0].try_get(0).map_err(error)?;
        let store_version: String = rows[0].try_get(1).map_err(error)?;
        if schema_version != crate::schema::CORE_SCHEMA_VERSION || store_version != STORE_VERSION {
            return Err(invalid("unsupported core schema/store version"));
        }
        tx.query_schema(crate::schema::SHAPE_PROBE)?;
        tx.commit()
    }

    pub fn check_permissions(&self) -> Result<(), PortError> {
        let mut client = self.pool.acquire()?;
        let row = client
            .query_one(
                "SELECT has_table_privilege(current_user, 'brain.conversation_owners_v1', 'SELECT')
                    AND has_table_privilege(current_user, 'brain.conversation_owners_v1', 'INSERT')",
                &[],
            )?;
        let allowed: bool = row.try_get(0).map_err(error)?;
        if allowed {
            Ok(())
        } else {
            Err(PortError::PermissionDenied(
                "required database permissions missing".into(),
            ))
        }
    }

    fn release_row(row: Row) -> Result<CorpusRelease, PortError> {
        serde_json::from_value(row.try_get(0).map_err(error)?)
            .map_err(|_| invalid("invalid release JSON"))
    }
}

impl SnapshotReadPort for LocalPgReader {
    fn read_entity_evidence(
        &self,
        query: &brain_contracts::Query,
        context: &brain_contracts::AuthorizedContext,
        patch_date: Option<&str>,
        provider: bool,
        purpose: brain_contracts::store::AnswerPurpose,
    ) -> Result<Option<Vec<brain_contracts::Evidence>>, PortError> {
        use brain_contracts::{
            entity_profile::{EntityIdentity, EntityProfileFact, PatchValidity},
            SourceRecordV2,
        };
        let deadline = context.request_deadline.as_ref();
        request_check(deadline)?;
        let mut client = self.pool.acquire_until(deadline)?;
        let mut tx = client.transaction(true, true)?;
        let available: bool = tx.query_one("SELECT to_regclass('brain.entity_profile_entities_v1') IS NOT NULL AND to_regclass('brain.entity_profile_facts_v1') IS NOT NULL", &[])?.try_get(0).map_err(error)?;
        if !available {
            return Ok(None);
        }
        let release = Self::release_row(
            tx.query_opt(
                "SELECT release_json FROM brain.corpus_releases_v1 WHERE release_id=$1",
                &[&context.knowledge_release],
            )?
            .ok_or_else(|| invalid("Release fehlt"))?,
        )?;
        validate_release(&release)?;
        let words = brain_contracts::lexical::terms(&query.text);
        let hero_count = (words.contains(&"held".into())
            || words.contains(&"helden".into())
            || words.contains(&"hero".into())
            || words.contains(&"heroes".into())
            || words.contains(&"heldzahl".into()))
            && (query.text.to_lowercase().contains("wie viele")
                || words.contains(&"anzahl".into())
                || words.contains(&"how".into())
                || query.text.to_lowercase().contains("heldzahl"));
        if hero_count {
            if patch_date.is_some()
                || query.patch.is_some()
                || purpose == brain_contracts::store::AnswerPurpose::ExternalPublication
            {
                return Ok(Some(Vec::new()));
            }
            let pins = serde_json::to_value(&release.source_revisions)
                .map_err(|_| invalid("Release-Pins sind ungültig"))?;
            let rows = tx.query("SELECT DISTINCT e.entity_key,r.record_json,h.record_json FROM brain.entity_profile_entities_v1 e JOIN brain.entity_profile_facts_v1 f USING(entity_key) JOIN brain.source_record_revisions r USING(source_id,logical_id,revision) JOIN brain.source_record_heads h USING(source_id,logical_id) WHERE e.identity_json->>'kind'='hero' AND ($1::jsonb->f.source_id->>f.logical_id)::bigint=f.revision ORDER BY e.entity_key LIMIT 1001", &[&pins])?;
            if rows.len() > 1000 {
                return Err(PortError::BudgetExceeded);
            }
            let mut heroes = std::collections::BTreeSet::new();
            for row in rows {
                request_check(deadline)?;
                let record: SourceRecordV2 = serde_json::from_value(row.try_get(1).map_err(error)?)
                    .map_err(|_| invalid("Heldenquelle ist ungültig"))?;
                let head: SourceRecordV2 =
                    serde_json::from_value(row.try_get(2).map_err(error)?)
                        .map_err(|_| invalid("Aktuelle Heldenquelle ist ungültig"))?;
                if entity_source_visible(&release, record, head, context, provider, purpose)?
                    .is_some()
                {
                    heroes.insert(row.try_get::<_, String>(0).map_err(error)?);
                }
            }
            if heroes.is_empty() {
                return Ok(Some(Vec::new()));
            }
            let canonical: Option<i64> = if tx
                .query_one("SELECT to_regclass('brain.entities') IS NOT NULL", &[])?
                .try_get::<_, bool>(0)
                .map_err(error)?
            {
                Some(
                    tx.query_one(
                        "SELECT count(*) FROM brain.entities WHERE entity_type='hero'",
                        &[],
                    )?
                    .try_get(0)
                    .map_err(error)?,
                )
            } else {
                None
            };
            let coverage = if canonical == Some(heroes.len() as i64) {
                "Die belegte Liste deckt den gespeicherten Heldenkatalog vollständig ab."
            } else {
                "Die vollständige Abdeckung des Heldenkatalogs ist nicht belegt."
            };
            let result = brain_contracts::Evidence {
                evidence_id: "entity-profile:hero-count".into(),
                source_id: "brain.entity_profile_entities_v1".into(),
                logical_id: "hero-count".into(),
                revision: 1,
                kind: brain_contracts::EvidenceKind::Prose,
                content: format!("Im freigegebenen DB-Quellenstand sind {} Helden belegt. {coverage} Patchaktualität ist damit nicht belegt.", heroes.len()),
                citation: format!("brain.entity_profile_entities_v1:{}", release.release_id),
                visibility: brain_contracts::SourceVisibility::Internal,
                allowed_scopes: context.principal.scopes.clone(),
                score: 1.0,
                patch: None,
                provenance: None,
            };
            tx.commit()?;
            return Ok(Some(vec![result]));
        }
        let identities = tx.query("SELECT identity_json FROM brain.entity_profile_entities_v1 e WHERE position(lower(e.identity_json->>'name') in lower($1))>0 OR EXISTS(SELECT 1 FROM jsonb_array_elements_text(e.identity_json->'aliases') a WHERE a.value<>'' AND position(lower(a.value) in lower($1))>0) ORDER BY entity_key LIMIT 3", &[&query.text])?;
        let mut matched = Vec::new();
        for row in identities {
            let identity: EntityIdentity =
                serde_json::from_value(row.try_get(0).map_err(error)?)
                    .map_err(|_| invalid("Entitätsidentität ist ungültig"))?;
            if std::iter::once(&identity.name)
                .chain(identity.aliases.iter())
                .any(|name| {
                    let name = brain_contracts::lexical::terms(name);
                    !name.is_empty() && words.windows(name.len()).any(|part| part == name)
                })
            {
                matched.push(identity);
            }
        }
        if matched.len() != 1 {
            return Ok(if matched.is_empty() {
                None
            } else {
                Some(Vec::new())
            });
        }
        let entity = &matched[0];
        let pins = serde_json::to_value(&release.source_revisions)
            .map_err(|_| invalid("Release-Pins sind ungültig"))?;
        let rows = tx.query("WITH selected AS (SELECT f.* FROM brain.entity_profile_facts_v1 f WHERE f.entity_key=$1 AND ($2::jsonb->f.source_id->>f.logical_id)::bigint=f.revision ORDER BY (SELECT count(*) FROM unnest($3::text[]) word WHERE length(word)>2 AND position(word in lower((f.fact_json::jsonb)->>'predicate'))>0) DESC,f.source_id,f.logical_id,f.revision,f.fact_id LIMIT 101) SELECT array_agg(f.fact_json ORDER BY f.fact_id),r.record_json,h.record_json FROM selected f JOIN brain.source_record_revisions r USING(source_id,logical_id,revision) JOIN brain.source_record_heads h USING(source_id,logical_id) GROUP BY f.source_id,f.logical_id,f.revision,r.record_json,h.record_json ORDER BY f.source_id,f.logical_id,f.revision", &[&entity.entity_key, &pins, &words])?;
        let mut evidence = Vec::new();
        let mut authorized_entity = None;
        let mut authorized_names = std::collections::BTreeSet::new();
        for row in rows {
            request_check(deadline)?;
            let values: Vec<String> = row.try_get(0).map_err(error)?;
            let record: SourceRecordV2 = serde_json::from_value(row.try_get(1).map_err(error)?)
                .map_err(|_| invalid("Faktenquelle ist ungültig"))?;
            let head: SourceRecordV2 = serde_json::from_value(row.try_get(2).map_err(error)?)
                .map_err(|_| invalid("Aktuelle Faktenquelle ist ungültig"))?;
            if release
                .source_revisions
                .get(&record.source_id)
                .and_then(|pins| pins.get(&record.logical_id))
                != Some(&record.revision)
            {
                continue;
            }
            let Some(record) =
                entity_source_visible(&release, record, head, context, provider, purpose)?
            else {
                continue;
            };
            authorized_entity = Some((record.visibility, record.allowed_scopes.clone()));
            let facts: Vec<EntityProfileFact> = values
                .iter()
                .map(|encoded| {
                    serde_json::from_str(encoded).map_err(|_| invalid("Faktenbeleg ist ungültig"))
                })
                .collect::<Result<_, _>>()?;
            let ids: Vec<_> = facts.iter().map(|fact| fact.fact_id.clone()).collect();
            let originals = crate::entity_profile::project_entity_facts(&record, &ids)
                .map_err(|_| invalid("Originalfakten können nicht geprüft werden"))?;
            if originals != facts {
                return Err(invalid(
                    "Fakten widersprechen dem gespeicherten Originalbeleg",
                ));
            }
            for fact in facts {
                request_check(deadline)?;
                let name = fact
                    .subject
                    .strip_prefix("hero:")
                    .or_else(|| fact.subject.strip_prefix("ability:"))
                    .or_else(|| fact.subject.strip_prefix("item:"));
                if let Some(name) = name.filter(|name| !name.trim().is_empty()) {
                    authorized_names.insert(name.to_owned());
                }
                if patch_date.is_some() {
                    continue;
                }
                let requested = query.patch.as_deref();
                let known = match &fact.validity {
                    PatchValidity::Known {
                        from_patch,
                        to_patch_exclusive,
                        ..
                    } => {
                        let patch = requested.unwrap_or(&release.patch);
                        from_patch.as_str() <= patch
                            && to_patch_exclusive.as_deref().is_none_or(|end| patch < end)
                    }
                    PatchValidity::Unknown { .. } => false,
                };
                if requested.is_some() && !known {
                    continue;
                }
                let mut metadata = std::collections::BTreeMap::new();
                metadata.insert("name".into(), name.unwrap_or(&fact.subject).to_owned());
                metadata.insert("entity_key".into(), entity.entity_key.clone());
                metadata.insert("fact_key".into(), fact.predicate.clone());
                let content = format!("entity: {}\n{}: {}\nEinheit: {}\nQualifier: {}\nQuellenstand: {}\nPatchgültigkeit: {}", name.unwrap_or(&fact.subject), fact.predicate, fact.value, fact.unit.as_deref().unwrap_or("unbekannt"), serde_json::to_string(&fact.qualifiers).map_err(|_| invalid("Qualifier sind ungültig"))?, fact.provenance.observed_at, if known { "belegt" } else { "unbekannt; kein belegter aktueller Patchwert" });
                evidence.push(brain_contracts::Evidence {
                    evidence_id: format!(
                        "entity-profile:{}:{}:{}:{}:{}",
                        entity.entity_key,
                        record.source_id,
                        record.logical_id,
                        record.revision,
                        fact.fact_id
                    ),
                    source_id: record.source_id.clone(),
                    logical_id: record.logical_id.clone(),
                    revision: record.revision,
                    kind: if known && fact.evidence_status == "extracted_value" {
                        brain_contracts::EvidenceKind::Fact
                    } else {
                        brain_contracts::EvidenceKind::Prose
                    },
                    content,
                    citation: fact.provenance.origin.locator.clone(),
                    visibility: record.visibility,
                    allowed_scopes: record.allowed_scopes.clone(),
                    score: 1.0,
                    patch: requested.filter(|_| known).map(str::to_owned),
                    provenance: Some(brain_contracts::ChunkProvenance {
                        document: brain_contracts::DocumentRevision {
                            source_id: record.source_id.clone(),
                            logical_id: record.logical_id.clone(),
                            revision: record.revision,
                            content_hash: record.content_hash.clone(),
                        },
                        chunker_version: "entity-profile-db-v1".into(),
                        ordinal: 0,
                        byte_start: 0,
                        byte_end: record.content.len(),
                        source_locator: fact.provenance.origin.locator.clone(),
                        release_id: release.release_id.clone(),
                        knowledge_version: release.knowledge_version.clone(),
                        valid_from: None,
                        valid_to: None,
                        metadata,
                    }),
                });
            }
        }
        if let (Some(date), Some((visibility, scopes))) = (patch_date, authorized_entity) {
            let names: Vec<_> = authorized_names.into_iter().collect();
            if names.is_empty() {
                return Ok(Some(Vec::new()));
            }
            let kind = match entity.kind {
                brain_contracts::entity_profile::EntityKind::Hero => "hero",
                brain_contracts::entity_profile::EntityKind::Ability => "ability",
                brain_contracts::entity_profile::EntityKind::Item => "item",
            };
            let dates = tx.query("SELECT DISTINCT patch_date::text FROM brain.patch_changes WHERE patch_date::text LIKE $1 AND ((entity_type=$3 AND lower(entity_name)=ANY(SELECT lower(n) FROM unnest($2::text[]) n)) OR ($3='ability' AND entity_type='hero' AND lower(ability_name)=ANY(SELECT lower(n) FROM unnest($2::text[]) n))) ORDER BY 1 LIMIT 2", &[&date, &names, &kind])?;
            if dates.len() != 1 {
                return Ok(Some(Vec::new()));
            }
            let date: String = dates[0].try_get(0).map_err(error)?;
            let changes = tx.query("SELECT jsonb_build_object('patch_date',patch_date,'entity_type',entity_type,'entity_name',entity_name,'ability_name',ability_name,'stat_name',stat_name,'old_value',old_value,'new_value',new_value,'change_type',change_type,'confidence',confidence) FROM brain.patch_changes WHERE patch_date::text=$1 AND ((entity_type=$3 AND lower(entity_name)=ANY(SELECT lower(n) FROM unnest($2::text[]) n)) OR ($3='ability' AND entity_type='hero' AND lower(ability_name)=ANY(SELECT lower(n) FROM unnest($2::text[]) n))) ORDER BY stat_name,ability_name,old_value,new_value LIMIT 101", &[&date, &names, &kind])?;
            if changes.len() > 100 {
                return Err(PortError::BudgetExceeded);
            }
            for (index, row) in changes.into_iter().enumerate() {
                request_check(deadline)?;
                let change: serde_json::Value = row.try_get(0).map_err(error)?;
                evidence.push(brain_contracts::Evidence {
                    evidence_id: format!(
                        "entity-profile:patch:{}:{date}:{index}",
                        entity.entity_key
                    ),
                    source_id: "brain.patch_changes".into(),
                    logical_id: entity.entity_key.clone(),
                    revision: 1,
                    kind: brain_contracts::EvidenceKind::Prose,
                    content: format!("Gespeicherte Patchänderung: {change}"),
                    citation: format!("brain.patch_changes:{date}:{}", entity.entity_key),
                    visibility,
                    allowed_scopes: scopes.clone(),
                    score: 1.0,
                    provenance: None,
                    patch: Some(date.clone()),
                });
            }
        }
        request_check(deadline)?;
        tx.commit()?;
        Ok(Some(evidence))
    }

    fn read_heads(
        &self,
        documents: &[brain_contracts::DocumentRevision],
    ) -> Result<Vec<brain_contracts::DocumentHead>, PortError> {
        self.read_heads_bounded(documents, None)
    }
    fn read_snapshot(&self, release_id: &str) -> Result<CorpusSnapshot, PortError> {
        self.read_snapshot_bounded(release_id, None)
    }
    fn read_heads_until(
        &self,
        documents: &[brain_contracts::DocumentRevision],
        deadline: Option<&brain_contracts::RequestDeadline>,
    ) -> Result<Vec<brain_contracts::DocumentHead>, PortError> {
        request_check(deadline)?;
        let result = self.read_heads_bounded(documents, deadline);
        request_check(deadline)?;
        result
    }
    fn read_snapshot_until(
        &self,
        release_id: &str,
        deadline: Option<&brain_contracts::RequestDeadline>,
    ) -> Result<CorpusSnapshot, PortError> {
        request_check(deadline)?;
        let result = self.read_snapshot_bounded(release_id, deadline);
        request_check(deadline)?;
        result
    }
}
fn entity_source_visible(
    release: &brain_contracts::CorpusRelease,
    record: brain_contracts::SourceRecordV2,
    head: brain_contracts::SourceRecordV2,
    context: &brain_contracts::AuthorizedContext,
    provider: bool,
    purpose: brain_contracts::store::AnswerPurpose,
) -> Result<Option<brain_contracts::SourceRecordV2>, PortError> {
    let mut view_release = release.clone();
    view_release.source_revisions = std::collections::BTreeMap::from([(
        record.source_id.clone(),
        std::collections::BTreeMap::from([(record.logical_id.clone(), record.revision)]),
    )]);
    let view = CorpusSnapshot {
        release: view_release,
        revisions: vec![record],
        heads: vec![head],
    };
    let visible = if purpose == brain_contracts::store::AnswerPurpose::ExternalPublication {
        view.authorized_for_publication(&context.principal)?
    } else {
        view.authorized(&context.principal, provider)?
    };
    Ok(visible.into_iter().find(|record| {
        brain_contracts::store::record_allowed(record, &context.principal, provider)
    }))
}

impl LocalPgReader {
    fn current_maintenance_policies(
        client: &mut Client,
        sources: &[String],
    ) -> Result<
        std::collections::BTreeMap<String, Option<brain_contracts::source::SourcePolicy>>,
        PortError,
    > {
        if sources.is_empty() {
            return Ok(Default::default());
        }
        let available: bool = client
            .query_one(
                "SELECT to_regclass('brain.maintenance_sources_v1') IS NOT NULL",
                &[],
            )?
            .try_get(0)
            .map_err(error)?;
        if !available {
            return Ok(Default::default());
        }
        let rows=client.query("SELECT registration_json FROM brain.maintenance_sources_v1 WHERE registration_json->>'source_id'=ANY($1)",&[&sources])?;
        let mut policies = std::collections::BTreeMap::new();
        for row in rows {
            let value: serde_json::Value = row.try_get(0).map_err(error)?;
            let registration: brain_contracts::maintenance::MaintenanceSourceRegistration =
                serde_json::from_value(value).map_err(|_| invalid("maintenance registry JSON"))?;
            registration.validate()?;
            if policies
                .insert(registration.source_id, registration.policy)
                .is_some()
            {
                return Err(invalid("duplicate maintenance source identity"));
            }
        }
        Ok(policies)
    }

    fn restrict_maintenance_head(
        head: &mut brain_contracts::DocumentHead,
        policies: &std::collections::BTreeMap<
            String,
            Option<brain_contracts::source::SourcePolicy>,
        >,
    ) -> Result<(), PortError> {
        if !head.source_id.starts_with("maintenance-docs:") {
            return Ok(());
        }
        use brain_contracts::source::{OriginArtifact, Versioned, ORIGIN_METADATA_KEY};
        let encoded = head
            .metadata
            .get(ORIGIN_METADATA_KEY)
            .ok_or_else(|| invalid("maintenance origin missing"))?;
        let mut versioned: Versioned<OriginArtifact> =
            serde_json::from_str(encoded).map_err(|_| invalid("maintenance origin JSON"))?;
        if policies.get(&head.source_id).and_then(Option::as_ref) == Some(&versioned.data.policy) {
            return Ok(());
        }
        // Ein widerrufenes Dokument verschwindet vor Retrieval und vor Cacheausgabe.
        head.visibility = brain_contracts::SourceVisibility::Private;
        head.allowed_scopes = std::collections::BTreeSet::from(["maintenance.revoked".into()]);
        head.metadata.insert("egress".into(), "none".into());
        versioned.data.policy.visibility = head.visibility;
        versioned.data.policy.allowed_scopes = head.allowed_scopes.clone();
        versioned.data.policy.provider_egress_allowed = false;
        versioned.data.policy.publication_allowed = false;
        head.metadata.insert(
            ORIGIN_METADATA_KEY.into(),
            serde_json::to_string(&versioned).map_err(|_| invalid("maintenance denial JSON"))?,
        );
        Ok(())
    }

    fn read_heads_bounded(
        &self,
        documents: &[brain_contracts::DocumentRevision],
        deadline: Option<&brain_contracts::RequestDeadline>,
    ) -> Result<Vec<brain_contracts::DocumentHead>, PortError> {
        if documents.len() > 256 {
            return Err(invalid("head batch too large"));
        }
        if documents.is_empty() {
            return Ok(Vec::new());
        }
        let keys = serde_json::to_value(documents).map_err(|_| invalid("invalid head keys"))?;
        let mut client = self.pool.acquire_until(deadline)?;
        request_check(deadline)?;
        // A single statement snapshot, bounded key lookup; never fetch record bodies or release pins.
        let rows = client
            .query(
                "SELECT jsonb_build_object('source_id',h.source_id,'logical_id',h.logical_id,'revision',h.revision,'visibility',h.record_json->'visibility','allowed_scopes',h.record_json->'allowed_scopes','tombstone',h.record_json->'tombstone','metadata',h.record_json->'metadata') FROM jsonb_to_recordset($1::jsonb) AS k(source_id text, logical_id text) JOIN brain.source_record_heads h ON h.source_id=k.source_id AND h.logical_id=k.logical_id ORDER BY h.source_id,h.logical_id",
                &[&keys],
            )?;
        let sources: Vec<_> = documents
            .iter()
            .filter(|d| d.source_id.starts_with("maintenance-docs:"))
            .map(|d| d.source_id.clone())
            .collect();
        let policies = Self::current_maintenance_policies(&mut client, &sources)?;
        rows.iter()
            .map(|row| {
                let mut head: brain_contracts::DocumentHead =
                    serde_json::from_value(row.try_get(0).map_err(error)?)
                        .map_err(|_| invalid("invalid current head JSON"))?;
                head.validate()?;
                Self::restrict_maintenance_head(&mut head, &policies)?;
                Ok(head)
            })
            .collect()
    }

    fn read_snapshot_bounded(
        &self,
        release_id: &str,
        deadline: Option<&brain_contracts::RequestDeadline>,
    ) -> Result<CorpusSnapshot, PortError> {
        let mut client = self.pool.acquire_until(deadline)?;
        request_check(deadline)?;
        let mut tx = client.transaction(true, true)?;
        request_check(deadline)?;
        let row = tx
            .query_opt(
                "SELECT release_json FROM brain.corpus_releases_v1 WHERE release_id=$1",
                &[&release_id],
            )?
            .ok_or_else(|| invalid("unknown release"))?;
        let release = Self::release_row(row)?;
        validate_release(&release)?;
        if release.release_id != release_id {
            return Err(invalid("release identity mismatch"));
        }
        let pins = serde_json::to_value(&release.source_revisions)
            .map_err(|_| invalid("invalid release pins"))?;
        request_check(deadline)?;
        let rows=tx.query("SELECT r.record_json,h.record_json FROM jsonb_each($1::jsonb) s CROSS JOIN LATERAL jsonb_each_text(s.value) d JOIN brain.source_record_revisions r ON r.source_id=s.key AND r.logical_id=d.key AND r.revision=d.value::bigint JOIN brain.source_record_heads h ON h.source_id=r.source_id AND h.logical_id=r.logical_id ORDER BY r.source_id,r.logical_id",&[&pins])?;
        let mut revisions = Vec::new();
        let mut heads = Vec::new();
        for row in rows {
            request_check(deadline)?;
            revisions.push(
                serde_json::from_value(row.try_get(0).map_err(error)?)
                    .map_err(|_| invalid("invalid revision JSON"))?,
            );
            heads.push(
                serde_json::from_value(row.try_get(1).map_err(error)?)
                    .map_err(|_| invalid("invalid head JSON"))?,
            );
        }
        let sources: Vec<_> = release
            .source_revisions
            .keys()
            .filter(|s| s.starts_with("maintenance-docs:"))
            .cloned()
            .collect();
        let policies = Self::current_maintenance_policies(&mut tx, &sources)?;
        for head in &mut heads {
            let mut projected = brain_contracts::DocumentHead::from(&*head);
            Self::restrict_maintenance_head(&mut projected, &policies)?;
            head.visibility = projected.visibility;
            head.allowed_scopes = projected.allowed_scopes;
            head.metadata = projected.metadata;
        }
        if revisions.len()
            != release
                .source_revisions
                .values()
                .map(std::collections::BTreeMap::len)
                .sum::<usize>()
        {
            return Err(invalid("incomplete release snapshot"));
        }
        request_check(deadline)?;
        tx.commit()?;
        request_check(deadline)?;
        Ok(CorpusSnapshot {
            release,
            revisions,
            heads,
        })
    }
}

impl ConversationOwnershipPort for LocalPgReader {
    fn claim_conversation(&self, conversation: &str, actor: &str) -> Result<(), PortError> {
        self.claim_conversation_bounded(conversation, actor, None)
    }
    fn claim_conversation_until(
        &self,
        conversation: &str,
        actor: &str,
        deadline: &brain_contracts::RequestDeadline,
    ) -> Result<(), PortError> {
        deadline.check()?;
        let result = self.claim_conversation_bounded(conversation, actor, Some(deadline));
        deadline.check()?;
        result
    }
}
impl LocalPgReader {
    fn claim_conversation_bounded(
        &self,
        conversation: &str,
        actor: &str,
        deadline: Option<&brain_contracts::RequestDeadline>,
    ) -> Result<(), PortError> {
        if [conversation, actor]
            .iter()
            .any(|v| v.trim().is_empty() || v.len() > 512 || v.chars().any(char::is_control))
        {
            return Err(invalid("invalid conversation identity"));
        }
        let mut client = self.pool.acquire_until(deadline)?;
        request_check(deadline)?;
        let mut tx = client.transaction(false, false)?;
        request_check(deadline)?;
        tx.execute("INSERT INTO brain.conversation_owners_v1(conversation_id,actor_id) VALUES($1,$2) ON CONFLICT(conversation_id) DO NOTHING",&[&conversation,&actor])?;
        request_check(deadline)?;
        let owner: String = tx
            .query_one(
                "SELECT actor_id FROM brain.conversation_owners_v1 WHERE conversation_id=$1",
                &[&conversation],
            )?
            .try_get(0)
            .map_err(error)?;
        if owner != actor {
            return Err(invalid("conversation owner mismatch"));
        }
        request_check(deadline)?;
        tx.commit()?;
        request_check(deadline)
    }
}
