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
