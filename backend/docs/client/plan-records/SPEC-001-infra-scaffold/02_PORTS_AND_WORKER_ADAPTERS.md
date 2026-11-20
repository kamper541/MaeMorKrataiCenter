# Task 02: `ClientPorts` Traits, `WorkerClientAdapter` & `MemoryClientStore`

- **Executor / Coder Rule**:
  1. ALWAYS read [00_OVERVIEW.md](./00_OVERVIEW.md) first before starting execution.
  2. Execute ONLY this single task. When done and verified, terminate so a fresh Executor / Coder agent handles the next task.
  3. Apply exact code below without assumption or improvisation.

## 1. Action Items
- [ ] Create `backend-client/src/ports.rs`
- [ ] Create `backend-client/src/adapters/mod.rs`
- [ ] Create `backend-client/src/adapters/worker_env.rs` (strictly binds `DB`, `PUBLIC_ASSETS`, `PUBLIC_RATE_LIMITER` — zero `PRIVATE_STORAGE`)
- [ ] Create `backend-client/src/adapters/memory.rs`
- [ ] Update temporary `backend-client/src/lib.rs` to export `ports` and `adapters` (replaced in Task 04).
- [ ] Verify host unit tests and `wasm32-unknown-unknown` build.

## 2. Exact Code / File Changes

### File: `backend-client/src/ports.rs`
```rust
use crate::config::ClientConfig;
use crate::error::ClientError;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait HealthPort: Send + Sync {
    async fn check_d1(&self) -> Result<(), ClientError>;
    async fn check_public_r2(&self) -> Result<(), ClientError>;
}

#[async_trait]
pub trait RateLimitPort: Send + Sync {
    async fn check_limit(&self, key: &str) -> Result<bool, ClientError>;
}

pub trait SecretProvider: Send + Sync {
    fn turnstile_secret(&self) -> Result<String, ClientError>;
}

pub trait Clock: Send + Sync {
    fn now_secs(&self) -> u64;
}

#[derive(Clone)]
pub struct ClientPorts {
    pub config: ClientConfig,
    pub health: Arc<dyn HealthPort>,
    pub rate_limiter: Arc<dyn RateLimitPort>,
    pub secrets: Arc<dyn SecretProvider>,
    pub clock: Arc<dyn Clock>,
}
```

### File: `backend-client/src/adapters/mod.rs`
```rust
pub mod memory;
pub mod worker_env;
```

### File: `backend-client/src/adapters/worker_env.rs`
```rust
use crate::config::{ClientConfig, Environment};
use crate::error::ClientError;
use crate::ports::{ClientPorts, Clock, HealthPort, RateLimitPort, SecretProvider};
use async_trait::async_trait;
use std::sync::Arc;
use worker::send::{SendFuture, SendWrapper};
use worker::Env;

#[derive(Clone)]
pub struct WorkerClientAdapter {
    env: SendWrapper<Env>,
}

impl WorkerClientAdapter {
    pub fn new(env: Env) -> Self {
        Self {
            env: SendWrapper::new(env),
        }
    }

    pub fn load_config(&self) -> Result<ClientConfig, ClientError> {
        let env_str = self
            .env
            .var("ENVIRONMENT")
            .map(|v| v.to_string())
            .map_err(|_| ClientError::internal("missing ENVIRONMENT var"))?;
        let environment = Environment::parse(&env_str)?;

        let origins_raw = self
            .env
            .var("ALLOWED_ORIGINS")
            .map(|v| v.to_string())
            .map_err(|_| ClientError::internal("missing ALLOWED_ORIGINS var"))?;
        let allowed_origins = ClientConfig::parse_origins(&origins_raw)?;

        let max_body_bytes = self
            .env
            .var("MAX_BODY_BYTES")
            .ok()
            .and_then(|v| v.to_string().parse::<usize>().ok())
            .unwrap_or(32_768);

        let turnstile_verify_url = self
            .env
            .var("TURNSTILE_VERIFY_URL")
            .map(|v| v.to_string())
            .unwrap_or_else(|_| {
                "https://challenges.cloudflare.com/turnstile/v0/siteverify".to_string()
            });
        let jwks_url = self
            .env
            .var("JWKS_URL")
            .map(|v| v.to_string())
            .map_err(|_| ClientError::internal("missing JWKS_URL var"))?;
        let jwt_issuer = self
            .env
            .var("JWT_ISSUER")
            .map(|v| v.to_string())
            .map_err(|_| ClientError::internal("missing JWT_ISSUER var"))?;
        let jwt_audience = self
            .env
            .var("JWT_AUDIENCE")
            .map(|v| v.to_string())
            .map_err(|_| ClientError::internal("missing JWT_AUDIENCE var"))?;

        Ok(ClientConfig {
            environment,
            allowed_origins,
            max_body_bytes,
            turnstile_verify_url,
            jwks_url,
            jwt_issuer,
            jwt_audience,
        })
    }

    pub fn into_ports(self) -> Result<ClientPorts, ClientError> {
        let config = self.load_config()?;
        let shared = Arc::new(self);
        Ok(ClientPorts {
            config,
            health: shared.clone(),
            rate_limiter: shared.clone(),
            secrets: shared.clone(),
            clock: shared,
        })
    }
}

#[async_trait]
impl HealthPort for WorkerClientAdapter {
    async fn check_d1(&self) -> Result<(), ClientError> {
        let db = self
            .env
            .d1("DB")
            .map_err(|e| ClientError::service_unavailable(format!("D1 binding error: {e}")))?;
        let stmt = db.prepare("SELECT 1 AS ok");
        SendFuture::new(stmt.first::<serde_json::Value>(None))
            .await
            .map_err(|e| ClientError::service_unavailable(format!("D1 query error: {e}")))?;
        Ok(())
    }

    async fn check_public_r2(&self) -> Result<(), ClientError> {
        let bucket = SendWrapper::new(self.env.bucket("PUBLIC_ASSETS").map_err(|e| {
            ClientError::service_unavailable(format!("PUBLIC_ASSETS binding error: {e}"))
        })?);
        SendFuture::new(bucket.head("__health_probe__"))
            .await
            .map_err(|e| ClientError::service_unavailable(format!("PUBLIC_ASSETS probe error: {e}")))?;
        Ok(())
    }
}

#[async_trait]
impl RateLimitPort for WorkerClientAdapter {
    async fn check_limit(&self, key: &str) -> Result<bool, ClientError> {
        let limiter = self
            .env
            .rate_limiter("PUBLIC_RATE_LIMITER")
            .map_err(|e| ClientError::internal(format!("PUBLIC_RATE_LIMITER missing: {e}")))?;
        let outcome = limiter
            .limit(key.to_string())
            .await
            .map_err(|e| ClientError::internal(format!("rate limit call failed: {e}")))?;
        Ok(outcome.success)
    }
}

impl SecretProvider for WorkerClientAdapter {
    fn turnstile_secret(&self) -> Result<String, ClientError> {
        self.env
            .secret("TURNSTILE_SECRET_KEY")
            .map(|s| s.to_string())
            .map_err(|_| ClientError::service_unavailable("TURNSTILE_SECRET_KEY missing"))
    }
}

impl Clock for WorkerClientAdapter {
    fn now_secs(&self) -> u64 {
        (worker::Date::now().as_millis() / 1000) as u64
    }
}
```

