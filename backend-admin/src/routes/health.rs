use crate::error::AppError;
use crate::middleware::security::RequestId;
use crate::ports::AppPorts;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    d1: &'static str,
    r2: &'static str,
}

#[axum::debug_handler]
pub async fn get_health(
    State(ports): State<AppPorts>,
    Extension(req_id): Extension<RequestId>,
) -> Response {
    if let Err(err) = ports.repo.health_check_db().await {
        return err.into_response_with_request_id(&req_id.0);
    }
    if let Err(err) = ports.storage.check_r2_bindings().await {
        return err.into_response_with_request_id(&req_id.0);
    }

    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "ok",
            d1: "ok",
            r2: "ok",
        }),
    )
        .into_response()
}

pub async fn fallback_not_found(Extension(req_id): Extension<RequestId>) -> Response {
    AppError {
        status: StatusCode::NOT_FOUND,
        code: "NOT_FOUND",
        public_message: "Resource not found.".to_string(),
        internal_detail: None,
        clear_refresh_cookie: None,
    }
    .into_response_with_request_id(&req_id.0)
}
