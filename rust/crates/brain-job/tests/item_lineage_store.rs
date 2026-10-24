use brain_job::{
    ingest,
    item_lineage::{self, ItemLineage},
    patch_extract::{PatchExtractor, SOURCE},
    Alias, Entity, Extractor, Fact, FactSet, FactValue, GameFactsStore, InputObject, Outcome,
    RawObject, Receipt,
};
use brain_storage::PgStore;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgPool,
};

#[path = "../../brain-storage/tests/support/scratch_pg.rs"]
mod scratch_pg;

struct Assets;

fn pointer(locator: &str, lexeme: &str) -> Receipt {
    Receipt {
        locator_kind: "json_pointer".into(),
        locator: locator.into(),
        value_sha256: Sha256::digest(lexeme.as_bytes()).into(),
    }
}

impl Extractor for Assets {
    fn name(&self) -> &'static str {
        "assets_fixture"
    }
    fn version(&self) -> &'static str {
        "1"
    }
    fn accepts(&self, source: &str, _: &str) -> bool {
        source == "assets"
    }
    fn extract(&self, raw: &RawObject) -> Outcome {
        let Ok(names) = serde_json::from_slice::<Value>(&raw.bytes) else {
            return Outcome::Ignored;
        };
        let key = raw.path.trim_end_matches(".json").to_owned();
        let mut set = FactSet::default();
        set.entities.extend([Entity {
            kind: "item".into(),
            key: key.clone(),
            receipt: Receipt {
                locator_kind: "raw_bytes".into(),
                locator: String::new(),
                value_sha256: Sha256::digest(&raw.bytes).into(),
            },
        }]);
        for (field, lang) in [("name_en", "en"), ("name_de", "de")] {
            let Some(name) = names[field].as_str() else {
                return Outcome::Ignored;
            };
            let lexeme = serde_json::to_string(name).unwrap_or_default();
            set.aliases.extend([Alias {
                entity_key: key.clone(),
                alias: name.into(),
                lang: lang.into(),
                receipt: pointer(&format!("/{field}"), &lexeme),
            }]);
            set.facts.extend([Fact {
                entity_key: key.clone(),
                stat_key: field.into(),
                value: FactValue::Text(name.into()),
                receipt: pointer(&format!("/{field}"), &lexeme),
            }]);
        }
        Outcome::Accepted(set)
    }
}

fn asset(key: &str, en: &str, de: &str) -> anyhow::Result<InputObject> {
    Ok(InputObject {
        revision: "assets_r1".into(),
        path: format!("{key}.json"),
        bytes: serde_json::to_vec(&json!({"name_en":en,"name_de":de}))?,
    })
}

fn post(id: u64, posted_at: &str, content: &str) -> anyhow::Result<InputObject> {
    Ok(InputObject {
        revision: "patch_r1".into(),
        path: id.to_string(),
        bytes: serde_json::to_vec(&json!({"id":id,"title":format!("Update {id}"),
            "url":format!("https://forums.playdeadlock.com/threads/update.{id}/"),
            "posted_at":posted_at,"raw_content":content}))?,
    })
}

async fn scratch() -> (scratch_pg::ScratchPg, PgPool, GameFactsStore) {
    let pg = scratch_pg::ScratchPg::start();
    let options = PgConnectOptions::new_without_pgpass()
        .host(&pg.directory.join("socket").to_string_lossy())
        .port(55439)
        .username("brain_core_test")
        .database("postgres")
        .ssl_mode(PgSslMode::Disable);
    let admin = PgPoolOptions::new()
        .max_connections(2)
        .connect_with(options.clone())
        .await
        .unwrap();
    sqlx::raw_sql("CREATE ROLE brain_migrate NOLOGIN; CREATE ROLE brain_ingest NOLOGIN; CREATE ROLE brain_service NOLOGIN; CREATE ROLE brain_readonly NOLOGIN; GRANT CREATE ON DATABASE postgres TO brain_migrate; GRANT USAGE,CREATE ON SCHEMA public TO brain_migrate").execute(&admin).await.unwrap();
    PgStore::new(admin.clone())
        .migrate_gamefacts()
        .await
        .unwrap();
    let writer = PgPoolOptions::new()
        .max_connections(2)
        .after_connect(|c, _| {
            Box::pin(async move {
                sqlx::query("SET ROLE brain_ingest").execute(c).await?;
                Ok(())
            })
        })
        .connect_with(options)
        .await
        .unwrap();
    (pg, admin, GameFactsStore::new(writer))
}

async fn id(pool: &PgPool, key: &str) -> i64 {
    sqlx::query_scalar("SELECT id FROM canon.entity WHERE key=$1 AND kind='item'")
        .bind(key)
        .fetch_one(pool)
        .await
        .unwrap()
}

async fn lineage(pool: &PgPool, key: &str) -> Value {
    sqlx::query_scalar("SELECT serve.item_lineage($1,5)")
        .bind(id(pool, key).await)
        .fetch_one(pool)
        .await
        .unwrap()
}