### File: `backend-client/src/adapters/memory.rs`
```rust
use crate::config::{ClientConfig, Environment};
use crate::error::ClientError;
use crate::ports::{ClientPorts, Clock, HealthPort, RateLimitPort, SecretProvider};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct MemoryClientStore {
    pub d1_healthy: AtomicBool,
    pub public_r2_healthy: AtomicBool,
    pub rate_counts: Mutex<HashMap<String, u32>>,
    pub rate_limit_max: AtomicU64,
    pub now_secs: AtomicU64,
    pub turnstile_secret: String,
}

impl MemoryClientStore {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            d1_healthy: AtomicBool::new(true),
            public_r2_healthy: AtomicBool::new(true),
            rate_counts: Mutex::new(HashMap::new()),
            rate_limit_max: AtomicU64::new(60),
            now_secs: AtomicU64::new(1_700_000_000),
            turnstile_secret: "1x0000000000000000000000000000000AA".to_string(),
        })
    }

    pub fn into_ports(self: &Arc<Self>, env: Environment) -> ClientPorts {
        ClientPorts {
            config: ClientConfig {
                environment: env,
                allowed_origins: vec!["https://staging.maemorkratai.com".to_string()],
                max_body_bytes: 32_768,
                turnstile_verify_url:
                    "https://challenges.cloudflare.com/turnstile/v0/siteverify".to_string(),
                jwks_url: "https://admin-api-staging.maemorkratai.com/.well-known/jwks.json"
                    .to_string(),
                jwt_issuer: "https://admin-api-staging.maemorkratai.com".to_string(),
                jwt_audience: "maemorkratai-client".to_string(),
            },
            health: self.clone(),
            rate_limiter: self.clone(),
            secrets: self.clone(),
            clock: self.clone(),
        }
    }
}

#[async_trait]
impl HealthPort for MemoryClientStore {
    async fn check_d1(&self) -> Result<(), ClientError> {
        if self.d1_healthy.load(Ordering::SeqCst) {
            Ok(())
        } else {
            Err(ClientError::service_unavailable("simulated D1 outage"))
        }
    }

    async fn check_public_r2(&self) -> Result<(), ClientError> {
        if self.public_r2_healthy.load(Ordering::SeqCst) {
            Ok(())
        } else {
            Err(ClientError::service_unavailable("simulated PUBLIC_ASSETS outage"))
        }
    }
}

#[async_trait]
impl RateLimitPort for MemoryClientStore {
    async fn check_limit(&self, key: &str) -> Result<bool, ClientError> {
        let max = self.rate_limit_max.load(Ordering::SeqCst) as u32;
        let mut counts = self.rate_counts.lock().unwrap();
        let count = counts.entry(key.to_string()).or_insert(0);
        *count += 1;
        Ok(*count <= max)
    }
}

impl SecretProvider for MemoryClientStore {
    fn turnstile_secret(&self) -> Result<String, ClientError> {
        Ok(self.turnstile_secret.clone())
    }
}

impl Clock for MemoryClientStore {
    fn now_secs(&self) -> u64 {
        self.now_secs.load(Ordering::SeqCst)
    }
}
```

### File: `backend-client/src/lib.rs` (Temporary Task 02 compile stub; replaced in Task 04)
```rust
pub mod adapters;
pub mod config;
pub mod error;
pub mod ports;
```

## 3. Verification Command
```bash
cd backend-client && cargo test --lib && cargo check --target wasm32-unknown-unknown
```
