# Task 05: Security Middleware, RBAC Extractors, Auth Routes & Worker Entrypoint

- **Executor / Coder Rule**:
  1. ALWAYS read [00_OVERVIEW.md](./00_OVERVIEW.md) first before starting execution.
  2. Execute ONLY this single task. When done and verified, terminate so a fresh Executor / Coder agent handles the next task.
  3. Apply exact code below without assumption or improvisation.

## 1. Action Items
- [ ] Create `backend-admin/src/middleware/mod.rs`
- [ ] Create `backend-admin/src/middleware/security.rs`
- [ ] Create `backend-admin/src/middleware/auth.rs`
- [ ] Create `backend-admin/src/routes/mod.rs`
- [ ] Create `backend-admin/src/routes/health.rs`
- [ ] Create `backend-admin/src/routes/jwks.rs`
- [ ] Create `backend-admin/src/routes/auth.rs`
- [ ] Replace `backend-admin/src/lib.rs` with full `build_router` and `#[event(fetch)]` Worker entrypoint.
- [ ] Verify host compilation, unit tests, and `wasm32-unknown-unknown` build.

## 2. Exact Code / File Changes

### File: `backend-admin/src/middleware/mod.rs`
```rust
pub mod auth;
pub mod security;
```

### File: `backend-admin/src/middleware/security.rs`
```rust
use crate::error::AppError;
use crate::ports::AppPorts;
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

pub fn client_ip_key(req: &Request, scope: &str) -> String {
    let ip = req
        .headers()
        .get("cf-connecting-ip")
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
        .filter(|s| !s.is_empty() && s.len() <= 64)
        .unwrap_or("unknown");
    format!("{scope}:{ip}")
}

pub async fn request_id_and_headers_middleware(
    State(ports): State<AppPorts>,
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
        let mut res = AppError::forbidden("Origin not allowed.").into_response_with_request_id(&req_id);
        apply_security_headers(&mut res, &req_id, true);
        return res;
    }

    if let Some(ref _orig) = origin {
        if !origin_allowed && is_state_changing(&method) {
            let mut res = AppError::forbidden("Cross-origin request rejected.")
                .into_response_with_request_id(&req_id);
            apply_security_headers(&mut res, &req_id, true);
            return res;
        }
    }

    // Enforce early Content-Length cap and body byte cap before handlers run.
    let max_bytes = ports.config.max_body_bytes;
    if let Some(cl_val) = req.headers().get(header::CONTENT_LENGTH) {
        if let Ok(cl_str) = cl_val.to_str() {
            if let Ok(len) = cl_str.trim().parse::<usize>() {
                if len > max_bytes {
                    let mut res = AppError::payload_too_large()
                        .into_response_with_request_id(&req_id);
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
    }

    let (parts, body) = req.into_parts();
    let buffered = match axum::body::to_bytes(body, max_bytes).await {
        Ok(b) => b,
        Err(_) => {
            let mut res =
                AppError::payload_too_large().into_response_with_request_id(&req_id);
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
        header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("true"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET, POST, OPTIONS"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("Authorization, Content-Type, X-CSRF-Protection"),
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
            Err(_) => Err(AppError::bad_request("Malformed JSON payload.")
                .into_response_with_request_id(&req_id)),
        }
    }
}
```

