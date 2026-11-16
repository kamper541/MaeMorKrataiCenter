# Task 03: Security Middleware, `TurnstileGuard` & `JwksVerifier`

- **Executor / Coder Rule**:
  1. ALWAYS read [00_OVERVIEW.md](./00_OVERVIEW.md) first before starting execution.
  2. Execute ONLY this single task. When done and verified, terminate so a fresh Executor / Coder agent handles the next task.
  3. Apply exact code below without assumption or improvisation.

## 1. Action Items
- [ ] Create `backend-client/src/middleware/mod.rs`
- [ ] Create `backend-client/src/middleware/security.rs`
- [ ] Create `backend-client/src/security/mod.rs`
- [ ] Create `backend-client/src/security/turnstile.rs`
- [ ] Create `backend-client/src/security/jwks.rs`
- [ ] Update temporary `backend-client/src/lib.rs` to export `middleware` and `security` (replaced in Task 04).
- [ ] Run host unit tests and `wasm32-unknown-unknown` check.

## 2. Exact Code / File Changes

### File: `backend-client/src/middleware/mod.rs`
```rust
pub mod security;
```

### File: `backend-client/src/middleware/security.rs`
```rust
use crate::error::ClientError;
use crate::ports::ClientPorts;
use axum::body::Body;
use axum::extract::{FromRequest, Request, State};
use axum::http::{header, HeaderValue, Method, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use axum::Json;
use serde::de::DeserializeOwned;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RequestId(pub String);

pub fn extract_request_id(req: &Request) -> String {
    req.extensions()
        .get::<RequestId>()
        .map(|r| r.0.clone())
        .unwrap_or_else(|| "unassigned".to_string())
}

pub fn client_ip_from_headers(headers: &axum::http::HeaderMap) -> String {
    headers
        .get("cf-connecting-ip")
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
        .filter(|s| !s.is_empty() && s.len() <= 64)
        .unwrap_or("unknown")
        .to_string()
}

pub async fn security_pipeline_middleware(
    State(ports): State<ClientPorts>,
    mut req: Request,
    next: Next,
) -> Response {
    let req_id = req
        .headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
        .filter(|s| !s.is_empty() && s.len() <= 64 && s.bytes().all(|b| b.is_ascii_graphic()))
        .map(ToString::to_string)
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    req.extensions_mut().insert(RequestId(req_id.clone()));

    let origin = req
        .headers()
        .get(header::ORIGIN)
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
        .map(ToString::to_string);
    let method = req.method().clone();
    let origin_allowed = origin
        .as_deref()
        .map(|o| ports.config.is_origin_allowed(o))
        .unwrap_or(false);

    if method == Method::OPTIONS {
        if let Some(ref orig) = origin {
            if origin_allowed {
                let mut res = Response::new(Body::empty());
                *res.status_mut() = StatusCode::NO_CONTENT;
                apply_cors_headers(&mut res, orig);
                apply_security_headers(&mut res, &req_id, false);
                return res;
            }
        }
        let mut res = ClientError::forbidden("Origin not allowed.")
            .into_response_with_request_id(&req_id);
        apply_security_headers(&mut res, &req_id, true);
        return res;
    }

    if origin.is_some() && !origin_allowed && is_state_changing(&method) {
        let mut res = ClientError::forbidden("Cross-origin request rejected.")
            .into_response_with_request_id(&req_id);
        apply_security_headers(&mut res, &req_id, true);
        return res;
    }

    let rate_key = format!("global:{}:{}", req.uri().path(), client_ip_from_headers(req.headers()));
    match ports.rate_limiter.check_limit(&rate_key).await {
        Ok(true) => {}
        Ok(false) => {
            let mut res = ClientError::rate_limited().into_response_with_request_id(&req_id);
            if origin_allowed {
                if let Some(ref orig) = origin {
                    apply_cors_headers(&mut res, orig);
                }
            }
            apply_security_headers(&mut res, &req_id, true);
            return res;
        }
        Err(err) => {
            let mut res = err.into_response_with_request_id(&req_id);
            apply_security_headers(&mut res, &req_id, true);
            return res;
        }
    }

    let max_bytes = ports.config.max_body_bytes;
    if let Some(cl_val) = req.headers().get(header::CONTENT_LENGTH) {
        if let Ok(cl_str) = cl_val.to_str() {
            let Ok(len) = cl_str.trim().parse::<usize>() else {
                let mut res = ClientError::bad_request("Invalid Content-Length header.")
                    .into_response_with_request_id(&req_id);
                apply_security_headers(&mut res, &req_id, true);
                return res;
            };
            if len > max_bytes {
                let mut res =
                    ClientError::payload_too_large().into_response_with_request_id(&req_id);
                if origin_allowed {
                    if let Some(ref orig) = origin {
                        apply_cors_headers(&mut res, orig);
                    }
                }
                apply_security_headers(&mut res, &req_id, true);
                return res;
            }
        }
    }

    let (parts, body) = req.into_parts();
    let buffered = match axum::body::to_bytes(body, max_bytes).await {
        Ok(b) => b,
        Err(_) => {
            let mut res = ClientError::payload_too_large().into_response_with_request_id(&req_id);
            if origin_allowed {
                if let Some(ref orig) = origin {
                    apply_cors_headers(&mut res, orig);
                }
            }
            apply_security_headers(&mut res, &req_id, true);
            return res;
        }
    };
    let rebuilt = Request::from_parts(parts, Body::from(buffered));

    let mut res = next.run(rebuilt).await;
    if origin_allowed {
        if let Some(ref orig) = origin {
            apply_cors_headers(&mut res, orig);
        }
    }
    let preserve_cache = res.headers().contains_key(header::CACHE_CONTROL);
    apply_security_headers(&mut res, &req_id, !preserve_cache);
    res
}

fn is_state_changing(method: &Method) -> bool {
    matches!(
        *method,
        Method::POST | Method::PUT | Method::PATCH | Method::DELETE
    )
}

fn apply_cors_headers(res: &mut Response, allowed_origin: &str) {
    let headers = res.headers_mut();
    if let Ok(val) = HeaderValue::from_str(allowed_origin) {
        headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, val);
    }
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET, POST, OPTIONS"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("Authorization, Content-Type, CF-Turnstile-Response"),
    );
    headers.insert(header::VARY, HeaderValue::from_static("Origin"));
}

fn apply_security_headers(res: &mut Response, request_id: &str, force_no_store: bool) {
    let headers = res.headers_mut();
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    if force_no_store {
        headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    }
    if let Ok(val) = HeaderValue::from_str(request_id) {
        headers.insert("x-request-id", val);
    }
}

pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let req_id = extract_request_id(&req);
        match Json::<T>::from_request(req, state).await {
            Ok(Json(val)) => Ok(Self(val)),
            Err(_) => Err(ClientError::bad_request("Malformed JSON payload.")
                .into_response_with_request_id(&req_id)),
        }
    }
}
```

