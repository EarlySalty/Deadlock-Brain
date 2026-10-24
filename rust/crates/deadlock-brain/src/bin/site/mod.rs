mod artifacts;
mod cache;
mod compare;
pub(crate) mod database;
mod hero;
mod pages;
pub(crate) mod profiles;

use anyhow::{ensure, Context, Result};
use axum::{
    extract::DefaultBodyLimit,
    http::{header, HeaderValue},
    response::Response,
    routing::get,
    Router,
};
use sqlx::PgPool;
use std::{net::SocketAddr, path::PathBuf, sync::Arc, time::Duration};
use tokio::sync::Semaphore;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[arg(long, hide = true)]
    pub corpus_root: Option<PathBuf>,
    #[command(flatten)]
    pub database: database::ConnectionArgs,
    #[arg(long, hide = true)]
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
    database::preflight(&pool).await?;
    let app = router(pool);
    let listener = tokio::net::TcpListener::bind(args.bind)
        .await
        .context("Der lokale Siteport ist nicht verfügbar.")?;
    axum::serve(listener, app)
        .await
        .context("Der Brain-Sitedienst wurde unterbrochen.")
}

#[derive(Clone)]
struct Site {
    pool: PgPool,
    requests: Arc<Semaphore>,
    pages: cache::PageCache,
}

const QUEUE_WAIT: Duration = Duration::from_secs(10);

fn router(pool: PgPool) -> Router {
    let site = Site {
        pool,
        requests: Arc::new(Semaphore::new(4)),
        pages: cache::PageCache::default(),
    };
    Router::<Site>::new()
        .route("/compare/{id}", get(compare::page))
        .route("/compare/{id}/chart.svg", get(artifacts::chart))
        .route("/site/compare/{id}", get(compare::page))
        .route("/site/compare/{id}/chart.svg", get(artifacts::chart))
        .route("/site/steckbriefe", get(profiles::index))
        .route("/site/steckbriefe/", get(profiles::index))
        .route("/site/steckbriefe/{kind}/{id}", get(profiles::page))
        .route("/site/entities/{kind}/{id}", get(profiles::page))
        .fallback(pages::page)
        .layer(DefaultBodyLimit::max(65536))
        .layer(axum::middleware::from_fn_with_state(
            site.clone(),
            limit_requests,
        ))
        .layer(axum::middleware::map_response(security_headers))
        .with_state(site)
}

async fn security_headers(mut response: Response) -> Response {
    let headers = response.headers_mut();
    headers.insert(
        "x-content-type-options",
        HeaderValue::from_static("nosniff"),
    );
    headers.insert("referrer-policy", HeaderValue::from_static("same-origin"));
    headers.insert("content-security-policy", HeaderValue::from_static("default-src 'none'; style-src 'unsafe-inline'; base-uri 'none'; frame-ancestors 'none'; form-action 'none'"));
    headers
        .entry(header::CACHE_CONTROL)
        .or_insert(HeaderValue::from_static("no-store"));
    response
}

#[cfg(test)]
mod tests;

async fn limit_requests(
    axum::extract::State(site): axum::extract::State<Site>,
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Response {
    let Ok(Ok(_permit)) = tokio::time::timeout(QUEUE_WAIT, site.requests.acquire_owned()).await
    else {
        eprintln!("site_busy");
        return profiles::unavailable(request.method().clone());
    };
    next.run(request).await
}
