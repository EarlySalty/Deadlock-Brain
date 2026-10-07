use anyhow::{anyhow, ensure, Result};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct PatchWindow {
    pub patch_tag: String,
    pub min_unix_timestamp: i64,
}

pub async fn latest_patch_window(pool: &PgPool) -> Result<PatchWindow> {
    let patch_tag = latest_patch_tag(pool).await?;
    let start = sqlx::query_scalar::<_, Option<i64>>(
        "SELECT floor(extract(epoch FROM min(pe.posted_at)))::bigint FROM brain.patch_events pe \
         WHERE COALESCE(NULLIF(to_jsonb(pe)->>'patch_external_id',''), \
           to_char(pe.posted_at AT TIME ZONE 'UTC','YYYY-MM-DD'))=$1",
    )
    .bind(&patch_tag)
    .fetch_one(pool)
    .await?
    .ok_or_else(|| anyhow!("Kein belegter Beginn des aktiven Patches {patch_tag}"))?;
    ensure!(
        patch_tag != "unknown" && start > 0,
        "Aktiver Patch ist nicht zeitlich belegt"
    );
    Ok(PatchWindow {
        patch_tag,
        min_unix_timestamp: start,
    })
}

pub async fn latest_patch_tag(pool: &PgPool) -> Result<String, sqlx::Error> {
    let tag = sqlx::query_scalar::<_, String>(
        "SELECT COALESCE(NULLIF(to_jsonb(pe)->>'patch_external_id', ''), to_char(pe.posted_at AT TIME ZONE 'UTC', 'YYYY-MM-DD')) FROM brain.patch_events pe WHERE NULLIF(to_jsonb(pe)->>'patch_external_id', '') IS NOT NULL OR pe.posted_at IS NOT NULL ORDER BY pe.posted_at DESC NULLS LAST, pe.id DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;
    Ok(tag.unwrap_or_else(|| "unknown".to_string()))
}