### File: `backend-admin/src/middleware/auth.rs`
```rust
use crate::auth::jwt::{verify_jwt_with_jwks, Role};
use crate::error::AppError;
use crate::middleware::security::RequestId;
use crate::ports::{AppPorts, UserRecord};
use axum::extract::FromRequestParts;
use axum::http::header;
use axum::http::request::Parts;
use axum::response::Response;

#[derive(Debug, Clone)]
pub struct AuthenticatedCaller {
    pub user: UserRecord,
    pub jti: String,
}

#[derive(Debug, Clone)]
pub struct RequireAdmin(pub AuthenticatedCaller);

#[derive(Debug, Clone)]
pub struct RequireOwner(pub AuthenticatedCaller);

async fn authenticate_from_parts(
    parts: &Parts,
    ports: &AppPorts,
    min_role: Role,
) -> Result<AuthenticatedCaller, Response> {
    let req_id = parts
        .extensions
        .get::<RequestId>()
        .map(|r| r.0.clone())
        .unwrap_or_else(|| "unassigned".to_string());

    let auth_header = parts
        .headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let Some(token) = auth_header.strip_prefix("Bearer ").map(str::trim) else {
        return Err(AppError::unauthorized("Authentication required.")
            .into_response_with_request_id(&req_id));
    };
    if token.is_empty() {
        return Err(AppError::unauthorized("Authentication required.")
            .into_response_with_request_id(&req_id));
    }

    let jwks = match ports.repo.list_public_jwks().await {
        Ok(k) => k,
        Err(err) => return Err(err.into_response_with_request_id(&req_id)),
    };

    let now = ports.clock.now_secs();
    let claims = match verify_jwt_with_jwks(
        token,
        &jwks,
        &ports.config.jwt_issuer,
        &ports.config.jwt_audience,
        now,
    ) {
        Ok(c) => c,
        Err(err) => return Err(err.into_response_with_request_id(&req_id)),
    };

    let user = match ports.repo.find_user_by_id(&claims.sub).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return Err(AppError::unauthorized("Invalid token.")
                .into_response_with_request_id(&req_id));
        }
        Err(err) => return Err(err.into_response_with_request_id(&req_id)),
    };

    if !user.is_active {
        let _ = ports.repo.revoke_user_sessions(&user.id, now).await;
        return Err(AppError::forbidden("Account is inactive.")
            .into_response_with_request_id(&req_id));
    }

    if user.role < min_role || claims.role < min_role {
        return Err(AppError::forbidden("Insufficient role privileges.")
            .into_response_with_request_id(&req_id));
    }

    Ok(AuthenticatedCaller {
        user,
        jti: claims.jti,
    })
}

impl FromRequestParts<AppPorts> for RequireAdmin {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppPorts,
    ) -> Result<Self, Self::Rejection> {
        authenticate_from_parts(parts, state, Role::Admin)
            .await
            .map(Self)
    }
}

impl FromRequestParts<AppPorts> for RequireOwner {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppPorts,
    ) -> Result<Self, Self::Rejection> {
        authenticate_from_parts(parts, state, Role::Owner)
            .await
            .map(Self)
    }
}
```

### File: `backend-admin/src/routes/mod.rs`
```rust
pub mod auth;
pub mod health;
pub mod jwks;
```

### File: `backend-admin/src/routes/health.rs`
```rust
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
```

### File: `backend-admin/src/routes/jwks.rs`
```rust
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
```

