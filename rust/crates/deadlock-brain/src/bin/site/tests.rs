use super::*;
use axum::http::StatusCode;
use brain_storage::{
    gamefacts::{Counts, Entity, Fact, FactSet, FactValue, GameFactsStore, Receipt, Relation},
    PgStore,
};
use reqwest::Client;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use std::{fs, path::PathBuf};

#[path = "../../../../brain-storage/tests/support/scratch_pg.rs"]
mod scratch_pg;

#[path = "../../../../brain-storage/tests/support/playable_hero.rs"]
mod assets_fixture;

async fn publish(
    pool: &PgPool,
    kind: &str,
    key: &str,
    name: &str,
    health: f64,
    succeeded: bool,
) -> i64 {
    let store = GameFactsStore::new(pool.clone());
    let run = store.begin_run("site-fixture").await.unwrap();
    let bytes = serde_json::to_vec(&json!({"name":name,"health":health})).unwrap();
    let raw = store
        .capture(
            run,
            "site-fixture",
            &format!("observed-{run}"),
            &format!("{key}.json"),
            bytes.clone(),
        )
        .await
        .unwrap();
    let receipt = |locator: &str, token: Vec<u8>| Receipt {
        locator_kind: "json_pointer".into(),
        locator: locator.into(),
        value_sha256: Sha256::digest(token).into(),
    };
    let set = FactSet {
        entities: vec![Entity {
            kind: kind.into(),
            key: key.into(),
            receipt: Receipt {
                locator_kind: "raw_bytes".into(),
                locator: String::new(),
                value_sha256: Sha256::digest(&bytes).into(),
            },
        }],
        facts: vec![
            Fact {
                entity_key: key.into(),
                stat_key: "name_en".into(),
                value: FactValue::Text(name.into()),
                receipt: receipt("/name", serde_json::to_vec(name).unwrap()),
            },
            Fact {
                entity_key: key.into(),
                stat_key: "health".into(),
                value: FactValue::Number(health),
                receipt: receipt("/health", serde_json::to_vec(&health).unwrap()),
            },
        ],
        ..FactSet::default()
    };
    store
        .apply(run, &raw, "site-fixture-v1", &set)
        .await
        .unwrap();
    store
        .finish_run(
            run,
            succeeded,
            Counts {
                accepted: 1,
                ..Counts::default()
            },
        )
        .await
        .unwrap();
    sqlx::query_scalar("SELECT id FROM canon.entity WHERE key=$1")
        .bind(key)
        .fetch_one(pool)
        .await
        .unwrap()
}

async fn relate(pool: &PgPool, from: &str, to: &str) {
    let store = GameFactsStore::new(pool.clone());
    let run = store.begin_run("site-fixture").await.unwrap();
    let bytes = serde_json::to_vec(&json!({"from":from,"to":to})).unwrap();
    let raw = store
        .capture(
            run,
            "site-fixture",
            &format!("observed-{run}"),
            &format!("relation-{from}-{to}.json"),
            bytes.clone(),
        )
        .await
        .unwrap();
    let set = FactSet {
        relations: vec![Relation {
            from_key: from.into(),
            kind: "related_to".into(),
            to_key: to.into(),
            receipt: Receipt {
                locator_kind: "raw_bytes".into(),
                locator: String::new(),
                value_sha256: Sha256::digest(&bytes).into(),
            },
        }],
        ..FactSet::default()
    };
    store
        .apply(run, &raw, "site-fixture-v1", &set)
        .await
        .unwrap();
    store
        .finish_run(run, true, Counts::default())
        .await
        .unwrap();
}

async fn doc_receipt(
    connection: &mut sqlx::PgConnection,
    raw: i64,
    run: i64,
    section: bool,
) -> i64 {
    sqlx::query_scalar("INSERT INTO canon.receipt(raw_object_id,ingest_run_id,extractor,locator_kind,locator,value_sha256) VALUES($1,$2,'docs',$3,$4,$5) RETURNING id")
        .bind(raw)
        .bind(run)
        .bind(if section { "byte_ranges" } else { "bytes" })
        .bind(if section { "[[0,4]]" } else { "0:4" })
        .bind(vec![1_u8; 32])
        .fetch_one(connection)
        .await
        .unwrap()
}

