# Task 01: Rust Crate, Wrangler Config, Config & Error Envelope

- **Executor / Coder Rule**:
  1. ALWAYS read [00_OVERVIEW.md](./00_OVERVIEW.md) first before starting execution.
  2. Execute ONLY this single task. When done and verified, terminate so a fresh Executor / Coder agent handles the next task.
  3. Apply exact code below without assumption or improvisation.

## 1. Action Items
- [ ] Update `backend-client/.gitignore`
- [ ] Create `backend-client/.cargo/config.toml`
- [ ] Create `backend-client/Cargo.toml`
- [ ] Create `backend-client/package.json`
- [ ] Create `backend-client/.dev.vars.example`
- [ ] Create `backend-client/wrangler.toml` (NO `PRIVATE_STORAGE`, NO `migrations_dir`)
- [ ] Create `backend-client/Makefile`
- [ ] Create `backend-client/src/config.rs`
- [ ] Create `backend-client/src/error.rs`
- [ ] Create temporary `backend-client/src/lib.rs` export stub (`pub mod config; pub mod error;`) replaced in Task 04.
- [ ] Verify `cargo test --lib` and `cargo check --target wasm32-unknown-unknown`.

## 2. Exact Code / File Changes

### File: `backend-client/.gitignore`
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

### File: `backend-client/.cargo/config.toml`
```toml
[target.wasm32-unknown-unknown]
rustflags = ["--cfg", 'getrandom_backend="wasm_js"']
```

### File: `backend-client/Cargo.toml`
```toml
[package]
name = "backend-client"
version = "0.1.0"
edition = "2021"
rust-version = "1.85"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
async-trait = "0.1.88"
axum = { version = "0.8", default-features = false, features = ["json", "macros", "query"] }
base64 = "0.22"
bytes = "1"
getrandom = { version = "0.2", features = ["js"] }
http = "1.3"
p256 = { version = "0.13", features = ["ecdsa"] }
rsa = { version = "0.9.10", features = ["sha2"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sha2 = "0.10"
tower-service = "0.3"
uuid = { version = "1", features = ["v4", "js", "serde"] }
worker = { version = "0.8.5", features = ["http", "d1", "axum"] }

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"

[dev-dependencies]
rand_core = { version = "0.6", features = ["getrandom"] }
signature = "2.2"
tower = { version = "0.5", features = ["util"] }

[profile.release]
opt-level = "s"
lto = true
strip = true
codegen-units = 1
```

### File: `backend-client/package.json`
```json
{
  "name": "backend-client-worker",
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

### File: `backend-client/.dev.vars.example`
```dotenv
# Copy to .dev.vars for local wrangler dev (never commit .dev.vars)
# Cloudflare Turnstile always-passes dummy secret key for local testing:
TURNSTILE_SECRET_KEY="1x0000000000000000000000000000000AA"
```

### File: `backend-client/wrangler.toml`
```toml
name = "maemorkratai-backend-client-dev"
main = "build/worker/shim.mjs"
compatibility_date = "2026-11-01"

[build]
command = "worker-build --release"

[vars]
ENVIRONMENT = "dev"
ALLOWED_ORIGINS = "http://localhost:3000,http://127.0.0.1:3000"
MAX_BODY_BYTES = "32768"
TURNSTILE_VERIFY_URL = "https://challenges.cloudflare.com/turnstile/v0/siteverify"
JWKS_URL = "http://127.0.0.1:8787/.well-known/jwks.json"
JWT_ISSUER = "https://admin-api.local.maemorkratai.test"
JWT_AUDIENCE = "maemorkratai-client"

# Canonical D1 migrations are owned by ../backend-admin/migrations/ (D01).
[[d1_databases]]
binding = "DB"
database_name = "maemorkratai-db-dev"
database_id = "00000000-0000-0000-0000-000000000001"

# Least-privilege: PUBLIC_ASSETS only, never PRIVATE_STORAGE (D02).
[[r2_buckets]]
binding = "PUBLIC_ASSETS"
bucket_name = "maemorkratai-public-dev"

[[ratelimits]]
name = "PUBLIC_RATE_LIMITER"
namespace_id = "2001"
simple = { limit = 60, period = 60 }

[env.staging]
name = "maemorkratai-backend-client-staging"

[env.staging.vars]
ENVIRONMENT = "staging"
ALLOWED_ORIGINS = "https://staging.maemorkratai.com"
MAX_BODY_BYTES = "32768"
TURNSTILE_VERIFY_URL = "https://challenges.cloudflare.com/turnstile/v0/siteverify"
JWKS_URL = "https://admin-api-staging.maemorkratai.com/.well-known/jwks.json"
JWT_ISSUER = "https://admin-api-staging.maemorkratai.com"
JWT_AUDIENCE = "maemorkratai-client"

[[env.staging.d1_databases]]
binding = "DB"
database_name = "maemorkratai-db-staging"
database_id = "00000000-0000-0000-0000-000000000002"

[[env.staging.r2_buckets]]
binding = "PUBLIC_ASSETS"
bucket_name = "maemorkratai-public-staging"

[[env.staging.ratelimits]]
name = "PUBLIC_RATE_LIMITER"
namespace_id = "2002"
simple = { limit = 60, period = 60 }

[env.prod]
name = "maemorkratai-backend-client-prod"

