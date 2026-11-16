# Task 01: Rust Workspace, Wrangler Config, Config & Error Envelope

- **Executor / Coder Rule**:
  1. ALWAYS read [00_OVERVIEW.md](./00_OVERVIEW.md) first before starting execution.
  2. Execute ONLY this single task. When done and verified, terminate so a fresh Executor / Coder agent handles the next task.
  3. Apply exact code below without assumption or improvisation.

## 1. Action Items
- [ ] Update `backend-admin/.gitignore`
- [ ] Create `backend-admin/.cargo/config.toml`
- [ ] Create `backend-admin/Cargo.toml`
- [ ] Create `backend-admin/package.json`
- [ ] Create `backend-admin/.dev.vars.example`
- [ ] Create `backend-admin/wrangler.toml`
- [ ] Create `backend-admin/src/config.rs`
- [ ] Create `backend-admin/src/error.rs`
- [ ] Create minimal temporary `backend-admin/src/lib.rs` placeholder module exports (`pub mod config; pub mod error;`) for compile check of Task 01 only (replaced in Task 05).
- [ ] Verify `cargo check` on `src/config.rs` and `src/error.rs`.

## 2. Exact Code / File Changes

### File: `backend-admin/.gitignore`
```gitignore
.DS_Store
.env*
!.env.example
.dev.vars
!.dev.vars.example
.wrangler/
target/
build/
node_modules/
```

### File: `backend-admin/.cargo/config.toml`
```toml
[target.wasm32-unknown-unknown]
rustflags = ["--cfg", 'getrandom_backend="wasm_js"']
```

### File: `backend-admin/Cargo.toml`
```toml
[package]
name = "backend-admin"
version = "0.1.0"
edition = "2021"
rust-version = "1.85"

[workspace]
members = [".", "tools/create-user"]
default-members = ["."]

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
aes-gcm = "0.10.3"
async-trait = "0.1.88"
axum = { version = "0.8", default-features = false, features = ["json", "macros", "query"] }
base64 = "0.22"
bytes = "1"
hmac = "0.12"
http = "1.3"
p256 = { version = "0.13", features = ["ecdsa", "pkcs8"] }
pbkdf2 = { version = "0.12", features = ["hmac"] }
rand_core = { version = "0.6", features = ["getrandom"] }
getrandom = { version = "0.2", features = ["js"] }
rsa = { version = "0.9.10", features = ["sha2"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sha2 = "0.10"
subtle = "2.6"
tower-service = "0.3"
uuid = { version = "1", features = ["v4", "js", "serde"] }
worker = { version = "0.8.5", features = ["http", "d1", "axum"] }
zeroize = { version = "1.8", features = ["zeroize_derive"] }

[target.'cfg(target_arch = "wasm32")'.dependencies]
js-sys = "0.3"
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
web-sys = { version = "0.3", features = [
    "Crypto",
    "CryptoKey",
    "Pbkdf2Params",
    "SubtleCrypto",
    "WorkerGlobalScope",
] }

[dev-dependencies]
tower = { version = "0.5", features = ["util"] }

[profile.release]
opt-level = "s"
lto = true
strip = true
codegen-units = 1
```

### File: `backend-admin/package.json`
```json
{
  "name": "backend-admin-worker",
  "version": "0.1.0",
  "private": true,
  "scripts": {
    "dev": "wrangler dev",
    "deploy:staging": "wrangler deploy --env staging",
    "deploy:prod": "wrangler deploy --env prod"
  },
  "devDependencies": {
    "wrangler": "^4.0.0"
  }
}
```

### File: `backend-admin/.dev.vars.example`
```dotenv
# Copy to .dev.vars for local wrangler dev (never commit .dev.vars)
PASSWORD_PEPPER="replace-with-local-dev-32-byte-hex-or-base64-pepper"
KEY_ENCRYPTION_KEY="MDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDA="
```

