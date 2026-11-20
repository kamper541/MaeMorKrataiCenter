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
