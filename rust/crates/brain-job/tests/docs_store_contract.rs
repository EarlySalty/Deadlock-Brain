use brain_job::{
    docs::{self, SOURCE},
    docs_extract,
    docs_store::DocsStore,
    extract::{self, Atom},
    git::{Blob, Repository},
    store::{Counts, Store},
};
use sha2::{Digest, Sha256};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    ConnectOptions, PgPool,
};
use std::{
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

struct Scratch {
    root: tempfile::TempDir,
}

impl Scratch {
    fn new() -> Self {
        let root = tempfile::Builder::new()
            .prefix("brain-p5-docs-pg-")
            .tempdir()
            .unwrap();
        std::fs::create_dir(root.path().join("socket")).unwrap();
        let scratch = Self { root };
        let status = Command::new("/usr/lib/postgresql/16/bin/initdb")
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .args([
                "--auth=trust",
                "--no-locale",
                "--encoding=UTF8",
                "--username=brain_test_dba",
                "--pgdata",
            ])
            .arg(scratch.root.path().join("data"))
            .output()
            .unwrap();
        assert!(status.status.success(), "initdb");
        let status = Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .arg("--pgdata")
            .arg(scratch.root.path().join("data"))
            .arg("--log")
            .arg(scratch.root.path().join("postgres.log"))
            .args(["--wait", "--options"])
            .arg(format!(
                "-k {} -p 55469 -h '' -c fsync=off",
                scratch.root.path().join("socket").display()
            ))
            .arg("start")
            .output()
            .unwrap();
        assert!(status.status.success(), "pg_ctl start");
        scratch
    }

    async fn pool(&self, database: &str, user: &str) -> PgPool {
        PgPoolOptions::new()
            .max_connections(2)
            .connect_with(
                PgConnectOptions::new_without_pgpass()
                    .host(self.root.path().join("socket").to_str().unwrap())
                    .port(55469)
                    .database(database)
                    .username(user)
                    .password("")
                    .ssl_mode(PgSslMode::Disable)
                    .disable_statement_logging(),
            )
            .await
            .unwrap()
    }

    async fn prepare(&self) -> PgPool {
        let admin = self.pool("postgres", "brain_test_dba").await;
        let directory: String = sqlx::query_scalar("SELECT current_setting('data_directory')")
            .fetch_one(&admin)
            .await
            .unwrap();
        assert_eq!(PathBuf::from(directory), self.root.path().join("data"));
        sqlx::raw_sql("CREATE ROLE brain_migrate LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE; CREATE ROLE brain_ingest LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE; CREATE ROLE brain_service LOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE; CREATE ROLE brain_tool NOLOGIN NOSUPERUSER NOCREATEDB NOCREATEROLE NOREPLICATION NOBYPASSRLS;").execute(&admin).await.unwrap();
        sqlx::query("CREATE DATABASE brain_docs_test OWNER brain_migrate")
            .execute(&admin)
            .await
            .unwrap();
        let owner = self.pool("brain_docs_test", "brain_migrate").await;
        sqlx::raw_sql(include_str!(
            "../../../../scripts/migrations/2026-10-12-brain-gamefacts-v1.sql"
        ))
        .execute(&owner)
        .await
        .unwrap();
        let migration = std::env::var_os("BRAIN_DOCS_MIGRATION")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../../../scripts/migrations/2026-10-12-brain-gamefacts-p5-docs-v1.sql")
            });
        let sql = std::fs::read_to_string(migration).expect("P5 migration file");
        sqlx::raw_sql(&sql).execute(&owner).await.unwrap();
        sqlx::query("ALTER ROLE brain_ingest SET default_transaction_isolation='repeatable read'")
            .execute(&admin)
            .await
            .unwrap();
        owner.close().await;
        admin.close().await;
        self.pool("brain_docs_test", "brain_ingest").await
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let status = Command::new("/usr/lib/postgresql/16/bin/pg_ctl")
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .arg("--pgdata")
            .arg(self.root.path().join("data"))
            .args(["--wait", "--mode=immediate", "stop"])
            .output();
        assert!(
            status.is_ok_and(|output| output.status.success()),
            "pg_ctl stop"
        );
    }
}