### File: `backend-admin/wrangler.toml`
```toml
name = "maemorkratai-backend-admin-dev"
main = "build/worker/shim.mjs"
compatibility_date = "2026-11-01"
migrations_dir = "migrations"

[build]
command = "worker-build --release"

[vars]
ENVIRONMENT = "dev"
ALLOWED_ORIGINS = "http://localhost:3000,http://127.0.0.1:3000"
JWT_ISSUER = "https://admin-api.local.maemorkratai.test"
JWT_AUDIENCE = "maemorkratai-admin"
ACCESS_TOKEN_TTL_SECS = "900"
REFRESH_TOKEN_TTL_SECS = "604800"
COOKIE_NAME = "mmk_admin_rt"
MAX_BODY_BYTES = "65536"

[[d1_databases]]
binding = "DB"
database_name = "maemorkratai-db-dev"
database_id = "00000000-0000-0000-0000-000000000001"
migrations_dir = "migrations"

[[r2_buckets]]
binding = "PUBLIC_ASSETS"
bucket_name = "maemorkratai-public-dev"

[[r2_buckets]]
binding = "PRIVATE_STORAGE"
bucket_name = "maemorkratai-private-dev"

[[ratelimits]]
name = "AUTH_RATE_LIMITER"
namespace_id = "1001"
simple = { limit = 10, period = 60 }

[env.staging]
name = "maemorkratai-backend-admin-staging"

[env.staging.vars]
ENVIRONMENT = "staging"
ALLOWED_ORIGINS = "https://admin-staging.maemorkratai.com"
JWT_ISSUER = "https://admin-api-staging.maemorkratai.com"
JWT_AUDIENCE = "maemorkratai-admin"
ACCESS_TOKEN_TTL_SECS = "900"
REFRESH_TOKEN_TTL_SECS = "604800"
COOKIE_NAME = "mmk_admin_rt"
MAX_BODY_BYTES = "65536"

[[env.staging.d1_databases]]
binding = "DB"
database_name = "maemorkratai-db-staging"
database_id = "00000000-0000-0000-0000-000000000002"
migrations_dir = "migrations"

[[env.staging.r2_buckets]]
binding = "PUBLIC_ASSETS"
bucket_name = "maemorkratai-public-staging"

[[env.staging.r2_buckets]]
binding = "PRIVATE_STORAGE"
bucket_name = "maemorkratai-private-staging"

[[env.staging.ratelimits]]
name = "AUTH_RATE_LIMITER"
namespace_id = "1002"
simple = { limit = 10, period = 60 }

[env.prod]
name = "maemorkratai-backend-admin-prod"

[env.prod.vars]
ENVIRONMENT = "prod"
ALLOWED_ORIGINS = "https://admin.maemorkratai.com"
JWT_ISSUER = "https://admin-api.maemorkratai.com"
JWT_AUDIENCE = "maemorkratai-admin"
ACCESS_TOKEN_TTL_SECS = "900"
REFRESH_TOKEN_TTL_SECS = "604800"
COOKIE_NAME = "mmk_admin_rt"
MAX_BODY_BYTES = "65536"

[[env.prod.d1_databases]]
binding = "DB"
database_name = "maemorkratai-db-prod"
database_id = "00000000-0000-0000-0000-000000000003"
migrations_dir = "migrations"

[[env.prod.r2_buckets]]
binding = "PUBLIC_ASSETS"
bucket_name = "maemorkratai-public-prod"

[[env.prod.r2_buckets]]
binding = "PRIVATE_STORAGE"
bucket_name = "maemorkratai-private-prod"

[[env.prod.ratelimits]]
name = "AUTH_RATE_LIMITER"
namespace_id = "1003"
simple = { limit = 10, period = 60 }
```

### File: `backend-admin/src/config.rs`
```rust
use crate::error::AppError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Dev,
    Staging,
    Prod,
}

impl Environment {
    pub fn parse(raw: &str) -> Result<Self, AppError> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "dev" => Ok(Self::Dev),
            "staging" => Ok(Self::Staging),
            "prod" | "production" => Ok(Self::Prod),
            _ => Err(AppError::internal("invalid ENVIRONMENT configuration")),
        }
    }

    pub fn is_secure_cookie(&self) -> bool {
        matches!(self, Self::Staging | Self::Prod)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Dev => "dev",
            Self::Staging => "staging",
            Self::Prod => "prod",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub environment: Environment,
    pub allowed_origins: Vec<String>,
    pub jwt_issuer: String,
    pub jwt_audience: String,
    pub access_token_ttl_secs: u64,
    pub refresh_token_ttl_secs: u64,
    pub cookie_name: String,
    pub max_body_bytes: usize,
}

impl AppConfig {
    pub fn parse_origins(raw: &str) -> Result<Vec<String>, AppError> {
        let origins: Vec<String> = raw
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.trim_end_matches('/').to_string())
            .collect();

        if origins.is_empty() || origins.iter().any(|o| o == "*") {
            return Err(AppError::internal(
                "ALLOWED_ORIGINS must contain explicit origins and never '*'",
            ));
        }
        Ok(origins)
    }

    pub fn is_origin_allowed(&self, origin: &str) -> bool {
        let normalized = origin.trim().trim_end_matches('/');
        self.allowed_origins.iter().any(|o| o == normalized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_wildcard_origin() {
        assert!(AppConfig::parse_origins("*").is_err());
        assert!(AppConfig::parse_origins("https://admin.example.com,*").is_err());
        assert!(AppConfig::parse_origins("   ").is_err());
    }

    #[test]
    fn cookie_security_matches_environment() {
        assert!(!Environment::Dev.is_secure_cookie());
        assert!(Environment::Staging.is_secure_cookie());
        assert!(Environment::Prod.is_secure_cookie());
    }
}
```