async fn publish_doc(pool: &PgPool, path: &str, version: u16, title: &str, body: &str) {
    let mut tx = pool.begin().await.unwrap();
    let run: i64 = sqlx::query_scalar(
        "INSERT INTO ops.ingest_run(source,status) VALUES('deadlock-docs','failed') RETURNING id",
    )
    .fetch_one(&mut *tx)
    .await
    .unwrap();
    let raw: i64 = sqlx::query_scalar("INSERT INTO raw.object(source,revision,path,source_path_sha256,content_sha256,bytes,ingest_run_id) VALUES('deadlock-docs',$1,$2,$3,$4,$5,$6) RETURNING id")
        .bind(format!("{version:040x}"))
        .bind(path)
        .bind(Sha256::digest(format!("{path}:{version}").as_bytes()).to_vec())
        .bind(Sha256::digest(version.to_string().as_bytes()).to_vec())
        .bind(b"test".to_vec())
        .bind(run)
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    let identity = doc_receipt(&mut tx, raw, run, false).await;
    let entity: i64 = sqlx::query_scalar(
        "INSERT INTO canon.entity(kind,key,receipt_id) VALUES('doc_page',$1,$2) RETURNING id",
    )
    .bind(format!("doc_{version}"))
    .bind(identity)
    .fetch_one(&mut *tx)
    .await
    .unwrap();
    sqlx::query("INSERT INTO canon.entity_identity(entity_id,receipt_id) VALUES($1,$2)")
        .bind(entity)
        .bind(identity)
        .execute(&mut *tx)
        .await
        .unwrap();
    let meta = json!({"ordinal":0,"locator":"intro","heading_path":[title],"line_start":1,"line_end":1,"chunks":1}).to_string();
    for (key, value, section) in [
        ("doc_path", path, false),
        ("doc_title", title, false),
        ("doc_lang", "de", false),
        ("doc_intro_meta", meta.as_str(), true),
        ("doc_intro_body_0", body, true),
    ] {
        let receipt = doc_receipt(&mut tx, raw, run, section).await;
        sqlx::query("INSERT INTO canon.fact(entity_id,stat_key,value_text,valid_from_run,receipt_id) VALUES($1,$2,$3,$4,$5)")
            .bind(entity)
            .bind(key)
            .bind(value)
            .bind(run)
            .bind(receipt)
            .execute(&mut *tx)
            .await
            .unwrap();
    }
    let count = doc_receipt(&mut tx, raw, run, false).await;
    sqlx::query("INSERT INTO canon.fact(entity_id,stat_key,value_num,valid_from_run,receipt_id) VALUES($1,'doc_section_count',1,$2,$3)")
        .bind(entity)
        .bind(run)
        .bind(count)
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query("UPDATE ops.ingest_run SET status='succeeded',finished=clock_timestamp() WHERE id=$1 AND status='failed'")
        .bind(run)
        .execute(&mut *tx)
        .await
        .unwrap();
    tx.commit().await.unwrap();
}

async fn insert_doc_fact(
    connection: &mut sqlx::PgConnection,
    entity: i64,
    run: i64,
    receipt: i64,
    key: &str,
    value: &str,
) {
    sqlx::query("INSERT INTO canon.fact(entity_id,stat_key,value_text,valid_from_run,receipt_id) VALUES($1,$2,$3,$4,$5)")
        .bind(entity)
        .bind(key)
        .bind(value)
        .bind(run)
        .bind(receipt)
        .execute(connection)
        .await
        .unwrap();
}

fn corpus_path(page: usize) -> String {
    let scope = if page < 116 { "public" } else { "internal" };
    format!("{scope}/korpus/seite-{page}.html")
}

