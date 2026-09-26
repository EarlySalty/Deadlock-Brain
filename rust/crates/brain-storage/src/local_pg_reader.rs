//! Native synchronous PostgreSQL adapter backed by one hard-bounded shared pool.
//! Unix sockets only: remote TLS/credential provisioning is deliberately not implicit.
use crate::memory_repository::validate_release;
use brain_contracts::{
    store::{ConversationOwnershipPort, STORE_VERSION},
    CorpusRelease, CorpusSnapshot, PortError, SnapshotReadPort,
};
use postgres::{Client, Config, IsolationLevel, NoTls, Row};
use std::{
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    sync::{Arc, Condvar, Mutex, MutexGuard},
    time::{Duration, Instant},
};

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
    available: Condvar,
}

struct PooledClient {
    client: Option<Client>,
    pool: Arc<ClientPool>,
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
        drop(state);
        self.pool.available.notify_one();
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
            available: Condvar::new(),
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

    fn connect(&self) -> Result<Client, PortError> {
        let mut config = Config::new();
        config
            .host_path(&self.config.socket)
            .port(self.config.port)
            .user(&self.config.user)
            .dbname(&self.config.database)
            .application_name("deadlock-brain-reader")
            .connect_timeout(self.config.connect_timeout);
        if let Some(password) = &self.config.password {
            config.password(password);
        }
        let mut client = config.connect(NoTls).map_err(error)?;
        client
            .batch_execute(&format!(
                "SET statement_timeout='{}ms'; SET lock_timeout='{}ms'",
                self.config.statement_timeout.as_millis(),
                self.config.lock_timeout.as_millis()
            ))
            .map_err(error)?;
        Ok(client)
    }

