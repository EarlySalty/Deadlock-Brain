use super::Site;
use anyhow::{ensure, Context, Result};
use axum::{
    extract::{rejection::JsonRejection, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, Row};
use std::{collections::BTreeMap, sync::atomic::Ordering};

#[derive(Serialize)]
struct Comment {
    text: String,
    ts: String,
}

#[derive(Deserialize)]
pub(super) struct Request {
    #[serde(default = "general")]
    key: String,
    #[serde(default)]
    action: String,
    #[serde(default)]
    text: String,
    #[serde(default)]
    ts: String,
}

fn general() -> String {
    "general".into()
}

pub(super) async fn preflight(pool: &PgPool) -> Result<()> {
    let mut tx = pool
        .begin()
        .await
        .context("Die Kommentardatenbank ist nicht verfügbar.")?;
    sqlx::raw_sql("SET TRANSACTION READ ONLY; SET LOCAL statement_timeout='5000ms'")
        .execute(&mut *tx)
        .await
        .context("Die Kommentardatenbank ist nicht verfügbar.")?;
    sqlx::query("SELECT id,group_key,text,ts FROM brain.site_comments_v1 LIMIT 0")
        .fetch_all(&mut *tx)
        .await
        .context("Die Kommentarmigration oder Leserechte fehlen.")?;
    let valid: bool = sqlx::query_scalar(
        "SELECT current_user='brain_site'
         AND NOT (SELECT rolsuper OR rolcreatedb OR rolcreaterole OR rolbypassrls FROM pg_roles WHERE rolname=current_user)
         AND NOT has_schema_privilege(current_user,'brain','CREATE')
         AND has_table_privilege(current_user,'brain.site_comments_v1','SELECT')
         AND has_table_privilege(current_user,'brain.site_comments_v1','INSERT')
         AND NOT has_table_privilege(current_user,'brain.site_comments_v1','UPDATE,DELETE,TRUNCATE')
         AND has_sequence_privilege(current_user,'brain.site_comments_v1_id_seq','USAGE')
         AND NOT has_sequence_privilege(current_user,'brain.site_comments_v1_id_seq','UPDATE')
         AND NOT EXISTS(SELECT 1 FROM pg_class c JOIN pg_namespace n ON n.oid=c.relnamespace
             WHERE n.nspname='brain' AND c.relkind IN ('r','p','v','m','f')
             AND c.relname<>'site_comments_v1'
             AND has_table_privilege(current_user,c.oid,'SELECT,INSERT,UPDATE,DELETE,TRUNCATE'))",
    )
    .fetch_one(&mut *tx)
    .await
    .context("Die isolierte Kommentarrolle konnte nicht geprüft werden.")?;
    ensure!(
        valid,
        "Die Site benötigt die isolierte Kommentarrolle brain_site."
    );
    tx.commit()
        .await
        .context("Die Kommentardatenbank ist nicht verfügbar.")
}

fn availability(site: &Site, available: bool) {
    if site.comments_available.swap(available, Ordering::Relaxed) != available {
        eprintln!(
            "Brain-Site: Kommentare {}.",
            if available {
                "wieder verfügbar"
            } else {
                "nicht verfügbar"
            }
        );
    }
}

fn unavailable(site: &Site) -> Response {
    availability(site, false);
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(json!({"error": "Kommentare sind gerade nicht verfügbar."})),
    )
        .into_response()
}

pub(super) async fn get_comments(State(site): State<Site>) -> Response {
    let result = sqlx::query("SELECT group_key,text,ts FROM brain.site_comments_v1 ORDER BY id")
        .fetch_all(&site.pool)
        .await;
    let Ok(rows) = result else {
        return unavailable(&site);
    };
    let mut groups: BTreeMap<String, Vec<Comment>> = BTreeMap::new();
    for row in rows {
        let Ok(key) = row.try_get::<String, _>("group_key") else {
            return unavailable(&site);
        };
        let Ok(comment) = comment(&row) else {
            return unavailable(&site);
        };
        groups.entry(key).or_default().push(comment);
    }
    availability(&site, true);
    Json(groups).into_response()
}

fn comment(row: &sqlx::postgres::PgRow) -> std::result::Result<Comment, sqlx::Error> {
    Ok(Comment {
        text: row.try_get("text")?,
        ts: row.try_get("ts")?,
    })
}

pub(super) async fn post_comment(
    State(site): State<Site>,
    body: std::result::Result<Json<Request>, JsonRejection>,
) -> Response {
    let request = match body {
        Ok(Json(mut request)) => {
            request.text = request.text.chars().take(4000).collect();
            request.ts = request.ts.chars().take(40).collect();
            if request.text.contains('\0')
                || request.key.chars().count() > 40
                || request.key.chars().any(char::is_control)
                || request.ts.contains(['<', '>', '&'])
                || request.ts.chars().any(char::is_control)
            {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "Ungültiger Kommentar."})),
                )
                    .into_response();
            }
            request
        }
        Err(error) => {
            return (
                if error.status() == StatusCode::PAYLOAD_TOO_LARGE {
                    StatusCode::PAYLOAD_TOO_LARGE
                } else {
                    StatusCode::BAD_REQUEST
                },
                Json(json!({"error": "Ungültige Kommentaranfrage."})),
            )
                .into_response();
        }
    };
    match append(&site.pool, request).await {
        Ok(comments) => {
            availability(&site, true);
            Json(json!({"ok": true, "comments": comments})).into_response()
        }
        Err(_) => unavailable(&site),
    }
}

async fn append(pool: &PgPool, request: Request) -> std::result::Result<Vec<Comment>, sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::query("SET LOCAL statement_timeout='5000ms'")
        .execute(&mut *tx)
        .await?;
    sqlx::query("SET LOCAL lock_timeout='5000ms'")
        .execute(&mut *tx)
        .await?;
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1,742110026113))")
        .bind(&request.key)
        .execute(&mut *tx)
        .await?;
    if request.action == "add" && !request.text.trim().is_empty() {
        sqlx::query("INSERT INTO brain.site_comments_v1(group_key,text,ts) VALUES($1,$2,$3)")
            .bind(&request.key)
            .bind(&request.text)
            .bind(&request.ts)
            .execute(&mut *tx)
            .await?;
    }
    let comments =
        sqlx::query("SELECT text,ts FROM brain.site_comments_v1 WHERE group_key=$1 ORDER BY id")
            .bind(&request.key)
            .fetch_all(&mut *tx)
            .await?
            .iter()
            .map(comment)
            .collect::<std::result::Result<Vec<_>, _>>()?;
    tx.commit().await?;
    Ok(comments)
}