async fn publish_doc_corpus(pool: &PgPool, pages: usize) -> i64 {
    let mut tx = pool.begin().await.unwrap();
    let run: i64 = sqlx::query_scalar(
        "INSERT INTO ops.ingest_run(source,status) VALUES('deadlock-docs','failed') RETURNING id",
    )
    .fetch_one(&mut *tx)
    .await
    .unwrap();
    let revision = format!("{:040x}", 0xc0ffee_u32);
    for page in 0..pages {
        let path = corpus_path(page);
        let sections: u32 = match page {
            42 => 40,
            0..87 => 6,
            _ => 5,
        };
        let raw: i64 = sqlx::query_scalar("INSERT INTO raw.object(source,revision,path,source_path_sha256,content_sha256,bytes,ingest_run_id) VALUES('deadlock-docs',$1,$2,$3,$4,$5,$6) RETURNING id")
            .bind(&revision)
            .bind(&path)
            .bind(Sha256::digest(path.as_bytes()).to_vec())
            .bind(Sha256::digest(format!("inhalt:{path}").as_bytes()).to_vec())
            .bind(b"test".to_vec())
            .bind(run)
            .fetch_one(&mut *tx)
            .await
            .unwrap();
        let identity = doc_receipt(&mut tx, raw, run, false).await;
        let entity: i64 = sqlx::query_scalar(
            "INSERT INTO canon.entity(kind,key,receipt_id) VALUES('doc_page',$1,$2) RETURNING id",
        )
        .bind(format!("doc_korpus_{page}"))
        .bind(identity)
        .fetch_one(&mut *tx)
        .await
        .unwrap();
        sqlx::query("INSERT INTO canon.entity_identity(entity_id,receipt_id) VALUES($1,$2)")
            .bind(entity)
            .bind(identity)
            .execute(&mut *tx)
            .await
            .unwrap();
        let title = format!("Korpusseite {page}");
        for (key, value) in [
            ("doc_path", path.as_str()),
            ("doc_title", title.as_str()),
            ("doc_lang", "de"),
        ] {
            let receipt = doc_receipt(&mut tx, raw, run, false).await;
            insert_doc_fact(&mut tx, entity, run, receipt, key, value).await;
        }
        for ordinal in 0..sections {
            let prefix = if ordinal == 0 {
                "doc_intro".to_owned()
            } else {
                format!("doc_section_{ordinal}")
            };
            let meta = json!({"ordinal":ordinal,"locator":format!("abschnitt-{ordinal}"),"heading_path":[title.clone(),format!("Abschnitt {ordinal}")],"line_start":ordinal+1,"line_end":ordinal+1,"chunks":1}).to_string();
            let body =
                format!("Seite {page} Abschnitt {ordinal}: Seelen, Items und Helden im Überblick.");
            let receipt = doc_receipt(&mut tx, raw, run, true).await;
            insert_doc_fact(
                &mut tx,
                entity,
                run,
                receipt,
                &format!("{prefix}_meta"),
                &meta,
            )
            .await;
            let receipt = doc_receipt(&mut tx, raw, run, true).await;
            insert_doc_fact(
                &mut tx,
                entity,
                run,
                receipt,
                &format!("{prefix}_body_0"),
                &body,
            )
            .await;
        }
        let count = doc_receipt(&mut tx, raw, run, false).await;
        sqlx::query("INSERT INTO canon.fact(entity_id,stat_key,value_num,valid_from_run,receipt_id) VALUES($1,'doc_section_count',$2,$3,$4)")
            .bind(entity)
            .bind(f64::from(sections))
            .bind(run)
            .bind(count)
            .execute(&mut *tx)
            .await
            .unwrap();
    }
    sqlx::query("UPDATE ops.ingest_run SET status='succeeded',finished=clock_timestamp() WHERE id=$1 AND status='failed'")
        .bind(run)
        .execute(&mut *tx)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    run
}

async fn audit_doc_corpus(pool: &PgPool, run: i64, rejected_path: &str) {
    let audit: i64 =
        sqlx::query_scalar("INSERT INTO ops.receipt_audit_run DEFAULT VALUES RETURNING id")
            .fetch_one(pool)
            .await
            .unwrap();
    sqlx::query("INSERT INTO ops.receipt_audit(run_id,receipt_id,outcome,observed_value_sha256) SELECT $1,f.receipt_id,'match',sha256('match'::bytea) FROM canon.fact f JOIN canon.receipt r ON r.id=f.receipt_id WHERE r.ingest_run_id=$2 AND f.value_text IS DISTINCT FROM $3 ORDER BY f.receipt_id LIMIT 199")
        .bind(audit)
        .bind(run)
        .bind(rejected_path)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO ops.receipt_audit(run_id,receipt_id,outcome,observed_value_sha256) SELECT $1,f.receipt_id,'mismatch',sha256('mismatch'::bytea) FROM canon.fact f JOIN canon.receipt r ON r.id=f.receipt_id WHERE r.ingest_run_id=$2 AND f.stat_key='doc_path' AND f.value_text=$3")
        .bind(audit)
        .bind(run)
        .bind(rejected_path)
        .execute(pool)
        .await
        .unwrap();
}

