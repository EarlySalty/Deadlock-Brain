use super::*;
use axum::http::StatusCode;
use brain_storage::{
    gamefacts::{Counts, Entity, Fact, FactSet, FactValue, GameFactsStore, Receipt},
    PgStore,
};
use reqwest::Client;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use std::{fs, path::PathBuf};

#[path = "../../../../brain-storage/tests/support/scratch_pg.rs"]
mod scratch_pg;

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
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let handle = tokio::spawn(async move {
        axum::serve(listener, router(site)).await.unwrap();
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
    assert!(!index.contains("Nicht veröffentlicht"));
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
    sqlx::raw_sql("SET ROLE brain_migrate; CREATE FUNCTION serve.doc_meta(text) RETURNS text LANGUAGE sql AS 'SELECT $1'; CREATE FUNCTION serve.doc_tsv(text) RETURNS text LANGUAGE sql AS 'SELECT $1'; CREATE VIEW serve.doc_section AS SELECT 'public/test.html'::text AS path,'Öffentliche Hilfe'::text AS title,'Text <script>kein Code</script>'::text AS body,'public'::text AS scope,0 AS ordinal,source_revision,id AS receipt_id FROM serve.published_receipt LIMIT 1; RESET ROLE").execute(&admin).await.unwrap();
    sqlx::raw_sql(include_str!(
        "../../../../../../scripts/migrations/2026-10-15-brain-site-p7-v30.sql"
    ))
    .execute(&admin)
    .await
    .unwrap();
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
