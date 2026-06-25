use rusqlite::{params, Connection};
use serde_json::{json, Value};
use tempfile::TempDir;

use crate::{
    enrich_legacy_entities_with_conn, enrich_lineage_with_conn, extract_lineage_candidates,
    normalize_entities_with_conn, normalize_sheet_stats_with_conn, normalize_sheet_tabs_with_conn,
    parse_patchnotes_with_conn, resolve_gaps_with_conn, LineageEvent,
};

fn test_conn() -> (TempDir, Connection) {
    let temp = tempfile::tempdir().expect("tempdir");
    let conn = Connection::open(temp.path().join("brain.sqlite3")).expect("open temp db");
    deadlock_brain_core::db::apply_pragmas(&conn).expect("apply pragmas");
    deadlock_brain_core::schema::ensure_schema(&conn).expect("ensure schema");
    (temp, conn)
}

fn insert_source_document(conn: &Connection, external_id: &str) -> i64 {
    conn.execute(
        r#"
        INSERT INTO source_documents(
          source, external_id, title, url, content_type, raw_path, content_hash, fetched_at, metadata_json
        )
        VALUES('deadlock_assets_api', ?1, NULL, NULL, 'json', '/tmp/test.json', ?2, 1, '{}')
        "#,
        params![external_id, format!("hash-{external_id}")],
    )
    .expect("insert source document");
    conn.last_insert_rowid()
}

fn insert_snapshot(
    conn: &Connection,
    source: &str,
    entity_type: &str,
    external_id: &str,
    canonical_name: Option<&str>,
    payload: Value,
    source_document_id: Option<i64>,
) -> i64 {
    conn.execute(
        r#"
        INSERT INTO entity_snapshots(
          source, entity_type, external_id, canonical_name, payload_hash,
          payload_json, fetched_at, source_document_id
        )
        VALUES(?1,?2,?3,?4,?5,?6,1,?7)
        "#,
        params![
            source,
            entity_type,
            external_id,
            canonical_name,
            format!("{source}-{entity_type}-{external_id}"),
            serde_json::to_string(&payload).expect("payload json"),
            source_document_id,
        ],
    )
    .expect("insert snapshot");
    conn.last_insert_rowid()
}

fn insert_entity(conn: &Connection, entity_type: &str, canonical_name: &str) -> i64 {
    conn.execute(
        r#"
        INSERT INTO entities(
          entity_type, canonical_name, primary_external_id, source,
          first_snapshot_id, metadata_json, created_at, updated_at
        )
        VALUES(?1,?2,NULL,'test',NULL,'{}',1,1)
        "#,
        params![entity_type, canonical_name],
    )
    .expect("insert entity");
    conn.last_insert_rowid()
}

fn insert_alias(conn: &Connection, entity_id: i64, alias: &str, alias_kind: &str) {
    conn.execute(
        r#"
        INSERT INTO entity_aliases(entity_id, alias, alias_norm, alias_kind, source, external_id, snapshot_id, created_at)
        VALUES(?1,?2,?3,?4,'test',NULL,NULL,1)
        "#,
        params![entity_id, alias, crate::normalize_alias(alias), alias_kind],
    )
    .expect("insert alias");
}

