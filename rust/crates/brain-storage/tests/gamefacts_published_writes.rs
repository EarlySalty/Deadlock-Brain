use brain_storage::gamefacts::{
    Alias, Counts, Entity, Fact, FactSet, FactValue, GameFactsStore, Receipt, Relation,
};
use sha2::{Digest, Sha256};
use sqlx::{PgConnection, PgPool};
use tokio::task::JoinHandle;

#[path = "support/gamefacts_pg.rs"]
pub mod gamefacts_pg;

fn receipt(bytes: &[u8]) -> Receipt {
    Receipt {
        locator_kind: "raw_bytes".into(),
        locator: String::new(),
        value_sha256: Sha256::digest(bytes).into(),
    }
}

fn set(bytes: &[u8]) -> FactSet {
    FactSet {
        entities: ["hero_first", "hero_second"]
            .into_iter()
            .map(|key| Entity {
                kind: "hero".into(),
                key: key.into(),
                receipt: receipt(bytes),
            })
            .collect(),
        aliases: vec![Alias {
            entity_key: "hero_first".into(),
            alias: "First".into(),
            lang: "de".into(),
            receipt: receipt(bytes),
        }],
        facts: vec![Fact {
            entity_key: "hero_first".into(),
            stat_key: "health".into(),
            value: FactValue::Number(120.0),
            receipt: receipt(bytes),
        }],
        relations: vec![Relation {
            from_key: "hero_first".into(),
            kind: "related".into(),
            to_key: "hero_second".into(),
            receipt: receipt(bytes),
        }],
    }
}

async fn assert_login(pool: &PgPool) {
    let users: (String, String, bool) = sqlx::query_as("SELECT session_user::text,current_user::text,(SELECT rolsuper FROM pg_roles WHERE rolname=current_user)")
        .fetch_one(pool).await.unwrap();
    assert_eq!(users, ("brain_ingest".into(), "brain_ingest".into(), false));
    let version: i32 = sqlx::query_scalar("SELECT current_setting('server_version_num')::integer")
        .fetch_one(pool)
        .await
        .unwrap();
    assert!((160000..170000).contains(&version));
    let denied = sqlx::query("SELECT * FROM serve.published_receipt LIMIT 0")
        .execute(pool)
        .await
        .unwrap_err();
    assert_eq!(
        denied.as_database_error().unwrap().code().as_deref(),
        Some("42501")
    );
}

async fn snapshot(pool: &PgPool) -> Vec<Vec<serde_json::Value>> {
    let mut rows = Vec::new();
    for table in [
        "receipt",
        "entity",
        "entity_identity",
        "alias",
        "fact",
        "relation",
    ] {
        rows.push(
            sqlx::query_scalar(&format!(
                "SELECT to_jsonb(r) FROM canon.{table} r ORDER BY to_jsonb(r)::text"
            ))
            .fetch_all(pool)
            .await
            .unwrap(),
        );
    }
    rows
}

fn assert_denied(result: Result<(), sqlx::Error>) {
    assert_eq!(
        result
            .unwrap_err()
            .as_database_error()
            .unwrap()
            .code()
            .as_deref(),
        Some("23514")
    );
}

struct InsertCase {
    table: &'static str,
    columns: &'static str,
    values: Vec<String>,
    conflict: Option<Vec<String>>,
}

impl InsertCase {
    async fn insert(
        &self,
        connection: &mut PgConnection,
        conflict: bool,
        ignore: bool,
    ) -> Result<(), sqlx::Error> {
        let values = if conflict {
            self.conflict.as_ref().unwrap()
        } else {
            &self.values
        };
        sqlx::query(&format!(
            "INSERT INTO canon.{}({}) VALUES({}){}",
            self.table,
            self.columns,
            values.join(","),
            if ignore {
                " ON CONFLICT DO NOTHING"
            } else {
                ""
            }
        ))
        .execute(connection)
        .await
        .map(|_| ())
    }

    async fn copy(&self, connection: &mut PgConnection) -> Result<(), sqlx::Error> {
        let values = self
            .values
            .iter()
            .map(|v| v.trim_matches('\'').replace('\\', "\\\\"))
            .collect::<Vec<_>>()
            .join("\t")
            + "\n";
        let mut copy = connection
            .copy_in_raw(&format!(
                "COPY canon.{}({}) FROM STDIN",
                self.table, self.columns
            ))
            .await?;
        copy.send(values.as_bytes()).await?;
        copy.finish().await.map(|_| ())
    }
}

