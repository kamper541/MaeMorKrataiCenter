use crate::auth::jwt::{PublicJwk, Role};
use crate::config::AppConfig;
use crate::error::AppError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserRecord {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub role: Role,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRecord {
    pub id: String,
    pub user_id: String,
    pub token_hash: String,
    pub family_id: String,
    pub expires_at: u64,
    pub revoked_at: Option<u64>,
    pub replaced_by_session_id: Option<String>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwksKeyRecord {
    pub kid: String,
    pub alg: String,
    pub public_jwk_json: String,
    pub encrypted_private_key: String,
    pub status: String,
    pub created_at: u64,
}

#[derive(Debug, Clone)]
pub struct AuditEventInput {
    pub id: String,
    pub actor_user_id: Option<String>,
    pub event_type: String,
    pub target_id: Option<String>,
    pub metadata_json: String,
    pub created_at: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefreshOutcome {
    Rotated {
        user: UserRecord,
        family_id: String,
        new_session_id: String,
    },
    Replayed {
        user_id: String,
        family_id: String,
    },
    NotFoundOrExpired,
}

#[async_trait]
pub trait AuthRepo: Send + Sync {
    async fn health_check_db(&self) -> Result<(), AppError>;
    async fn find_user_by_email(&self, email: &str) -> Result<Option<UserRecord>, AppError>;
    async fn find_user_by_id(&self, user_id: &str) -> Result<Option<UserRecord>, AppError>;
    async fn insert_session(&self, session: SessionRecord) -> Result<(), AppError>;
    async fn rotate_refresh_session(
        &self,
        token_hash: &str,
        new_session: SessionRecord,
        now_secs: u64,
    ) -> Result<RefreshOutcome, AppError>;
    async fn revoke_session_by_token_hash(
        &self,
        token_hash: &str,
        now_secs: u64,
    ) -> Result<Option<String>, AppError>;
    async fn revoke_user_sessions(&self, user_id: &str, now_secs: u64) -> Result<(), AppError>;
    async fn get_active_signing_key(&self) -> Result<Option<JwksKeyRecord>, AppError>;
    async fn list_public_jwks(&self) -> Result<Vec<PublicJwk>, AppError>;
    async fn rotate_signing_key(
        &self,
        new_key: JwksKeyRecord,
        audit: AuditEventInput,
    ) -> Result<(), AppError>;
    async fn insert_audit_event(&self, audit: AuditEventInput) -> Result<(), AppError>;
}

#[async_trait]
pub trait StorageProbe: Send + Sync {
    async fn check_r2_bindings(&self) -> Result<(), AppError>;
}

#[async_trait]
pub trait RateLimitPort: Send + Sync {
    async fn check_limit(&self, key: &str) -> Result<bool, AppError>;
}

pub trait SecretStore: Send + Sync {
    fn password_pepper(&self) -> Result<String, AppError>;
    fn key_encryption_key(&self) -> Result<String, AppError>;
}

pub trait Clock: Send + Sync {
    fn now_secs(&self) -> u64;
}

#[derive(Clone)]
pub struct AppPorts {
    pub config: AppConfig,
    pub repo: Arc<dyn AuthRepo>,
    pub storage: Arc<dyn StorageProbe>,
    pub rate_limiter: Arc<dyn RateLimitPort>,
    pub secrets: Arc<dyn SecretStore>,
    pub clock: Arc<dyn Clock>,
}