async fn rename_targets(pool: &PgPool, name: &str) -> Vec<i64> {
    sqlx::query_scalar("SELECT entity_id FROM serve.item_rename_target($1)")
        .bind(name)
        .fetch_all(pool)
        .await
        .unwrap()
}

#[tokio::test]
async fn lineage_resolves_former_names_removals_and_patch_history() {
    let (_pg, admin, store) = scratch().await;
    let named = [
        (
            "upgrade_reduce_debuff_duration",
            "Dispel Magic",
            "Magie der Entzauberung",
        ),
        (
            "upgrade_tech_purge",
            "Spirit Resilience",
            "Verstärkte Magierüstung",
        ),
        ("upgrade_tech_armor", "Spirit Armor", "Magierüstung"),
    ];
    let data: Value = serde_json::from_str(include_str!("../data/item-lineage.json")).unwrap();
    let mut keys: Vec<String> = data["entries"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|entry| [&entry["item"], &entry["successor"]])
        .filter_map(|key| key.as_str().map(str::to_owned))
        .filter(|key| named.iter().all(|(named, _, _)| named != key))
        .collect();
    keys.sort();
    keys.dedup();
    let assets: Vec<_> = named
        .iter()
        .map(|(key, en, de)| asset(key, en, de))
        .chain(keys.iter().map(|key| asset(key, key, key)))
        .collect();
    let expected = assets.len() as i64;
    let report = ingest(&store, "assets", assets, &[&Assets]).await.unwrap();
    assert_eq!(report.counts.accepted as i64, expected);
    let root = std::path::Path::new("/");
    let report = ingest(
        &store,
        item_lineage::SOURCE,
        item_lineage::load(root, false),
        &[&ItemLineage],
    )
    .await
    .unwrap();
    assert_eq!((report.counts.accepted, report.counts.rejected), (1, 0));
    let posts = [
        post(
            7,
            "2026-01-30T20:00:00+00:00",
            "[ Items ]\n- Debuff Remover: Now renamed to Dispel Magic\n- Old Purger: Renamed to Dispel Magic\n[ Heroes ]\n- Abrams: Dispel Magic now affects him",
        ),
        post(
            8,
            "2026-03-01T20:00:00+00:00",
            "Items:\n- Dispel Magic: Cooldown increased from 45s to 50s\n- Spirit Resilience: Spirit Resist increased from 8% to 10%",
        ),
    ];
    let report = ingest(&store, SOURCE, posts, &[&PatchExtractor])
        .await
        .unwrap();
    assert_eq!((report.counts.accepted, report.counts.rejected), (2, 0));

    let dispel = lineage(&admin, "upgrade_reduce_debuff_duration").await;
    assert_eq!(dispel["status"], "current");
    assert_eq!(dispel["item"]["name_en"], "Dispel Magic");
    let former: Vec<(&str, &str)> = dispel["former_names"]
        .as_array()
        .unwrap()
        .iter()
        .map(|name| {
            (
                name["name"].as_str().unwrap(),
                name["origin"].as_str().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        former,
        [("Debuff Remover", "lineage"), ("Old Purger", "patch_note")]
    );
    assert!(dispel["former_names"]
        .as_array()
        .unwrap()
        .iter()
        .all(
            |name| name["citation"].as_str().is_some_and(|c| !c.is_empty())
                && name["source_revision"].as_str().is_some()
                && name["patch"].is_null()
        ));
    let changes: Vec<&str> = dispel["history"]["changes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|change| change["line_text"].as_str().unwrap())
        .collect();
    assert_eq!(
        changes,
        [
            "Dispel Magic: Cooldown increased from 45s to 50s",
            "Debuff Remover: Now renamed to Dispel Magic",
            "Old Purger: Renamed to Dispel Magic",
        ]
    );
    assert_eq!(dispel["history"]["changes"][0]["patch_date"], "2026-03-01");
    assert_eq!(dispel["history"]["truncated"], false);

    let armor = lineage(&admin, "upgrade_tech_armor").await;
    assert_eq!(armor["status"], "removed");
    assert_eq!(
        armor["removed"]["successor"]["name_en"],
        "Spirit Resilience"
    );
    let resilience = lineage(&admin, "upgrade_tech_purge").await;
    assert_eq!(resilience["status"], "current");
    assert_eq!(resilience["predecessors"][0]["name_en"], "Spirit Armor");
    assert_eq!(
        resilience["former_names"][0]["name"],
        "Improved Spirit Armor"
    );
    assert_eq!(
        resilience["history"]["changes"].as_array().unwrap().len(),
        1
    );

    let dispel_id = id(&admin, "upgrade_reduce_debuff_duration").await;
    assert_eq!(rename_targets(&admin, "old purger").await, [dispel_id]);
    assert_eq!(rename_targets(&admin, "debuff remover").await, [dispel_id]);
    assert!(rename_targets(&admin, "abrams").await.is_empty());
    let site: Value = sqlx::query_scalar("SELECT serve.site_item_lineage($1)")
        .bind("upgrade_reduce_debuff_duration")
        .fetch_one(&admin)
        .await
        .unwrap();
    assert_eq!(site["former_names"], dispel["former_names"]);
}