fn blob(bytes: &[u8]) -> Blob {
    Blob {
        bytes: bytes.to_vec(),
        hash: Sha256::digest(bytes).to_vec(),
    }
}

fn html(text: &str) -> String {
    format!("<!doctype html>\n<html lang=\"de\"><head><title>Fixture</title></head><body><main>\n<h1>Billy</h1>\n<p>Einleitung.</p>\n<section id=\"play\">\n<h2>Spielweise</h2>\n<p>{text}</p>\n</section>\n</main></body></html>")
}

async fn section_bodies(pool: &PgPool) -> Vec<String> {
    sqlx::query_scalar("SELECT body FROM docs.current_section ORDER BY path,ordinal")
        .fetch_all(pool)
        .await
        .unwrap()
}

#[tokio::test]
async fn stored_raw_receipts_versions_dedup_resume_rollback_aliases_and_fences() {
    let scratch = Scratch::new();
    let pool = scratch.prepare().await;
    let game = Store::start(pool.clone(), 300).await.unwrap();
    let source = brain_job::git::Source::ALL[0];
    let game_id = game.source(source).await.unwrap();
    let mut counts = Counts::default();
    let game_run = game
        .begin_run(game_id, &"1".repeat(40))
        .await
        .unwrap()
        .unwrap();
    game.object(
        &game_run,
        source,
        "data/json/hero-data.json",
        Some(blob(
            br#"{"hero_billy":{"Name":"Billy"},"hero_ivy":{"Name":"Ivy"}}"#,
        )),
        &mut counts,
    )
    .await
    .unwrap();
    game.publish(&game_run).await.unwrap();
    game.finish(&counts, false).await.unwrap();
    let store = DocsStore::start(pool.clone(), 300).await.unwrap();
    assert!(DocsStore::start(pool.clone(), 300).await.is_err());
    let source_id = store.source().await.unwrap();
    let path = "public/billy.html";
    let a = html("Billy spielt nah. Privy gehört nicht zu Ivy.");
    let b = html("Billy spielt kontrolliert.");
    let mut counts = Counts::default();
    let run_a = store
        .begin_run(source_id, &"a".repeat(40))
        .await
        .unwrap()
        .unwrap();
    store
        .object(&run_a, path, Some(blob(a.as_bytes())), &mut counts)
        .await
        .unwrap();
    store
        .object(
            &run_a,
            "internal/guide.html",
            Some(blob(a.as_bytes())),
            &mut counts,
        )
        .await
        .unwrap();
    assert!(section_bodies(&pool).await.is_empty());
    store.publish(&run_a).await.unwrap();
    let old = section_bodies(&pool).await;
    assert_eq!(old.len(), 4);
    assert!(store
        .begin_run(source_id, &run_a.target)
        .await
        .unwrap()
        .is_none());
    let entities: Vec<(String, i64)> =
        sqlx::query_as("SELECT key,id FROM canon.entity ORDER BY key")
            .fetch_all(&pool)
            .await
            .unwrap();
    let actual: (Vec<i64>, String, String, Vec<u8>, Vec<u8>, Vec<u8>) = sqlx::query_as("SELECT s.entity_ids,s.url,c.locator,c.value_sha256,o.content_sha256,o.body FROM docs.current_section s JOIN canon.receipt c ON c.id=s.receipt_id JOIN raw.object o ON o.id=c.raw_object_id WHERE s.path=$1 AND s.ordinal=1")
        .bind(path).fetch_one(&pool).await.unwrap();
    assert_eq!(
        actual.0,
        entities.iter().map(|(_, id)| *id).collect::<Vec<_>>()
    );
    assert!(actual.1.ends_with("/public/billy.html#L5-L8"));
    assert_eq!(actual.2, "main > section[id=\"play\"]");
    assert_eq!(actual.3, Sha256::digest(old[3].as_bytes()).to_vec());
    assert_eq!(actual.4, Sha256::digest(a.as_bytes()).to_vec());
    assert_eq!(actual.5, a.as_bytes());
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM canon.entity")
            .fetch_one(&pool)
            .await
            .unwrap(),
        2
    );
    let run_b = store
        .begin_run(source_id, &"b".repeat(40))
        .await
        .unwrap()
        .unwrap();
    store
        .object(&run_b, path, Some(blob(b.as_bytes())), &mut counts)
        .await
        .unwrap();
    assert_eq!(section_bodies(&pool).await, old);
    store.finish(&counts, true).await.unwrap();
    let resumed = DocsStore::start(pool.clone(), 300).await.unwrap();
    let pending = resumed.pending(source_id).await.unwrap().unwrap();
    assert_eq!(pending.id, run_b.id);
    assert!(resumed.completed(&pending, path).await.unwrap());
    resumed.publish(&pending).await.unwrap();
    let published = section_bodies(&pool).await;
    assert_ne!(published, old);
    let run_c = resumed
        .begin_run(source_id, &"c".repeat(40))
        .await
        .unwrap()
        .unwrap();
    resumed
        .object(&run_c, path, Some(blob(a.as_bytes())), &mut counts)
        .await
        .unwrap();
    resumed.publish(&run_c).await.unwrap();
    assert_eq!(counts.raw_inserted, 3);
    assert_eq!(counts.raw_reused, 1);
    let run_d = resumed
        .begin_run(source_id, &"d".repeat(40))
        .await
        .unwrap()
        .unwrap();
    let malformed = a.replace("</section>", "");
    assert!(resumed
        .object(&run_d, path, Some(blob(malformed.as_bytes())), &mut counts)
        .await
        .is_err());
    assert!(!resumed.completed(&run_d, path).await.unwrap());
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM raw.object WHERE source_id=$1")
            .bind(source_id)
            .fetch_one(&pool)
            .await
            .unwrap(),
        3
    );
    assert_eq!(section_bodies(&pool).await, old);
    let mut corrupt = blob(a.as_bytes());
    corrupt.hash[0] ^= 1;
    assert!(resumed
        .object(&run_d, path, Some(corrupt), &mut counts)
        .await
        .is_err());
    resumed
        .object(&run_d, path, None, &mut counts)
        .await
        .unwrap();
    assert_eq!(section_bodies(&pool).await, old);
    resumed.publish(&run_d).await.unwrap();
    assert_eq!(section_bodies(&pool).await.len(), 2);
    let mut tx = pool.begin().await.unwrap();
    sqlx::query("SET TRANSACTION ISOLATION LEVEL READ COMMITTED")
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query("UPDATE ops.job_lease SET epoch=epoch+1 WHERE lease_key='brain-job:ingest-docs'")
        .execute(&mut *tx)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    assert!(resumed.renew().await.is_err());
    pool.close().await;
}