### File: `backend-admin/src/routes/auth.rs`
```rust
use crate::auth::envelope::{decrypt_private_key, encrypt_private_key};
use crate::auth::jwt::{generate_es256_keypair, sign_es256_jwt, AccessClaims, Role};
use crate::auth::password::verify_password;
use crate::config::AppConfig;
use crate::error::AppError;
use crate::middleware::auth::{RequireAdmin, RequireOwner};
use crate::middleware::security::{RequestId, ValidatedJson};
use crate::ports::{AppPorts, AuditEventInput, JwksKeyRecord, RefreshOutcome, SessionRecord};
use axum::extract::State;
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use rand_core::{OsRng, RngCore};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: &'static str,
    pub expires_in: u64,
    pub role: &'static str,
}

#[derive(Serialize)]
pub struct MeResponse {
    pub id: String,
    pub email: String,
    pub role: &'static str,
    pub is_active: bool,
}

#[derive(Serialize)]
pub struct RotateKeyResponse {
    pub kid: String,
    pub alg: String,
    pub status: &'static str,
}

#[derive(Serialize)]
pub struct LogoutResponse {
    pub status: &'static str,
}

fn sha256_hex(input: &str) -> String {
    let digest = Sha256::digest(input.as_bytes());
    let mut out = String::with_capacity(digest.len() * 2);
    for byte in digest {
        use std::fmt::Write;
        let _ = write!(&mut out, "{byte:02x}");
    }
    out
}

fn generate_refresh_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

fn format_refresh_cookie(config: &AppConfig, raw_token: &str) -> String {
    if config.environment.is_secure_cookie() {
        format!(
            "{}={}; HttpOnly; Path=/v1/auth; SameSite=None; Secure; Max-Age={}",
            config.cookie_name, raw_token, config.refresh_token_ttl_secs
        )
    } else {
        format!(
            "{}={}; HttpOnly; Path=/v1/auth; SameSite=Lax; Max-Age={}",
            config.cookie_name, raw_token, config.refresh_token_ttl_secs
        )
    }
}

fn format_clear_refresh_cookie(config: &AppConfig) -> String {
    if config.environment.is_secure_cookie() {
        format!(
            "{}=; HttpOnly; Path=/v1/auth; SameSite=None; Secure; Max-Age=0",
            config.cookie_name
        )
    } else {
        format!(
            "{}=; HttpOnly; Path=/v1/auth; SameSite=Lax; Max-Age=0",
            config.cookie_name
        )
    }
}

fn extract_cookie_value(headers: &HeaderMap, cookie_name: &str) -> Option<String> {
    let raw = headers.get(header::COOKIE)?.to_str().ok()?;
    for pair in raw.split(';') {
        let trimmed = pair.trim();
        if let Some((name, value)) = trimmed.split_once('=') {
            if name.trim() == cookie_name && !value.trim().is_empty() {
                return Some(value.trim().to_string());
            }
        }
    }
    None
}

fn verify_csrf_and_origin(headers: &HeaderMap, config: &AppConfig) -> Result<(), AppError> {
    let origin = headers
        .get(header::ORIGIN)
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
        .ok_or_else(|| AppError::forbidden("Missing Origin header."))?;

    if !config.is_origin_allowed(origin) {
        return Err(AppError::forbidden("Origin not allowed."));
    }

    let csrf = headers
        .get("x-csrf-protection")
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
        .unwrap_or("");
    if csrf != "1" {
        return Err(AppError::forbidden("Missing or invalid CSRF header."));
    }

    Ok(())
}

fn rate_limit_key(headers: &HeaderMap, scope: &str) -> String {
    let ip = headers
        .get("cf-connecting-ip")
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
        .filter(|s| !s.is_empty() && s.len() <= 64)
        .unwrap_or("unknown");
    format!("{scope}:{ip}")
}

async fn ensure_active_signing_key(
    ports: &AppPorts,
    actor_user_id: Option<&str>,
    now_secs: u64,
) -> Result<(String, zeroize::Zeroizing<Vec<u8>>), AppError> {
    let kek = ports.secrets.key_encryption_key()?;
    if let Some(record) = ports.repo.get_active_signing_key().await? {
        let pkcs8 = decrypt_private_key(
            &kek,
            &record.kid,
            &record.alg,
            &record.encrypted_private_key,
        )?;
        return Ok((record.kid, pkcs8));
    }

    let generated = generate_es256_keypair()?;
    let encrypted = encrypt_private_key(
        &kek,
        &generated.kid,
        &generated.alg,
        &generated.pkcs8_private_der,
    )?;
    let public_jwk_json = serde_json::to_string(&generated.public_jwk)
        .map_err(|_| AppError::internal("failed to serialize public JWK"))?;

    let key_record = JwksKeyRecord {
        kid: generated.kid.clone(),
        alg: generated.alg.clone(),
        public_jwk_json,
        encrypted_private_key: encrypted,
        status: "active".to_string(),
        created_at: now_secs,
    };
    let audit = AuditEventInput {
        id: Uuid::new_v4().to_string(),
        actor_user_id: actor_user_id.map(ToString::to_string),
        event_type: "jwks_initial_key_created".to_string(),
        target_id: Some(generated.kid.clone()),
        metadata_json: serde_json::json!({ "alg": generated.alg }).to_string(),
        created_at: now_secs,
    };
    ports.repo.rotate_signing_key(key_record, audit).await?;
    Ok((generated.kid, generated.pkcs8_private_der))
}

#[axum::debug_handler]
pub async fn post_login(
    State(ports): State<AppPorts>,
    Extension(req_id): Extension<RequestId>,
    headers: HeaderMap,
    ValidatedJson(payload): ValidatedJson<LoginRequest>,
) -> Response {
    match handle_login(&ports, &headers, payload).await {
        Ok(res) => res,
        Err(err) => err.into_response_with_request_id(&req_id.0),
    }
}

async fn handle_login(
    ports: &AppPorts,
    headers: &HeaderMap,
    payload: LoginRequest,
) -> Result<Response, AppError> {
    if !ports
        .rate_limiter
        .check_limit(&rate_limit_key(headers, "login"))
        .await?
    {
        return Err(AppError::rate_limited());
    }

    let email = payload.email.trim().to_ascii_lowercase();
    if email.is_empty() || payload.password.is_empty() {
        return Err(AppError::unauthorized("Invalid credentials."));
    }

    let Some(user) = ports.repo.find_user_by_email(&email).await? else {
        return Err(AppError::unauthorized("Invalid credentials."));
    };

    let pepper = ports.secrets.password_pepper()?;
    let valid = verify_password(&payload.password, &user.password_hash, &pepper).await?;
    if !valid {
        return Err(AppError::unauthorized("Invalid credentials."));
    }

    let now = ports.clock.now_secs();
    if !user.is_active {
        let _ = ports.repo.revoke_user_sessions(&user.id, now).await;
        return Err(AppError::forbidden("Account is inactive."));
    }
    if user.role < Role::Admin {
        return Err(AppError::forbidden("Insufficient role privileges."));
    }

    let (kid, pkcs8_der) = ensure_active_signing_key(ports, Some(&user.id), now).await?;
    let claims = AccessClaims {
        iss: ports.config.jwt_issuer.clone(),
        aud: ports.config.jwt_audience.clone(),
        sub: user.id.clone(),
        role: user.role,
        jti: Uuid::new_v4().to_string(),
        iat: now,
        exp: now + ports.config.access_token_ttl_secs,
        nbf: None,
    };
    let access_token = sign_es256_jwt(&kid, &pkcs8_der, &claims)?;

    let raw_refresh = generate_refresh_token();
    let session = SessionRecord {
        id: Uuid::new_v4().to_string(),
        user_id: user.id.clone(),
        token_hash: sha256_hex(&raw_refresh),
        family_id: Uuid::new_v4().to_string(),
        expires_at: now + ports.config.refresh_token_ttl_secs,
        revoked_at: None,
        replaced_by_session_id: None,
        created_at: now,
    };
    ports.repo.insert_session(session).await?;

    let _ = ports
        .repo
        .insert_audit_event(AuditEventInput {
            id: Uuid::new_v4().to_string(),
            actor_user_id: Some(user.id.clone()),
            event_type: "auth_login_succeeded".to_string(),
            target_id: Some(user.id.clone()),
            metadata_json: serde_json::json!({ "role": user.role.as_str() }).to_string(),
            created_at: now,
        })
        .await;

    let mut res = (
        StatusCode::OK,
        Json(TokenResponse {
            access_token,
            token_type: "Bearer",
            expires_in: ports.config.access_token_ttl_secs,
            role: user.role.as_str(),
        }),
    )
        .into_response();

    let cookie = format_refresh_cookie(&ports.config, &raw_refresh);
    if let Ok(val) = HeaderValue::from_str(&cookie) {
        res.headers_mut().append(header::SET_COOKIE, val);
    }
    Ok(res)
}

#[axum::debug_handler]
pub async fn post_refresh(
    State(ports): State<AppPorts>,
    Extension(req_id): Extension<RequestId>,
    headers: HeaderMap,
) -> Response {
    match handle_refresh(&ports, &headers).await {
        Ok(res) => res,
        Err(err) => err.into_response_with_request_id(&req_id.0),
    }
}

async fn handle_refresh(ports: &AppPorts, headers: &HeaderMap) -> Result<Response, AppError> {
    verify_csrf_and_origin(headers, &ports.config)?;

    if !ports
        .rate_limiter
        .check_limit(&rate_limit_key(headers, "refresh"))
        .await?
    {
        return Err(AppError::rate_limited());
    }

    let Some(raw_token) = extract_cookie_value(headers, &ports.config.cookie_name) else {
        return Err(AppError::unauthorized("Missing refresh session."));
    };

    let now = ports.clock.now_secs();
    let next_raw_refresh = generate_refresh_token();
    let new_session = SessionRecord {
        id: Uuid::new_v4().to_string(),
        user_id: String::new(),
        token_hash: sha256_hex(&next_raw_refresh),
        family_id: String::new(),
        expires_at: now + ports.config.refresh_token_ttl_secs,
        revoked_at: None,
        replaced_by_session_id: None,
        created_at: now,
    };

    let token_hash = sha256_hex(&raw_token);
    let outcome = ports
        .repo
        .rotate_refresh_session(&token_hash, new_session, now)
        .await?;

    match outcome {
        RefreshOutcome::Rotated { user, .. } => {
            if !user.is_active {
                let _ = ports.repo.revoke_user_sessions(&user.id, now).await;
                return Err(AppError::forbidden("Account is inactive."));
            }
            if user.role < Role::Admin {
                return Err(AppError::forbidden("Insufficient role privileges."));
            }

            let (kid, pkcs8_der) = ensure_active_signing_key(ports, Some(&user.id), now).await?;
            let claims = AccessClaims {
                iss: ports.config.jwt_issuer.clone(),
                aud: ports.config.jwt_audience.clone(),
                sub: user.id.clone(),
                role: user.role,
                jti: Uuid::new_v4().to_string(),
                iat: now,
                exp: now + ports.config.access_token_ttl_secs,
                nbf: None,
            };
            let access_token = sign_es256_jwt(&kid, &pkcs8_der, &claims)?;

            let mut res = (
                StatusCode::OK,
                Json(TokenResponse {
                    access_token,
                    token_type: "Bearer",
                    expires_in: ports.config.access_token_ttl_secs,
                    role: user.role.as_str(),
                }),
            )
                .into_response();

            let cookie = format_refresh_cookie(&ports.config, &next_raw_refresh);
            if let Ok(val) = HeaderValue::from_str(&cookie) {
                res.headers_mut().append(header::SET_COOKIE, val);
            }
            Ok(res)
        }
        RefreshOutcome::Replayed { user_id, family_id } => {
            let _ = ports
                .repo
                .insert_audit_event(AuditEventInput {
                    id: Uuid::new_v4().to_string(),
                    actor_user_id: Some(user_id),
                    event_type: "refresh_token_replay_detected".to_string(),
                    target_id: Some(family_id),
                    metadata_json: serde_json::json!({ "action": "family_revoked" }).to_string(),
                    created_at: now,
                })
                .await;
            Err(AppError::unauthorized_and_clear_cookie(
                "Refresh session invalidated.",
                format_clear_refresh_cookie(&ports.config),
            ))
        }
        RefreshOutcome::NotFoundOrExpired => Err(AppError::unauthorized_and_clear_cookie(
            "Invalid or expired refresh session.",
            format_clear_refresh_cookie(&ports.config),
        )),
    }
}

#[axum::debug_handler]
pub async fn post_logout(
    State(ports): State<AppPorts>,
    Extension(req_id): Extension<RequestId>,
    headers: HeaderMap,
) -> Response {
    match handle_logout(&ports, &headers).await {
        Ok(res) => res,
        Err(err) => err.into_response_with_request_id(&req_id.0),
    }
}

async fn handle_logout(ports: &AppPorts, headers: &HeaderMap) -> Result<Response, AppError> {
    verify_csrf_and_origin(headers, &ports.config)?;

    let now = ports.clock.now_secs();
    if let Some(raw_token) = extract_cookie_value(headers, &ports.config.cookie_name) {
        let token_hash = sha256_hex(&raw_token);
        if let Some(user_id) = ports
            .repo
            .revoke_session_by_token_hash(&token_hash, now)
            .await?
        {
            let _ = ports
                .repo
                .insert_audit_event(AuditEventInput {
                    id: Uuid::new_v4().to_string(),
                    actor_user_id: Some(user_id),
                    event_type: "auth_logout".to_string(),
                    target_id: None,
                    metadata_json: "{}".to_string(),
                    created_at: now,
                })
                .await;
        }
    }

    let mut res = (StatusCode::OK, Json(LogoutResponse { status: "logged_out" })).into_response();
    let clear_cookie = format_clear_refresh_cookie(&ports.config);
    if let Ok(val) = HeaderValue::from_str(&clear_cookie) {
        res.headers_mut().append(header::SET_COOKIE, val);
    }
    Ok(res)
}

#[axum::debug_handler]
pub async fn get_me(RequireAdmin(caller): RequireAdmin) -> Response {
    (
        StatusCode::OK,
        Json(MeResponse {
            id: caller.user.id,
            email: caller.user.email,
            role: caller.user.role.as_str(),
            is_active: caller.user.is_active,
        }),
    )
        .into_response()
}

#[axum::debug_handler]
pub async fn post_rotate_keys(
    State(ports): State<AppPorts>,
    Extension(req_id): Extension<RequestId>,
    RequireOwner(caller): RequireOwner,
) -> Response {
    match handle_rotate_keys(&ports, &caller.user.id).await {
        Ok(res) => res,
        Err(err) => err.into_response_with_request_id(&req_id.0),
    }
}

async fn handle_rotate_keys(ports: &AppPorts, actor_user_id: &str) -> Result<Response, AppError> {
    let now = ports.clock.now_secs();
    let kek = ports.secrets.key_encryption_key()?;
    let generated = generate_es256_keypair()?;
    let encrypted = encrypt_private_key(
        &kek,
        &generated.kid,
        &generated.alg,
        &generated.pkcs8_private_der,
    )?;
    let public_jwk_json = serde_json::to_string(&generated.public_jwk)
        .map_err(|_| AppError::internal("failed to serialize public JWK"))?;

    let key_record = JwksKeyRecord {
        kid: generated.kid.clone(),
        alg: generated.alg.clone(),
        public_jwk_json,
        encrypted_private_key: encrypted,
        status: "active".to_string(),
        created_at: now,
    };
    let audit = AuditEventInput {
        id: Uuid::new_v4().to_string(),
        actor_user_id: Some(actor_user_id.to_string()),
        event_type: "jwks_key_rotated".to_string(),
        target_id: Some(generated.kid.clone()),
        metadata_json: serde_json::json!({ "alg": generated.alg }).to_string(),
        created_at: now,
    };

    ports.repo.rotate_signing_key(key_record, audit).await?;

    Ok((
        StatusCode::OK,
        Json(RotateKeyResponse {
            kid: generated.kid,
            alg: generated.alg,
            status: "active",
        }),
    )
        .into_response())
}
```