async fn cases(pool: &PgPool, raw: i64, run: i64) -> Vec<InsertCase> {
    let ids: Vec<i64> = sqlx::query_scalar(
        "SELECT id FROM canon.entity WHERE key IN ('hero_first','hero_second') ORDER BY key",
    )
    .fetch_all(pool)
    .await
    .unwrap();
    let first = ids[0];
    let second = ids[1];
    let base: i64 = sqlx::query_scalar("SELECT i.receipt_id FROM canon.entity_identity i JOIN canon.receipt r ON r.id=i.receipt_id WHERE i.entity_id=$1 AND r.ingest_run_id=$2 ORDER BY r.id LIMIT 1")
        .bind(first).bind(run).fetch_one(pool).await.unwrap();
    let alias: i64 = sqlx::query_scalar("SELECT a.receipt_id FROM canon.alias a JOIN canon.receipt r ON r.id=a.receipt_id WHERE r.ingest_run_id=$1 ORDER BY r.id LIMIT 1")
        .bind(run).fetch_one(pool).await.unwrap();
    let fact: i64 = sqlx::query_scalar(
        "SELECT receipt_id FROM canon.fact WHERE valid_from_run=$1 ORDER BY id LIMIT 1",
    )
    .bind(run)
    .fetch_one(pool)
    .await
    .unwrap();
    let relation: i64 = sqlx::query_scalar("SELECT l.receipt_id FROM canon.relation l JOIN canon.receipt r ON r.id=l.receipt_id WHERE r.ingest_run_id=$1 ORDER BY r.id LIMIT 1")
        .bind(run).fetch_one(pool).await.unwrap();
    vec![
        InsertCase {
            table: "receipt",
            columns: "raw_object_id,ingest_run_id,extractor,locator_kind,locator,value_sha256",
            values: vec![
                raw.to_string(),
                run.to_string(),
                "'late'".into(),
                "'raw_bytes'".into(),
                "''".into(),
                format!("'\\x{}'", "00".repeat(32)),
            ],
            conflict: None,
        },
        InsertCase {
            table: "entity",
            columns: "kind,key,receipt_id",
            values: vec!["'hero'".into(), "'hero_late'".into(), base.to_string()],
            conflict: Some(vec![
                "'hero'".into(),
                "'hero_first'".into(),
                base.to_string(),
            ]),
        },
        InsertCase {
            table: "entity_identity",
            columns: "entity_id,receipt_id",
            values: vec![second.to_string(), base.to_string()],
            conflict: Some(vec![first.to_string(), base.to_string()]),
        },
        InsertCase {
            table: "alias",
            columns: "entity_id,alias_norm,lang,receipt_id",
            values: vec![
                first.to_string(),
                "'late'".into(),
                "'de'".into(),
                alias.to_string(),
            ],
            conflict: Some(vec![
                first.to_string(),
                "'first'".into(),
                "'de'".into(),
                alias.to_string(),
            ]),
        },
        InsertCase {
            table: "fact",
            columns: "entity_id,stat_key,value_num,valid_from_run,receipt_id",
            values: vec![
                first.to_string(),
                "'late'".into(),
                "999".into(),
                run.to_string(),
                fact.to_string(),
            ],
            conflict: Some(vec![
                first.to_string(),
                "'health'".into(),
                "999".into(),
                run.to_string(),
                fact.to_string(),
            ]),
        },
        InsertCase {
            table: "relation",
            columns: "from_entity,kind,to_entity,receipt_id",
            values: vec![
                second.to_string(),
                "'late'".into(),
                first.to_string(),
                relation.to_string(),
            ],
            conflict: Some(vec![
                first.to_string(),
                "'related'".into(),
                second.to_string(),
                relation.to_string(),
            ]),
        },
    ]
}

async fn seed(store: &GameFactsStore, path: &str) -> (i64, brain_storage::gamefacts::RawObject) {
    let run = store.begin_run("published_test").await.unwrap();
    let raw = store
        .capture(run, "published_test", "r1", path, b"{}".to_vec())
        .await
        .unwrap();
    store
        .apply(run, &raw, "seed", &set(&raw.bytes))
        .await
        .unwrap();
    (run, raw)
}

