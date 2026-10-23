use anyhow::{ensure, Result};
use axum::{
    extract::State,
    http::{header, Method, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use serde_json::json;
use sqlx::Row;

use super::{profiles, Site};

pub(super) async fn page(State(site): State<Site>, method: Method, uri: Uri) -> Response {
    if method != Method::GET && method != Method::HEAD {
        return (
            StatusCode::METHOD_NOT_ALLOWED,
            [(header::ALLOW, "GET, HEAD")],
            "Diese Anfrage wird nicht unterstützt.",
        )
            .into_response();
    }
    if matches!(uri.path(), "/site" | "/site/" | "/site/index.html") {
        return (
            StatusCode::TEMPORARY_REDIRECT,
            [(header::LOCATION, "/brain/site/steckbriefe")],
        )
            .into_response();
    }
    let Ok(path) = urlencoding::decode(uri.path()) else {
        return profiles::missing(method);
    };
    if path.len() > 2048
        || path.contains(['\\', '\0', '%'])
        || path.chars().any(char::is_control)
        || path.split('/').any(|p| p == "." || p == "..")
    {
        return profiles::missing(method);
    }
    let path = path
        .strip_prefix("/site/")
        .or_else(|| path.strip_prefix('/'))
        .unwrap_or(&path);
    if !path.starts_with("public/") || !path.ends_with(".html") {
        return profiles::missing(method);
    }
    let result: Result<Option<String>> = async {
        let mut tx = profiles::transaction(&site).await?;
        let available: bool = sqlx::query_scalar("SELECT to_regprocedure('serve.site_doc_page(text)') IS NOT NULL").fetch_one(&mut *tx).await?;
        if !available { return Ok(None); }
        let rows = sqlx::query("SELECT title,body,source_revision,receipt_id::text AS citation FROM serve.site_doc_page($1) ORDER BY ordinal LIMIT 65")
            .bind(path).fetch_all(&mut *tx).await?;
        if rows.is_empty() { return Ok(None); }
        ensure!(rows.len() <= 64, "doc_section_limit");
        let title: String = rows[0].try_get("title")?;
        let mut body = format!("<h1>{}</h1>", profiles::escape(&title));
        let mut citations = Vec::new();
        for row in rows {
            let text: String = row.try_get("body")?;
            ensure!(body.len().saturating_add(text.len().saturating_mul(6)) <= 2 * 1024 * 1024, "doc_size_limit");
            body.push_str(&format!("<article><pre>{}</pre><a href=\"#beleg-{}\">Beleg</a></article>", profiles::escape(&text), profiles::escape(&row.try_get::<String,_>("citation")?)));
            citations.push(json!({"citation":row.try_get::<String,_>("citation")?,"source_revision":row.try_get::<String,_>("source_revision")?}));
        }
        body.push_str(&profiles::receipts(&mut tx, &json!(citations)).await?);
        tx.commit().await?;
        Ok(Some(profiles::document(&title, &body)))
    }.await;
    match result {
        Ok(Some(html)) => profiles::response(StatusCode::OK, html, method),
        Ok(None) => profiles::missing(method),
        Err(_) => {
            eprintln!("site_document_unavailable");
            profiles::unavailable(method)
        }
    }
}