### File: `backend-client/src/security/mod.rs`
```rust
pub mod jwks;
pub mod turnstile;
```

### File: `backend-client/src/security/turnstile.rs`
```rust
use crate::error::ClientError;
use crate::middleware::security::{client_ip_from_headers, RequestId};
use crate::ports::ClientPorts;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::response::Response;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize)]
pub struct TurnstileSiteVerifyRequest {
    pub secret: String,
    pub response: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remoteip: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TurnstileSiteVerifyResponse {
    pub success: bool,
    #[serde(default, rename = "error-codes")]
    pub error_codes: Vec<String>,
}

#[async_trait]
pub trait TurnstileTransport: Send + Sync {
    async fn siteverify(
        &self,
        verify_url: &str,
        payload: TurnstileSiteVerifyRequest,
    ) -> Result<TurnstileSiteVerifyResponse, ClientError>;
}

pub struct WorkerTurnstileTransport;

#[cfg(target_arch = "wasm32")]
#[async_trait]
impl TurnstileTransport for WorkerTurnstileTransport {
    async fn siteverify(
        &self,
        verify_url: &str,
        payload: TurnstileSiteVerifyRequest,
    ) -> Result<TurnstileSiteVerifyResponse, ClientError> {
        use wasm_bindgen::JsValue;
        use worker::send::SendFuture;

        let body_json = serde_json::to_string(&payload)
            .map_err(|_| ClientError::internal("failed to serialize Turnstile payload"))?;
        let headers = worker::Headers::new();
        headers
            .set("content-type", "application/json")
            .map_err(|_| ClientError::internal("failed to set Turnstile header"))?;

        let mut init = worker::RequestInit::new();
        init.with_method(worker::Method::Post)
            .with_headers(headers)
            .with_body(Some(JsValue::from_str(&body_json)));

        let req = worker::Request::new_with_init(verify_url, &init)
            .map_err(|_| ClientError::service_unavailable("Turnstile request build failed"))?;
        let mut resp = SendFuture::new(worker::Fetch::Request(req).send())
            .await
            .map_err(|_| ClientError::service_unavailable("Turnstile verification unreachable"))?;

        if resp.status_code() != 200 {
            return Err(ClientError::service_unavailable(
                "Turnstile verification returned non-200 status",
            ));
        }

        let text = SendFuture::new(resp.text())
            .await
            .map_err(|_| ClientError::service_unavailable("Turnstile response read failed"))?;
        serde_json::from_str::<TurnstileSiteVerifyResponse>(&text)
            .map_err(|_| ClientError::service_unavailable("Turnstile response decode failed"))
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[async_trait]
impl TurnstileTransport for WorkerTurnstileTransport {
    async fn siteverify(
        &self,
        _verify_url: &str,
        _payload: TurnstileSiteVerifyRequest,
    ) -> Result<TurnstileSiteVerifyResponse, ClientError> {
        Err(ClientError::service_unavailable(
            "WorkerTurnstileTransport requires wasm32 runtime",
        ))
    }
}

#[derive(Clone)]
pub struct TurnstileGuard {
    transport: Arc<dyn TurnstileTransport>,
}

impl TurnstileGuard {
    pub fn new(transport: Arc<dyn TurnstileTransport>) -> Self {
        Self { transport }
    }

    pub async fn verify_token(
        &self,
        ports: &ClientPorts,
        token: Option<&str>,
        remoteip: Option<String>,
    ) -> Result<(), ClientError> {
        let Some(clean_token) = token.map(str::trim).filter(|t| !t.is_empty()) else {
            return Err(ClientError::forbidden("Turnstile verification token required."));
        };

        let secret = ports.secrets.turnstile_secret()?;
        let response = self
            .transport
            .siteverify(
                &ports.config.turnstile_verify_url,
                TurnstileSiteVerifyRequest {
                    secret,
                    response: clean_token.to_string(),
                    remoteip,
                },
            )
            .await?;

        if !response.success {
            return Err(ClientError::forbidden("Turnstile verification failed."));
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct VerifiedTurnstile;

#[derive(Clone)]
pub struct TurnstileRouterState {
    pub ports: ClientPorts,
    pub guard: TurnstileGuard,
}

impl FromRequestParts<TurnstileRouterState> for VerifiedTurnstile {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &TurnstileRouterState,
    ) -> Result<Self, Self::Rejection> {
        let req_id = parts
            .extensions
            .get::<RequestId>()
            .map(|r| r.0.clone())
            .unwrap_or_else(|| "unassigned".to_string());

        let token = parts
            .headers
            .get("cf-turnstile-response")
            .and_then(|v| v.to_str().ok());
        let ip = client_ip_from_headers(&parts.headers);
        let remoteip = (ip != "unknown").then_some(ip);

        state
            .guard
            .verify_token(&state.ports, token, remoteip)
            .await
            .map(|()| Self)
            .map_err(|err| err.into_response_with_request_id(&req_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::memory::MemoryClientStore;
    use crate::config::Environment;
    use std::future::Future;
    use std::pin::pin;
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    fn block_on<F: Future>(fut: F) -> F::Output {
        fn noop(_: *const ()) {}
        fn clone(ptr: *const ()) -> RawWaker {
            RawWaker::new(ptr, &VTABLE)
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
        let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
        let mut cx = Context::from_waker(&waker);
        let mut pinned = pin!(fut);
        match pinned.as_mut().poll(&mut cx) {
            Poll::Ready(val) => val,
            Poll::Pending => panic!("fixture transport must resolve synchronously"),
        }
    }

    enum FixtureOutcome {
        Pass,
        Reject,
        Outage,
    }

    struct FixtureTransport(FixtureOutcome);

    #[async_trait]
    impl TurnstileTransport for FixtureTransport {
        async fn siteverify(
            &self,
            _verify_url: &str,
            _payload: TurnstileSiteVerifyRequest,
        ) -> Result<TurnstileSiteVerifyResponse, ClientError> {
            match self.0 {
                FixtureOutcome::Pass => Ok(TurnstileSiteVerifyResponse {
                    success: true,
                    error_codes: vec![],
                }),
                FixtureOutcome::Reject => Ok(TurnstileSiteVerifyResponse {
                    success: false,
                    error_codes: vec!["invalid-input-response".to_string()],
                }),
                FixtureOutcome::Outage => Err(ClientError::service_unavailable("upstream timeout")),
            }
        }
    }

    #[test]
    fn turnstile_guard_fails_closed_on_missing_invalid_and_outage() {
        block_on(async {
            let ports = MemoryClientStore::new().into_ports(Environment::Staging);

            let pass_guard = TurnstileGuard::new(Arc::new(FixtureTransport(FixtureOutcome::Pass)));
            assert!(pass_guard.verify_token(&ports, Some("valid-tok"), None).await.is_ok());

            // Missing token -> 403
            let missing_err = pass_guard.verify_token(&ports, None, None).await.unwrap_err();
            assert_eq!(missing_err.status, axum::http::StatusCode::FORBIDDEN);

            // Invalid token -> 403
            let reject_guard =
                TurnstileGuard::new(Arc::new(FixtureTransport(FixtureOutcome::Reject)));
            let reject_err = reject_guard
                .verify_token(&ports, Some("bad-tok"), None)
                .await
                .unwrap_err();
            assert_eq!(reject_err.status, axum::http::StatusCode::FORBIDDEN);

            // Upstream outage -> 503 (never bypassed)
            let outage_guard =
                TurnstileGuard::new(Arc::new(FixtureTransport(FixtureOutcome::Outage)));
            let outage_err = outage_guard
                .verify_token(&ports, Some("valid-tok"), None)
                .await
                .unwrap_err();
            assert_eq!(outage_err.status, axum::http::StatusCode::SERVICE_UNAVAILABLE);
        });
    }
}
```

