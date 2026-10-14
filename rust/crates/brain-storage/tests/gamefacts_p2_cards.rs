use brain_contracts::tools::{ToolName, ToolResult};
use brain_storage::gamefacts::{
    receipts, Alias, Counts, Entity, Fact, FactSet, FactValue, GameFactsStore, Receipt,
};
use brain_storage::PgStore;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions, PgSslMode};
#[path = "support/scratch_pg.rs"]
mod scratch_pg;

const RAW: &str = r#"[{"id":1,"class_name":"upgrade_toxic_bullets","type":"upgrade","cost":1250,"item_tier":2,"item_slot_type":"weapon","description":{"passive":"Bleeds"},"component_items":["upgrade_headshot_booster","upgrade_missing_item"]},{"id":2,"class_name":"upgrade_headshot_booster","type":"upgrade","cost":500,"item_tier":1},{"id":3,"class_name":"citadel_ability_haze_dagger","type":"ability","properties":{"Damage":{"value":40}},"upgrades":[{"property_upgrades":[{"name":"Damage","bonus":10}]}]}]"#;
const BULK: &str = "WITH run AS (INSERT INTO ops.ingest_run(source) VALUES('bulk') RETURNING id)
INSERT INTO raw.object(source,revision,path,source_path_sha256,content_sha256,bytes,ingest_run_id)
SELECT 'bulk','bulk-rev','bulk/'||g,sha256(convert_to('bulk/'||g,'UTF8')),sha256(convert_to(g::text,'UTF8')),convert_to(g::text,'UTF8'),run.id FROM generate_series(1,3000) g,run;
INSERT INTO canon.receipt(raw_object_id,ingest_run_id,extractor,locator_kind,locator,value_sha256)
SELECT o.id,o.ingest_run_id,'bulk','raw_bytes','/'||s,o.content_sha256 FROM raw.object o,generate_series(0,24) s WHERE o.source='bulk';
INSERT INTO canon.entity(kind,key,receipt_id) SELECT 'item','bulk_item_'||raw_object_id,id FROM canon.receipt WHERE extractor='bulk' AND locator='/0';
INSERT INTO canon.entity_identity(entity_id,receipt_id) SELECT id,receipt_id FROM canon.entity WHERE key LIKE 'bulk\\_%';
INSERT INTO canon.fact(entity_id,stat_key,value_num,valid_from_run,receipt_id)
SELECT e.id,'bulk_stat_'||substr(r.locator,2),substr(r.locator,2)::int,r.ingest_run_id,r.id
FROM canon.receipt r JOIN canon.receipt er ON er.raw_object_id=r.raw_object_id AND er.locator='/0'
JOIN canon.entity e ON e.receipt_id=er.id WHERE r.extractor='bulk' AND r.locator<>'/0';
UPDATE ops.ingest_run SET status='succeeded' WHERE source='bulk' AND status='failed';
ANALYZE";
const COMPARE: &str = include_str!("../../brain-serve/src/pg_asset_compare.sql");

fn receipt(pointer: &str) -> Receipt {
    let mut value = Receipt {
        locator_kind: "json_pointer".into(),
        locator: pointer.into(),
        value_sha256: [0; 32],
    };
    let bytes = receipts::value_bytes(RAW.as_bytes(), &value).expect("pointer");
    value.value_sha256 = Sha256::digest(bytes).into();
    value
}

fn fact(entity: &str, stat: &str, value: FactValue, pointer: &str) -> Fact {
    Fact {
        entity_key: entity.into(),
        stat_key: stat.into(),
        value,
        receipt: receipt(pointer),
    }
}