#[tokio::test]
async fn real_ingest_rejects_published_insert_copy_and_conflict_paths_on_every_canonical_table() {
    let (_pg, admin, writer) = gamefacts_pg::fixture(2).await;
    assert_login(&writer).await;
    let store = GameFactsStore::new(writer.clone());
    let (run, raw) = seed(&store, "matrix").await;
    let cases = cases(&writer, raw.id, run).await;
    store
        .finish_run(run, true, Counts::default())
        .await
        .unwrap();
    let before = snapshot(&admin).await;
    let mut connection = writer.acquire().await.unwrap();
    let mut attempts = 0;
    for case in &cases {
        for ignore in [false, true] {
            assert_denied(case.insert(&mut connection, false, ignore).await);
            attempts += 1;
        }
        if case.conflict.is_some() {
            assert_denied(case.insert(&mut connection, true, true).await);
            attempts += 1;
        }
        assert_denied(case.copy(&mut connection).await);
        attempts += 1;
        assert_eq!(snapshot(&admin).await, before, "{}", case.table);
    }
    assert_eq!(attempts, 23);
    drop(connection);
    let failed = store.begin_run("published_test").await.unwrap();
    let published_receipt: i64 = sqlx::query_scalar("SELECT receipt_id FROM canon.fact LIMIT 1")
        .fetch_one(&writer)
        .await
        .unwrap();
    assert_denied(sqlx::query("INSERT INTO canon.fact(entity_id,stat_key,value_num,valid_from_run,receipt_id) SELECT entity_id,'mismatch',999,$1,$2 FROM canon.fact LIMIT 1")
        .bind(failed).bind(published_receipt).execute(&writer).await.map(|_| ()));
    assert_eq!(snapshot(&admin).await, before);
    let no_update = sqlx::query("UPDATE canon.entity SET kind='item'")
        .execute(&writer)
        .await
        .unwrap_err();
    assert_eq!(
        no_update.as_database_error().unwrap().code().as_deref(),
        Some("42501")
    );
    assert!(!store.was_applied(raw.id, "late").await.unwrap());
    for twin in [
        FactSet::default(),
        FactSet {
            entities: set(&raw.bytes).entities,
            ..FactSet::default()
        },
        FactSet {
            aliases: set(&raw.bytes).aliases,
            ..FactSet::default()
        },
        FactSet {
            facts: set(&raw.bytes).facts,
            ..FactSet::default()
        },
        FactSet {
            relations: set(&raw.bytes).relations,
            ..FactSet::default()
        },
    ] {
        let denied = store.apply(run, &raw, "late", &twin).await.unwrap_err();
        assert_eq!(denied.code, "sql_write");
        assert!(denied.detail.contains("23514"));
        assert_eq!(snapshot(&admin).await, before);
        assert!(!store.was_applied(raw.id, "late").await.unwrap());
    }
    writer.close().await;
    admin.close().await;
}

#[tokio::test]
async fn failed_run_accepts_insert_copy_and_conflicts_with_transactional_rollback() {
    let (_pg, admin, writer) = gamefacts_pg::fixture(2).await;
    assert_login(&writer).await;
    let store = GameFactsStore::new(writer.clone());
    let (run, raw) = seed(&store, "failed_matrix").await;
    let before = snapshot(&admin).await;
    for case in cases(&writer, raw.id, run).await {
        for ignore in [false, true] {
            let mut transaction = writer.begin().await.unwrap();
            case.insert(&mut transaction, false, ignore).await.unwrap();
            transaction.rollback().await.unwrap();
            assert_eq!(snapshot(&admin).await, before, "{}", case.table);
        }
        if case.conflict.is_some() {
            let mut transaction = writer.begin().await.unwrap();
            case.insert(&mut transaction, true, true).await.unwrap();
            transaction.commit().await.unwrap();
            assert_eq!(snapshot(&admin).await, before, "{}", case.table);
        }
        let mut transaction = writer.begin().await.unwrap();
        case.copy(&mut transaction).await.unwrap();
        transaction.rollback().await.unwrap();
        assert_eq!(snapshot(&admin).await, before, "{}", case.table);
    }
    writer.close().await;
    admin.close().await;
}

async fn assert_waiting(
    admin: &PgPool,
    pid: i32,
    waiter: &mut JoinHandle<Result<(), sqlx::Error>>,
) {
    loop {
        tokio::select! {
            result = &mut *waiter => panic!("publication or insert bypassed the run lock: {result:?}"),
            blocked = sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE pid=$1 AND wait_event_type='Lock')")
                .bind(pid).fetch_one(admin) => {
                if blocked.unwrap() { return; }
            }
        }
    }
}

#[tokio::test]
async fn each_guard_serializes_both_publication_race_orderings() {
    let (_pg, admin, writer) = gamefacts_pg::fixture(3).await;
    assert_login(&writer).await;
    let store = GameFactsStore::new(writer.clone());
    for index in 0..6 {
        let (run, raw) = seed(&store, &format!("publisher_first_{index}")).await;
        let case = cases(&writer, raw.id, run).await.remove(index);
        let mut publication = writer.begin().await.unwrap();
        sqlx::query("UPDATE ops.ingest_run SET status='succeeded' WHERE id=$1")
            .bind(run)
            .execute(&mut *publication)
            .await
            .unwrap();
        let mut connection = writer.acquire().await.unwrap();
        let pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
            .fetch_one(&mut *connection)
            .await
            .unwrap();
        let mut insertion =
            tokio::spawn(async move { case.insert(&mut connection, false, true).await });
        assert_waiting(&admin, pid, &mut insertion).await;
        publication.commit().await.unwrap();
        let before = snapshot(&admin).await;
        assert_denied(insertion.await.unwrap());
        assert_eq!(snapshot(&admin).await, before);

        let (run, raw) = seed(&store, &format!("writer_first_{index}")).await;
        let case = cases(&writer, raw.id, run).await.remove(index);
        let mut insertion = writer.begin().await.unwrap();
        case.insert(&mut insertion, false, false).await.unwrap();
        let mut connection = writer.acquire().await.unwrap();
        let pid: i32 = sqlx::query_scalar("SELECT pg_backend_pid()")
            .fetch_one(&mut *connection)
            .await
            .unwrap();
        let mut publication = tokio::spawn(async move {
            sqlx::query("UPDATE ops.ingest_run SET status='succeeded' WHERE id=$1")
                .bind(run)
                .execute(&mut *connection)
                .await
                .map(|_| ())
        });
        assert_waiting(&admin, pid, &mut publication).await;
        insertion.commit().await.unwrap();
        publication.await.unwrap().unwrap();
        let status: String = sqlx::query_scalar("SELECT status FROM ops.ingest_run WHERE id=$1")
            .bind(run)
            .fetch_one(&writer)
            .await
            .unwrap();
        assert_eq!(status, "succeeded");
    }
    writer.close().await;
    admin.close().await;
}

