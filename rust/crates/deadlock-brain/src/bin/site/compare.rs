use axum::{
    extract::{Path, State},
    http::{header, Method, StatusCode},
    response::{IntoResponse, Response},
};
use brain_contracts::Principal;
use brain_maintenance::compare_artifact::render_stored_compare;
use brain_storage::PgStore;
use std::collections::BTreeSet;

use super::Site;

pub(super) async fn page(
    State(site): State<Site>,
    Path(id): Path<String>,
    method: Method,
) -> Response {
    no_store(deliver(site, id, method, false).await)
}

pub(super) async fn chart(
    State(site): State<Site>,
    Path(id): Path<String>,
    method: Method,
) -> Response {
    no_store(deliver(site, id, method, true).await)
}

fn no_store(mut response: Response) -> Response {
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, "no-store".parse().unwrap());
    response
}

async fn deliver(site: Site, id: String, method: Method, svg: bool) -> Response {
    let principal = Principal {
        actor_id: "public-site".into(),
        channel: "public-site".into(),
        scopes: BTreeSet::new(),
        provider_egress: BTreeSet::new(),
    };
    let artifact = PgStore::new(site.pool)
        .read_public_compare(&id, &principal)
        .await;
    let rendered = match artifact {
        Ok(Some(artifact)) => render_stored_compare(&artifact),
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                "Dieser Vergleich ist nicht verfügbar.",
            )
                .into_response()
        }
        Err(_) => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                "Dieser Vergleich ist gerade nicht verfügbar.",
            )
                .into_response()
        }
    };
    let Ok(rendered) = rendered else {
        return (
            StatusCode::NOT_FOUND,
            "Dieser Vergleich ist nicht verfügbar.",
        )
            .into_response();
    };
    let (content_type, bytes) = if svg {
        ("image/svg+xml; charset=utf-8", rendered.svg.into_bytes())
    } else {
        ("text/html; charset=utf-8", rendered.html.into_bytes())
    };
    let length = bytes.len().to_string();
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, content_type.to_owned()),
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
