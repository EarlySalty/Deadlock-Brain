//! HTTP adapter only; binding sockets and runtime activation are explicit caller decisions.
use super::{json_error, ApiResponse, ApiService};
use axum::{
    body::Bytes,
    extract::{rejection::BytesRejection, DefaultBodyLimit, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use brain_kernel::AnswerKernelPort;
use std::{sync::Arc, time::Duration};
use tokio::sync::Semaphore;
struct HttpState<K> {
    service: ApiService<K>,
    slots: Arc<Semaphore>,
}
pub fn router<K: AnswerKernelPort + 'static>(service: ApiService<K>) -> Router {
    Router::new()
        .route("/v1/answer", post(answer::<K>))
        .fallback(|| async { respond(json_error(404, "not_found", "Route nicht verfügbar")) })
        .layer(DefaultBodyLimit::max(64 * 1024))
        .with_state(Arc::new(HttpState {
            service,
            // Keep HTTP work bounded, but let the shared DB pool provide the tighter
            // backpressure for the documented 8/16/32-worker load envelope.
            slots: Arc::new(Semaphore::new(64)),
        }))
}
async fn answer<K: AnswerKernelPort + 'static>(
    State(state): State<Arc<HttpState<K>>>,
    headers: HeaderMap,
    body: Result<Bytes, BytesRejection>,
) -> Response {
    if headers.get_all(header::AUTHORIZATION).iter().count() != 1 {
        return respond(json_error(
            401,
            "unauthorized",
            "Genau ein Bearer-Header erforderlich",
        ));
    }
    if !headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|v| {
            v.split(';')
                .next()
                .is_some_and(|m| m.trim().eq_ignore_ascii_case("application/json"))
        })
    {
        return respond(json_error(
            415,
            "unsupported_media_type",
            "application/json erforderlich",
        ));
    }
    let body = match body {
        Ok(body) => body,
        Err(_) => return respond(json_error(413, "payload_too_large", "Request ist zu groß")),
    };
    let authorization = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);
    let permit = match state.slots.clone().try_acquire_owned() {
        Ok(permit) => permit,
        Err(_) => {
            return respond(json_error(
                429,
                "overloaded",
                "Anfragekapazität ausgeschöpft",
            ))
        }
    };
    let timeout = Duration::from_millis(state.service.deadline_ms.clamp(1, 60000));
    // The permit remains held until the blocking worker exits, even if the client timed out.
    let worker = tokio::task::spawn_blocking(move || {
        let _permit = permit;
        state.service.handle_answer(authorization.as_deref(), &body)
    });
    match tokio::time::timeout(timeout, worker).await {
        Ok(Ok(result)) => respond(result),
        Ok(Err(_)) => respond(json_error(
            503,
            "worker_unavailable",
            "Antwortdienst nicht verfügbar",
        )),
        Err(_) => respond(json_error(
            504,
            "deadline_exceeded",
            "Request Deadline erreicht",
        )),
    }
}
fn respond(result: ApiResponse) -> Response {
    (
        StatusCode::from_u16(result.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        [
            (header::CONTENT_TYPE, result.content_type),
            (header::CACHE_CONTROL, "no-store"),
        ],
        result.body,
    )
        .into_response()
}