#[tokio::test]
async fn historical_raw_and_published_entities_accept_new_failed_run_references_and_rollback() {
    let (_pg, admin, writer) = gamefacts_pg::fixture(2).await;
    assert_login(&writer).await;
    let store = GameFactsStore::new(writer.clone());
    let (first, original) = seed(&store, "historical").await;
    store
        .finish_run(first, true, Counts::default())
        .await
        .unwrap();
    let before = snapshot(&admin).await;
    let next = store.begin_run("published_test").await.unwrap();
    let reused = store
        .capture(
            next,
            "published_test",
            "r2",
            "historical",
            original.bytes.clone(),
        )
        .await
        .unwrap();
    assert_eq!(reused.id, original.id);
    let raw_run: i64 = sqlx::query_scalar("SELECT ingest_run_id FROM raw.object WHERE id=$1")
        .bind(reused.id)
        .fetch_one(&writer)
        .await
        .unwrap();
    assert_eq!(raw_run, first);
    let mut transaction = writer.begin().await.unwrap();
    let marker: i64 = sqlx::query_scalar("INSERT INTO canon.receipt(raw_object_id,ingest_run_id,extractor,locator_kind,locator,value_sha256) VALUES($1,$2,'rollback','raw_bytes','',decode(repeat('00',32),'hex')) RETURNING id")
        .bind(reused.id).bind(next).fetch_one(&mut *transaction).await.unwrap();
    sqlx::query("INSERT INTO canon.alias(entity_id,alias_norm,lang,receipt_id) SELECT id,'rolled back','de',$1 FROM canon.entity WHERE key='hero_first'")
        .bind(marker).execute(&mut *transaction).await.unwrap();
    transaction.rollback().await.unwrap();
    assert_eq!(snapshot(&admin).await, before);
    store
        .apply(next, &reused, "fresh", &set(&reused.bytes))
        .await
        .unwrap();
    store
        .apply(next, &reused, "empty", &FactSet::default())
        .await
        .unwrap();
    let old_bases: i64 = sqlx::query_scalar("SELECT count(*) FROM canon.entity e JOIN canon.receipt r ON r.id=e.receipt_id WHERE r.ingest_run_id=$1")
        .bind(first).fetch_one(&writer).await.unwrap();
    assert_eq!(old_bases, 2);
    let new_references: (i64, i64, i64, i64, i64) = sqlx::query_as("SELECT (SELECT count(*) FROM canon.receipt WHERE ingest_run_id=$1),(SELECT count(*) FROM canon.entity_identity i JOIN canon.receipt r ON r.id=i.receipt_id WHERE r.ingest_run_id=$1),(SELECT count(*) FROM canon.alias a JOIN canon.receipt r ON r.id=a.receipt_id WHERE r.ingest_run_id=$1),(SELECT count(*) FROM canon.fact WHERE valid_from_run=$1),(SELECT count(*) FROM canon.relation l JOIN canon.receipt r ON r.id=l.receipt_id WHERE r.ingest_run_id=$1)")
        .bind(next).fetch_one(&writer).await.unwrap();
    assert_eq!(new_references, (6, 2, 1, 1, 1));
    assert!(!store.was_applied(reused.id, "empty").await.unwrap());
    store
        .finish_run(next, true, Counts::default())
        .await
        .unwrap();
    assert!(store.was_applied(reused.id, "empty").await.unwrap());
    let update = sqlx::query("UPDATE canon.receipt SET value_sha256=decode(repeat('01',32),'hex') WHERE ingest_run_id=$1")
        .bind(next).execute(&admin).await.unwrap();
    assert_eq!(update.rows_affected(), 6);
    writer.close().await;
    admin.close().await;
}
