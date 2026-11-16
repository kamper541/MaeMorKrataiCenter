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
