use brain_storage::gamefacts::{
    limits, Alias, Entity, Fact, FactSet, FactValue, GameFactsStore, Receipt, Relation,
};
use sha2::{Digest, Sha256};

#[path = "support/gamefacts_pg.rs"]
pub mod gamefacts_pg;

#[tokio::test]
async fn mixed_boundary_applications_and_empty_sets_do_not_accumulate_old_receipts() {
    let (_pg, admin, writer) = gamefacts_pg::fixture(1).await;
    let store = GameFactsStore::new(writer.clone());
    let run = store.begin_run("cap").await.unwrap();
    let a = store
        .capture(run, "cap", "r1", "hero.json", b"120".to_vec())
        .await
        .unwrap();
    let b = store
        .capture(run, "cap", "r1", "hero.json", b"125".to_vec())
        .await
        .unwrap();
    for raw in [&a, &b, &a] {
        let receipt = Receipt {
            locator_kind: "raw_bytes".into(),
            locator: String::new(),
            value_sha256: Sha256::digest(&raw.bytes).into(),
        };
        let set = gamefacts_pg::fill_to_row_limit(FactSet {
            entities: vec![Entity {
                kind: "hero".into(),
                key: "hero_cap".into(),
                receipt: receipt.clone(),
            }],
            aliases: vec![Alias {
                entity_key: "hero_cap".into(),
                alias: "Cap".into(),
                lang: "en".into(),
                receipt: receipt.clone(),
            }],
            facts: vec![Fact {
                entity_key: "hero_cap".into(),
                stat_key: "health".into(),
                value: FactValue::Number(120.0),
                receipt: receipt.clone(),
            }],
            relations: vec![Relation {
                from_key: "hero_cap".into(),
                kind: "related_to".into(),
                to_key: "hero_cap".into(),
                receipt,
            }],
        });
        store.apply(run, raw, "cap", &set).await.unwrap();
    }
    gamefacts_pg::assert_boundary_applications(&admin, run).await;
    for raw in [&a, &b, &a] {
        store
            .apply(run, raw, "cap", &FactSet::default())
            .await
            .unwrap();
    }
    gamefacts_pg::assert_count(
        &admin,
        "SELECT count(*) FROM canon.receipt",
        (3 * limits::ROWS + 3) as i64,
    )
    .await;
    writer.close().await;
    admin.close().await;
}