const SITE_DOC_PAGE: &str = "SELECT title,body,source_revision,receipt_id::text AS citation FROM serve.site_doc_page($1) ORDER BY ordinal LIMIT 65";

async fn site_doc_lookup_ms(site: &mut sqlx::PgConnection, path: &str) -> (f64, f64, f64) {
    let mut tx = sqlx::Connection::begin(site).await.unwrap();
    sqlx::query("SET LOCAL statement_timeout='500ms'")
        .execute(&mut *tx)
        .await
        .unwrap();
    let plan: Value =
        sqlx::query_scalar(&format!("EXPLAIN (ANALYZE, FORMAT JSON) {SITE_DOC_PAGE}"))
            .bind(path)
            .fetch_one(&mut *tx)
            .await
            .unwrap();
    let started = std::time::Instant::now();
    let rows = sqlx::query(SITE_DOC_PAGE)
        .bind(path)
        .fetch_all(&mut *tx)
        .await
        .unwrap();
    let wall = started.elapsed().as_secs_f64() * 1000.0;
    assert_eq!(rows.len(), 40);
    tx.commit().await.unwrap();
    (
        plan[0]["Planning Time"].as_f64().unwrap(),
        plan[0]["Execution Time"].as_f64().unwrap(),
        wall,
    )
}

async fn explain(pool: &PgPool, sql: &str, path: &str) -> String {
    let lines: Vec<String> = sqlx::query_scalar(&format!("EXPLAIN (ANALYZE, BUFFERS) {sql}"))
        .bind(path)
        .fetch_all(pool)
        .await
        .unwrap();
    format!("{}\n", lines.join("\n"))
}