#[test]
fn normalizes_entities_aliases_and_classification() {
    let (_temp, conn) = test_conn();
    let heroes_doc = insert_source_document(&conn, "heroes");
    let items_doc = insert_source_document(&conn, "items");

    insert_snapshot(
        &conn,
        "deadlock_assets_api",
        "hero",
        "1",
        Some("Abrams"),
        json!({
            "id": 1,
            "type": "hero",
            "name": "Abrams",
            "class_name": "hero_atlas",
            "disabled": false,
            "player_selectable": true,
            "in_development": false,
            "prerelease_only": false,
            "assigned_players_only": false
        }),
        Some(heroes_doc),
    );
    insert_snapshot(
        &conn,
        "deadlock_assets_api",
        "item_or_ability",
        "ability_1",
        Some("Siphon Life"),
        json!({
            "id": "ability_1",
            "type": "ability",
            "name": "Siphon Life",
            "class_name": "citadel_ability_atlas_siphon_life",
            "hero": "1"
        }),
        Some(items_doc),
    );
    insert_snapshot(
        &conn,
        "deadlock_assets_api",
        "item_or_ability",
        "item_1",
        Some("Basic Magazine"),
        json!({
            "id": "item_1",
            "type": "upgrade",
            "name": "Basic Magazine",
            "class_name": "item_basic_magazine",
            "shopable": true,
            "item_slot_type": "weapon",
            "item_tier": 1,
            "cost": 500
        }),
        Some(items_doc),
    );

    let summary = normalize_entities_with_conn(&conn, true).expect("normalize entities");
    assert_eq!(summary["entities"], 3);
    assert_eq!(summary["by_type"]["hero"], 1);
    assert_eq!(summary["by_type"]["ability"], 1);
    assert_eq!(summary["by_type"]["item"], 1);

    let hero_aliases: i64 = conn
        .query_row(
            r#"
            SELECT COUNT(DISTINCT a.alias_norm)
            FROM entity_aliases a
            JOIN entities e ON e.id=a.entity_id
            WHERE e.entity_type='hero'
              AND e.canonical_name='Abrams'
              AND a.alias_norm IN ('abrams', 'hero atlas')
            "#,
            [],
            |row| row.get(0),
        )
        .expect("hero aliases");
    assert_eq!(hero_aliases, 2);

    let ability_type: String = conn
        .query_row(
            "SELECT entity_type FROM entities WHERE canonical_name='Siphon Life'",
            [],
            |row| row.get(0),
        )
        .expect("ability entity");
    assert_eq!(ability_type, "ability");
}

