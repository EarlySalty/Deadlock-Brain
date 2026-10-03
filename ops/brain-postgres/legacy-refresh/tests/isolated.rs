use brain_legacy_refresh::*;
use postgres::{Client, NoTls};
use std::fs::{File, OpenOptions};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::process::{Child, Command, Stdio};
use tempfile::TempDir;

fn private_tempdir() -> TempDir {
    let directory = tempfile::tempdir().unwrap();
    std::fs::set_permissions(directory.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
    directory
}

struct Cluster {
    child: Child,
    directory: TempDir,
}
impl Drop for Cluster {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
impl Cluster {
    fn start() -> Self {
        let directory = private_tempdir();
        let data = directory.path().join("data");
        let status = Command::new("/usr/lib/postgresql/16/bin/initdb")
            .env_clear()
            .env("LC_ALL", "C")
            .arg("-D")
            .arg(&data)
            .args([
                "--auth-local=trust",
                "--auth-host=reject",
                "--no-locale",
                "--encoding=UTF8",
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .unwrap();
        assert!(status.success());
        let log = File::create(directory.path().join("server.log")).unwrap();
        let child = Command::new("/usr/lib/postgresql/16/bin/postgres")
            .env_clear()
            .env("LC_ALL", "C")
            .arg("-D")
            .arg(&data)
            .arg("-k")
            .arg(directory.path())
            .args([
                "-p",
                "5548",
                "-c",
                "listen_addresses=",
                "-c",
                "shared_buffers=16MB",
                "-c",
                "max_connections=10",
            ])
            .stdin(Stdio::null())
            .stdout(log.try_clone().unwrap())
            .stderr(log)
            .spawn()
            .unwrap();
        let mut cluster = Self { child, directory };
        for _ in 0..100 {
            if cluster.try_connect("postgres").is_ok() {
                return cluster;
            }
            assert!(
                cluster.child.try_wait().unwrap().is_none(),
                "isolated postgres exited"
            );
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        panic!("isolated postgres did not start")
    }
    fn try_connect(&self, database: &str) -> std::result::Result<Client, postgres::Error> {
        postgres::Config::new()
            .host_path(self.directory.path())
            .port(5548)
            .dbname(database)
            .connect(NoTls)
    }
    fn connect(&self, database: &str) -> Client {
        self.try_connect(database).unwrap()
    }
    fn fixture(&self, id: char) -> (Plan, Client, Client) {
        let mut root = self.connect("postgres");
        root.batch_execute("CREATE DATABASE refresh_test_source")
            .unwrap();
        root.batch_execute("CREATE DATABASE refresh_test_target")
            .unwrap();
        let mut source = self.connect("refresh_test_source");
        let mut target = self.connect("refresh_test_target");
        source.batch_execute("CREATE SCHEMA brain; CREATE TABLE brain.documents (id bigint PRIMARY KEY, body text, meta jsonb); CREATE TABLE brain.empty_table (id bigint); INSERT INTO brain.documents VALUES (1,'alt', '{\"a\":1}'),(2,NULL,NULL); CREATE TABLE brain.feeder_runs (id bigint); INSERT INTO brain.feeder_runs VALUES (42)").unwrap();
        target.batch_execute("CREATE ROLE archive_reader; CREATE SCHEMA brain; CREATE TABLE brain.core_guard(id bigint); INSERT INTO brain.core_guard VALUES (77); CREATE SCHEMA brain_legacy; REVOKE ALL ON SCHEMA brain_legacy FROM PUBLIC; GRANT USAGE ON SCHEMA brain_legacy TO archive_reader; CREATE TABLE brain_legacy.documents (id bigint PRIMARY KEY, body text, meta jsonb); CREATE TABLE brain_legacy.empty_table (id bigint); GRANT SELECT ON ALL TABLES IN SCHEMA brain_legacy TO archive_reader; INSERT INTO brain_legacy.documents VALUES (1,'vorher','{}')").unwrap();
        fn endpoint(c: &mut Client, socket: &std::path::Path) -> Endpoint {
            let row = c.query_one("SELECT current_database(),current_user,oid,pg_get_userbyid(datdba) FROM pg_database WHERE datname=current_database()",&[]).unwrap();
            Endpoint {
                socket: socket.to_path_buf(),
                port: 5548,
                database: row.get(0),
                role: row.get(1),
                database_oid: row.get(2),
                database_owner: row.get(3),
            }
        }
        let src = endpoint(&mut source, self.directory.path());
        let dst = endpoint(&mut target, self.directory.path());
        let source_schema_oid = source
            .query_one("SELECT oid FROM pg_namespace WHERE nspname='brain'", &[])
            .unwrap()
            .get(0);
        let archive_schema_oid = target
            .query_one(
                "SELECT oid FROM pg_namespace WHERE nspname='brain_legacy'",
                &[],
            )
            .unwrap()
            .get(0);
        let archive_owner = dst.role.clone();
        (
            Plan {
                generation: id.to_string().repeat(32),
                snapshot_label: "isolated-proof".into(),
                source: src,
                source_schema_oid,
                target: dst,
                archive_schema_oid,
                archive_owner,
                added_tables: Vec::new(),
            },
            source,
            target,
        )
    }
}

#[test]
fn snapshot_stage_swap_preserves_predecessor_core_and_rights() {
    let cluster = Cluster::start();
    let (plan, mut source, mut target) = cluster.fixture('a');
    inspect_plan(&plan, &mut source, &mut target).unwrap();
    let dir = private_tempdir();
    let artifacts = PrivateDirectory::open(dir.path()).unwrap();
    let manifest = capture_and_stage(&plan, &mut source, &mut target, &artifacts).unwrap();
    assert_eq!(manifest.source.tables.len(), 2);
    assert_eq!(
        artifacts.read_manifest().unwrap().archive_sha256,
        manifest.archive_sha256
    );
    let old_oid: u32 = target
        .query_one(
            "SELECT oid FROM pg_namespace WHERE nspname='brain_legacy'",
            &[],
        )
        .unwrap()
        .get(0);
    let proof = validate_generation(&manifest, &mut target).unwrap();
    let receipt = activate_generation(proof, &mut target, &artifacts).unwrap();
    assert_ne!(receipt.new_archive_schema_oid, old_oid);
    assert_eq!(receipt.predecessor_schema_oid, old_oid);
    assert!(receipt.q_binding_required);
    assert!(!receipt.g5_complete);
    assert_eq!(
        target
            .query_one("SELECT body FROM brain_legacy.documents WHERE id=1", &[])
            .unwrap()
            .get::<_, String>(0),
        "alt"
    );
    assert_eq!(
        target
            .query_one(
                &format!(
                    "SELECT body FROM {}.documents WHERE id=1",
                    receipt.predecessor_schema
                ),
                &[]
            )
            .unwrap()
            .get::<_, String>(0),
        "vorher"
    );
    assert_eq!(
        target
            .query_one("SELECT id FROM brain.core_guard", &[])
            .unwrap()
            .get::<_, i64>(0),
        77
    );
    assert!(validate_generation(&manifest, &mut target).is_err());
}

#[test]
fn drift_after_validation_blocks_activation() {
    let cluster = Cluster::start();
    let (plan, mut source, mut target) = cluster.fixture('b');
    let dir = private_tempdir();
    let artifacts = PrivateDirectory::open(dir.path()).unwrap();
    let manifest = capture_and_stage(&plan, &mut source, &mut target, &artifacts).unwrap();
    let proof = validate_generation(&manifest, &mut target).unwrap();
    target
        .batch_execute(&format!(
            "UPDATE {}.documents SET body='drift' WHERE id=1",
            plan.generation_schema()
        ))
        .unwrap();
    assert!(activate_generation(proof, &mut target, &artifacts).is_err());
    assert_eq!(
        target
            .query_one(
                "SELECT oid FROM pg_namespace WHERE nspname='brain_legacy'",
                &[]
            )
            .unwrap()
            .get::<_, u32>(0),
        plan.archive_schema_oid
    );
    assert_eq!(
        target
            .query_one(
                "SELECT count(*) FROM pg_namespace WHERE nspname=$1",
                &[&plan.predecessor_schema()]
            )
            .unwrap()
            .get::<_, i64>(0),
        0
    );
}

#[test]
fn second_rename_failure_rolls_back_both_names() {
    let cluster = Cluster::start();
    let (plan, mut source, mut target) = cluster.fixture('c');
    let dir = private_tempdir();
    let artifacts = PrivateDirectory::open(dir.path()).unwrap();
    let manifest = capture_and_stage(&plan, &mut source, &mut target, &artifacts).unwrap();
    let proof = validate_generation(&manifest, &mut target).unwrap();
    target.batch_execute(&format!("CREATE FUNCTION public.fail_second() RETURNS event_trigger LANGUAGE plpgsql AS $$ BEGIN IF TG_TAG='ALTER SCHEMA' AND EXISTS(SELECT FROM pg_namespace WHERE nspname='brain_legacy' AND oid <> {}) THEN RAISE EXCEPTION 'isolated failure'; END IF; END $$; CREATE EVENT TRIGGER fail_second ON ddl_command_end EXECUTE FUNCTION public.fail_second()",plan.archive_schema_oid)).unwrap();
    assert!(activate_generation(proof, &mut target, &artifacts).is_err());
    assert_eq!(
        target
            .query_one(
                "SELECT oid FROM pg_namespace WHERE nspname='brain_legacy'",
                &[]
            )
            .unwrap()
            .get::<_, u32>(0),
        plan.archive_schema_oid
    );
    assert_eq!(
        target
            .query_one(
                "SELECT count(*) FROM pg_namespace WHERE nspname=$1",
                &[&plan.generation_schema()]
            )
            .unwrap()
            .get::<_, i64>(0),
        1
    );
    assert_eq!(
        target
            .query_one(
                "SELECT count(*) FROM pg_namespace WHERE nspname=$1",
                &[&plan.predecessor_schema()]
            )
            .unwrap()
            .get::<_, i64>(0),
        0
    );
}

#[test]
fn wrong_oid_write_grant_and_unsupported_schema_fail_closed() {
    let cluster = Cluster::start();
    let (mut plan, mut source, mut target) = cluster.fixture('d');
    plan.target.database_oid += 1;
    assert!(inspect_plan(&plan, &mut source, &mut target).is_err());
    plan.target.database_oid -= 1;
    target
        .batch_execute("GRANT INSERT ON brain_legacy.documents TO archive_reader")
        .unwrap();
    assert!(inspect_plan(&plan, &mut source, &mut target).is_err());
    target
        .batch_execute("REVOKE INSERT ON brain_legacy.documents FROM archive_reader")
        .unwrap();
    source
        .batch_execute("CREATE SEQUENCE brain.unsafe_sequence")
        .unwrap();
    assert!(inspect_plan(&plan, &mut source, &mut target).is_err());
}

#[test]
fn artifacts_reject_symlinks_hardlinks_overwrite_and_hash_drift() {
    let dir = private_tempdir();
    let link_parent = private_tempdir();
    std::os::unix::fs::symlink(dir.path(), link_parent.path().join("link")).unwrap();
    assert!(PrivateDirectory::open(&link_parent.path().join("link")).is_err());
    std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o755)).unwrap();
    assert!(PrivateDirectory::open(dir.path()).is_err());
    std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
    let artifacts = PrivateDirectory::open(dir.path()).unwrap();
    artifacts.write_json("receipt.json", &true).unwrap();
    assert!(artifacts.write_json("receipt.json", &false).is_err());
    assert!(PrivateDirectory::open(dir.path()).is_err());
    drop(artifacts);
    std::fs::hard_link(
        dir.path().join("receipt.json"),
        dir.path().join("manifest.json"),
    )
    .unwrap();
    let artifacts = PrivateDirectory::open(dir.path()).unwrap();
    assert!(artifacts.read_manifest().is_err());
    drop(artifacts);
    std::fs::remove_file(dir.path().join("manifest.json")).unwrap();
    let _file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .mode(0o600)
        .open(dir.path().join("manifest.json"))
        .unwrap();
}

#[test]
fn manifest_dump_hash_tamper_is_rejected() {
    use std::io::Write;
    let cluster = Cluster::start();
    let (plan, mut source, mut target) = cluster.fixture('e');
    let dir = private_tempdir();
    let artifacts = PrivateDirectory::open(dir.path()).unwrap();
    let manifest = capture_and_stage(&plan, &mut source, &mut target, &artifacts).unwrap();
    assert!(validate_generation(&manifest, &mut target).is_ok());
    OpenOptions::new()
        .append(true)
        .open(dir.path().join("archive.dump"))
        .unwrap()
        .write_all(b"tamper")
        .unwrap();
    assert!(artifacts.read_manifest().is_err());
}

#[test]
fn sequences_defaults_foreign_keys_checks_and_new_tables_survive_swap() {
    let cluster = Cluster::start();
    let (mut plan, mut source, mut target) = cluster.fixture('1');
    source.batch_execute("CREATE SEQUENCE brain.documents_id_seq OWNED BY brain.documents.id; ALTER TABLE brain.documents ALTER COLUMN id SET DEFAULT nextval('brain.documents_id_seq'); ALTER TABLE brain.documents ALTER COLUMN body SET DEFAULT 'brain.literal'; ALTER TABLE brain.documents ALTER COLUMN meta SET DEFAULT '{}'::jsonb; SELECT setval('brain.documents_id_seq',42,true); CREATE TABLE brain.children(id bigint PRIMARY KEY, document_id bigint REFERENCES brain.documents(id)); INSERT INTO brain.children VALUES (3,1); CREATE TABLE brain.application_catalog (id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY, entity_type text NOT NULL CHECK (entity_type = ANY (ARRAY['hero'::text,'item'::text,'ability'::text])), observed_at timestamptz DEFAULT now(), entity_uuid uuid DEFAULT gen_random_uuid(), tags text[] DEFAULT '{}'::text[]); INSERT INTO brain.application_catalog(entity_type) VALUES ('hero'); CREATE TABLE brain.application_emojis (id bigint PRIMARY KEY, catalog_id bigint REFERENCES brain.application_catalog(id)); INSERT INTO brain.application_emojis VALUES(1,1)").unwrap();
    target.batch_execute("CREATE SEQUENCE brain_legacy.documents_id_seq OWNED BY brain_legacy.documents.id; ALTER TABLE brain_legacy.documents ALTER COLUMN id SET DEFAULT nextval('brain_legacy.documents_id_seq'); ALTER TABLE brain_legacy.documents ALTER COLUMN body SET DEFAULT 'brain.literal'; ALTER TABLE brain_legacy.documents ALTER COLUMN meta SET DEFAULT '{}'::jsonb; SELECT setval('brain_legacy.documents_id_seq',9,true); GRANT SELECT ON SEQUENCE brain_legacy.documents_id_seq TO archive_reader; CREATE TABLE brain_legacy.children(id bigint PRIMARY KEY, document_id bigint REFERENCES brain_legacy.documents(id)); GRANT SELECT ON brain_legacy.children TO archive_reader").unwrap();
    plan.added_tables = vec!["application_catalog".into(), "application_emojis".into()];
    inspect_plan(&plan, &mut source, &mut target).unwrap();
    let dir = private_tempdir();
    let artifacts = PrivateDirectory::open(dir.path()).unwrap();
    let manifest = capture_and_stage(&plan, &mut source, &mut target, &artifacts).unwrap();
    assert_eq!(manifest.source.sequences.len(), 2);
    let proof = validate_generation(&manifest, &mut target).unwrap();
    let receipt = activate_generation(proof, &mut target, &artifacts).unwrap();
    assert_eq!(
        target
            .query_one("SELECT nextval('brain_legacy.documents_id_seq')", &[])
            .unwrap()
            .get::<_, i64>(0),
        43
    );
    assert_eq!(
        target
            .query_one(
                &format!(
                    "SELECT last_value FROM {}.documents_id_seq",
                    receipt.predecessor_schema
                ),
                &[]
            )
            .unwrap()
            .get::<_, i64>(0),
        9
    );
    assert!(!target.query_one("SELECT has_table_privilege('archive_reader','brain_legacy.application_catalog','SELECT')", &[]).unwrap().get::<_,bool>(0));
    assert!(target
        .query_one(
            "SELECT has_table_privilege('archive_reader','brain_legacy.documents','SELECT')",
            &[]
        )
        .unwrap()
        .get::<_, bool>(0));
    target.batch_execute("INSERT INTO brain_legacy.documents(meta) VALUES('{}'); INSERT INTO brain_legacy.application_catalog(entity_type) VALUES('item')").unwrap();
    assert_eq!(
        target
            .query_one("SELECT body FROM brain_legacy.documents WHERE id=44", &[])
            .unwrap()
            .get::<_, String>(0),
        "brain.literal"
    );
    assert!(target
        .batch_execute(
            "INSERT INTO brain_legacy.application_catalog(entity_type) VALUES('invalid')"
        )
        .is_err());
}

#[test]
fn unbound_additions_unsafe_defaults_external_keys_and_new_grants_block() {
    let cluster = Cluster::start();
    let (mut plan, mut source, mut target) = cluster.fixture('2');
    source
        .batch_execute("CREATE TABLE brain.new_table(id bigint)")
        .unwrap();
    assert!(inspect_plan(&plan, &mut source, &mut target).is_err());
    plan.added_tables = vec!["new_table".into()];
    inspect_plan(&plan, &mut source, &mut target).unwrap();
    source
        .batch_execute("ALTER TABLE brain.new_table ALTER COLUMN id SET DEFAULT pg_backend_pid()")
        .unwrap();
    assert!(inspect_plan(&plan, &mut source, &mut target).is_err());
    source.batch_execute("ALTER TABLE brain.new_table ALTER COLUMN id DROP DEFAULT; CREATE TABLE public.outside(id bigint PRIMARY KEY); ALTER TABLE brain.new_table ADD CONSTRAINT bad_fk FOREIGN KEY(id) REFERENCES public.outside(id)").unwrap();
    assert!(inspect_plan(&plan, &mut source, &mut target).is_err());
    source
        .batch_execute("ALTER TABLE brain.new_table DROP CONSTRAINT bad_fk")
        .unwrap();
    let dir = private_tempdir();
    let artifacts = PrivateDirectory::open(dir.path()).unwrap();
    let manifest = capture_and_stage(&plan, &mut source, &mut target, &artifacts).unwrap();
    target
        .batch_execute(&format!(
            "GRANT SELECT ON {}.new_table TO archive_reader",
            plan.generation_schema()
        ))
        .unwrap();
    assert!(validate_generation(&manifest, &mut target).is_err());
}

#[test]
#[ignore = "private schema-only fixtures required"]
fn real_schema_only_structure_round_trip() {
    use std::os::unix::fs::MetadataExt;
    let source_file =
        std::env::var("BRAIN_REFRESH_SOURCE_SCHEMA").expect("private source fixture required");
    let archive_file =
        std::env::var("BRAIN_REFRESH_ARCHIVE_SCHEMA").expect("private archive fixture required");
    let cluster = Cluster::start();
    let (mut plan, mut source, mut target) = cluster.fixture('3');
    source.batch_execute("DROP SCHEMA brain CASCADE").unwrap();
    target
        .batch_execute("DROP SCHEMA brain_legacy CASCADE")
        .unwrap();
    for (file, database) in [
        (&source_file, "refresh_test_source"),
        (&archive_file, "refresh_test_target"),
    ] {
        let path = std::path::Path::new(file);
        assert!(path.is_absolute());
        let metadata = std::fs::symlink_metadata(path).unwrap();
        assert!(
            metadata.is_file()
                && metadata.uid() == unsafe { libc::geteuid() }
                && metadata.mode() & 0o077 == 0
                && metadata.nlink() == 1
        );
        assert_eq!(
            std::fs::symlink_metadata(path.parent().unwrap())
                .unwrap()
                .mode()
                & 0o077,
            0
        );
        let output = Command::new("/usr/lib/postgresql/16/bin/psql")
            .env_clear()
            .env("LC_ALL", "C")
            .args(["-X", "-q", "--no-password", "-v", "ON_ERROR_STOP=1", "-v", "VERBOSITY=sqlstate", "-h"])
            .arg(cluster.directory.path())
            .args(["-p", "5548", "-d", database, "-f"])
            .arg(path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()
            .unwrap();
        assert!(output.status.success(), "private schema fixture restore failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    plan.source_schema_oid = source
        .query_one("SELECT oid FROM pg_namespace WHERE nspname='brain'", &[])
        .unwrap()
        .get(0);
    plan.archive_schema_oid = target
        .query_one(
            "SELECT oid FROM pg_namespace WHERE nspname='brain_legacy'",
            &[],
        )
        .unwrap()
        .get(0);
    target.batch_execute("REVOKE ALL ON SCHEMA brain_legacy FROM PUBLIC; GRANT USAGE ON SCHEMA brain_legacy TO archive_reader; GRANT SELECT ON ALL TABLES IN SCHEMA brain_legacy TO archive_reader; GRANT SELECT ON ALL SEQUENCES IN SCHEMA brain_legacy TO archive_reader").unwrap();
    plan.added_tables = vec!["application_catalog".into(), "application_emojis".into()];
    inspect_plan(&plan, &mut source, &mut target).unwrap();
    let dir = private_tempdir();
    let artifacts = PrivateDirectory::open(dir.path()).unwrap();
    let manifest = capture_and_stage(&plan, &mut source, &mut target, &artifacts).unwrap();
    let expected_tables: i64 = source.query_one("SELECT count(*) FROM pg_class WHERE relnamespace='brain'::regnamespace AND relkind='r' AND relname<>ALL($1)", &[&EXCLUDED.map(String::from).to_vec()]).unwrap().get(0);
    let expected_sequences: i64 = source.query_one("SELECT count(*) FROM pg_class s JOIN pg_depend d ON d.classid='pg_class'::regclass AND d.objid=s.oid AND d.refclassid='pg_class'::regclass AND d.deptype IN ('a','i') JOIN pg_class t ON t.oid=d.refobjid WHERE s.relnamespace='brain'::regnamespace AND s.relkind='S' AND t.relname<>ALL($1)", &[&EXCLUDED.map(String::from).to_vec()]).unwrap().get(0);
    assert_eq!(manifest.source.tables.len() as i64, expected_tables);
    assert_eq!(manifest.source.sequences.len() as i64, expected_sequences);
    println!("REAL_STRUCTURE tables={expected_tables} sequences={expected_sequences} product_rows_imported=0");
    assert_eq!(
        manifest.source.tables.iter().map(|t| t.rows).sum::<u64>(),
        0
    );
    let proof = validate_generation(&manifest, &mut target).unwrap();
    let receipt = activate_generation(proof, &mut target, &artifacts).unwrap();
    assert!(!receipt.g5_complete);
    assert!(receipt.q_binding_required);
    assert_eq!(
        target
            .query_one("SELECT id FROM brain.core_guard", &[])
            .unwrap()
            .get::<_, i64>(0),
        77
    );
}

#[test]
fn cli_stage_validate_hash_guard_and_activation_use_existing_clients() {
    use std::io::Write;
    let cluster = Cluster::start();
    let (plan, _source, mut target) = cluster.fixture('4');
    let plan_dir = private_tempdir();
    let plan_path = plan_dir.path().join("plan.json");
    let mut plan_file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .mode(0o600)
        .open(&plan_path)
        .unwrap();
    plan_file
        .write_all(&serde_json::to_vec(&plan).unwrap())
        .unwrap();
    plan_file.sync_all().unwrap();
    let artifacts_dir = private_tempdir();
    let binary = env!("CARGO_BIN_EXE_brain-legacy-refresh");
    let run = |args: &[&std::ffi::OsStr]| {
        Command::new(binary)
            .env_clear()
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .unwrap()
    };
    assert!(run(&[
        "stage".as_ref(),
        plan_path.as_os_str(),
        artifacts_dir.path().as_os_str()
    ])
    .success());
    assert!(run(&["validate".as_ref(), artifacts_dir.path().as_os_str()]).success());
    assert!(!run(&[
        "activate".as_ref(),
        artifacts_dir.path().as_os_str(),
        "wrong".as_ref()
    ])
    .success());
    assert_eq!(
        target
            .query_one(
                "SELECT oid FROM pg_namespace WHERE nspname='brain_legacy'",
                &[]
            )
            .unwrap()
            .get::<_, u32>(0),
        plan.archive_schema_oid
    );
    let artifacts = PrivateDirectory::open(artifacts_dir.path()).unwrap();
    let manifest = artifacts.read_manifest().unwrap();
    drop(artifacts);
    assert!(run(&[
        "activate".as_ref(),
        artifacts_dir.path().as_os_str(),
        manifest.archive_sha256.as_ref()
    ])
    .success());
    let receipt: ActivationReceipt =
        serde_json::from_reader(File::open(artifacts_dir.path().join("receipt.json")).unwrap())
            .unwrap();
    assert_eq!(receipt.source_schema_oid, plan.source_schema_oid);
    assert_eq!(receipt.schema_dump_sha256, manifest.schema_dump_sha256);
    assert_eq!(receipt.predecessor_schema_oid, plan.archive_schema_oid);
    assert!(receipt.q_binding_required && !receipt.g5_complete);
}

#[test]
fn exported_read_only_snapshot_does_not_freeze_sequence_state() {
    let cluster = Cluster::start();
    let (_, mut source, _target) = cluster.fixture('5');
    source
        .batch_execute(
            "CREATE SEQUENCE brain.sequence_probe; SELECT setval('brain.sequence_probe',10,true)",
        )
        .unwrap();
    let mut tx = source
        .build_transaction()
        .isolation_level(postgres::IsolationLevel::RepeatableRead)
        .read_only(true)
        .start()
        .unwrap();
    let _snapshot: String = tx
        .query_one("SELECT pg_export_snapshot()", &[])
        .unwrap()
        .get(0);
    let before: i64 = tx
        .query_one("SELECT last_value FROM brain.sequence_probe", &[])
        .unwrap()
        .get(0);
    let mut writer = cluster.connect("refresh_test_source");
    writer
        .query_one("SELECT nextval('brain.sequence_probe')", &[])
        .unwrap();
    let after: i64 = tx
        .query_one("SELECT last_value FROM brain.sequence_probe", &[])
        .unwrap()
        .get(0);
    assert_eq!(before, 10);
    assert_eq!(after, 11);
    tx.rollback().unwrap();
}

#[test]
fn failed_stage_keeps_archive_and_never_creates_generation() {
    let cluster = Cluster::start();
    let (plan, mut source, mut target) = cluster.fixture('f');
    source
        .batch_execute("ALTER TABLE brain.documents ADD COLUMN incompatible text")
        .unwrap();
    let dir = private_tempdir();
    let artifacts = PrivateDirectory::open(dir.path()).unwrap();
    assert!(capture_and_stage(&plan, &mut source, &mut target, &artifacts).is_err());
    assert_eq!(
        target
            .query_one(
                "SELECT count(*) FROM pg_namespace WHERE nspname=$1",
                &[&plan.generation_schema()]
            )
            .unwrap()
            .get::<_, i64>(0),
        0
    );
    assert_eq!(
        target
            .query_one("SELECT body FROM brain_legacy.documents WHERE id=1", &[])
            .unwrap()
            .get::<_, String>(0),
        "vorher"
    );
}