[env.prod.vars]
ENVIRONMENT = "prod"
ALLOWED_ORIGINS = "https://maemorkratai.com,https://www.maemorkratai.com"
MAX_BODY_BYTES = "32768"
TURNSTILE_VERIFY_URL = "https://challenges.cloudflare.com/turnstile/v0/siteverify"
JWKS_URL = "https://admin-api.maemorkratai.com/.well-known/jwks.json"
JWT_ISSUER = "https://admin-api.maemorkratai.com"
JWT_AUDIENCE = "maemorkratai-client"

[[env.prod.d1_databases]]
binding = "DB"
database_name = "maemorkratai-db-prod"
database_id = "00000000-0000-0000-0000-000000000003"

[[env.prod.r2_buckets]]
binding = "PUBLIC_ASSETS"
bucket_name = "maemorkratai-public-prod"

[[env.prod.ratelimits]]
name = "PUBLIC_RATE_LIMITER"
namespace_id = "2003"
simple = { limit = 60, period = 60 }
```

### File: `backend-client/Makefile`
```makefile
.PHONY: check test build dev

check:
	cargo check --target wasm32-unknown-unknown

test:
	cargo test

build:
	worker-build --release

dev:
	npx wrangler dev
```

### File: `backend-client/src/config.rs`
```rust
use crate::error::ClientError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Dev,
    Staging,
    Prod,
}

impl Environment {
    pub fn parse(raw: &str) -> Result<Self, ClientError> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "dev" => Ok(Self::Dev),
            "staging" => Ok(Self::Staging),
            "prod" | "production" => Ok(Self::Prod),
            _ => Err(ClientError::internal("invalid ENVIRONMENT configuration")),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub environment: Environment,
    pub allowed_origins: Vec<String>,
    pub max_body_bytes: usize,
    pub turnstile_verify_url: String,
    pub jwks_url: String,
    pub jwt_issuer: String,
    pub jwt_audience: String,
}

impl ClientConfig {
    pub fn parse_origins(raw: &str) -> Result<Vec<String>, ClientError> {
        let origins: Vec<String> = raw
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(|s| s.trim_end_matches('/').to_string())
            .collect();

        if origins.is_empty() || origins.iter().any(|o| o == "*") {
            return Err(ClientError::internal(
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
    fn rejects_wildcard_and_empty_origins() {
        assert!(ClientConfig::parse_origins("*").is_err());
        assert!(ClientConfig::parse_origins("").is_err());
        let parsed = ClientConfig::parse_origins("https://maemorkratai.com/").unwrap();
        assert_eq!(parsed, vec!["https://maemorkratai.com"]);
    }
}
```

### File: `backend-client/src/error.rs`
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
pub struct ClientError {
    pub status: StatusCode,
    pub code: &'static str,
    pub public_message: String,
    pub internal_detail: Option<String>,
}

impl ClientError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: "BAD_REQUEST",
            public_message: message.into(),
            internal_detail: None,
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::UNAUTHORIZED,
            code: "UNAUTHORIZED",
            public_message: message.into(),
            internal_detail: None,
        }
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            code: "FORBIDDEN",
            public_message: message.into(),
            internal_detail: None,
        }
    }

    pub fn not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code: "NOT_FOUND",
            public_message: "Resource not found.".to_string(),
            internal_detail: None,
        }
    }

    pub fn payload_too_large() -> Self {
        Self {
            status: StatusCode::PAYLOAD_TOO_LARGE,
            code: "PAYLOAD_TOO_LARGE",
            public_message: "Request body exceeds maximum allowed size.".to_string(),
            internal_detail: None,
        }
    }

    pub fn rate_limited() -> Self {
        Self {
            status: StatusCode::TOO_MANY_REQUESTS,
            code: "RATE_LIMITED",
            public_message: "Too many requests. Try again later.".to_string(),
            internal_detail: None,
        }
    }

    pub fn service_unavailable(detail: impl Into<String>) -> Self {
        Self {
            status: StatusCode::SERVICE_UNAVAILABLE,
            code: "SERVICE_UNAVAILABLE",
            public_message: "Required service dependency is unavailable.".to_string(),
            internal_detail: Some(detail.into()),
        }
    }

    pub fn internal(detail: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: "INTERNAL_ERROR",
            public_message: "An unexpected internal error occurred.".to_string(),
            internal_detail: Some(detail.into()),
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
        headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
        headers.insert(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        );
        if let Ok(val) = HeaderValue::from_str(request_id) {
            headers.insert("x-request-id", val);
        }
        response
    }
}

impl IntoResponse for ClientError {
    fn into_response(self) -> Response {
        self.into_response_with_request_id("unassigned")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_envelope_never_leaks_internal_sql_or_stack() {
        let err = ClientError::service_unavailable("D1_ERROR: no such table cards");
        assert_eq!(err.status, StatusCode::SERVICE_UNAVAILABLE);
        assert!(!err.public_message.contains("D1_ERROR"));
        assert!(!err.public_message.contains("cards"));
    }
}
```

### File: `backend-client/src/lib.rs` (Temporary Task 01 compile stub; replaced in Task 04)
```rust
pub mod config;
pub mod error;
```

## 3. Verification Command
```bash
cd backend-client && cargo test --lib && cargo check --target wasm32-unknown-unknown
```