### File: `backend-client/src/security/jwks.rs`
```rust
use crate::error::ClientError;
use crate::middleware::security::RequestId;
use crate::ports::ClientPorts;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::header;
use axum::http::request::Parts;
use axum::response::Response;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use p256::ecdsa::signature::Verifier;
use p256::ecdsa::{Signature as P256Signature, VerifyingKey as P256VerifyingKey};
use p256::elliptic_curve::sec1::FromEncodedPoint;
use p256::{EncodedPoint, PublicKey as P256PublicKey};
use rsa::pkcs1v15::{Signature as RsaSignature, VerifyingKey as RsaVerifyingKey};
use rsa::{BigUint, RsaPublicKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize)]
struct JwtHeader {
    alg: String,
    kid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PublicJwk {
    pub kty: String,
    #[serde(rename = "use", skip_serializing_if = "Option::is_none")]
    pub key_use: Option<String>,
    pub alg: String,
    pub kid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwksDocument {
    pub keys: Vec<PublicJwk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedJwtClaims {
    pub iss: String,
    pub aud: String,
    pub sub: String,
    #[serde(default)]
    pub role: Option<String>,
    pub jti: String,
    pub iat: u64,
    pub exp: u64,
    #[serde(default)]
    pub nbf: Option<u64>,
}

#[async_trait]
pub trait JwksSource: Send + Sync {
    async fn fetch_jwks(&self, jwks_url: &str) -> Result<Vec<PublicJwk>, ClientError>;
}

pub struct WorkerJwksSource;

#[cfg(target_arch = "wasm32")]
#[async_trait]
impl JwksSource for WorkerJwksSource {
    async fn fetch_jwks(&self, jwks_url: &str) -> Result<Vec<PublicJwk>, ClientError> {
        use worker::send::SendFuture;

        let url = worker::Url::parse(jwks_url)
            .map_err(|_| ClientError::service_unavailable("invalid JWKS URL"))?;
        let mut resp = SendFuture::new(worker::Fetch::Url(url).send())
            .await
            .map_err(|_| ClientError::service_unavailable("JWKS endpoint unreachable"))?;
        if resp.status_code() != 200 {
            return Err(ClientError::service_unavailable(
                "JWKS endpoint returned non-200 status",
            ));
        }
        let text = SendFuture::new(resp.text())
            .await
            .map_err(|_| ClientError::service_unavailable("JWKS body read failed"))?;
        let doc: JwksDocument = serde_json::from_str(&text)
            .map_err(|_| ClientError::service_unavailable("JWKS decode failed"))?;
        Ok(doc.keys)
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[async_trait]
impl JwksSource for WorkerJwksSource {
    async fn fetch_jwks(&self, _jwks_url: &str) -> Result<Vec<PublicJwk>, ClientError> {
        Err(ClientError::service_unavailable(
            "WorkerJwksSource requires wasm32 runtime",
        ))
    }
}

#[derive(Clone)]
pub struct JwksVerifier {
    source: Arc<dyn JwksSource>,
}

impl JwksVerifier {
    pub fn new(source: Arc<dyn JwksSource>) -> Self {
        Self { source }
    }

    pub async fn verify_bearer_token(
        &self,
        ports: &ClientPorts,
        token: &str,
    ) -> Result<VerifiedJwtClaims, ClientError> {
        let keys = self
            .source
            .fetch_jwks(&ports.config.jwks_url)
            .await
            .map_err(|_| ClientError::unauthorized("Unable to verify token."))?;

        verify_jwt_against_keys(
            token,
            &keys,
            &ports.config.jwt_issuer,
            &ports.config.jwt_audience,
            ports.clock.now_secs(),
        )
    }
}

pub fn verify_jwt_against_keys(
    token: &str,
    jwks: &[PublicJwk],
    expected_iss: &str,
    expected_aud: &str,
    now_secs: u64,
) -> Result<VerifiedJwtClaims, ClientError> {
    let mut parts = token.split('.');
    let (Some(header_b64), Some(claims_b64), Some(sig_b64), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return Err(ClientError::unauthorized("Invalid token."));
    };

    let header_bytes = URL_SAFE_NO_PAD
        .decode(header_b64)
        .map_err(|_| ClientError::unauthorized("Invalid token."))?;
    let header: JwtHeader = serde_json::from_slice(&header_bytes)
        .map_err(|_| ClientError::unauthorized("Invalid token."))?;

    if header.alg != "ES256" && header.alg != "RS256" {
        return Err(ClientError::unauthorized("Invalid token."));
    }

    let jwk = jwks
        .iter()
        .find(|k| k.kid == header.kid)
        .ok_or_else(|| ClientError::unauthorized("Invalid token."))?;
    if jwk.alg != header.alg {
        return Err(ClientError::unauthorized("Invalid token."));
    }

    let sig_bytes = URL_SAFE_NO_PAD
        .decode(sig_b64)
        .map_err(|_| ClientError::unauthorized("Invalid token."))?;
    let signing_input = format!("{header_b64}.{claims_b64}");

    match header.alg.as_str() {
        "ES256" => {
            if jwk.kty != "EC" || jwk.crv.as_deref() != Some("P-256") {
                return Err(ClientError::unauthorized("Invalid token."));
            }
            let x = URL_SAFE_NO_PAD
                .decode(jwk.x.as_deref().ok_or_else(|| ClientError::unauthorized("Invalid token."))?)
                .map_err(|_| ClientError::unauthorized("Invalid token."))?;
            let y = URL_SAFE_NO_PAD
                .decode(jwk.y.as_deref().ok_or_else(|| ClientError::unauthorized("Invalid token."))?)
                .map_err(|_| ClientError::unauthorized("Invalid token."))?;
            if x.len() != 32 || y.len() != 32 {
                return Err(ClientError::unauthorized("Invalid token."));
            }
            let mut sec1 = Vec::with_capacity(65);
            sec1.push(0x04);
            sec1.extend_from_slice(&x);
            sec1.extend_from_slice(&y);
            let point = EncodedPoint::from_bytes(&sec1)
                .map_err(|_| ClientError::unauthorized("Invalid token."))?;
            let pub_key = Option::<P256PublicKey>::from(P256PublicKey::from_encoded_point(&point))
                .ok_or_else(|| ClientError::unauthorized("Invalid token."))?;
            let vk = P256VerifyingKey::from(&pub_key);
            let sig = P256Signature::from_slice(&sig_bytes)
                .map_err(|_| ClientError::unauthorized("Invalid token."))?;
            vk.verify(signing_input.as_bytes(), &sig)
                .map_err(|_| ClientError::unauthorized("Invalid token."))?;
        }
        "RS256" => {
            if jwk.kty != "RSA" {
                return Err(ClientError::unauthorized("Invalid token."));
            }
            let n_bytes = URL_SAFE_NO_PAD
                .decode(jwk.n.as_deref().ok_or_else(|| ClientError::unauthorized("Invalid token."))?)
                .map_err(|_| ClientError::unauthorized("Invalid token."))?;
            let e_bytes = URL_SAFE_NO_PAD
                .decode(jwk.e.as_deref().ok_or_else(|| ClientError::unauthorized("Invalid token."))?)
                .map_err(|_| ClientError::unauthorized("Invalid token."))?;
            let rsa_pub = RsaPublicKey::new(
                BigUint::from_bytes_be(&n_bytes),
                BigUint::from_bytes_be(&e_bytes),
            )
            .map_err(|_| ClientError::unauthorized("Invalid token."))?;
            let vk = RsaVerifyingKey::<Sha256>::new(rsa_pub);
            let sig = RsaSignature::try_from(sig_bytes.as_slice())
                .map_err(|_| ClientError::unauthorized("Invalid token."))?;
            vk.verify(signing_input.as_bytes(), &sig)
                .map_err(|_| ClientError::unauthorized("Invalid token."))?;
        }
        _ => return Err(ClientError::unauthorized("Invalid token.")),
    }

    let claims_bytes = URL_SAFE_NO_PAD
        .decode(claims_b64)
        .map_err(|_| ClientError::unauthorized("Invalid token."))?;
    let claims: VerifiedJwtClaims = serde_json::from_slice(&claims_bytes)
        .map_err(|_| ClientError::unauthorized("Invalid token."))?;

    if claims.iss != expected_iss || claims.aud != expected_aud {
        return Err(ClientError::unauthorized("Invalid token."));
    }
    if claims.exp <= now_secs {
        return Err(ClientError::unauthorized("Invalid token."));
    }
    if let Some(nbf) = claims.nbf {
        if nbf > now_secs {
            return Err(ClientError::unauthorized("Invalid token."));
        }
    }
    Ok(claims)
}

#[derive(Clone)]
pub struct JwksRouterState {
    pub ports: ClientPorts,
    pub verifier: JwksVerifier,
}

impl FromRequestParts<JwksRouterState> for VerifiedJwtClaims {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &JwksRouterState,
    ) -> Result<Self, Self::Rejection> {
        let req_id = parts
            .extensions
            .get::<RequestId>()
            .map(|r| r.0.clone())
            .unwrap_or_else(|| "unassigned".to_string());

        let token = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .map(str::trim)
            .filter(|t| !t.is_empty())
            .ok_or_else(|| {
                ClientError::unauthorized("Authentication required.")
                    .into_response_with_request_id(&req_id)
            })?;

        state
            .verifier
            .verify_bearer_token(&state.ports, token)
            .await
            .map_err(|err| err.into_response_with_request_id(&req_id))
    }
}
```

### File: `backend-client/src/lib.rs` (Temporary Task 03 compile stub; replaced in Task 04)
```rust
pub mod adapters;
pub mod config;
pub mod error;
pub mod middleware;
pub mod ports;
pub mod security;
```

## 3. Verification Command
```bash
cd backend-client && cargo test --lib && cargo check --target wasm32-unknown-unknown
```