#[tokio::test]
async fn aliases_require_a_published_canonical_identity() {
    let scratch = Scratch::new();
    let pool = scratch.prepare().await;
    let game = Store::start(pool.clone(), 300).await.unwrap();
    let mut game_counts = Counts::default();
    let identity_source = brain_job::git::Source::ALL[1];
    let identity_id = game.source(identity_source).await.unwrap();
    let identity_run = game
        .begin_run(identity_id, &"1".repeat(40))
        .await
        .unwrap()
        .unwrap();
    game.object(
        &identity_run,
        identity_source,
        "game/citadel/pak01_dir/scripts/heroes.vdata",
        Some(blob(br#"hero_ghost { m_eHeroDevelopmentState "EHeroDevState_Release" }"#)),
        &mut game_counts,
    )
    .await
    .unwrap();
    let alias_source = brain_job::git::Source::ALL[0];
    let alias_id = game.source(alias_source).await.unwrap();
    let alias_run = game
        .begin_run(alias_id, &"2".repeat(40))
        .await
        .unwrap()
        .unwrap();
    game.object(
        &alias_run,
        alias_source,
        "data/json/hero-data.json",
        Some(blob(br#"{"hero_ghost":{"Name":"Ghost"}}"#)),
        &mut game_counts,
    )
    .await
    .unwrap();
    game.publish(&alias_run).await.unwrap();
    let entity: i64 = sqlx::query_scalar(
        "SELECT entity_id FROM serve.current_alias WHERE alias_norm='ghost' LIMIT 1",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    let docs = DocsStore::start(pool.clone(), 300).await.unwrap();
    let source = docs.source().await.unwrap();
    let path = "public/billy.html";
    let content = html("Ghost hilft.");
    let mut counts = Counts::default();
    let run = docs
        .begin_run(source, &"a".repeat(40))
        .await
        .unwrap()
        .unwrap();
    docs.object(&run, path, Some(blob(content.as_bytes())), &mut counts)
        .await
        .unwrap();
    docs.publish(&run).await.unwrap();
    let linked: Vec<Vec<i64>> =
        sqlx::query_scalar("SELECT entity_ids FROM docs.current_section ORDER BY ordinal")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(linked, [Vec::<i64>::new(), Vec::<i64>::new()]);
    game.publish(&identity_run).await.unwrap();
    let run = docs
        .begin_run(source, &"b".repeat(40))
        .await
        .unwrap()
        .unwrap();
    docs.object(&run, path, Some(blob(content.as_bytes())), &mut counts)
        .await
        .unwrap();
    docs.publish(&run).await.unwrap();
    let linked: Vec<Vec<i64>> =
        sqlx::query_scalar("SELECT entity_ids FROM docs.current_section ORDER BY ordinal")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(linked, [Vec::<i64>::new(), vec![entity]]);
    assert_eq!(counts.raw_inserted, 1);
    assert_eq!(counts.raw_reused, 1);
    docs.finish(&counts, false).await.unwrap();
    game.finish(&game_counts, false).await.unwrap();
    pool.close().await;
}

fn fixture_git(path: &Path, args: &[&str], input: &[u8]) -> String {
    let mut child = Command::new("/usr/bin/git")
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_AUTHOR_NAME", "Fixture")
        .env("GIT_AUTHOR_EMAIL", "fixture@example.invalid")
        .env("GIT_COMMITTER_NAME", "Fixture")
        .env("GIT_COMMITTER_EMAIL", "fixture@example.invalid")
        .args(["--git-dir"])
        .arg(path)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(input).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

fn commit(path: &Path, content: Option<&[u8]>, mode: &str, base: Option<&str>) -> String {
    let content_oid =
        content.map(|content| fixture_git(path, &["hash-object", "-w", "--stdin"], content));
    let subtree = content_oid
        .map(|oid| format!("{mode} blob {oid}\tbilly.html\n"))
        .unwrap_or_default();
    let public = fixture_git(path, &["mktree"], subtree.as_bytes());
    let binary = fixture_git(path, &["hash-object", "-w", "--stdin"], &vec![0; 100_000]);
    let tree = fixture_git(
        path,
        &["mktree"],
        format!("100644 blob {binary}\tignored.bin\n040000 tree {public}\tpublic\n").as_bytes(),
    );
    let mut args = vec!["commit-tree", &tree];
    if let Some(base) = base {
        args.extend(["-p", base]);
    }
    fixture_git(path, &args, b"fixture\n")
}

#[tokio::test]
async fn git_regular_modes_filtering_incremental_deletion_and_unchanged_contract() {
    let state = tempfile::tempdir().unwrap();
    let repo = Repository::open(state.path(), SOURCE, 30).await.unwrap();
    let bare = state.path().join("public-gamefiles-v1/deadlock-docs.git");
    let html = html("Billy spielt.");
    let a = commit(&bare, Some(html.as_bytes()), "100644", None);
    let symlink = commit(&bare, Some(html.as_bytes()), "120000", Some(&a));
    assert!(docs::verify_modes(state.path(), &symlink, 30)
        .await
        .is_err());
    let deleted = commit(&bare, None, "100644", Some(&a));
    docs::verify_modes(state.path(), &a, 30).await.unwrap();
    let scratch = Scratch::new();
    let pool = scratch.prepare().await;
    let store = DocsStore::start(pool.clone(), 300).await.unwrap();
    let id = store.source().await.unwrap();
    let mut counts = Counts::default();
    let run = store.begin_run(id, &a).await.unwrap().unwrap();
    docs::process(&store, &repo, state.path(), &run, 30, 8192, &mut counts)
        .await
        .unwrap();
    assert_eq!(counts.raw_inserted, 1);
    assert_eq!(section_bodies(&pool).await.len(), 2);
    assert!(store.begin_run(id, &a).await.unwrap().is_none());
    let run = store.begin_run(id, &symlink).await.unwrap().unwrap();
    assert!(
        docs::process(&store, &repo, state.path(), &run, 30, 8192, &mut counts)
            .await
            .is_err()
    );
    assert_eq!(section_bodies(&pool).await.len(), 2);
    store.finish(&counts, true).await.unwrap();
    let resumed = DocsStore::start(pool.clone(), 300).await.unwrap();
    assert_eq!(resumed.pending(id).await.unwrap().unwrap().target, symlink);
    resumed.finish(&counts, true).await.unwrap();
    let scratch2 = Scratch::new();
    let pool2 = scratch2.prepare().await;
    let store2 = DocsStore::start(pool2.clone(), 300).await.unwrap();
    let id = store2.source().await.unwrap();
    let run = store2.begin_run(id, &a).await.unwrap().unwrap();
    docs::process(&store2, &repo, state.path(), &run, 30, 8192, &mut counts)
        .await
        .unwrap();
    let run = store2.begin_run(id, &deleted).await.unwrap().unwrap();
    assert!(!run.full);
    docs::process(&store2, &repo, state.path(), &run, 30, 8192, &mut counts)
        .await
        .unwrap();
    assert!(section_bodies(&pool2).await.is_empty());
    assert_eq!(counts.paths_deleted, 1);
    store2.finish(&counts, false).await.unwrap();
    pool2.close().await;
    pool.close().await;
}

#[tokio::test]
async fn extractor_backfill_reuses_stored_raw_and_the_second_run_is_idempotent() {
    let state = tempfile::tempdir().unwrap();
    let repository = Repository::open(state.path(), SOURCE, 30).await.unwrap();
    let bare = state.path().join("public-gamefiles-v1/deadlock-docs.git");
    let content = html("Billy spielt.");
    let target = commit(&bare, Some(content.as_bytes()), "100644", None);
    let path = "public/billy.html";
    let scratch = Scratch::new();
    let pool = scratch.prepare().await;
    let owner = scratch.pool("brain_docs_test", "brain_migrate").await;
    let mut tx = owner.begin().await.unwrap();
    let source: i64 =
        sqlx::query_scalar("INSERT INTO raw.source(kind,locator) VALUES ('git',$1) RETURNING id")
            .bind(SOURCE.url())
            .fetch_one(&mut *tx)
            .await
            .unwrap();
    let job: i64 = sqlx::query_scalar("INSERT INTO ops.job_run(job,extractor,extractor_version) VALUES ('ingest-docs',$1,'p1-v1') RETURNING id")
        .bind(extract::EXTRACTOR).fetch_one(&mut *tx).await.unwrap();
    let run: i64 = sqlx::query_scalar("INSERT INTO raw.run(source_id,job_run_id,target_revision,extractor,extractor_version) VALUES ($1,$2,$3,$4,'p1-v1') RETURNING id")
        .bind(source).bind(job).bind(&target).bind(extract::EXTRACTOR).fetch_one(&mut *tx).await.unwrap();
    let revision: i64 = sqlx::query_scalar("INSERT INTO canon.revision(source_id,raw_run_id,source_revision,extractor,extractor_version) VALUES ($1,$2,$3,$4,'p1-v1') RETURNING id")
        .bind(source).bind(run).bind(&target).bind(extract::EXTRACTOR).fetch_one(&mut *tx).await.unwrap();
    let object: i64 = sqlx::query_scalar("INSERT INTO raw.object(source_id,external_id,revision,content_sha256,media_type,body) VALUES ($1,$2,$3,$4,'text/html',$5) RETURNING id")
        .bind(source).bind(path).bind(&target).bind(Sha256::digest(content.as_bytes()).to_vec()).bind(content.as_bytes()).fetch_one(&mut *tx).await.unwrap();
    sqlx::query(
        "INSERT INTO raw.run_object(run_id,path,object_id,action) VALUES ($1,$2,$3,'upsert')",
    )
    .bind(run)
    .bind(path)
    .bind(object)
    .execute(&mut *tx)
    .await
    .unwrap();
    let page: i64 = sqlx::query_scalar("INSERT INTO docs.page(source_id,path,title,lang,raw_object_id,valid_from_revision_id) VALUES ($1,$2,'Billy','de',$3,$4) RETURNING id")
        .bind(source).bind(path).bind(object).bind(revision).fetch_one(&mut *tx).await.unwrap();
    let assertions = extract::extract(SOURCE, path, content.as_bytes()).unwrap();
    extract::prove(SOURCE, path, content.as_bytes(), &assertions).unwrap();
    for assertion in assertions {
        let metadata = docs_extract::metadata(&assertion).unwrap();
        let Atom::Text(body) = assertion.value else {
            panic!("docs text")
        };
        let receipt: i64 = sqlx::query_scalar("INSERT INTO canon.receipt(raw_object_id,source_id,revision_id,locator_kind,locator,value_sha256,extractor,extractor_version) VALUES ($1,$2,$3,'css_selector',$4,$5,$6,'p1-v1') RETURNING id")
            .bind(object).bind(source).bind(revision).bind(assertion.locator).bind(assertion.hash).bind(extract::EXTRACTOR).fetch_one(&mut *tx).await.unwrap();
        sqlx::query("INSERT INTO docs.section(page_id,ordinal,heading_path,body,url,receipt_id) VALUES ($1,$2,$3,$4,$5,$6)")
            .bind(page).bind(metadata.ordinal).bind(&metadata.heading_path).bind(body).bind(docs_extract::section_url(path, &target, &metadata).unwrap()).bind(receipt).execute(&mut *tx).await.unwrap();
    }
    sqlx::query("UPDATE raw.run_object SET parse_status='succeeded' WHERE run_id=$1")
        .bind(run)
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query("UPDATE raw.run SET status='succeeded' WHERE id=$1")
        .bind(run)
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query("UPDATE raw.source SET last_complete_revision=$2 WHERE id=$1")
        .bind(source)
        .bind(&target)
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query("INSERT INTO canon.source_head(source_id,published_revision_id) VALUES ($1,$2)")
        .bind(source)
        .bind(revision)
        .execute(&mut *tx)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    owner.close().await;
    let store = DocsStore::start(pool.clone(), 300).await.unwrap();
    let run = store.backfill_run(source).await.unwrap().unwrap();
    assert!(run.full);
    assert_eq!(run.target, target);
    assert_eq!(
        store.backfill_blob(&run, path).await.unwrap().bytes,
        content.as_bytes()
    );
    let mut counts = Counts::default();
    assert!(docs::process(
        &store,
        &repository,
        state.path(),
        &run,
        30,
        content.len() - 1,
        &mut counts,
    )
    .await
    .is_err());
    assert!(!store.completed(&run, path).await.unwrap());
    assert_eq!(counts.objects_completed, 0);
    docs::process(
        &store,
        &repository,
        state.path(),
        &run,
        30,
        8192,
        &mut counts,
    )
    .await
    .unwrap();
    assert_eq!(counts.raw_inserted, 0);
    assert_eq!(counts.raw_reused, 1);
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM raw.object")
            .fetch_one(&pool)
            .await
            .unwrap(),
        1
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM docs.page")
            .fetch_one(&pool)
            .await
            .unwrap(),
        2
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM canon.receipt")
            .fetch_one(&pool)
            .await
            .unwrap(),
        4
    );
    assert_eq!(section_bodies(&pool).await.len(), 2);
    assert!(store.backfill_run(source).await.unwrap().is_none());
    assert!(store.begin_run(source, &target).await.unwrap().is_none());
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM canon.receipt")
            .fetch_one(&pool)
            .await
            .unwrap(),
        4
    );
    store.finish(&counts, false).await.unwrap();
    pool.close().await;
}

#[tokio::test]
#[ignore]
async fn anonymous_latest_docs_html_replays_all_sections_and_second_run_adds_nothing() {
    let state = tempfile::tempdir().unwrap();
    let repository = Repository::open(state.path(), SOURCE, 300).await.unwrap();
    let revision = repository.fetch(SOURCE).await.unwrap();
    docs::verify_modes(state.path(), &revision, 30)
        .await
        .unwrap();
    let mut changes = repository.changes(None, &revision).unwrap();
    let mut blobs = repository.blobs(docs_extract::MAX_BYTES).unwrap();
    let mut public = 0;
    let mut internal = 0;
    let mut hero_pages = 0;
    let mut baba_mentions = 0;
    let mut section_count = 0;
    while let Some(change) = changes.next().await.unwrap() {
        if !docs_extract::supported(&change.path) {
            continue;
        }
        let blob = blobs.read(change.oid.as_deref().unwrap()).await.unwrap();
        let assertions = extract::extract(SOURCE, &change.path, &blob.bytes)
            .unwrap_or_else(|error| panic!("source path {}: {error}", change.path));
        extract::prove(SOURCE, &change.path, &blob.bytes, &assertions).unwrap();
        section_count += assertions.len();
        if change.path.starts_with("public/") {
            public += 1;
            hero_pages += usize::from(change.path.starts_with("public/deadlock-helden/"));
            for fact in &assertions {
                let Atom::Text(body) = &fact.value else {
                    panic!("docs text")
                };
                baba_mentions += usize::from(
                    !docs_extract::linked_entities(body, &[("baba".into(), 1), ("yaga".into(), 1)])
                        .unwrap()
                        .is_empty(),
                );
            }
        } else {
            internal += 1;
        }
        if change.path == "public/deadlock-helden/billy.html" {
            let assertion = assertions
                .iter()
                .find(|fact| fact.locator == "main > section[id=\"wie-spielt-man-billy\"]")
                .unwrap();
            let metadata = docs_extract::metadata(assertion).unwrap();
            assert_eq!((metadata.line_start, metadata.line_end), (35, 38));
            let url = docs_extract::section_url(&change.path, &revision, &metadata).unwrap();
            assert!(url.ends_with("#L35-L38"));
        }
    }
    blobs.finish().await.unwrap();
    assert!(public > 0 && internal > 0 && hero_pages > 0);
    println!("docs_source_revision={revision} public_pages={public} internal_pages={internal} hero_pages={hero_pages} sections={section_count} baba_or_yaga_sections={baba_mentions}");
    let scratch = Scratch::new();
    let pool = scratch.prepare().await;
    let store = DocsStore::start(pool.clone(), 300).await.unwrap();
    let source_id = store.source().await.unwrap();
    let mut counts = Counts::default();
    let run = store
        .begin_run(source_id, &revision)
        .await
        .unwrap()
        .unwrap();
    docs::process(
        &store,
        &repository,
        state.path(),
        &run,
        30,
        docs_extract::MAX_BYTES,
        &mut counts,
    )
    .await
    .unwrap();
    assert_eq!(counts.raw_inserted, (public + internal) as u64);
    let before: i64 = sqlx::query_scalar("SELECT count(*) FROM docs.section")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(before, section_count as i64);
    assert!(store
        .begin_run(source_id, &revision)
        .await
        .unwrap()
        .is_none());
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT count(*) FROM docs.section")
            .fetch_one(&pool)
            .await
            .unwrap(),
        before
    );
    store.finish(&counts, false).await.unwrap();
    let second = DocsStore::start(pool.clone(), 300).await.unwrap();
    assert!(second
        .begin_run(source_id, &revision)
        .await
        .unwrap()
        .is_none());
    let second_counts = Counts::default();
    second.finish(&second_counts, false).await.unwrap();
    assert_eq!(second_counts.raw_inserted, 0);
    pool.close().await;
}