fn fact_set() -> FactSet {
    let (toxic, booster, dagger) = (
        "upgrade_toxic_bullets",
        "upgrade_headshot_booster",
        "citadel_ability_haze_dagger",
    );
    let entity = |kind: &str, key: &str, at: usize| Entity {
        kind: kind.into(),
        key: key.into(),
        receipt: receipt(&format!("/{at}/class_name")),
    };
    let text = |value: &str| FactValue::Text(value.into());
    let mut set = FactSet {
        entities: vec![
            entity("item", toxic, 0),
            entity("item", booster, 1),
            entity("ability", dagger, 2),
        ],
        aliases: [toxic, booster, dagger]
            .into_iter()
            .enumerate()
            .map(|(at, key)| Alias {
                entity_key: key.into(),
                alias: key.into(),
                lang: "und".into(),
                receipt: receipt(&format!("/{at}/class_name")),
            })
            .collect(),
        facts: vec![
            fact(toxic, "item_cost", FactValue::Number(1250.0), "/0/cost"),
            fact(toxic, "item_tier", FactValue::Number(2.0), "/0/item_tier"),
            fact(toxic, "item_slot", text("weapon"), "/0/item_slot_type"),
            fact(
                toxic,
                "item_description_passive",
                text("Bleeds"),
                "/0/description/passive",
            ),
            fact(
                toxic,
                "component_item_0",
                text(booster),
                "/0/component_items/0",
            ),
            fact(
                toxic,
                "component_item_1",
                text("upgrade_missing_item"),
                "/0/component_items/1",
            ),
            fact(booster, "item_cost", FactValue::Number(500.0), "/1/cost"),
            fact(booster, "item_tier", FactValue::Number(1.0), "/1/item_tier"),
            fact(
                dagger,
                "ability_damage",
                FactValue::Number(40.0),
                "/2/properties/Damage/value",
            ),
        ],
        ..FactSet::default()
    };
    let bonus = "/2/upgrades/0/property_upgrades/0/bonus";
    set.facts.push(fact(
        dagger,
        "identity_key_en",
        text(dagger),
        "/2/class_name",
    ));
    set.facts.push(fact(
        dagger,
        "p2_v_0000000000000001_ability_damage",
        FactValue::Number(10.0),
        bonus,
    ));
    for (key, value) in [
        ("source_language", "en"),
        ("upgrade_tier", "1"),
        ("upgrade_entry", "0"),
        ("operation", "delta"),
        ("count", "4"),
    ] {
        set.facts.push(fact(
            dagger,
            &format!("p2_q_0000000000000001_{key}"),
            text(value),
            bonus,
        ));
    }
    set
}

fn citations(value: &Value, out: &mut Vec<i64>) {
    match value {
        Value::Object(map) => {
            if let Some(id) = map.get("citation").and_then(Value::as_str) {
                out.push(id.parse().expect("numeric receipt id"));
            }
            map.values().for_each(|child| citations(child, out));
        }
        Value::Array(rows) => rows.iter().for_each(|child| citations(child, out)),
        _ => {}
    }
}

fn plan_nodes(plan: &Value, out: &mut Vec<String>) {
    let node = plan["Node Type"].as_str().unwrap_or_default();
    match plan["Relation Name"].as_str() {
        Some(table) => out.push(format!(
            "{node} {}.{table}",
            plan["Schema"].as_str().unwrap_or_default()
        )),
        None if node.contains("Index") => out.push(node.into()),
        None => {}
    }
    for child in plan["Plans"].as_array().into_iter().flatten() {
        plan_nodes(child, out);
    }
}

fn index_nodes(plan: &Value) -> Vec<String> {
    let mut nodes = Vec::new();
    plan_nodes(&plan[0]["Plan"], &mut nodes);
    for table in ["canon.fact", "canon.receipt"] {
        assert!(!nodes.contains(&format!("Seq Scan {table}")), "{nodes:?}");
    }
    assert!(nodes
        .iter()
        .any(|n| n.ends_with("canon.fact") && n.contains("Index")));
    nodes
}

async fn role_pool(options: &PgConnectOptions, role: &'static str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(1)
        .after_connect(move |c, _| {
            Box::pin(async move {
                sqlx::query(&format!("SET ROLE {role}"))
                    .execute(&mut *c)
                    .await?;
                if role == "brain_tool" {
                    sqlx::query("SET SESSION default_transaction_read_only=on")
                        .execute(&mut *c)
                        .await?;
                    sqlx::query("SET SESSION statement_timeout='500ms'")
                        .execute(&mut *c)
                        .await?;
                }
                Ok(())
            })
        })
        .connect_with(options.clone())
        .await
        .expect("role pool")
}

