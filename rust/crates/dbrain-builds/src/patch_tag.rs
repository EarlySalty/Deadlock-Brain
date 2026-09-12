use sqlx::PgPool;

pub async fn latest_patch_tag(pool: &PgPool) -> Result<String, sqlx::Error> {
    let tag = sqlx::query_scalar::<_, String>(
        "SELECT COALESCE(NULLIF(to_jsonb(pe)->>'patch_external_id', ''), to_char(pe.posted_at AT TIME ZONE 'UTC', 'YYYY-MM-DD')) FROM brain.patch_events pe WHERE NULLIF(to_jsonb(pe)->>'patch_external_id', '') IS NOT NULL OR pe.posted_at IS NOT NULL ORDER BY pe.posted_at DESC NULLS LAST, pe.id DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;
    Ok(tag.unwrap_or_else(|| "unknown".to_string()))
}
