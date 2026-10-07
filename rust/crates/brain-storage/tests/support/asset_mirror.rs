use brain_storage::asset_mirror::{latest_mirrored_client_version, load_mirrored_assets};
use serde_json::{json, Value};
use sqlx::PgPool;

pub async fn check(pool: &PgPool) {
    sqlx::raw_sql("CREATE TABLE brain.source_runs(id bigserial PRIMARY KEY,source text,status text,summary jsonb,
            started_at timestamptz DEFAULT '2026-10-07T00:00:00Z', finished_at timestamptz DEFAULT '2026-10-07T00:01:00Z');
        CREATE TABLE brain.source_documents(id bigserial PRIMARY KEY,source text,metadata jsonb,
            url text, content_hash text NOT NULL DEFAULT '', fetched_at timestamptz DEFAULT '2026-10-07T00:00:30Z');")
        .execute(pool).await.unwrap();
    assert!(latest_mirrored_client_version(pool).await.is_err());
    for (version, status) in [(6757, "ok"), (6759, "ok"), (6760, "error")] {
        let mut endpoints = serde_json::Map::new();
        for kind in ["items", "heroes", "heroes_all"] {
            for language in ["english", "german"] {
                let payload = payload(kind, language);
                let metadata = json!({"adapter":{"client_version":version,"kind":kind,"language":language},
                    "validation":{"state":"validated"},"contract":{"data":{"payload":{"status":"known","value":payload}}}});
                let id: i64 = sqlx::query_scalar("INSERT INTO brain.source_documents(source,metadata) VALUES('deadlock_assets_api',$1) RETURNING id")
                    .bind(metadata).fetch_one(pool).await.unwrap();
                endpoints.insert(
                    format!("{kind}/{language}"),
                    json!({"source_document_id":id}),
                );
            }
        }
        sqlx::query("INSERT INTO brain.source_runs(source,status,summary) VALUES('assets',$1,$2)")
            .bind(status)
            .bind(json!({"client_version":version,"mirror_complete":true,"endpoints":endpoints}))
            .execute(pool)
            .await
            .unwrap();
    }
    assert_eq!(latest_mirrored_client_version(pool).await.unwrap(), 6759);
    for version in [6757, 6759] {
        for language in ["english", "german"] {
            assert_eq!(
                load_mirrored_assets(pool, version, "items", language)
                    .await
                    .unwrap(),
                payload("items", language)
            );
        }
    }
    assert!(load_mirrored_assets(pool, 6760, "items", "english")
        .await
        .is_err());
    assert!(load_mirrored_assets(pool, 6759, "items", "french")
        .await
        .is_err());
    assert!(load_mirrored_assets(pool, 6759, "unknown", "english")
        .await
        .is_err());
    sqlx::query("UPDATE brain.source_documents SET metadata=jsonb_set(metadata,'{adapter,client_version}','1') WHERE metadata->'adapter'->>'client_version'='6759'")
        .execute(pool).await.unwrap();
    assert!(load_mirrored_assets(pool, 6759, "items", "german")
        .await
        .is_err());
}

fn payload(kind: &str, language: &str) -> Value {
    if kind == "items" {
        json!([{"id":1998374645,"name":if language=="german" {"Mystischer Ausbruch"} else {"Mystic Burst"},
            "properties":{"Damage":"40","MinimumDamage":"80","AbilityCooldown":"14","Radius":"16m"}}])
    } else {
        json!([{"id":13,"name":"Haze","items":{"signature1":"ability_sleep_dagger"}}])
    }
}