#[tokio::test]
async fn p2_cards_carry_published_receipts_hide_unresolved_components_and_use_indexes() {
    let pg = scratch_pg::ScratchPg::start();
    let options = PgConnectOptions::new_without_pgpass()
        .host(&pg.directory.join("socket").to_string_lossy())
        .port(55439)
        .username("brain_core_test")
        .database("postgres")
        .password("")
        .ssl_mode(PgSslMode::Disable);
    let admin = PgPoolOptions::new()
        .max_connections(2)
        .connect_with(options.clone())
        .await
        .expect("scratch DB");
    sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly NOLOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate; GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate").execute(&admin).await.expect("roles");
    PgStore::new(admin.clone())
        .migrate_gamefacts()
        .await
        .expect("registered migrations");
    let store = GameFactsStore::new(role_pool(&options, "brain_ingest").await);
    let run = store.begin_run("assets").await.expect("run");
    let raw = store
        .capture(run, "assets", "assets-a", "items/en.json", RAW.into())
        .await
        .expect("capture");
    store
        .apply(run, &raw, "assets", &fact_set())
        .await
        .expect("apply");
    let counts = Counts {
        accepted: 1,
        ..Counts::default()
    };
    store.finish_run(run, true, counts).await.expect("finish");
    sqlx::raw_sql(BULK)
        .execute(&admin)
        .await
        .expect("bulk seed");
    let tool = role_pool(&options, "brain_tool").await;
    let id = |key: &'static str| {
        sqlx::query_scalar::<_, i64>("SELECT id FROM serve.current_entity WHERE key=$1")
            .bind(key)
            .fetch_one(&admin)
    };
    let (toxic, booster, dagger) = (
        id("upgrade_toxic_bullets").await.expect("toxic"),
        id("upgrade_headshot_booster").await.expect("booster"),
        id("citadel_ability_haze_dagger").await.expect("dagger"),
    );
    let card: Value = sqlx::query_scalar("SELECT card FROM serve.item_card WHERE entity_id=$1")
        .bind(toxic)
        .fetch_one(&tool)
        .await
        .expect("item card");
    let keys: Vec<&str> = card["facts"]["facts"]
        .as_array()
        .expect("facts")
        .iter()
        .filter_map(|f| f["stat_key"].as_str())
        .collect();
    assert_eq!(
        keys,
        [
            "item_cost",
            "item_description_passive",
            "item_slot",
            "item_tier"
        ]
    );
    assert_eq!(card["facts"]["facts"][0]["value"].as_f64(), Some(1250.0));
    assert_eq!(card["facts"]["omitted_count"], 0);
    let links = card["links"]["links"].as_array().expect("links");
    assert_eq!(links.len(), 1);
    assert_eq!(links[0]["entity_id"], json!(booster));
    assert_eq!(links[0]["identity"]["entity_id"], json!(booster));
    assert_eq!(card["links"]["omitted_count"], 0);
    assert!(!card.to_string().contains("upgrade_missing_item"));
    let ability: Value =
        sqlx::query_scalar("SELECT card FROM serve.ability_card WHERE entity_id=$1")
            .bind(dagger)
            .fetch_one(&tool)
            .await
            .expect("ability card");
    assert_eq!(ability["facts"]["facts"][0]["stat_key"], "ability_damage");
    let damage: Vec<_> = ability["facts"]["facts"]
        .as_array()
        .expect("ability facts")
        .iter()
        .filter(|fact| fact["stat_key"] == "ability_damage")
        .collect();
    assert_eq!(damage.len(), 2);
    assert!(damage
        .iter()
        .any(|fact| fact["value"].as_f64() == Some(40.0) && fact["condition"] == json!({})));
    assert!(damage
        .iter()
        .any(|fact| fact["value"].as_f64() == Some(10.0)
            && fact["condition"]["upgrade_tier"] == 1
            && fact["condition"]["operation"] == "delta"));
    assert_eq!(ability["hero_selection"]["available"], false);
    let bulk: Vec<i64> = sqlx::query_scalar(
        "SELECT id FROM serve.current_entity WHERE key LIKE 'bulk\\_%' ORDER BY id DESC LIMIT 4",
    )
    .fetch_all(&admin)
    .await
    .expect("bulk ids");
    let stats: Vec<String> = (1..=12).map(|s| format!("bulk_stat_{s}")).collect();
    let compare: Value = sqlx::query_scalar(COMPARE)
        .bind(&bulk)
        .bind(&stats)
        .fetch_one(&tool)
        .await
        .expect("compare");
    assert_eq!(compare["stats"]["facts"].as_array().map(Vec::len), Some(48));
    let mut ids = Vec::new();
    for value in [&card, &ability, &compare] {
        citations(value, &mut ids);
    }
    ids.sort_unstable();
    ids.dedup();
    let published: i64 =
        sqlx::query_scalar("SELECT count(*) FROM serve.published_receipt WHERE id=ANY($1)")
            .bind(&ids)
            .fetch_one(&admin)
            .await
            .expect("published receipts");
    assert_eq!(published, ids.len() as i64);
    let result = ToolResult {
        call_id: "\"".repeat(512),
        name: ToolName::Compare,
        result: compare,
        evidence_ids: ids.iter().map(i64::to_string).collect(),
        is_error: false,
    };
    let bytes = serde_json::to_vec(&result).expect("serialize").len();
    println!("compare 4x12 ToolResult bytes: {bytes}");
    assert!(bytes <= 16_384);
    let plan: Value = sqlx::query_scalar(
        "EXPLAIN (FORMAT JSON,VERBOSE) SELECT card FROM serve.item_card WHERE entity_id=$1",
    )
    .bind(bulk[0])
    .fetch_one(&tool)
    .await
    .expect("item card plan");
    println!("item card plan: {:?}", index_nodes(&plan));
    let plan: Value = sqlx::query_scalar(&format!("EXPLAIN (FORMAT JSON,VERBOSE) {COMPARE}"))
        .bind(&bulk)
        .bind(&stats)
        .fetch_one(&tool)
        .await
        .expect("compare plan");
    println!("compare plan: {:?}", index_nodes(&plan));
}