### File: `backend-admin/src/lib.rs`
```rust
pub mod adapters;
pub mod auth;
pub mod config;
pub mod error;
pub mod middleware;
pub mod ports;
pub mod routes;

use crate::adapters::worker_env::WorkerEnvAdapter;
use crate::error::AppError;
use crate::middleware::security::request_id_and_headers_middleware;
use crate::ports::AppPorts;
use crate::routes::auth::{get_me, post_login, post_logout, post_refresh, post_rotate_keys};
use crate::routes::health::{fallback_not_found, get_health};
use crate::routes::jwks::get_jwks;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};
use axum::Router;
use tower_service::Service;
use worker::{event, Context, Env, HttpRequest};

pub fn build_router(ports: AppPorts) -> Router {
    Router::new()
        .route("/health", get(get_health))
        .route("/.well-known/jwks.json", get(get_jwks))
        .route("/v1/auth/login", post(post_login))
        .route("/v1/auth/refresh", post(post_refresh))
        .route("/v1/auth/logout", post(post_logout))
        .route("/v1/me", get(get_me))
        .route("/v1/admin/auth/keys/rotate", post(post_rotate_keys))
        .fallback(fallback_not_found)
        .layer(from_fn_with_state(
            ports.clone(),
            request_id_and_headers_middleware,
        ))
        .with_state(ports)
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> worker::Result<http::Response<axum::body::Body>> {
    let adapter = WorkerEnvAdapter::new(env);
    let ports = match adapter.into_ports() {
        Ok(p) => p,
        Err(err) => return Ok(err.into_response_with_request_id("startup")),
    };

    let mut router = build_router(ports);
    let axum_req = req.map(axum::body::Body::new);
    match router.call(axum_req).await {
        Ok(resp) => Ok(resp),
        Err(_) => Ok(AppError::internal("router dispatch failure").into_response_with_request_id("dispatch")),
    }
}
```

## 3. Verification Command
```bash
cd backend-admin && cargo test --lib && cargo check --target wasm32-unknown-unknown
```
