use brain_storage::gamefacts::{Counts, Fact, FactSet, FactValue, GameFactsStore, Receipt};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::PgPool;

const ASSETS_REVISION: &str = "assets-fixture";
const HERO_SLOTS: &str = "v1:17:assets_hero_slots:1:1";

pub async fn playable(pool: &PgPool, key: &str) {
    flagged(
        pool,
        key,
        json!({"player_selectable":true,"disabled":false,"in_development":false}),
    )
    .await;
}

pub async fn flagged(pool: &PgPool, key: &str, flags: Value) {
    let present: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM canon.fact f JOIN canon.entity e ON e.id=f.entity_id WHERE e.key=$1 AND f.stat_key='p2_slot_1_hero')")
        .bind(key).fetch_one(pool).await.unwrap();
    assert!(!present, "assets hero fixture for {key} is already set up");
    let entity: i64 = sqlx::query_scalar("SELECT id FROM canon.entity WHERE key=$1")
        .bind(key)
        .fetch_one(pool)
        .await
        .unwrap();
    let mut object = flags;
    object["class_name"] = json!(key);
    let store = GameFactsStore::new(pool.clone());
    let run = store.begin_run("assets").await.unwrap();
    let raw = store
        .capture(
            run,
            "assets",
            ASSETS_REVISION,
            &format!("heroes/en/{entity}.json"),
            serde_json::to_vec(&object).unwrap(),
        )
        .await
        .unwrap();
    let set = FactSet {
        facts: vec![Fact {
            entity_key: key.into(),
            stat_key: "p2_slot_1_hero".into(),
            value: FactValue::Text(key.into()),
            receipt: Receipt {
                locator_kind: "json_pointer".into(),
                locator: "/class_name".into(),
                value_sha256: Sha256::digest(serde_json::to_vec(key).unwrap()).into(),
            },
        }],
        ..FactSet::default()
    };
    store.apply(run, &raw, HERO_SLOTS, &set).await.unwrap();
    let snapshot = store
        .capture(
            run,
            "assets",
            ASSETS_REVISION,
            "heroes/en/observed/complete.snapshot",
            ASSETS_REVISION.as_bytes().to_vec(),
        )
        .await
        .unwrap();
    store
        .apply(run, &snapshot, HERO_SLOTS, &FactSet::default())
        .await
        .unwrap();
    store
        .finish_run(
            run,
            true,
            Counts {
                accepted: 2,
                ..Counts::default()
            },
        )
        .await
        .unwrap();
}