    fn acquire(self: &Arc<Self>) -> Result<PooledClient, PortError> {
        let started = Instant::now();
        let mut waited = false;
        loop {
            let mut state = self.lock()?;
            if let Some(client) = state.idle.pop() {
                state.checked_out += 1;
                state.reused = state.reused.saturating_add(1);
                if waited {
                    Self::record_wait(&mut state, started.elapsed());
                }
                return Ok(PooledClient {
                    client: Some(client),
                    pool: self.clone(),
                });
            }

            if state.open + state.connecting < self.max_connections {
                state.connecting += 1;
                drop(state);
                let connected = self.connect();
                let mut state = self.lock()?;
                state.connecting = state.connecting.saturating_sub(1);
                match connected {
                    Ok(client) => {
                        state.open += 1;
                        state.checked_out += 1;
                        state.created = state.created.saturating_add(1);
                        state.peak = state.peak.max(state.open);
                        if waited {
                            Self::record_wait(&mut state, started.elapsed());
                        }
                        return Ok(PooledClient {
                            client: Some(client),
                            pool: self.clone(),
                        });
                    }
                    Err(error) => {
                        drop(state);
                        self.available.notify_one();
                        return Err(error);
                    }
                }
            }

            if !waited {
                state.wait_count = state.wait_count.saturating_add(1);
                waited = true;
            }
            let remaining = self.acquire_timeout.saturating_sub(started.elapsed());
            if remaining.is_zero() {
                state.wait_timeout_count = state.wait_timeout_count.saturating_add(1);
                Self::record_wait(&mut state, started.elapsed());
                return Err(unavailable("local PostgreSQL pool exhausted"));
            }
            let (next, result) = self
                .available
                .wait_timeout(state, remaining)
                .map_err(|_| unavailable("local PostgreSQL pool state unavailable"))?;
            state = next;
            if result.timed_out()
                && state.idle.is_empty()
                && state.open + state.connecting >= self.max_connections
            {
                state.wait_timeout_count = state.wait_timeout_count.saturating_add(1);
                Self::record_wait(&mut state, started.elapsed());
                return Err(unavailable("local PostgreSQL pool exhausted"));
            }
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
        let mut tx = client
            .build_transaction()
            .read_only(true)
            .start()
            .map_err(error)?;
        let rows = tx
            .query(
                "SELECT schema_version, store_contract FROM brain.core_schema_version",
                &[],
            )
            .map_err(error)?;
        if rows.len() != 1 {
            return Err(invalid("unsupported core schema/store version"));
        }
        let schema_version: i32 = rows[0].try_get(0).map_err(error)?;
        let store_version: String = rows[0].try_get(1).map_err(error)?;
        if schema_version != crate::schema::CORE_SCHEMA_VERSION || store_version != STORE_VERSION {
            return Err(invalid("unsupported core schema/store version"));
        }
        tx.query(crate::schema::SHAPE_PROBE, &[]).map_err(error)?;
        tx.commit().map_err(error)
    }

    pub fn check_permissions(&self) -> Result<(), PortError> {
        let mut client = self.pool.acquire()?;
        let row = client
            .query_one(
                "SELECT has_table_privilege(current_user, 'brain.conversation_owners_v1', 'SELECT')
                    AND has_table_privilege(current_user, 'brain.conversation_owners_v1', 'INSERT')",
                &[],
            )
            .map_err(error)?;
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
    fn read_heads(
        &self,
        documents: &[brain_contracts::DocumentRevision],
    ) -> Result<Vec<brain_contracts::DocumentHead>, PortError> {
        if documents.len() > 256 {
            return Err(invalid("head batch too large"));
        }
        if documents.is_empty() {
            return Ok(Vec::new());
        }
        let keys = serde_json::to_value(documents).map_err(|_| invalid("invalid head keys"))?;
        let mut client = self.pool.acquire()?;
        // A single statement snapshot, bounded key lookup; never fetch record bodies or release pins.
        let rows = client
            .query(
                "SELECT jsonb_build_object('source_id',h.source_id,'logical_id',h.logical_id,'revision',h.revision,'visibility',h.record_json->'visibility','allowed_scopes',h.record_json->'allowed_scopes','tombstone',h.record_json->'tombstone','metadata',h.record_json->'metadata') FROM jsonb_to_recordset($1::jsonb) AS k(source_id text, logical_id text) JOIN brain.source_record_heads h ON h.source_id=k.source_id AND h.logical_id=k.logical_id ORDER BY h.source_id,h.logical_id",
                &[&keys],
            )
            .map_err(error)?;
        rows.iter()
            .map(|row| {
                let head: brain_contracts::DocumentHead =
                    serde_json::from_value(row.try_get(0).map_err(error)?)
                        .map_err(|_| invalid("invalid current head JSON"))?;
                head.validate()?;
                Ok(head)
            })
            .collect()
    }

    fn read_snapshot(&self, release_id: &str) -> Result<CorpusSnapshot, PortError> {
        let mut client = self.pool.acquire()?;
        let mut tx = client
            .build_transaction()
            .isolation_level(IsolationLevel::RepeatableRead)
            .read_only(true)
            .start()
            .map_err(error)?;
        let row = tx
            .query_opt(
                "SELECT release_json FROM brain.corpus_releases_v1 WHERE release_id=$1",
                &[&release_id],
            )
            .map_err(error)?
            .ok_or_else(|| invalid("unknown release"))?;
        let release = Self::release_row(row)?;
        validate_release(&release)?;
        if release.release_id != release_id {
            return Err(invalid("release identity mismatch"));
        }
        let pins = serde_json::to_value(&release.source_revisions)
            .map_err(|_| invalid("invalid release pins"))?;
        let rows=tx.query("SELECT r.record_json,h.record_json FROM jsonb_each($1::jsonb) s CROSS JOIN LATERAL jsonb_each_text(s.value) d JOIN brain.source_record_revisions r ON r.source_id=s.key AND r.logical_id=d.key AND r.revision=d.value::bigint JOIN brain.source_record_heads h ON h.source_id=r.source_id AND h.logical_id=r.logical_id ORDER BY r.source_id,r.logical_id",&[&pins]).map_err(error)?;
        let mut revisions = Vec::new();
        let mut heads = Vec::new();
        for row in rows {
            revisions.push(
                serde_json::from_value(row.try_get(0).map_err(error)?)
                    .map_err(|_| invalid("invalid revision JSON"))?,
            );
            heads.push(
                serde_json::from_value(row.try_get(1).map_err(error)?)
                    .map_err(|_| invalid("invalid head JSON"))?,
            );
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
        tx.commit().map_err(error)?;
        Ok(CorpusSnapshot {
            release,
            revisions,
            heads,
        })
    }
}

impl ConversationOwnershipPort for LocalPgReader {
    fn claim_conversation(&self, conversation: &str, actor: &str) -> Result<(), PortError> {
        if [conversation, actor]
            .iter()
            .any(|v| v.trim().is_empty() || v.len() > 512 || v.chars().any(char::is_control))
        {
            return Err(invalid("invalid conversation identity"));
        }
        let mut client = self.pool.acquire()?;
        let mut tx = client.transaction().map_err(error)?;
        tx.execute("INSERT INTO brain.conversation_owners_v1(conversation_id,actor_id) VALUES($1,$2) ON CONFLICT(conversation_id) DO NOTHING",&[&conversation,&actor]).map_err(error)?;
        let owner: String = tx
            .query_one(
                "SELECT actor_id FROM brain.conversation_owners_v1 WHERE conversation_id=$1",
                &[&conversation],
            )
            .map_err(error)?
            .try_get(0)
            .map_err(error)?;
        if owner != actor {
            return Err(invalid("conversation owner mismatch"));
        }
        tx.commit().map_err(error)?;
        Ok(())
    }
}
