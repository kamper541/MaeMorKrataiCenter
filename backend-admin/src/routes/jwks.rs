use crate::auth::jwt::JwksDocument;
use crate::middleware::security::RequestId;
use crate::ports::AppPorts;
use axum::extract::State;
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};

#[axum::debug_handler]
pub async fn get_jwks(
    State(ports): State<AppPorts>,
    Extension(req_id): Extension<RequestId>,
) -> Response {
    match ports.repo.list_public_jwks().await {
        Ok(keys) => {
            let mut res = (StatusCode::OK, Json(JwksDocument { keys })).into_response();
            res.headers_mut().insert(
                header::CACHE_CONTROL,
                HeaderValue::from_static("public, max-age=300"),
            );
            res
        }
        Err(err) => err.into_response_with_request_id(&req_id.0),
    }
}
