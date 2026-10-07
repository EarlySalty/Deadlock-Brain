mod assets;
mod comments;

use anyhow::Result;
use axum::{
    extract::{DefaultBodyLimit, State},
    http::{header, Method, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use sqlx::PgPool;
use std::{
    path::Path,
    sync::{atomic::AtomicBool, Arc},
};

#[derive(Clone)]
struct Site {
    assets: Arc<assets::Assets>,
    pool: PgPool,
    comments_available: Arc<AtomicBool>,
}

pub async fn router(root: &Path, pool: PgPool) -> Result<Router> {
    comments::preflight(&pool).await?;
    let assets = assets::Assets::new(root)?;
    Ok(Router::new()
        .route(
            "/api/comments",
            get(comments::get_comments).post(comments::post_comment),
        )
        .fallback(file)
        .layer(DefaultBodyLimit::max(65536))
        .layer(axum::middleware::map_response(security_headers))
        .with_state(Site {
            assets: Arc::new(assets),
            pool,
            comments_available: Arc::new(AtomicBool::new(true)),
        }))
}

async fn security_headers(mut response: Response) -> Response {
    let headers = response.headers_mut();
    headers.insert("x-content-type-options", "nosniff".parse().unwrap());
    headers.insert("referrer-policy", "same-origin".parse().unwrap());
    headers.insert(
        "content-security-policy",
        "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; object-src 'none'; base-uri 'self'; frame-ancestors 'none'"
            .parse()
            .unwrap(),
    );
    headers.insert(header::CACHE_CONTROL, "no-cache".parse().unwrap());
    response
}

async fn file(State(site): State<Site>, method: Method, uri: Uri) -> Response {
    if method != Method::GET && method != Method::HEAD {
        return (
            StatusCode::METHOD_NOT_ALLOWED,
            [(header::ALLOW, "GET, HEAD")],
            "Diese Anfrage wird nicht unterstützt.",
        )
            .into_response();
    }
    if uri.path() == "/site" {
        return (StatusCode::MOVED_PERMANENTLY, [(header::LOCATION, "site/")]).into_response();
    }
    let assets = site.assets.clone();
    let path = uri.path().to_owned();
    let result = tokio::task::spawn_blocking(move || assets.read(&path)).await;
    match result {
        Ok(Ok((content_type, bytes))) => {
            let length = bytes.len().to_string();
            (
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, content_type.to_string()),
                    (header::CONTENT_LENGTH, length),
                ],
                if method == Method::HEAD {
                    Vec::new()
                } else {
                    bytes
                },
            )
                .into_response()
        }
        _ => (StatusCode::NOT_FOUND, "Diese Seite ist nicht verfügbar.").into_response(),
    }
}

#[cfg(test)]
mod tests;