#[test]
fn parses_patchnotes_and_builds_lineage() {
    let (_temp, conn) = test_conn();
    let hero_id = insert_entity(&conn, "hero", "Abrams");
    insert_alias(&conn, hero_id, "Abrams", "canonical");
    insert_snapshot(
        &conn,
        "patchnotes",
        "patchnote",
        "patch-1",
        Some("Patch One"),
        json!({
            "title": "Patch One",
            "url": "https://forums.playdeadlock.com/t/patch-one",
            "posted_at": "2024-01-01",
            "raw_content": "Heroes\nAbrams\n- renamed to Atlas.\n- Life Drain renamed to Siphon Life.\n- Bullet damage increased from 10 to 12.\n"
        }),
        None,
    );

    let summary = parse_patchnotes_with_conn(&conn, true).expect("parse patchnotes");
    assert_eq!(summary["events_inserted"], 3);
    assert_eq!(summary["source_kinds"]["forum"], 1);

    let values: (Option<String>, Option<String>) = conn
        .query_row(
            "SELECT old_value, new_value FROM patch_events WHERE normalized_line LIKE 'Bullet damage increased%'",
            [],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .expect("old/new values");
    assert_eq!(values, (Some("10".to_string()), Some("12".to_string())));

    let direct = extract_lineage_candidates(&LineageEvent {
        id: 99,
        patch_title: None,
        patch_url: None,
        source_kind: Some("forum".to_string()),
        posted_at: None,
        entity_type: Some("hero".to_string()),
        entity_name: Some("Abrams".to_string()),
        subject: None,
        change_type: Some("changed".to_string()),
        normalized_line: Some("Life Drain renamed to Siphon Life.".to_string()),
        raw_line: None,
    });
    assert_eq!(direct.len(), 1);
    assert_eq!(direct[0].source_entity_type.as_deref(), Some("ability"));
    assert_eq!(direct[0].owner_name.as_deref(), Some("Abrams"));

    let lineage_summary = enrich_lineage_with_conn(&conn, true).expect("enrich lineage");
    assert_eq!(lineage_summary["lineage_inserted"], 2);

    let ability_owner: String = conn
        .query_row(
            r#"
            SELECT owner_name
            FROM entity_lineage
            WHERE source_name='Life Drain' AND target_name='Siphon Life'
            "#,
            [],
            |row| row.get(0),
        )
        .expect("ability lineage owner");
    assert_eq!(ability_owner, "Abrams");
}

#[test]
fn parses_multi_entity_patch_line_with_deterministic_subject_and_event_hash() {
    let orders = [
        ["Ivy", "Viscous", "Magic Carpet"],
        ["Magic Carpet", "Viscous", "Ivy"],
        ["Viscous", "Ivy", "Magic Carpet"],
    ];
    let mut observed = Vec::new();
    for order in orders {
        observed.push(parse_multi_entity_patch_line(order));
    }

    let (subject, event_hash) = &observed[0];
    assert_eq!(subject, "Ivy");
    assert!(observed
        .iter()
        .all(|(next_subject, next_hash)| next_subject == subject && next_hash == event_hash));
}

#[test]
fn builds_legacy_entities_from_unknown_patch_names() {
    let (_temp, conn) = test_conn();
    let snapshot_id = insert_snapshot(
        &conn,
        "patchnotes",
        "patchnote",
        "patch-legacy",
        Some("Legacy Patch"),
        json!({"raw_content": ""}),
        None,
    );
    conn.execute(
        r#"
        INSERT INTO patch_events(
          patch_snapshot_id, patch_external_id, patch_title, patch_url,
          source_kind, posted_at, line_index, section, entity_type,
          entity_name, subject, change_type, raw_line, normalized_line,
          old_value, new_value, confidence, metadata_json, event_hash, created_at
        )
        VALUES(?1,'patch-legacy','Legacy Patch',NULL,'forum','2024-01-01',1,'Items','item',
               'Old Idol','Old Idol','removed','- Old Idol: removed.','removed.',NULL,NULL,0.75,'{}','legacy-hash',1)
        "#,
        [snapshot_id],
    )
    .expect("insert patch event");

    let summary = enrich_legacy_entities_with_conn(&conn, true).expect("legacy entities");
    assert_eq!(summary["legacy_inserted"], 1);
    assert_eq!(summary["by_type"]["legacy_item"], 1);

    let status: String = conn
        .query_row(
            "SELECT status FROM legacy_entities WHERE canonical_name='Old Idol'",
            [],
            |row| row.get(0),
        )
        .expect("legacy status");
    assert_eq!(status, "legacy_candidate");
}

#[test]
fn normalizes_sheet_stats_and_tabs() {
    let (_temp, conn) = test_conn();
    let hero_id = insert_entity(&conn, "hero", "Abrams");
    insert_alias(&conn, hero_id, "Abrams", "canonical");

    insert_snapshot(
        &conn,
        "deadlock_stats_sheet",
        "hero_stats_sheet",
        "hero-stats-1",
        Some("Abrams"),
        json!({
            "row_number": 2,
            "values": {
                "Hero Name": "Abrams",
                "Base HP": "600",
                "Column 7": "ignored",
                "|||": "ignored"
            }
        }),
        None,
    );
    let stats = normalize_sheet_stats_with_conn(&conn, true).expect("sheet stats");
    assert_eq!(stats["profiles"], 1);
    assert_eq!(stats["values"], 1);

    insert_snapshot(
        &conn,
        "deadlock_stats_sheet",
        "sheet_row",
        "ranking-1",
        Some("Abrams"),
        json!({
            "sheet_name": "Hero meta ranking",
            "values": {"hero name": "Abrams", "carry": "1", "Average Rank": "2.5"}
        }),
        None,
    );
    insert_snapshot(
        &conn,
        "deadlock_stats_sheet",
        "sheet_row",
        "boons-1",
        None,
        json!({
            "sheet_name": "Boons/AP",
            "values": {"Souls": "3000", "Boons": "2", "AP": "1", "Column 4": "note"}
        }),
        None,
    );
    insert_snapshot(
        &conn,
        "deadlock_stats_sheet",
        "sheet_row",
        "freeform-1",
        Some("Hidden Mechanics Row"),
        json!({
            "sheet_name": "Hidden Mechanics",
            "gid": "123",
            "row_number": 4,
            "values": {"Topic": "Parry", "Note": "test"}
        }),
        None,
    );

    let tabs = normalize_sheet_tabs_with_conn(&conn, true).expect("sheet tabs");
    assert_eq!(tabs["sheet_hero_rankings"]["inserted"], 1);
    assert_eq!(tabs["sheet_boons_ap"]["inserted"], 1);
    assert_eq!(tabs["sheet_tab_rows"]["total"], 1);

    let row_json: String = conn
        .query_row(
            "SELECT row_json FROM sheet_tab_rows WHERE tab_name='Hidden Mechanics'",
            [],
            |row| row.get(0),
        )
        .expect("freeform row");
    assert!(row_json.contains("Parry"));
}

#[test]
fn resolve_gaps_reresolves_patch_event_subjects_conservatively() {
    let (_temp, conn) = test_conn();
    let hero_id = insert_entity(&conn, "hero", "Doorman");
    insert_alias(&conn, hero_id, "Doorman", "canonical");
    let snapshot_id = insert_snapshot(
        &conn,
        "patchnotes",
        "patchnote",
        "patch-resolve-gap",
        Some("Resolve Gap Patch"),
        json!({"raw_content": ""}),
        None,
    );
    conn.execute(
        r#"
        INSERT INTO patch_events(
          patch_snapshot_id, patch_external_id, patch_title, patch_url,
          source_kind, posted_at, line_index, section, entity_type,
          entity_name, subject, change_type, raw_line, normalized_line,
          old_value, new_value, confidence, metadata_json, event_hash, created_at
        )
        VALUES(?1,'patch-resolve-gap','Resolve Gap Patch',NULL,'forum','2026-01-01',1,'Heroes','general',
               NULL,'Doorman','changed','- Doorman: damage increased.','damage increased.',NULL,NULL,0.45,'{}','gap-doorman',1),
              (?1,'patch-resolve-gap','Resolve Gap Patch',NULL,'forum','2026-01-01',2,'Heroes','general',
               NULL,'Unknown Hero','changed','- Unknown Hero: damage increased.','damage increased.',NULL,NULL,0.45,'{}','gap-unknown',1)
        "#,
        [snapshot_id],
    )
    .expect("insert patch gaps");

    let dry = resolve_gaps_with_conn(&conn, true).expect("dry resolve gaps");
    assert_eq!(dry["patch_events"]["changed"], 1);
    let dry_type: String = conn
        .query_row(
            "SELECT entity_type FROM patch_events WHERE event_hash='gap-doorman'",
            [],
            |row| row.get(0),
        )
        .expect("dry run unchanged");
    assert_eq!(dry_type, "general");

    let first = resolve_gaps_with_conn(&conn, false).expect("resolve gaps");
    assert_eq!(first["patch_events"]["changed"], 1);
    let resolved: (String, Option<String>, f64) = conn
        .query_row(
            "SELECT entity_type, entity_name, confidence FROM patch_events WHERE event_hash='gap-doorman'",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .expect("resolved patch event");
    assert_eq!(resolved.0, "hero");
    assert_eq!(resolved.1.as_deref(), Some("Doorman"));
    assert_eq!(resolved.2, 0.95);

    let heuristic_left: String = conn
        .query_row(
            "SELECT entity_type FROM patch_events WHERE event_hash='gap-unknown'",
            [],
            |row| row.get(0),
        )
        .expect("heuristic left untouched");
    assert_eq!(heuristic_left, "general");

    let second = resolve_gaps_with_conn(&conn, false).expect("resolve gaps again");
    assert_eq!(second["patch_events"]["changed"], 0);
    assert_eq!(second["claims"]["changed"], 0);
    assert_eq!(second["entity_id_backfill"]["changed"], 0);
}

#[test]
fn resolve_gaps_reresolves_claim_from_first_resolvable_split_part() {
    let (_temp, conn) = test_conn();
    let item_id = insert_entity(&conn, "item", "Close Quarters");
    insert_alias(&conn, item_id, "Close Quarters", "canonical");
    insert_youtube_video(&conn, "video-split-claim");
    conn.execute(
        r#"
        INSERT INTO youtube_learning_claims(
          video_id, claim_hash, claim_index, entity_type, entity_name,
          claim_type, claim_text, evidence_quote, timestamp_seconds,
          model_confidence, verifier_confidence, status, model, prompt_version,
          prompt_text, model_response_text, provider_metadata_json, verifier_json,
          created_at, updated_at
        )
        VALUES(
          'video-split-claim', 'claim-split', 0, NULL, 'Point Blank / Close Quarters',
          'item_note', 'claim text', 'quote', NULL,
          0.8, 0.2, 'needs_review', 'test-model', 'test-prompt',
          'prompt', 'response', '{}', ?1, 1, 1
        )
        "#,
        [json!({"reasons": ["entity_not_resolved", "low_context"]}).to_string()],
    )
    .expect("insert claim");

    let first = resolve_gaps_with_conn(&conn, false).expect("resolve claim gaps");
    assert_eq!(first["claims"]["changed"], 1);

    let resolved: (String, String, String, String) = conn
        .query_row(
            r#"
            SELECT entity_type, entity_name, status, verifier_json
            FROM youtube_learning_claims
            WHERE claim_hash='claim-split'
            "#,
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )
        .expect("resolved claim");
    assert_eq!(resolved.0, "item");
    assert_eq!(resolved.1, "Close Quarters");
    assert_eq!(resolved.2, "needs_review");
    let verifier: Value = serde_json::from_str(&resolved.3).expect("verifier json");
    assert_eq!(verifier["entity_resolution"], "reresolved_deadlock_data");
    assert_eq!(verifier["reasons"], json!(["low_context"]));

    let second = resolve_gaps_with_conn(&conn, false).expect("resolve claim gaps again");
    assert_eq!(second["claims"]["changed"], 0);
}

#[test]
fn resolve_gaps_backfills_entity_ids_and_skips_ambiguous_or_empty_names() {
    let (_temp, conn) = test_conn();
    let abrams_id = insert_entity(&conn, "hero", "Abrams");
    insert_alias(&conn, abrams_id, "Abrams", "canonical");
    let hero_a = insert_entity(&conn, "hero", "Hero A");
    let hero_b = insert_entity(&conn, "hero", "Hero B");
    insert_alias(&conn, hero_a, "Shared Hero", "snapshot_name");
    insert_alias(&conn, hero_b, "Shared Hero", "snapshot_name");

    insert_backfill_rows(&conn);

    let first = resolve_gaps_with_conn(&conn, false).expect("resolve backfill gaps");
    assert_eq!(first["entity_id_backfill"]["changed"], 5);
    for table in [
        "hero_stat_profiles",
        "hero_stat_values",
        "sheet_heroes_stats",
        "sheet_raw_heroes",
        "sheet_hero_rankings",
    ] {
        let count: i64 = conn
            .query_row(
                &format!("SELECT COUNT(*) FROM {table} WHERE hero_name='Abrams' AND entity_id=?1"),
                [abrams_id],
                |row| row.get(0),
            )
            .expect("backfilled table");
        assert_eq!(count, 1, "{table}");
    }

    let ambiguous: Option<i64> = conn
        .query_row(
            "SELECT entity_id FROM sheet_raw_heroes WHERE hero_name='Shared Hero'",
            [],
            |row| row.get(0),
        )
        .expect("ambiguous row");
    assert_eq!(ambiguous, None);
    let empty: Option<i64> = conn
        .query_row(
            "SELECT entity_id FROM sheet_heroes_stats WHERE hero_name=''",
            [],
            |row| row.get(0),
        )
        .expect("empty row");
    assert_eq!(empty, None);

    let second = resolve_gaps_with_conn(&conn, false).expect("resolve backfill gaps again");
    assert_eq!(second["entity_id_backfill"]["changed"], 0);
}

fn parse_multi_entity_patch_line(order: [&str; 3]) -> (String, String) {
    let (_temp, conn) = test_conn();
    for name in order {
        let entity_id = insert_entity(&conn, "hero", name);
        insert_alias(&conn, entity_id, name, "canonical");
    }
    insert_snapshot(
        &conn,
        "deadlock_data",
        "patchnote",
        "changelogs/raw/2026-06-01",
        Some("06-01-2026"),
        json!({
            "title": "06-01-2026",
            "posted_at": "2026-06-01",
            "raw_content": "General\n- Fixed Ivy, Viscous and Magic Carpet moving faster than intended.\n"
        }),
        None,
    );

    let summary = parse_patchnotes_with_conn(&conn, true).expect("parse multi entity patch");
    assert_eq!(summary["events_inserted"], 1);
    conn.query_row(
        "SELECT subject, event_hash FROM patch_events WHERE normalized_line LIKE 'Fixed Ivy%'",
        [],
        |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
    )
    .expect("multi entity patch event")
}

fn insert_youtube_video(conn: &Connection, video_id: &str) {
    conn.execute(
        r#"
        INSERT INTO youtube_feed_sources(
          feed_key, source_type, url, handle, playlist_id, channel_id, title,
          enabled, metadata_json, created_at, updated_at
        )
        VALUES('test-feed','channel','https://example.invalid',NULL,NULL,NULL,'Test',1,'{}',1,1)
        ON CONFLICT(feed_key) DO NOTHING
        "#,
        [],
    )
    .expect("insert youtube feed");
    conn.execute(
        r#"
        INSERT INTO youtube_videos(
          video_id, feed_key, channel_id, channel_title, title, url,
          published_at, description, metadata_json, transcript_status,
          learning_status, discovered_at, updated_at
        )
        VALUES(?1,'test-feed',NULL,NULL,'Test Video','https://example.invalid/video',
               NULL,NULL,'{}','missing','queued',1,1)
        "#,
        [video_id],
    )
    .expect("insert youtube video");
}

fn insert_backfill_rows(conn: &Connection) {
    let profile_snapshot = insert_snapshot(
        conn,
        "deadlock_stats_sheet",
        "hero_stats_sheet",
        "profile-backfill",
        Some("Abrams"),
        json!({"values": {"Hero Name": "Abrams"}}),
        None,
    );
    conn.execute(
        r#"
        INSERT INTO hero_stat_profiles(
          snapshot_id, entity_id, hero_name, source, external_id, payload_hash,
          row_number, created_at, updated_at
        )
        VALUES(?1,NULL,'Abrams','test','profile-backfill','profile-hash',1,1,1)
        "#,
        [profile_snapshot],
    )
    .expect("insert stat profile");
    let profile_id = conn.last_insert_rowid();
    conn.execute(
        r#"
        INSERT INTO hero_stat_values(
          profile_id, entity_id, hero_name, stat_key, stat_label,
          numeric_value, raw_value, created_at, updated_at
        )
        VALUES(?1,NULL,'Abrams','base_hp','Base HP',600.0,'600',1,1)
        "#,
        [profile_id],
    )
    .expect("insert stat value");

    let heroes_stats_snapshot = insert_snapshot(
        conn,
        "deadlock_stats_sheet",
        "sheet_row",
        "heroes-stats-backfill",
        Some("Abrams"),
        json!({"sheet_name": "Hero Stats"}),
        None,
    );
    conn.execute(
        r#"
        INSERT INTO sheet_heroes_stats(
          snapshot_id, entity_id, hero_name, payload_hash, created_at, updated_at
        )
        VALUES(?1,NULL,'Abrams','heroes-stats-hash',1,1)
        "#,
        [heroes_stats_snapshot],
    )
    .expect("insert heroes stats");

    let empty_snapshot = insert_snapshot(
        conn,
        "deadlock_stats_sheet",
        "sheet_row",
        "heroes-stats-empty",
        None,
        json!({"sheet_name": "Hero Stats"}),
        None,
    );
    conn.execute(
        r#"
        INSERT INTO sheet_heroes_stats(
          snapshot_id, entity_id, hero_name, payload_hash, created_at, updated_at
        )
        VALUES(?1,NULL,'','heroes-stats-empty-hash',1,1)
        "#,
        [empty_snapshot],
    )
    .expect("insert empty heroes stats");

    let raw_snapshot = insert_snapshot(
        conn,
        "deadlock_stats_sheet",
        "sheet_row",
        "raw-heroes-backfill",
        Some("Abrams"),
        json!({"sheet_name": "Raw Heroes"}),
        None,
    );
    conn.execute(
        r#"
        INSERT INTO sheet_raw_heroes(
          snapshot_id, entity_id, hero_name, payload_hash, created_at, updated_at
        )
        VALUES(?1,NULL,'Abrams','raw-heroes-hash',1,1)
        "#,
        [raw_snapshot],
    )
    .expect("insert raw heroes");

    let ambiguous_snapshot = insert_snapshot(
        conn,
        "deadlock_stats_sheet",
        "sheet_row",
        "raw-heroes-ambiguous",
        Some("Shared Hero"),
        json!({"sheet_name": "Raw Heroes"}),
        None,
    );
    conn.execute(
        r#"
        INSERT INTO sheet_raw_heroes(
          snapshot_id, entity_id, hero_name, payload_hash, created_at, updated_at
        )
        VALUES(?1,NULL,'Shared Hero','raw-heroes-ambiguous-hash',1,1)
        "#,
        [ambiguous_snapshot],
    )
    .expect("insert ambiguous raw heroes");

    let ranking_snapshot = insert_snapshot(
        conn,
        "deadlock_stats_sheet",
        "sheet_row",
        "ranking-backfill",
        Some("Abrams"),
        json!({"sheet_name": "Hero meta ranking"}),
        None,
    );
    conn.execute(
        r#"
        INSERT INTO sheet_hero_rankings(
          snapshot_id, entity_id, hero_name, payload_hash, created_at, updated_at
        )
        VALUES(?1,NULL,'Abrams','ranking-hash',1,1)
        "#,
        [ranking_snapshot],
    )
    .expect("insert ranking");
}