### File: `backend-admin/src/error.rs`
```rust
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ErrorEnvelope {
    pub request_id: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct AppError {
    pub status: StatusCode,
    pub code: &'static str,
    pub public_message: String,
    pub internal_detail: Option<String>,
    pub clear_refresh_cookie: Option<String>,
}

impl AppError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: "BAD_REQUEST",
            public_message: message.into(),
            internal_detail: None,
            clear_refresh_cookie: None,
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            code: "UNAUTHORIZED",
            public_message: message.into(),
            internal_detail: None,
            clear_refresh_cookie: None,
        }
    }

    pub fn unauthorized_and_clear_cookie(
        message: impl Into<String>,
        set_cookie_header: String,
    ) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            code: "UNAUTHORIZED",
            public_message: message.into(),
            internal_detail: None,
            clear_refresh_cookie: Some(set_cookie_header),
        }
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            code: "FORBIDDEN",
            public_message: message.into(),
            internal_detail: None,
            clear_refresh_cookie: None,
        }
    }

    pub fn payload_too_large() -> Self {
        Self {
            status: StatusCode::PAYLOAD_TOO_LARGE,
            code: "PAYLOAD_TOO_LARGE",
            public_message: "Request body exceeds maximum allowed size.".to_string(),
            internal_detail: None,
            clear_refresh_cookie: None,
        }
    }

    pub fn rate_limited() -> Self {
        Self {
            status: StatusCode::TOO_MANY_REQUESTS,
            code: "RATE_LIMITED",
            public_message: "Too many requests. Try again later.".to_string(),
            internal_detail: None,
            clear_refresh_cookie: None,
        }
    }

    pub fn service_unavailable(detail: impl Into<String>) -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code: "SERVICE_UNAVAILABLE",
            public_message: "Required service dependency is unavailable.".to_string(),
            internal_detail: Some(detail.into()),
            clear_refresh_cookie: None,
        }
    }

    pub fn internal(detail: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: "INTERNAL_ERROR",
            public_message: "An unexpected internal error occurred.".to_string(),
            internal_detail: Some(detail.into()),
            clear_refresh_cookie: None,
        }
    }

    pub fn into_response_with_request_id(self, request_id: &str) -> Response {
        let mut response = (
            self.status,
            Json(ErrorEnvelope {
                request_id: request_id.to_string(),
                code: self.code.to_string(),
                message: self.public_message,
            }),
        )
            .into_response();

        let headers = response.headers_mut();
        headers.insert(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-store"),
        );
        headers.insert(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        );
        if let Ok(val) = HeaderValue::from_str(request_id) {
            headers.insert("x-request-id", val);
        }
        if let Some(cookie) = self.clear_refresh_cookie {
            if let Ok(val) = HeaderValue::from_str(&cookie) {
                headers.append(header::SET_COOKIE, val);
            }
        }
        response
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        self.into_response_with_request_id("unassigned")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal_error_never_exposes_internal_detail() {
        let err = AppError::internal("SQLITE_CONSTRAINT: users.email leaked detail");
        assert_eq!(err.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(!err.public_message.contains("SQLITE"));
    }
}
```

### File: `backend-admin/src/lib.rs` (Temporary Task 01 compile stub; replaced in Task 05)
```rust
pub mod config;
pub mod error;
```

## 3. Verification Command
```bash
cd backend-admin && cargo test --lib && cargo check --target wasm32-unknown-unknown
```
