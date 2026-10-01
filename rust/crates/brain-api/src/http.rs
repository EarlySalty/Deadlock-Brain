//! HTTP adapter only; binding sockets and runtime activation are explicit caller decisions.
use super::{deadline_response, json_error, ApiResponse, ApiService};
use axum::{
    body::to_bytes,
    extract::{Request, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use brain_contracts::RequestDeadline;
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
        .with_state(Arc::new(HttpState {
            service,
            // The same permit bounds body readers, queued workers and active workers.
            slots: Arc::new(Semaphore::new(64)),
        }))
}
struct CancelOnDrop(RequestDeadline);
impl Drop for CancelOnDrop {
    fn drop(&mut self) {
        self.0.cancel();
    }
}
async fn answer<K: AnswerKernelPort + 'static>(
    State(state): State<Arc<HttpState<K>>>,
    request: Request,
) -> Response {
    // Request is not a body extractor: no body is polled before admission/authentication.
    let deadline = RequestDeadline::after(Duration::from_millis(
        state.service.deadline_ms.clamp(1, 60000),
    ));
    let _cancel = CancelOnDrop(deadline.clone());
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
    let (parts, body) = request.into_parts();
    let headers = parts.headers;
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
    let authorization = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);
    if !state.service.authenticate_header(authorization.as_deref()) {
        return respond(json_error(
            401,
            "unauthorized",
            "Zugangsdaten sind ungültig",
        ));
    }
    if headers
        .get(header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
        .is_some_and(|length| length > 64 * 1024)
    {
        return respond(json_error(413, "payload_too_large", "Request ist zu groß"));
    }
    let expires_at = deadline.expires_at().into();
    let operation = async move {
        if deadline.check().is_err() {
            return deadline_response();
        }
        // Enforce the same byte limit for Content-Length and streamed/chunked requests.
        let body = match to_bytes(body, 64 * 1024).await {
            Ok(body) => body,
            Err(_) => return json_error(413, "payload_too_large", "Request ist zu groß"),
        };
        if deadline.check().is_err() {
            return deadline_response();
        }
        let worker = tokio::task::spawn_blocking(move || {
            // A client timeout/drop cannot release an executing or queued worker's permit.
            let _permit = permit;
            state
                .service
                .handle_answer_until(authorization.as_deref(), &body, deadline)
        });
        match worker.await {
            Ok(result) => result,
            Err(_) => json_error(503, "worker_unavailable", "Antwortdienst nicht verfügbar"),
        }
    };
    match tokio::time::timeout_at(expires_at, operation).await {
        Ok(result) => respond(result),
        Err(_) => respond(deadline_response()),
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
