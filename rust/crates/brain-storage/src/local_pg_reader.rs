//! Native synchronous PostgreSQL read/auth adapter for bounded blocking workers.
//! Unix sockets only: remote TLS/credential provisioning is deliberately not implicit.
use crate::memory_repository::validate_release;
use brain_contracts::{
    store::ConversationOwnershipPort, CorpusRelease, CorpusSnapshot, PortError, SnapshotReadPort,
};
use postgres::{Config, IsolationLevel, NoTls};
use std::{
    path::{Path, PathBuf},
    time::Duration,
};
#[derive(Clone)]
pub struct LocalPgReader {
    socket: PathBuf,
    port: u16,
    user: String,
    database: String,
}
impl std::fmt::Debug for LocalPgReader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalPgReader")
            .field("transport", &"local Unix socket")
            .finish_non_exhaustive()
    }
}
fn error(_: postgres::Error) -> PortError {
    PortError::Unavailable("local PostgreSQL operation failed".into())
}
fn invalid(message: &str) -> PortError {
    PortError::InvalidResponse(message.into())
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
        Ok(Self {
            socket: socket.as_ref().into(),
            port,
            user: user.into(),
            database: database.into(),
        })
    }
    fn connect(&self) -> Result<postgres::Client, PortError> {
        let mut config = Config::new();
        config
            .host_path(&self.socket)
            .port(self.port)
            .user(&self.user)
            .dbname(&self.database)
            .connect_timeout(Duration::from_secs(2));
        let mut client = config.connect(NoTls).map_err(error)?;
        client
            .batch_execute("SET statement_timeout='2000ms'; SET lock_timeout='1000ms'")
            .map_err(error)?;
        Ok(client)
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
        let mut client = self.connect()?;
        // A single statement snapshot, bounded key lookup; never fetch record bodies or release pins.
        let rows = client.query(
            "SELECT jsonb_build_object('source_id',h.source_id,'logical_id',h.logical_id,'revision',h.revision,'visibility',h.record_json->'visibility','allowed_scopes',h.record_json->'allowed_scopes','tombstone',h.record_json->'tombstone','metadata',h.record_json->'metadata') FROM jsonb_to_recordset($1::jsonb) AS k(source_id text, logical_id text) JOIN brain.source_record_heads h ON h.source_id=k.source_id AND h.logical_id=k.logical_id ORDER BY h.source_id,h.logical_id",
            &[&keys],
        ).map_err(error)?;
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
        let mut client = self.connect()?;
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
        let release: CorpusRelease = serde_json::from_value(row.try_get(0).map_err(error)?)
            .map_err(|_| invalid("invalid release JSON"))?;
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
        let mut client = self.connect()?;
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
