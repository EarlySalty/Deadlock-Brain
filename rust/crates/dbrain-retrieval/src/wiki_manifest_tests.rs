//! Vertragsprüfung gegen eine ausdrücklich isolierte, leere Testdatenbank.
use super::load_latest_snapshots;

#[tokio::test]
#[ignore = "benötigt eine leere, isolierte brain_wiki_test-Datenbank und WIKI_SCRATCH_DSN"]
async fn successful_manifest_excludes_partial_deleted_and_renamed_pages() {
    let dsn = std::env::var("WIKI_SCRATCH_DSN").expect("WIKI_SCRATCH_DSN fehlt");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&dsn)
        .await
        .unwrap();
    let identity: (String, Option<String>, Option<i32>) = sqlx::query_as(
        "SELECT current_database(), host(inet_server_addr()), inet_server_port()",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(
        identity.0.starts_with("brain_wiki_test_"),
        "Keine Testdatenbank"
    );
    assert_eq!(
        identity.1.as_deref(),
        Some("127.0.0.1"),
        "Nur lokale Testinstanz"
    );
    assert!(
        identity
            .2
            .is_some_and(|port| port > 1024 && ![5432, 5433, 5434].contains(&port)),
        "Kein Produktionsport"
    );
    // CREATE ohne IF NOT EXISTS: eine vorhandene Umgebung wird nie überschrieben.
    sqlx::raw_sql("CREATE SCHEMA brain;
        CREATE TABLE brain.source_runs (id bigint PRIMARY KEY, source text, status text, summary jsonb);
        CREATE TABLE brain.source_documents (id bigint PRIMARY KEY, title text, url text, content_hash text, raw_path text);
        CREATE TABLE brain.entity_snapshots (id bigint PRIMARY KEY, source text, entity_type text, external_id text,
            canonical_name text, payload_hash text, payload jsonb, fetched_at timestamptz, source_document_id bigint);
        INSERT INTO brain.entity_snapshots VALUES
        (1,'deadlock_wiki','wiki_page','Legacy','Legacy','h1','{}',now(),NULL),
        (2,'deadlock_wiki','wiki_page','20','Hero','h2','{\"_wiki\":{\"schema_version\":1}}',now(),NULL),
        (3,'deadlock_wiki','wiki_page','30','Unfinished','h3','{\"_wiki\":{\"schema_version\":1}}',now(),NULL);
        INSERT INTO brain.source_runs VALUES (1,'deadlock_wiki_corpus','error','{\"complete\":true,\"snapshot_ids\":[2,3]}');")
        .execute(&pool).await.unwrap();
    let ids = || async {
        load_latest_snapshots(&pool)
            .await
            .unwrap()
            .into_iter()
            .map(|row| row.id)
            .collect::<Vec<_>>()
    };
    assert_eq!(
        ids().await,
        vec![1],
        "Erster Teilimport muss unsichtbar bleiben"
    );
    sqlx::raw_sql("INSERT INTO brain.source_runs VALUES (2,'deadlock_wiki_corpus','ok','{\"complete\":true,\"snapshot_ids\":[2]}');")
        .execute(&pool).await.unwrap();
    assert_eq!(
        ids().await,
        vec![2],
        "Nur Mitglieder des vollständigen Imports"
    );
    sqlx::raw_sql("INSERT INTO brain.entity_snapshots VALUES
        (4,'deadlock_wiki','wiki_page','20','Renamed Hero','h4','{\"_wiki\":{\"schema_version\":1}}',now(),NULL),
        (5,'deadlock_wiki','wiki_page','50','Removed later','h5','{\"_wiki\":{\"schema_version\":1}}',now(),NULL);
        INSERT INTO brain.source_runs VALUES
        (3,'deadlock_wiki_corpus','ok','{\"complete\":true,\"snapshot_ids\":[4,5]}'),
        (4,'deadlock_wiki_corpus','running','{\"complete\":true,\"snapshot_ids\":[3]}');")
        .execute(&pool).await.unwrap();
    let mut current = ids().await;
    current.sort();
    assert_eq!(
        current,
        vec![4, 5],
        "Umbenennung ersetzt alten Titel, laufender Import bleibt unsichtbar"
    );
    sqlx::raw_sql("INSERT INTO brain.source_runs VALUES
        (5,'deadlock_wiki_corpus','ok','{\"complete\":true,\"snapshot_ids\":[4]}'),
        (6,'deadlock_wiki_corpus','ok','{\"complete\":false,\"snapshot_ids\":[3]}'),
        (7,'deadlock_wiki_corpus','error','{\"complete\":true,\"snapshot_ids\":[2,3,5]}');")
        .execute(&pool).await.unwrap();
    assert_eq!(
        ids().await,
        vec![4],
        "Gelöschte Seiten verschwinden, unveränderte Snapshot-IDs bleiben nutzbar"
    );
    pool.close().await;
}
