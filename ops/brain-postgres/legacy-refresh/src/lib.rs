use postgres::{Client, GenericClient, IsolationLevel, NoTls, Transaction};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
mod schema;
pub use schema::SequenceFingerprint;
use schema::{create_structure, schema_shape};
use std::fs::File;
use std::io::{Read, Write};
use std::os::fd::{AsRawFd, FromRawFd};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub const EXCLUDED: [&str; 5] = [
    "patch_changes",
    "feeder_runs",
    "plan_items",
    "plan_items_verworfen",
    "plan_runs",
];
pub const ARCHIVE_LOCK: i64 = 0x425241494e4c4547;
const SETTINGS: &str = "SET LOCAL search_path = pg_catalog; SET LOCAL standard_conforming_strings = on; SET LOCAL TIME ZONE 'UTC'; SET LOCAL DateStyle = 'ISO, YMD'; SET LOCAL IntervalStyle = 'postgres'; SET LOCAL extra_float_digits = 1; SET LOCAL bytea_output = 'hex';";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Vertrag nicht erfüllt: {0}")]
    Contract(&'static str),
    #[error("PostgreSQL-Aufruf fehlgeschlagen")]
    Database(#[from] postgres::Error),
    #[error("Commitausgang unbekannt; Archiv-OIDs und Aktivierungsabsicht lesend prüfen, nicht erneut schreiben")]
    CommitUncertain,
    #[error("Dateizugriff fehlgeschlagen")]
    Io(#[from] std::io::Error),
    #[error("Manifestformat ungültig")]
    Json(#[from] serde_json::Error),
}
pub type Result<T> = std::result::Result<T, Error>;
fn require(ok: bool, message: &'static str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(Error::Contract(message))
    }
}
fn sha(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}
fn ident(name: &str) -> Result<String> {
    require(
        !name.is_empty()
            && name.len() <= 63
            && name
                .bytes()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_')
            && name.as_bytes()[0].is_ascii_lowercase(),
        "SQL-Bezeichner ungültig",
    )?;
    Ok(format!("\"{name}\""))
}
fn generation_id(id: &str) -> Result<()> {
    require(
        id.len() == 32
            && id
                .bytes()
                .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()),
        "Generation muss aus 32 kleinen Hexzeichen bestehen",
    )
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Endpoint {
    pub socket: PathBuf,
    pub port: u16,
    pub database: String,
    pub role: String,
    pub database_oid: u32,
    pub database_owner: String,
}
impl Endpoint {
    pub fn connect(&self) -> Result<Client> {
        require(
            self.socket.is_absolute() && self.port > 0,
            "Nur absolute lokale Socketpfade",
        )?;
        ident(&self.database)?;
        ident(&self.role)?;
        let mut config = postgres::Config::new();
        config
            .host_path(&self.socket)
            .port(self.port)
            .dbname(&self.database)
            .user(&self.role);
        Ok(config.connect(NoTls)?)
    }
    fn matches<G: GenericClient>(&self, client: &mut G) -> Result<()> {
        let row = client.query_one("SELECT current_database(), current_user, d.oid, pg_get_userbyid(d.datdba), inet_server_addr() IS NULL FROM pg_database d WHERE d.datname=current_database()", &[])?;
        let transport = client.query_one("SELECT current_setting('port')::integer, current_setting('unix_socket_directories'), r.rolsuper FROM pg_roles r WHERE r.rolname=current_user", &[])?;
        require(
            transport.get::<_, i32>(0) == i32::from(self.port)
                && transport
                    .get::<_, String>(1)
                    .split(',')
                    .map(str::trim)
                    .any(|p| Path::new(p) == self.socket),
            "Socket oder Serverport weicht ab",
        )?;
        if self.database == "brain" {
            require(
                !transport.get::<_, bool>(2),
                "Produktivziel darf keine Superuserverbindung sein",
            )?;
        }
        require(
            row.get::<_, String>(0) == self.database
                && row.get::<_, String>(1) == self.role
                && row.get::<_, u32>(2) == self.database_oid
                && row.get::<_, String>(3) == self.database_owner
                && row.get::<_, bool>(4),
            "Datenbank, Rolle, Owner oder Transport weicht ab",
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Plan {
    pub generation: String,
    pub snapshot_label: String,
    pub source: Endpoint,
    pub source_schema_oid: u32,
    pub target: Endpoint,
    pub archive_schema_oid: u32,
    pub archive_owner: String,
    #[serde(default)]
    pub added_tables: Vec<String>,
}
impl Plan {
    pub fn generation_schema(&self) -> String {
        format!("brain_legacy_g{}", self.generation)
    }
    pub fn predecessor_schema(&self) -> String {
        format!("brain_legacy_p{}", self.generation)
    }
    pub fn validate(&self) -> Result<()> {
        require(
            unsafe { libc::geteuid() } != 0,
            "Archivmechanik darf nicht als root laufen",
        )?;
        generation_id(&self.generation)?;
        ident(&self.archive_owner)?;
        for name in &self.added_tables {
            ident(name)?;
            require(
                !EXCLUDED.contains(&name.as_str()),
                "Ausgeschlossene Tabelle darf nicht ergänzt werden",
            )?;
        }
        require(
            self.added_tables.windows(2).all(|w| w[0] < w[1]),
            "Ergänzte Tabellen müssen eindeutig sortiert sein",
        )?;
        require(
            !self.snapshot_label.is_empty()
                && self.snapshot_label.len() <= 128
                && self
                    .snapshot_label
                    .bytes()
                    .all(|c| c.is_ascii_alphanumeric() || b"_.:-".contains(&c)),
            "Snapshotlabel ungültig",
        )?;
        require(
            self.source.database_oid > 0
                && self.target.database_oid > 0
                && self.source_schema_oid > 0
                && self.archive_schema_oid > 0,
            "Tatsächliche OIDs müssen gebunden sein",
        )?;
        require(
            self.source.socket != self.target.socket
                || self.source.port != self.target.port
                || self.source.database != self.target.database,
            "Quelle und Ziel dürfen nicht identisch sein",
        )?;
        let prod_source = self.source.socket == Path::new("/var/run/postgresql")
            && self.source.port == 5432
            && self.source.database == "deadlock"
            && self.source.role == "nathanael";
        let prod_target = self.target.socket == Path::new("/run/deadlock-brain-postgresql")
            && self.target.port == 5446
            && self.target.database == "brain"
            && self.target.role == "brain_migrate"
            && self.archive_owner == "brain_migrate"
            && unsafe { libc::geteuid() } == 1000;
        let isolated = self.source.socket.starts_with("/tmp")
            && self.target.socket.starts_with("/tmp")
            && self.source.port != 5432
            && self.target.port != 5446
            && self.source.database.starts_with("refresh_test_")
            && self.target.database.starts_with("refresh_test_");
        require(
            (prod_source && prod_target) || isolated,
            "Unzulässige Quell-/Zielbindung",
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct TableFingerprint {
    pub name: String,
    pub oid: u32,
    pub rows: u64,
    pub rows_sha256: String,
    pub definition: String,
    pub owner: String,
    pub acl: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SchemaFingerprint {
    pub oid: u32,
    pub owner: String,
    pub acl: String,
    pub schema_sha256: String,
    pub tables: Vec<TableFingerprint>,
    pub sequences: Vec<SequenceFingerprint>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub version: u32,
    pub plan: Plan,
    pub exported_snapshot: String,
    pub exclusions: Vec<String>,
    pub source: SchemaFingerprint,
    pub staged: Option<SchemaFingerprint>,
    pub dump_sha256: String,
    pub schema_dump_sha256: String,
    pub archive_sha256: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActivationReceipt {
    pub generation: String,
    pub snapshot_label: String,
    pub archive_sha256: String,
    pub dump_sha256: String,
    pub database_oid: u32,
    pub new_archive_schema_oid: u32,
    pub predecessor_schema: String,
    pub predecessor_schema_oid: u32,
    pub schema_sha256: String,
    pub tables: Vec<TableFingerprint>,
    pub schema_dump_sha256: String,
    pub source_database_oid: u32,
    pub source_schema_oid: u32,
    pub source_tables: Vec<TableFingerprint>,
    pub source_sequences: Vec<SequenceFingerprint>,
    pub added_tables: Vec<String>,
    pub sequences: Vec<SequenceFingerprint>,
    pub q_binding_required: bool,
    pub g5_complete: bool,
}

pub struct PrivateDirectory {
    directory: File,
}
impl PrivateDirectory {
    pub fn open(path: &Path) -> Result<Self> {
        require(
            path.is_absolute()
                && path.components().all(|c| {
                    matches!(
                        c,
                        std::path::Component::RootDir | std::path::Component::Normal(_)
                    )
                }),
            "Artefaktpfad muss absolut und normalisiert sein",
        )?;
        let mut anchor = File::open("/")?;
        for component in path.components() {
            if let std::path::Component::Normal(name) = component {
                use std::os::unix::ffi::OsStrExt;
                let name = std::ffi::CString::new(name.as_bytes())
                    .map_err(|_| Error::Contract("Pfad enthält NUL"))?;
                let fd = unsafe {
                    libc::openat(
                        anchor.as_raw_fd(),
                        name.as_ptr(),
                        libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
                    )
                };
                if fd < 0 {
                    return Err(std::io::Error::last_os_error().into());
                }
                let next = unsafe { File::from_raw_fd(fd) };
                let metadata = next.metadata()?;
                let uid = unsafe { libc::geteuid() };
                require(
                    metadata.uid() == 0 || metadata.uid() == uid,
                    "Fremder Pfadbesitzer",
                )?;
                require(
                    metadata.mode() & 0o022 == 0
                        || (metadata.uid() == 0 && metadata.mode() & 0o1000 != 0),
                    "Schreibbarer Pfadvorfahr ohne Sticky-Schutz",
                )?;
                anchor = next;
            }
        }
        let m = anchor.metadata()?;
        require(
            m.uid() == unsafe { libc::geteuid() } && m.mode() & 0o077 == 0,
            "Artefaktverzeichnis muss privat und selbst besessen sein",
        )?;
        if unsafe { libc::flock(anchor.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        Ok(Self { directory: anchor })
    }
    fn open_file(&self, name: &str, create: bool) -> Result<File> {
        ident(name.split('.').next().unwrap_or_default())?;
        require(
            !name.contains('/')
                && matches!(
                    name,
                    "archive.dump"
                        | "schema.dump"
                        | "manifest.json"
                        | "receipt.json"
                        | "activation_intent.json"
                ),
            "Fremder Artefaktname",
        )?;
        let name = std::ffi::CString::new(name).unwrap();
        let flags = libc::O_NOFOLLOW
            | libc::O_CLOEXEC
            | if create {
                libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL
            } else {
                libc::O_RDONLY
            };
        let fd = unsafe { libc::openat(self.directory.as_raw_fd(), name.as_ptr(), flags, 0o600) };
        if fd < 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        let file = unsafe { File::from_raw_fd(fd) };
        let m = file.metadata()?;
        require(
            m.is_file()
                && m.uid() == unsafe { libc::geteuid() }
                && m.mode() & 0o077 == 0
                && m.nlink() == 1,
            "Unsichere Artefaktdatei",
        )?;
        Ok(file)
    }
    pub fn write_json<T: Serialize>(&self, name: &str, value: &T) -> Result<()> {
        let mut file = self.open_file(name, true)?;
        file.write_all(&serde_json::to_vec_pretty(value)?)?;
        file.sync_all()?;
        self.directory.sync_all()?;
        Ok(())
    }
    pub fn read_manifest(&self) -> Result<Manifest> {
        let m: Manifest = serde_json::from_reader(
            self.open_file("manifest.json", false)?
                .take(4 * 1024 * 1024),
        )?;
        m.plan.validate()?;
        require(
            m.version == 2 && m.exclusions == EXCLUDED && m.archive_sha256 == archive_digest(&m)?,
            "Manifestbindung ungültig",
        )?;
        require(
            m.dump_sha256 == self.file_sha("archive.dump")?
                && m.schema_dump_sha256 == self.file_sha("schema.dump")?,
            "Dumphashe weichen ab",
        )?;
        Ok(m)
    }
    fn file_sha(&self, name: &str) -> Result<String> {
        hash_reader(self.open_file(name, false)?)
    }
}
fn hash_reader(mut reader: impl Read) -> Result<String> {
    let mut hash = Sha256::new();
    let mut bytes = [0; 65536];
    loop {
        let n = reader.read(&mut bytes)?;
        if n == 0 {
            break;
        }
        hash.update(&bytes[..n]);
    }
    Ok(hex::encode(hash.finalize()))
}
fn archive_digest(m: &Manifest) -> Result<String> {
    Ok(sha(&serde_json::to_vec(&(
        m.version,
        &m.plan,
        &m.exported_snapshot,
        &m.exclusions,
        &m.source,
        &m.staged,
        &m.dump_sha256,
        &m.schema_dump_sha256,
    ))?))
}

fn schema_identity<G: GenericClient>(
    client: &mut G,
    schema: &str,
) -> Result<(u32, String, String)> {
    let row = client.query_opt("SELECT oid, pg_get_userbyid(nspowner), coalesce((SELECT json_agg(json_build_array(a.grantee,a.grantor,a.privilege_type,a.is_grantable) ORDER BY a.grantee,a.grantor,a.privilege_type)::text FROM aclexplode(coalesce(nspacl,acldefault('n',nspowner))) a),'[]') FROM pg_namespace WHERE nspname=$1", &[&schema])?.ok_or(Error::Contract("Schema fehlt"))?;
    Ok((row.get(0), row.get(1), row.get(2)))
}
fn schema_absent<G: GenericClient>(client: &mut G, schema: &str) -> Result<()> {
    require(
        client
            .query_opt("SELECT oid FROM pg_namespace WHERE nspname=$1", &[&schema])?
            .is_none(),
        "Generation oder Vorgänger existiert bereits",
    )
}

fn fingerprint(
    client: &mut Transaction<'_>,
    schema: &str,
    excluded: &[String],
) -> Result<SchemaFingerprint> {
    let quoted = ident(schema)?;
    let (oid, owner, acl) = schema_identity(client, schema)?;
    let unsupported: i64 = client.query_one("SELECT (SELECT count(*) FROM pg_class WHERE relnamespace=$1 AND relname <> ALL($2) AND relkind NOT IN ('r','i','S')) + (SELECT count(*) FROM pg_proc WHERE pronamespace=$1) + (SELECT count(*) FROM pg_type WHERE typnamespace=$1 AND typtype NOT IN ('c') AND NOT (typelem <> 0 AND typlen=-1)) + (SELECT count(*) FROM pg_trigger t JOIN pg_class c ON c.oid=t.tgrelid WHERE c.relnamespace=$1 AND NOT t.tgisinternal) + (SELECT count(*) FROM pg_policy p JOIN pg_class c ON c.oid=p.polrelid WHERE c.relnamespace=$1) + (SELECT count(*) FROM pg_extension WHERE extnamespace=$1) + (SELECT count(*) FROM pg_default_acl WHERE defaclnamespace=$1)", &[&oid, &excluded])?.get(0);
    require(
        unsupported == 0,
        "Nicht unterstützte Schemaobjekte, Regeln oder Standardrechte",
    )?;
    let (shapes, sequences) = schema_shape(client, schema, oid, excluded)?;
    let mut tables = Vec::new();
    for row in client.query("SELECT c.oid, c.relname, pg_get_userbyid(c.relowner), coalesce((SELECT json_agg(json_build_array(a.grantee,a.grantor,a.privilege_type,a.is_grantable) ORDER BY a.grantee,a.grantor,a.privilege_type)::text FROM aclexplode(coalesce(c.relacl,acldefault('r',c.relowner))) a),'[]'), c.relrowsecurity OR c.relforcerowsecurity OR c.relispartition OR c.relhassubclass OR c.relhasrules, c.relpersistence FROM pg_class c WHERE c.relnamespace=$1 AND c.relkind='r' AND c.relname <> ALL($2) ORDER BY c.relname COLLATE \"C\"", &[&oid, &excluded])? {
        let table_oid: u32 = row.get(0);
        let name: String = row.get(1);
        let table = ident(&name)?;
        require(!row.get::<_, bool>(4) && row.get::<_, i8>(5) == b'p' as i8, "Partition, Vererbung, RLS, Regel oder temporäre Tabelle")?;
        let definition = shapes.iter().find(|(id, _, _)| *id == table_oid).ok_or(Error::Contract("Tabellenstruktur fehlt"))?.2.clone();
        let count: i64 = client.query_one(&format!("SELECT count(*) FROM ONLY {quoted}.{table}"), &[])?.get(0);
        let query = format!("COPY (SELECT t::text FROM ONLY {quoted}.{table} t ORDER BY t::text COLLATE \"C\") TO STDOUT (FORMAT binary)");
        let digest = hash_reader(client.copy_out(&query)?)?;
        tables.push(TableFingerprint { name, oid: table_oid, rows: count as u64, rows_sha256: digest, definition, owner: row.get(2), acl: row.get(3) });
    }
    require(!tables.is_empty(), "Leeres Archiv")?;
    let schema_sha256 = sha(&serde_json::to_vec(&(
        tables
            .iter()
            .map(|t| (&t.name, &t.definition))
            .collect::<Vec<_>>(),
        sequences
            .iter()
            .map(sequence_shape)
            .collect::<Result<Vec<_>>>()?,
    ))?);
    Ok(SchemaFingerprint {
        oid,
        owner,
        acl,
        schema_sha256,
        tables,
        sequences,
    })
}
fn sequence_shape(s: &SequenceFingerprint) -> Result<String> {
    Ok(serde_json::to_string(&(
        &s.name,
        &s.table,
        &s.column,
        s.identity,
        &s.data_type,
        s.start,
        s.increment,
        s.minimum,
        s.maximum,
        s.cache,
        s.cycle,
    ))?)
}
fn equal_content(a: &SchemaFingerprint, b: &SchemaFingerprint) -> Result<()> {
    require(
        a.schema_sha256 == b.schema_sha256 && a.tables.len() == b.tables.len(),
        "Schemastruktur oder Tabellenzahl weicht ab",
    )?;
    for (a, b) in a.tables.iter().zip(&b.tables) {
        require(
            a.name == b.name
                && a.rows == b.rows
                && a.rows_sha256 == b.rows_sha256
                && a.definition == b.definition,
            "Tabelleninhalt, Zeilenanzahl oder Struktur weicht ab",
        )?;
    }
    require(
        a.sequences.len() == b.sequences.len(),
        "Sequenzzahl weicht ab",
    )?;
    for (a, b) in a.sequences.iter().zip(&b.sequences) {
        require(
            sequence_shape(a)? == sequence_shape(b)? && a.value == b.value && a.called == b.called,
            "Sequenzstruktur oder Stand weicht ab",
        )?;
    }
    Ok(())
}
fn compatible_tables(old: &SchemaFingerprint, new: &SchemaFingerprint, plan: &Plan) -> Result<()> {
    let additions: Vec<String> = new
        .tables
        .iter()
        .filter(|t| !old.tables.iter().any(|o| o.name == t.name))
        .map(|t| t.name.clone())
        .collect();
    require(
        additions == plan.added_tables,
        "Neue Tabellen weichen von der ausdrücklichen Planbindung ab",
    )?;
    for table in &old.tables {
        require(
            new.tables
                .iter()
                .any(|t| t.name == table.name && t.definition == table.definition),
            "Bestehende Tabellenstruktur weicht ab",
        )?;
    }
    for seq in &old.sequences {
        let fresh = new
            .sequences
            .iter()
            .find(|s| s.name == seq.name)
            .ok_or(Error::Contract("Bestehende Sequenz fehlt"))?;
        require(
            sequence_shape(seq)? == sequence_shape(fresh)?,
            "Bestehende Sequenzstruktur weicht ab",
        )?;
    }
    require(
        new.sequences.iter().all(|s| {
            old.sequences.iter().any(|o| o.name == s.name) || additions.contains(&s.table)
        }),
        "Neue Sequenz gehört keiner ergänzten Tabelle",
    )
}
fn equal_permissions(a: &SchemaFingerprint, b: &SchemaFingerprint, plan: &Plan) -> Result<()> {
    compatible_tables(a, b, plan)?;
    require(
        a.owner == b.owner && a.acl == b.acl,
        "Schemarechte weichen vom bisherigen Archiv ab",
    )?;
    for old in &a.tables {
        let new = b
            .tables
            .iter()
            .find(|t| t.name == old.name)
            .ok_or(Error::Contract("Archivtabelle fehlt"))?;
        require(
            old.owner == new.owner && old.acl == new.acl,
            "Tabellenrechte weichen vom bisherigen Archiv ab",
        )?;
    }
    for old in &a.sequences {
        let new = b
            .sequences
            .iter()
            .find(|s| s.name == old.name)
            .ok_or(Error::Contract("Archivsequenz fehlt"))?;
        require(
            old.owner == new.owner && old.acl == new.acl,
            "Sequenzrechte weichen ab",
        )?;
    }
    Ok(())
}
fn new_permissions<G: GenericClient>(
    client: &mut G,
    old: &SchemaFingerprint,
    new: &SchemaFingerprint,
    plan: &Plan,
) -> Result<()> {
    let ids: Vec<u32> = new
        .tables
        .iter()
        .filter(|t| plan.added_tables.contains(&t.name))
        .map(|t| t.oid)
        .chain(
            new.sequences
                .iter()
                .filter(|s| !old.sequences.iter().any(|o| o.name == s.name))
                .map(|s| s.oid),
        )
        .collect();
    let bad: i64 = client.query_one("SELECT count(*) FROM pg_class c CROSS JOIN LATERAL aclexplode(coalesce(c.relacl,acldefault(CASE WHEN c.relkind='S' THEN 's'::\"char\" ELSE 'r'::\"char\" END,c.relowner))) a WHERE c.oid=ANY($1) AND (a.grantee<>c.relowner OR c.relowner<>$2::text::regrole::oid)", &[&ids, &plan.archive_owner])?.get(0);
    require(
        bad == 0,
        "Ergänzte Objekte dürfen keine Consumerrechte erhalten",
    )
}
fn read_only_permissions<G: GenericClient>(client: &mut G, schema_oid: u32) -> Result<()> {
    let row = client.query_one("SELECT (SELECT count(*) FROM pg_namespace n CROSS JOIN LATERAL aclexplode(coalesce(n.nspacl,acldefault('n',n.nspowner))) a WHERE n.oid=$1 AND a.grantee <> n.nspowner AND (a.privilege_type <> 'USAGE' OR a.is_grantable)) + (SELECT count(*) FROM pg_class c CROSS JOIN LATERAL aclexplode(coalesce(c.relacl,acldefault(CASE WHEN c.relkind='S' THEN 's'::\"char\" ELSE 'r'::\"char\" END,c.relowner))) a WHERE c.relnamespace=$1 AND c.relkind IN ('r','S') AND a.grantee <> c.relowner AND (a.privilege_type <> 'SELECT' OR a.is_grantable)) + (SELECT count(*) FROM pg_attribute a JOIN pg_class c ON c.oid=a.attrelid WHERE c.relnamespace=$1 AND a.attacl IS NOT NULL)", &[&schema_oid])?;
    require(
        row.get::<_, i64>(0) == 0,
        "Archiv besitzt fremde Schreib- oder Weitergaberechte",
    )
}
fn read_transaction(client: &mut Client) -> Result<Transaction<'_>> {
    let mut tx = client
        .build_transaction()
        .isolation_level(IsolationLevel::RepeatableRead)
        .read_only(true)
        .start()?;
    tx.batch_execute(SETTINGS)?;
    Ok(tx)
}

pub fn inspect_plan(plan: &Plan, source: &mut Client, target: &mut Client) -> Result<()> {
    plan.validate()?;
    let mut src = read_transaction(source)?;
    plan.source.matches(&mut src)?;
    require(
        schema_identity(&mut src, "brain")?.0 == plan.source_schema_oid,
        "Quellschema-OID weicht ab",
    )?;
    let source_fp = fingerprint(&mut src, "brain", &EXCLUDED.map(String::from))?;
    src.rollback()?;
    let mut dst = read_transaction(target)?;
    plan.target.matches(&mut dst)?;
    let old = fingerprint(&mut dst, "brain_legacy", &[])?;
    require(
        old.oid == plan.archive_schema_oid && old.owner == plan.archive_owner,
        "Archivbindung weicht ab",
    )?;
    compatible_tables(&old, &source_fp, plan)?;
    read_only_permissions(&mut dst, old.oid)?;
    schema_absent(&mut dst, &plan.generation_schema())?;
    schema_absent(&mut dst, &plan.predecessor_schema())?;
    dst.rollback()?;
    Ok(())
}

fn trusted_pg_dump() -> Result<PathBuf> {
    let path = PathBuf::from("/usr/lib/postgresql/16/bin/pg_dump");
    let mut cursor = path.as_path();
    loop {
        let m = std::fs::symlink_metadata(cursor)?;
        require(
            !m.file_type().is_symlink() && m.uid() == 0 && m.mode() & 0o022 == 0,
            "PostgreSQL-Werkzeugpfad ist nicht geschützt",
        )?;
        match cursor.parent() {
            Some(parent) if parent != cursor => cursor = parent,
            _ => break,
        }
    }
    Ok(path)
}
fn dump(endpoint: &Endpoint, snapshot: &str, output: File, schema_only: bool) -> Result<()> {
    require(
        unsafe { libc::geteuid() } != 0,
        "Werkzeug nicht als root starten",
    )?;
    let mut command = Command::new(trusted_pg_dump()?);
    command
        .env_clear()
        .env("LC_ALL", "C")
        .env("PGPASSFILE", "/dev/null")
        .args([
            "--no-password",
            "--format=custom",
            "--no-owner",
            "--no-acl",
            "--schema=brain",
        ])
        .arg("--host")
        .arg(&endpoint.socket)
        .arg("--port")
        .arg(endpoint.port.to_string())
        .arg("--dbname")
        .arg(&endpoint.database)
        .arg("--username")
        .arg(&endpoint.role)
        .arg("--snapshot")
        .arg(snapshot);
    for name in EXCLUDED {
        command.arg(format!("--exclude-table=brain.{name}"));
    }
    if schema_only {
        command.arg("--schema-only");
    }
    let durable = output.try_clone()?;
    require(
        command
            .stdin(Stdio::null())
            .stdout(output)
            .stderr(Stdio::null())
            .status()?
            .success(),
        "pg_dump fehlgeschlagen, keine Aktivierung",
    )?;
    durable.sync_all()?;
    Ok(())
}
pub fn capture(plan: &Plan, source: &mut Client, artifacts: &PrivateDirectory) -> Result<Manifest> {
    capture_inner(plan, source, artifacts, None)
}
pub fn capture_and_stage(
    plan: &Plan,
    source: &mut Client,
    target: &mut Client,
    artifacts: &PrivateDirectory,
) -> Result<Manifest> {
    capture_inner(plan, source, artifacts, Some(target))
}
fn capture_inner(
    plan: &Plan,
    source: &mut Client,
    artifacts: &PrivateDirectory,
    target: Option<&mut Client>,
) -> Result<Manifest> {
    plan.validate()?;
    let mut tx = read_transaction(source)?;
    plan.source.matches(&mut tx)?;
    require(
        schema_identity(&mut tx, "brain")?.0 == plan.source_schema_oid,
        "Quellschema-OID weicht ab",
    )?;
    let exported_snapshot: String = tx.query_one("SELECT pg_export_snapshot()", &[])?.get(0);
    require(
        exported_snapshot
            .bytes()
            .all(|c| c.is_ascii_hexdigit() || c == b'-'),
        "Snapshotkennung ungültig",
    )?;
    let source_fingerprint = fingerprint(&mut tx, "brain", &EXCLUDED.map(String::from))?;
    require(source_fingerprint.sequences.is_empty() || plan.source.database.starts_with("refresh_test_"), "Sequenzstände sind nicht MVCC-snapshotgebunden; produktive Aufnahme benötigt einen nachgewiesenen Stillstandsadapter")?;
    let names = source_fingerprint
        .tables
        .iter()
        .map(|t| ident(&t.name).map(|n| format!("\"brain\".{n}")))
        .collect::<Result<Vec<_>>>()?;
    tx.batch_execute(&format!(
        "LOCK TABLE {} IN ACCESS SHARE MODE",
        names.join(",")
    ))?;
    dump(
        &plan.source,
        &exported_snapshot,
        artifacts.open_file("archive.dump", true)?,
        false,
    )?;
    dump(
        &plan.source,
        &exported_snapshot,
        artifacts.open_file("schema.dump", true)?,
        true,
    )?;
    let after = fingerprint(&mut tx, "brain", &EXCLUDED.map(String::from))?;
    require(
        source_fingerprint == after,
        "Quellschema driftet während der Aufnahme",
    )?;
    let staged = if let Some(target) = target {
        Some(stage_from_snapshot(
            plan,
            &source_fingerprint,
            &mut tx,
            target,
        )?)
    } else {
        None
    };
    tx.rollback()?;
    let mut manifest = Manifest {
        version: 2,
        plan: plan.clone(),
        exported_snapshot,
        exclusions: EXCLUDED.map(String::from).to_vec(),
        source: source_fingerprint,
        staged,
        dump_sha256: artifacts.file_sha("archive.dump")?,
        schema_dump_sha256: artifacts.file_sha("schema.dump")?,
        archive_sha256: String::new(),
    };
    manifest.archive_sha256 = archive_digest(&manifest)?;
    artifacts.write_json("manifest.json", &manifest)?;
    Ok(manifest)
}

fn stage_from_snapshot(
    plan: &Plan,
    source_fingerprint: &SchemaFingerprint,
    source: &mut Transaction<'_>,
    target: &mut Client,
) -> Result<SchemaFingerprint> {
    let mut tx = target.transaction()?;
    tx.batch_execute(SETTINGS)?;
    tx.query_one("SELECT pg_advisory_xact_lock($1)", &[&ARCHIVE_LOCK])?;
    plan.target.matches(&mut tx)?;
    schema_absent(&mut tx, &plan.generation_schema())?;
    schema_absent(&mut tx, &plan.predecessor_schema())?;
    let old = fingerprint(&mut tx, "brain_legacy", &[])?;
    require(
        old.oid == plan.archive_schema_oid && old.owner == plan.archive_owner,
        "Archivbindung weicht ab",
    )?;
    compatible_tables(&old, source_fingerprint, plan)?;
    read_only_permissions(&mut tx, old.oid)?;
    let generation = ident(&plan.generation_schema())?;
    let owner = ident(&plan.archive_owner)?;
    tx.batch_execute(&format!("CREATE SCHEMA {generation} AUTHORIZATION {owner}; REVOKE ALL ON SCHEMA {generation} FROM PUBLIC;"))?;
    for row in tx.query("SELECT a.grantee, CASE WHEN a.grantee=0 THEN 'PUBLIC' ELSE pg_get_userbyid(a.grantee) END FROM pg_namespace n CROSS JOIN LATERAL aclexplode(coalesce(n.nspacl,acldefault('n',n.nspowner))) a WHERE n.oid=$1 AND a.grantee <> n.nspowner", &[&old.oid])? {
        let grantee = if row.get::<_,u32>(0)==0 { "PUBLIC".to_string() } else { ident(&row.get::<_,String>(1))? };
        tx.batch_execute(&format!("GRANT USAGE ON SCHEMA {generation} TO {grantee}"))?;
    }
    let global_defaults: i64 = tx
        .query_one(
            "SELECT count(*) FROM pg_default_acl WHERE defaclnamespace=0",
            &[],
        )?
        .get(0);
    require(
        global_defaults == 0,
        "Globale Standardrechte benötigen einen geprüften Adapter",
    )?;
    create_structure(&mut tx, &plan.generation_schema(), source_fingerprint)?;
    for table in &source_fingerprint.tables {
        let name = ident(&table.name)?;
        let previous = old.tables.iter().find(|t| t.name == table.name);
        let table_owner = ident(
            previous
                .map(|t| t.owner.as_str())
                .unwrap_or(&plan.archive_owner),
        )?;
        tx.batch_execute(&format!("ALTER TABLE {generation}.{name} OWNER TO {table_owner}; REVOKE ALL ON {generation}.{name} FROM PUBLIC;"))?;
        if let Some(previous) = previous {
            for row in tx.query("SELECT a.grantee, CASE WHEN a.grantee=0 THEN 'PUBLIC' ELSE pg_get_userbyid(a.grantee) END FROM pg_class c CROSS JOIN LATERAL aclexplode(coalesce(c.relacl,acldefault('r',c.relowner))) a WHERE c.oid=$1 AND a.grantee <> c.relowner", &[&previous.oid])? {
                let grantee = if row.get::<_,u32>(0)==0 { "PUBLIC".to_string() } else { ident(&row.get::<_,String>(1))? };
                tx.batch_execute(&format!("GRANT SELECT ON {generation}.{name} TO {grantee}"))?;
            }
        }
        let mut read =
            source.copy_out(&format!("COPY \"brain\".{name} TO STDOUT (FORMAT binary)"))?;
        let mut write = tx.copy_in(&format!(
            "COPY {generation}.{name} FROM STDIN (FORMAT binary)"
        ))?;
        std::io::copy(&mut read, &mut write)?;
        require(
            write.finish()?
                == source_fingerprint
                    .tables
                    .iter()
                    .find(|t| t.name == table.name)
                    .ok_or(Error::Contract("Tabelle fehlt"))?
                    .rows,
            "COPY-Zeilenanzahl weicht ab",
        )?;
    }
    schema::add_foreign_keys(&mut tx, &plan.generation_schema(), source_fingerprint)?;
    for seq in &source_fingerprint.sequences {
        let name = ident(&seq.name)?;
        tx.batch_execute(&format!(
            "REVOKE ALL ON SEQUENCE {generation}.{name} FROM PUBLIC"
        ))?;
        if let Some(previous) = old.sequences.iter().find(|s| s.name == seq.name) {
            for row in tx.query("SELECT a.grantee,CASE WHEN a.grantee=0 THEN 'PUBLIC' ELSE pg_get_userbyid(a.grantee) END FROM pg_class c CROSS JOIN LATERAL aclexplode(coalesce(c.relacl,acldefault('s',c.relowner))) a WHERE c.oid=$1 AND a.grantee <> c.relowner", &[&previous.oid])? {
                let grantee = if row.get::<_,u32>(0)==0 { "PUBLIC".to_string() } else { ident(&row.get::<_,String>(1))? };
                tx.batch_execute(&format!("GRANT SELECT ON SEQUENCE {generation}.{name} TO {grantee}"))?;
            }
        }
    }
    let new = fingerprint(&mut tx, &plan.generation_schema(), &[])?;
    equal_content(source_fingerprint, &new)?;
    equal_permissions(&old, &new, plan)?;
    new_permissions(&mut tx, &old, &new, plan)?;
    read_only_permissions(&mut tx, new.oid)?;
    tx.commit().map_err(|_| Error::CommitUncertain)?;
    Ok(new)
}

pub struct ValidatedGeneration {
    manifest: Manifest,
    old: SchemaFingerprint,
    new: SchemaFingerprint,
}
pub fn validate_generation(
    manifest: &Manifest,
    target: &mut Client,
) -> Result<ValidatedGeneration> {
    manifest.plan.validate()?;
    require(
        manifest.version == 2
            && manifest.exclusions == EXCLUDED
            && manifest.archive_sha256 == archive_digest(manifest)?,
        "Manifestbindung ungültig",
    )?;
    let mut tx = read_transaction(target)?;
    manifest.plan.target.matches(&mut tx)?;
    schema_absent(&mut tx, &manifest.plan.predecessor_schema())?;
    let old = fingerprint(&mut tx, "brain_legacy", &[])?;
    let new = fingerprint(&mut tx, &manifest.plan.generation_schema(), &[])?;
    require(
        manifest.staged.as_ref() == Some(&new),
        "Generation-OIDs, Inhalt oder Rechte weichen vom gebundenen Aufbau ab",
    )?;
    require(
        old.oid == manifest.plan.archive_schema_oid
            && old.owner == manifest.plan.archive_owner
            && new.oid != old.oid,
        "Alt-/Neubindung ungültig",
    )?;
    equal_content(&manifest.source, &new)?;
    equal_permissions(&old, &new, &manifest.plan)?;
    new_permissions(&mut tx, &old, &new, &manifest.plan)?;
    read_only_permissions(&mut tx, old.oid)?;
    read_only_permissions(&mut tx, new.oid)?;
    tx.rollback()?;
    Ok(ValidatedGeneration {
        manifest: manifest.clone(),
        old,
        new,
    })
}

pub fn activate_generation(
    proof: ValidatedGeneration,
    target: &mut Client,
    artifacts: &PrivateDirectory,
) -> Result<ActivationReceipt> {
    require(
        artifacts.read_manifest()?.archive_sha256 == proof.manifest.archive_sha256,
        "Artefaktbindung wurde nach Prüfung verändert",
    )?;
    let plan = &proof.manifest.plan;
    let mut tx = target.transaction()?;
    tx.batch_execute(SETTINGS)?;
    tx.query_one("SELECT pg_advisory_xact_lock($1)", &[&ARCHIVE_LOCK])?;
    plan.target.matches(&mut tx)?;
    schema_absent(&mut tx, &plan.predecessor_schema())?;
    require(
        schema_identity(&mut tx, "brain_legacy")?.0 == proof.old.oid
            && schema_identity(&mut tx, &plan.generation_schema())?.0 == proof.new.oid,
        "Schema-OIDs haben sich nach Prüfung geändert",
    )?;
    let mut relations = BTreeSet::new();
    for (schema, fp) in [
        ("brain_legacy".to_owned(), &proof.old),
        (plan.generation_schema(), &proof.new),
    ] {
        for t in &fp.tables {
            relations.insert(format!("{}.{}", ident(&schema)?, ident(&t.name)?));
        }
    }
    tx.batch_execute(&format!(
        "LOCK TABLE {} IN ACCESS EXCLUSIVE MODE",
        relations.into_iter().collect::<Vec<_>>().join(",")
    ))?;
    let old = fingerprint(&mut tx, "brain_legacy", &[])?;
    let new = fingerprint(&mut tx, &plan.generation_schema(), &[])?;
    require(
        old == proof.old && new == proof.new,
        "Inhalt oder Rechte wurden nach Prüfung verändert",
    )?;
    equal_content(&proof.manifest.source, &new)?;
    equal_permissions(&old, &new, plan)?;
    new_permissions(&mut tx, &old, &new, plan)?;
    read_only_permissions(&mut tx, old.oid)?;
    read_only_permissions(&mut tx, new.oid)?;
    tx.batch_execute(&format!(
        "ALTER SCHEMA \"brain_legacy\" RENAME TO {}; ALTER SCHEMA {} RENAME TO \"brain_legacy\";",
        ident(&plan.predecessor_schema())?,
        ident(&plan.generation_schema())?
    ))?;
    require(
        schema_identity(&mut tx, "brain_legacy")?.0 == new.oid
            && schema_identity(&mut tx, &plan.predecessor_schema())?.0 == old.oid,
        "Archivnamenwechsel nicht bestätigt",
    )?;
    let renamed_old = fingerprint(&mut tx, &plan.predecessor_schema(), &[])?;
    let renamed_new = fingerprint(&mut tx, "brain_legacy", &[])?;
    require(
        renamed_old == old && renamed_new == new,
        "Schemaobjekte drifteten beim atomaren Namenwechsel",
    )?;
    let receipt = ActivationReceipt {
        generation: plan.generation.clone(),
        snapshot_label: plan.snapshot_label.clone(),
        archive_sha256: proof.manifest.archive_sha256,
        dump_sha256: proof.manifest.dump_sha256,
        database_oid: plan.target.database_oid,
        new_archive_schema_oid: new.oid,
        predecessor_schema: plan.predecessor_schema(),
        predecessor_schema_oid: old.oid,
        schema_sha256: new.schema_sha256,
        tables: new.tables,
        sequences: new.sequences,
        schema_dump_sha256: proof.manifest.schema_dump_sha256,
        source_database_oid: plan.source.database_oid,
        source_schema_oid: plan.source_schema_oid,
        source_tables: proof.manifest.source.tables,
        source_sequences: proof.manifest.source.sequences,
        added_tables: plan.added_tables.clone(),
        q_binding_required: true,
        g5_complete: false,
    };
    artifacts.write_json("activation_intent.json", &receipt)?;
    tx.commit().map_err(|_| Error::CommitUncertain)?;
    Ok(receipt)
}

pub fn read_plan(path: &Path) -> Result<Plan> {
    use std::os::unix::ffi::OsStrExt;
    require(path.is_absolute(), "Planpfad muss absolut sein")?;
    let parent = PrivateDirectory::open(
        path.parent()
            .ok_or(Error::Contract("Planverzeichnis fehlt"))?,
    )?;
    let name = std::ffi::CString::new(
        path.file_name()
            .ok_or(Error::Contract("Planname fehlt"))?
            .as_bytes(),
    )
    .map_err(|_| Error::Contract("Planname enthält NUL"))?;
    let fd = unsafe {
        libc::openat(
            parent.directory.as_raw_fd(),
            name.as_ptr(),
            libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
        )
    };
    if fd < 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let file = unsafe { File::from_raw_fd(fd) };
    let m = file.metadata()?;
    require(
        m.is_file()
            && m.uid() == unsafe { libc::geteuid() }
            && m.mode() & 0o077 == 0
            && m.nlink() == 1,
        "Plan muss privat und selbst besessen sein",
    )?;
    let plan: Plan = serde_json::from_reader(file.take(1024 * 1024))?;
    plan.validate()?;
    Ok(plan)
}