#[tokio::test]
async fn published_site_contract_and_baba_visual_proof() {
    let pg = scratch_pg::ScratchPg::start();
    let options = PgConnectOptions::new_without_pgpass()
        .host(&pg.directory.join("socket").to_string_lossy())
        .port(55439)
        .username("brain_core_test")
        .database("postgres")
        .password("")
        .ssl_mode(PgSslMode::Disable);
    let admin = PgPoolOptions::new()
        .max_connections(3)
        .connect_with(options.clone())
        .await
        .unwrap();
    sqlx::query("CREATE DATABASE brain")
        .execute(&admin)
        .await
        .unwrap();
    sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest LOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly NOLOGIN;")
        .execute(&admin).await.unwrap();
    admin.close().await;
    let options = options.database("brain");
    let admin = PgPoolOptions::new()
        .max_connections(3)
        .connect_with(options.clone())
        .await
        .unwrap();
    sqlx::raw_sql("GRANT CREATE ON DATABASE brain TO brain_migrate; GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate").execute(&admin).await.unwrap();
    PgStore::new(admin.clone())
        .migrate_gamefacts()
        .await
        .unwrap();
    let hero = publish(&admin, "hero", "hero_baba", "Baba", 780.0, true).await;
    let other = publish(&admin, "hero", "hero_abrams", "Abrams", 800.0, true).await;
    publish(&admin, "item", "upgrade_fixture", "Prüfitem", 25.0, true).await;
    let retired = publish(&admin, "hero", "hero_retired", "Ausgemustert", 700.0, true).await;
    let dummy = publish(&admin, "hero", "hero_target_dummy", "Zielpuppe", 1.0, true).await;
    assets_fixture::flagged(
        &admin,
        "hero_baba",
        json!({"player_selectable":true,"disabled":false,"in_development":true}),
    )
    .await;
    assets_fixture::playable(&admin, "hero_abrams").await;
    assets_fixture::flagged(
        &admin,
        "hero_retired",
        json!({"player_selectable":false,"disabled":true,"in_development":false}),
    )
    .await;
    publish(
        &admin,
        "ability",
        "ability_fixture",
        "Prüffähigkeit",
        5.0,
        true,
    )
    .await;
    for owner in ["hero_baba", "hero_retired", "hero_target_dummy"] {
        for target in ["upgrade_fixture", "ability_fixture"] {
            relate(&admin, owner, target).await;
        }
    }
    publish(
        &admin,
        "hero",
        "hero_hidden",
        "Nicht veröffentlicht",
        999.0,
        false,
    )
    .await;
    sqlx::raw_sql("CREATE SCHEMA brain; CREATE TABLE brain.site_comments_v1(text text); CREATE FUNCTION brain.append_site_comment_v1(text,text,text) RETURNS void LANGUAGE sql AS 'SELECT'; REVOKE ALL ON SCHEMA brain FROM PUBLIC; REVOKE ALL ON FUNCTION brain.append_site_comment_v1(text,text,text) FROM PUBLIC; CREATE FUNCTION brain.read_compare_artifact_v1(text) RETURNS jsonb LANGUAGE sql SECURITY DEFINER AS 'SELECT NULL::jsonb'; CREATE FUNCTION brain.read_compare_artifact_v2(text) RETURNS jsonb LANGUAGE sql SECURITY DEFINER AS 'SELECT NULL::jsonb'; REVOKE ALL ON FUNCTION brain.read_compare_artifact_v1(text), brain.read_compare_artifact_v2(text) FROM PUBLIC; GRANT USAGE ON SCHEMA brain TO brain_migrate; GRANT EXECUTE ON FUNCTION brain.read_compare_artifact_v1(text), brain.read_compare_artifact_v2(text) TO brain_migrate;").execute(&admin).await.unwrap();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let role_sql = fs::read_to_string(root.join("ops/brain-postgres/site-role.sql")).unwrap();
    let role_sql = role_sql
        .lines()
        .filter(|line| !line.starts_with('\\'))
        .collect::<Vec<_>>()
        .join("\n");
    sqlx::raw_sql(&role_sql).execute(&admin).await.unwrap();
    sqlx::raw_sql("GRANT USAGE ON SCHEMA brain TO brain_site; GRANT SELECT ON brain.site_comments_v1 TO brain_site; GRANT EXECUTE ON FUNCTION brain.append_site_comment_v1(text,text,text) TO brain_site").execute(&admin).await.unwrap();
    let old_site = PgPoolOptions::new()
        .max_connections(4)
        .connect_with(options.clone().username("brain_site"))
        .await
        .unwrap();
    assert!(database::preflight(&old_site).await.is_err());
    old_site.close().await;
    sqlx::raw_sql(&role_sql).execute(&admin).await.unwrap();

    let site = PgPoolOptions::new()
        .max_connections(4)
        .connect_with(options.username("brain_site"))
        .await
        .unwrap();
    database::preflight(&site).await.unwrap();
    assert!(database::preflight(&admin).await.is_err());
    for sql in [
        "SELECT * FROM brain.site_comments_v1",
        "SELECT brain.append_site_comment_v1('x','x','x')",
        "SELECT brain.read_compare_artifact_v1('x')",
        "SELECT brain.read_compare_artifact_v2('x')",
        "SELECT * FROM canon.fact",
        "SELECT * FROM raw.object",
        "SELECT * FROM ops.ingest_run",
        "INSERT INTO canon.fact(entity_id) VALUES(1)",
    ] {
        assert!(sqlx::query(sql).execute(&site).await.is_err(), "{sql}");
    }
    let site_server = site.clone();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let handle = tokio::spawn(async move {
        axum::serve(listener, router(site_server)).await.unwrap();
    });
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let path = format!("/site/steckbriefe/hero/{}", hex::encode("hero_baba"));
    let response = client.get(format!("{url}{path}")).send().await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["x-content-type-options"], "nosniff");
    assert_eq!(response.headers()["cache-control"], "no-store");
    let html = response.text().await.unwrap();
    let get_hero: Value = sqlx::query_scalar("SELECT jsonb_build_object('identity',identity,'stats',stats,'abilities',abilities,'weapon',weapon) FROM serve.hero_profile WHERE entity_id=$1 LIMIT 1").bind(hero).fetch_one(&admin).await.unwrap();
    assert_eq!(get_hero["identity"]["name_en"], "Baba");
    for fact in get_hero["stats"]["facts"].as_array().unwrap() {
        let stat = fact["stat_key"].as_str().unwrap();
        assert!(html.contains(&format!(
            "<th scope=\"row\">{stat}</th><td>{}</td>",
            fact["value"]
        )));
        assert!(html.contains(&format!(
            "id=\"beleg-{}\"",
            fact["citation"].as_str().unwrap()
        )));
        assert!(html.contains(fact["source_revision"].as_str().unwrap()));
    }
    assert!(html.contains("gamefacts.receipt.v1:sha256:"));
    assert!(!html.contains("Kommentar"));
    assert!(!html.contains("<td>999.0</td>"));
    let head = client.head(format!("{url}{path}")).send().await.unwrap();
    assert_eq!(head.status(), StatusCode::OK);
    assert_eq!(
        head.headers()["content-length"]
            .to_str()
            .unwrap()
            .parse::<usize>()
            .unwrap(),
        html.len()
    );
    assert!(head.bytes().await.unwrap().is_empty());
    for path in [
        format!("/site/entities/hero/{}.html", hex::encode("hero_baba")),
        format!("/site/steckbriefe/item/{}", hex::encode("upgrade_fixture")),
        format!("/site/compare/{hero},{other}?stats=health"),
        format!("/compare/{hero},{other}?stats=health"),
    ] {
        let response = client.get(format!("{url}{path}")).send().await.unwrap();
        assert_eq!(response.status(), StatusCode::OK, "{path}");
        assert!(response
            .text()
            .await
            .unwrap()
            .contains("gamefacts.receipt.v1:sha256:"));
    }
    let index = client
        .get(format!("{url}/site/steckbriefe"))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert!(index.contains("Baba"));
    assert!(index.contains("Prüfitem"));
    assert!(!index.contains("Nicht veröffentlicht"));
    assert!(!index.contains("Ausgemustert"));
    assert!(!index.contains("Zielpuppe"));
    for (key, hidden) in [("hero_retired", retired), ("hero_target_dummy", dummy)] {
        let profile = client
            .get(format!("{url}/site/steckbriefe/hero/{}", hex::encode(key)))
            .send()
            .await
            .unwrap();
        assert_eq!(profile.status(), StatusCode::NOT_FOUND, "{key}");
        let compare = client
            .get(format!("{url}/site/compare/{hero},{hidden}?stats=health"))
            .send()
            .await
            .unwrap();
        assert_eq!(compare.status(), StatusCode::NOT_FOUND, "{key}");
        let compare = compare.text().await.unwrap();
        assert!(
            !compare.contains("Ausgemustert") && !compare.contains("Zielpuppe"),
            "{key}"
        );
    }
    assert!(!index.contains("Prüffähigkeit"));
    assert!(html.contains("Für diesen Helden sind keine belegten Fähigkeiten verfügbar."));
    for route in ["steckbriefe", "entities"] {
        let ability = client
            .get(format!(
                "{url}/site/{route}/ability/{}",
                hex::encode("ability_fixture")
            ))
            .send()
            .await
            .unwrap();
        assert_eq!(ability.status(), StatusCode::NOT_FOUND, "{route}");
    }
    for (kind, key) in [("item", "upgrade_fixture")] {
        let card = client
            .get(format!(
                "{url}/site/steckbriefe/{kind}/{}",
                hex::encode(key)
            ))
            .send()
            .await
            .unwrap();
        assert_eq!(card.status(), StatusCode::OK, "{key}");
        let card = card.text().await.unwrap();
        assert!(card.contains(&hex::encode("hero_baba")), "{key}");
        for hidden in ["hero_retired", "hero_target_dummy"] {
            assert!(!card.contains(&hex::encode(hidden)), "{key} {hidden}");
        }
        assert!(
            !card.contains("Ausgemustert") && !card.contains("Zielpuppe"),
            "{key}"
        );
    }
    for path in [
        "/api/comments",
        "/site/app.js",
        "/site/vendor/marked.min.js",
        "/builds_registry.json",
        "/hero_meta_compact.json",
        "/item_meta_compact.json",
        "/HERO_STRENGTH.md",
        "/ITEM_STRENGTH.md",
        "/MASTER_METHODOLOGY.md",
        "/understanding/Baba.md",
        "/site/compare/legacy-artifact/chart.svg",
        "/site/compare/legacy-artifact",
        "/site/compare/0000000000000000000000000000000000000000000000000000000000000000",
        "/site/compare/0000000000000000000000000000000000000000000000000000000000000000/chart.svg",
        "/compare/0000000000000000000000000000000000000000000000000000000000000000",
        "/compare/0000000000000000000000000000000000000000000000000000000000000000/chart.svg",
        "/site/steckbriefe/hero/00",
        "/site/entities/hero/ff.html",
        "/site/public/missing.html",
    ] {
        assert_eq!(
            client
                .get(format!("{url}{path}"))
                .send()
                .await
                .unwrap()
                .status(),
            StatusCode::NOT_FOUND,
            "{path}"
        );
    }
    assert_eq!(
        client
            .post(format!("{url}/api/comments"))
            .body("retired")
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::METHOD_NOT_ALLOWED
    );
    for path in ["/site", "/site/", "/site/index.html"] {
        let response = client.get(format!("{url}{path}")).send().await.unwrap();
        assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
        assert_eq!(response.headers()["location"], "/brain/site/steckbriefe");
    }
    publish_doc(
        &admin,
        "public/test.html",
        1,
        "Öffentliche Hilfe",
        "Text <script>kein Code</script>",
    )
    .await;
    publish_doc(&admin, "internal/test.html", 2, "Intern", "Nur intern").await;
    let internal: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM serve.doc_section WHERE path='internal/test.html'",
    )
    .fetch_one(&admin)
    .await
    .unwrap();
    assert_eq!(internal, 1);
    let public = client
        .get(format!("{url}/site/public/test.html"))
        .send()
        .await
        .unwrap();
    assert_eq!(public.status(), StatusCode::OK);
    let public = public.text().await.unwrap();
    assert!(public.contains("&lt;script&gt;kein Code&lt;/script&gt;"));
    assert!(!public.contains("<script>"));
    assert_eq!(
        client
            .get(format!("{url}/site/internal/test.html"))
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::NOT_FOUND
    );
    let corpus_run = publish_doc_corpus(&admin, 181).await;
    let rejected = corpus_path(1);
    audit_doc_corpus(&admin, corpus_run, &rejected).await;
    sqlx::query("ANALYZE").execute(&admin).await.unwrap();
    let scopes: Vec<(String, i64)> = sqlx::query_as(
        "SELECT scope,count(*) FROM serve.doc_section GROUP BY scope ORDER BY scope",
    )
    .fetch_all(&admin)
    .await
    .unwrap();
    assert!(
        scopes.iter().map(|(_, count)| count).sum::<i64>() >= 1020,
        "{scopes:?}"
    );
    let quarantined: i64 =
        sqlx::query_scalar("SELECT count(*) FROM canon.receipt_quarantine WHERE cause='audited'")
            .fetch_one(&admin)
            .await
            .unwrap();
    assert!(quarantined >= 1);
    let target = corpus_path(42);
    let mut timings = Vec::new();
    let mut lookup = site.acquire().await.unwrap();
    for _ in 0..4 {
        let (planning, execution, wall) = site_doc_lookup_ms(&mut lookup, &target).await;
        assert!(planning + execution < 500.0, "{planning} {execution}");
        assert!(wall < 500.0, "{wall}");
        timings.push(format!(
            "warmup planning_ms={planning:.3} execution_ms={execution:.3} fetch_wall_ms={wall:.3}"
        ));
    }
    for _ in 0..3 {
        let (planning, execution, wall) = site_doc_lookup_ms(&mut lookup, &target).await;
        assert!(planning + execution < 100.0, "{planning} {execution}");
        assert!(wall < 100.0, "{wall}");
        timings.push(format!(
            "planning_ms={planning:.3} execution_ms={execution:.3} fetch_wall_ms={wall:.3}"
        ));
    }
    drop(lookup);
    let started = std::time::Instant::now();
    let page = client
        .get(format!("{url}/site/{target}"))
        .send()
        .await
        .unwrap();
    let http_ms = started.elapsed().as_secs_f64() * 1000.0;
    assert_eq!(page.status(), StatusCode::OK);
    let page = page.text().await.unwrap();
    assert_eq!(page.matches("<article>").count(), 40);
    assert!(page.contains("Seite 42 Abschnitt 39"));
    for path in [
        format!("/site/{rejected}"),
        format!("/site/{}", corpus_path(150)),
        "/site/public/korpus/unbekannt.html".to_owned(),
    ] {
        assert_eq!(
            client
                .get(format!("{url}{path}"))
                .send()
                .await
                .unwrap()
                .status(),
            StatusCode::NOT_FOUND,
            "{path}"
        );
    }
    let internal_sections: i64 =
        sqlx::query_scalar("SELECT count(*) FROM serve.doc_section WHERE path=$1")
            .bind(corpus_path(150))
            .fetch_one(&admin)
            .await
            .unwrap();
    assert_eq!(internal_sections, 5);
    let hidden_sections: i64 = sqlx::query_scalar("SELECT count(*) FROM serve.site_doc_page($1)")
        .bind(corpus_path(150))
        .fetch_one(&admin)
        .await
        .unwrap();
    assert_eq!(hidden_sections, 0);
    let (view_rows, missing, extra): (i64, i64, i64) = sqlx::query_as(
        "WITH legacy AS MATERIALIZED (
            SELECT path,title::text,body::text,source_revision::text,receipt_id,ordinal FROM serve.site_doc_section
        ), keyed AS MATERIALIZED (
            SELECT p.path,x.title,x.body,x.source_revision,x.receipt_id,x.ordinal
            FROM (SELECT DISTINCT value_text::text AS path FROM canon.fact WHERE stat_key='doc_path') p
            CROSS JOIN LATERAL serve.site_doc_page(p.path) x
        )
        SELECT (SELECT count(*) FROM legacy),
            (SELECT count(*) FROM (SELECT * FROM legacy EXCEPT ALL SELECT * FROM keyed) d),
            (SELECT count(*) FROM (SELECT * FROM keyed EXCEPT ALL SELECT * FROM legacy) d)",
    )
    .fetch_one(&admin)
    .await
    .unwrap();
    assert!(view_rows >= 690, "{view_rows}");
    assert_eq!((missing, extra), (0, 0));
    if let Some(directory) = std::env::var_os("BRAIN_SITE_DOC_PROOF_DIR") {
        let directory = PathBuf::from(directory);
        fs::create_dir_all(&directory).unwrap();
        fs::write(
            directory.join("legacy-view-explain.txt"),
            explain(&admin, "SELECT title,body,source_revision,receipt_id::text AS citation FROM serve.site_doc_section WHERE path=$1 ORDER BY ordinal LIMIT 65", &target).await,
        )
        .unwrap();
        fs::write(
            directory.join("timings.txt"),
            format!(
                "sections_by_scope={scopes:?}\naudited_quarantine={quarantined}\nequivalence_view_rows={view_rows} missing={missing} extra={extra}\ntarget_sections=40\n{}\nhttp_page_ms={http_ms:.3}\n",
                timings.join("\n")
            ),
        )
        .unwrap();
    }
    if let Some(directory) = std::env::var_os("BRAIN_P7_PROOF_DIR") {
        let directory = PathBuf::from(directory);
        fs::create_dir_all(&directory).unwrap();
        fs::write(directory.join("baba.html"), &html).unwrap();
        fs::write(
            directory.join("get-hero.json"),
            serde_json::to_vec_pretty(&get_hero).unwrap(),
        )
        .unwrap();
        let output = tokio::process::Command::new("/home/nathanael/.local/bin/moli")
            .args([
                "fetch",
                "--layout",
                "--image",
                "--font",
                "--wait",
                "domcontentloaded",
                "--wait-selector",
                "h1",
                "--dump",
                "screenshot_full",
            ])
            .arg(format!("{url}{path}"))
            .output()
            .await
            .unwrap();
        assert!(
            output.status.success(),
            "Moli: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        fs::write(directory.join("baba.png"), output.stdout).unwrap();
        fs::write(
            directory.join("test-process-memory.txt"),
            fs::read_to_string("/proc/self/status")
                .unwrap()
                .lines()
                .filter(|line| line.starts_with("VmHWM:") || line.starts_with("VmRSS:"))
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .unwrap();
    }
    let audit_run: i64 =
        sqlx::query_scalar("INSERT INTO ops.receipt_audit_run DEFAULT VALUES RETURNING id")
            .fetch_one(&admin)
            .await
            .unwrap();
    sqlx::query("INSERT INTO ops.receipt_audit(run_id,receipt_id,outcome,observed_value_sha256) SELECT $1,receipt_id,'mismatch',sha256('mismatch'::bytea) FROM canon.fact WHERE entity_id=$2 AND stat_key='health'")
        .bind(audit_run).bind(hero).execute(&admin).await.unwrap();
    let hidden = client.get(format!("{url}{path}")).send().await.unwrap();
    assert_eq!(hidden.status(), StatusCode::OK);
    assert!(!hidden.text().await.unwrap().contains("<td>780.0</td>"));
    handle.abort();
    let _ = handle.await;
    admin.close().await;
}
