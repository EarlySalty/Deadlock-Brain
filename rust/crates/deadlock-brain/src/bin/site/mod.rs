mod assets;
mod comments;
mod compare;
pub(crate) mod database;
mod profiles;

use anyhow::{ensure, Context, Result};
use axum::{
    extract::{DefaultBodyLimit, State},
    http::{header, Method, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use sqlx::PgPool;
use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::{atomic::AtomicBool, Arc},
};

#[derive(Debug, clap::Args)]
pub struct Args {
    #[arg(long)]
    pub corpus_root: PathBuf,
    #[command(flatten)]
    pub database: database::ConnectionArgs,
    #[arg(long)]
    pub serve_config: Option<PathBuf>,
    #[arg(long, default_value = "127.0.0.1:8087")]
    pub bind: SocketAddr,
}

pub async fn serve(args: Args) -> Result<()> {
    ensure!(
        args.bind.ip().is_loopback(),
        "Die Brain-Site darf nur auf Loopback lauschen."
    );
    let pool = args.database.pool("brain_site").await?;
    let release = args
        .serve_config
        .as_deref()
        .map(profiles::release_from_config)
        .transpose()?;
    let app = router_with_release(&args.corpus_root, pool, release).await?;
    let listener = tokio::net::TcpListener::bind(args.bind)
        .await
        .context("Der lokale Siteport ist nicht verfügbar.")?;
    axum::serve(listener, app)
        .await
        .context("Der Brain-Sitedienst wurde unterbrochen.")
}

#[derive(Clone)]
struct Site {
    assets: Arc<assets::Assets>,
    release: Option<String>,
    pool: PgPool,
    comments_available: Arc<AtomicBool>,
}

#[cfg(test)]
pub async fn router(root: &Path, pool: PgPool) -> Result<Router> {
    router_with_release(root, pool, None).await
}

async fn router_with_release(root: &Path, pool: PgPool, release: Option<String>) -> Result<Router> {
    comments::preflight(&pool).await?;
    let assets = assets::Assets::new(root)?;
    Ok(Router::new()
        .route(
            "/api/comments",
            get(comments::get_comments).post(comments::post_comment),
        )
        .route("/compare/{id}", get(compare::page))
        .route("/compare/{id}/chart.svg", get(compare::chart))
        .route("/site/compare/{id}", get(compare::page))
        .route("/site/compare/{id}/chart.svg", get(compare::chart))
        .route("/site/steckbriefe", get(profiles::index))
        .route("/site/steckbriefe/", get(profiles::index))
        .route("/site/steckbriefe/{kind}/{id}", get(profiles::page))
        .fallback(file)
        .layer(DefaultBodyLimit::max(65536))
        .layer(axum::middleware::map_response(security_headers))
        .with_state(Site {
            assets: Arc::new(assets),
            release,
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
    headers
        .entry(header::CACHE_CONTROL)
        .or_insert("no-cache".parse().unwrap());
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
