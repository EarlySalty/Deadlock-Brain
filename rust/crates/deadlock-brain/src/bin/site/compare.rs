use axum::{
    extract::{Path, State},
    http::{header, HeaderMap, HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
};
use brain_contracts::Principal;
use brain_maintenance::compare_artifact::render_stored_compare;
use brain_storage::{compare_artifact::CompareStage, PgStore};
use std::collections::BTreeSet;

use super::Site;

const FORWARDING_HEADERS: [&str; 5] = [
    "forwarded",
    "x-forwarded-for",
    "x-forwarded-host",
    "x-forwarded-proto",
    "via",
];

pub(super) async fn page(
    State(site): State<Site>,
    Path(id): Path<String>,
    method: Method,
    headers: HeaderMap,
) -> Response {
    no_store(deliver(site, id, method, local_request(&headers), false).await)
}

pub(super) async fn chart(
    State(site): State<Site>,
    Path(id): Path<String>,
    method: Method,
    headers: HeaderMap,
) -> Response {
    no_store(deliver(site, id, method, local_request(&headers), true).await)
}

fn local_request(headers: &HeaderMap) -> bool {
    FORWARDING_HEADERS
        .iter()
        .all(|name| !headers.contains_key(*name))
}

fn no_store(mut response: Response) -> Response {
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

fn missing() -> Response {
    (
        StatusCode::NOT_FOUND,
        "Dieser Vergleich ist nicht verfügbar.",
    )
        .into_response()
}

async fn deliver(site: Site, id: String, method: Method, local: bool, svg: bool) -> Response {
    let principal = Principal {
        actor_id: "public-site".into(),
        channel: "public-site".into(),
        scopes: BTreeSet::new(),
        provider_egress: BTreeSet::new(),
    };
    let store = PgStore::new(site.pool);
    let artifact = if local {
        store.read_site_compare(&id, &principal).await
    } else {
        store
            .read_public_compare(&id, &principal)
            .await
            .map(|artifact| artifact.map(|artifact| (artifact, CompareStage::Published)))
    };
    let (artifact, stage) = match artifact {
        Ok(Some(found)) => found,
        Ok(None) => return missing(),
        Err(_) => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                "Dieser Vergleich ist gerade nicht verfügbar.",
            )
                .into_response()
        }
    };
    let Ok(rendered) = render_stored_compare(&artifact) else {
        return missing();
    };
    let (content_type, bytes) = if svg {
        ("image/svg+xml; charset=utf-8", rendered.svg.into_bytes())
    } else {
        ("text/html; charset=utf-8", rendered.html.into_bytes())
    };
    let length = bytes.len().to_string();
    let mut response = (
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
        .into_response();
    if stage == CompareStage::Preview {
        response.headers_mut().insert(
            header::HeaderName::from_static("x-robots-tag"),
            HeaderValue::from_static("noindex, nofollow"),
        );
    }
    response
}
