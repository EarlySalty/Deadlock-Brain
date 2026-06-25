use rusqlite::{params, Connection};
use serde_json::{json, Value};
use tempfile::TempDir;

use crate::{
    enrich_legacy_entities_with_conn, enrich_lineage_with_conn, extract_lineage_candidates,
    normalize_entities_with_conn, normalize_sheet_stats_with_conn, normalize_sheet_tabs_with_conn,
    parse_patchnotes_with_conn, LineageEvent,
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
