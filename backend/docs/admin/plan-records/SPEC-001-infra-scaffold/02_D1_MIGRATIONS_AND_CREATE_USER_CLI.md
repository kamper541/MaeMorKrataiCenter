# Task 02: Canonical D1 Migrations, `create-user` CLI & Makefile

- **Executor / Coder Rule**:
  1. ALWAYS read [00_OVERVIEW.md](./00_OVERVIEW.md) first before starting execution.
  2. Execute ONLY this single task. When done and verified, terminate so a fresh Executor / Coder agent handles the next task.
  3. Apply exact code below without assumption or improvisation.

## 1. Action Items
- [ ] Create `backend-admin/migrations/0001_init_auth.sql`
- [ ] Create `backend-admin/tools/create-user/Cargo.toml`
- [ ] Create `backend-admin/tools/create-user/src/main.rs`
- [ ] Create `backend-admin/Makefile`
- [ ] Run unit tests for `tools/create-user` and inspect SQL schema syntax.

## 2. Exact Code / File Changes

### File: `backend-admin/migrations/0001_init_auth.sql`
```sql
-- Canonical shared D1 schema migration 0001: Auth, JWKS keys, Sessions & Audit Events
-- Owned exclusively by backend-admin (Decision D01, D03)

CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('owner', 'admin', 'user')),
    is_active INTEGER NOT NULL DEFAULT 1 CHECK (is_active IN (0, 1)),
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_users_email ON users (email);

CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    token_hash TEXT NOT NULL UNIQUE,
    family_id TEXT NOT NULL,
    expires_at INTEGER NOT NULL,
    revoked_at INTEGER,
    replaced_by_session_id TEXT,
    created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON sessions (user_id);
CREATE INDEX IF NOT EXISTS idx_sessions_family_id ON sessions (family_id);
CREATE INDEX IF NOT EXISTS idx_sessions_token_hash ON sessions (token_hash);

CREATE TABLE IF NOT EXISTS jwks_keys (
    kid TEXT PRIMARY KEY NOT NULL,
    alg TEXT NOT NULL CHECK (alg IN ('ES256', 'RS256')),
    public_jwk_json TEXT NOT NULL,
    encrypted_private_key TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('active', 'retiring', 'revoked')),
    created_at INTEGER NOT NULL,
    expires_at INTEGER
);

CREATE INDEX IF NOT EXISTS idx_jwks_keys_status ON jwks_keys (status);

CREATE TABLE IF NOT EXISTS audit_events (
    id TEXT PRIMARY KEY NOT NULL,
    actor_user_id TEXT,
    event_type TEXT NOT NULL,
    target_id TEXT,
    metadata_json TEXT NOT NULL,
    created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_audit_events_created_at ON audit_events (created_at);
CREATE INDEX IF NOT EXISTS idx_audit_events_event_type ON audit_events (event_type);
```

### File: `backend-admin/tools/create-user/Cargo.toml`
```toml
[package]
name = "create-user"
version = "0.1.0"
edition = "2021"
rust-version = "1.85"

[dependencies]
base64 = "0.22"
getrandom = "0.2"
hmac = "0.12"
pbkdf2 = { version = "0.12", features = ["hmac"] }
rpassword = "7"
sha2 = "0.10"
uuid = { version = "1", features = ["v4"] }
```

### File: `backend-admin/tools/create-user/src/main.rs`
```rust
use base64::engine::general_purpose::STANDARD_NO_PAD;
use base64::Engine;
use hmac::{Hmac, Mac};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use std::env;
use std::fs;
use std::io::{self, BufRead, IsTerminal};
use std::process::{Command, ExitCode};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

const PBKDF2_ITERATIONS: u32 = 100_000;
const SALT_LEN: usize = 16;
const DERIVED_KEY_LEN: usize = 32;

#[derive(Debug, Clone, PartialEq, Eq)]
struct CliArgs {
    email: String,
    role: String,
    env_name: String,
}

fn parse_args(args: &[String]) -> Result<CliArgs, String> {
    let mut email: Option<String> = None;
    let mut role: Option<String> = None;
    let mut env_name = "dev".to_string();

    for arg in args {
        if let Some(val) = arg.strip_prefix("--email=") {
            email = Some(val.trim().to_ascii_lowercase());
        } else if let Some(val) = arg.strip_prefix("--role=") {
            role = Some(val.trim().to_ascii_lowercase());
        } else if let Some(val) = arg.strip_prefix("--env=") {
            env_name = val.trim().to_ascii_lowercase();
        } else if arg.starts_with("--password") || arg.starts_with("--pepper") {
            return Err(
                "Passing secrets or passwords as CLI arguments is forbidden. Use prompt or stdin."
                    .to_string(),
            );
        } else {
            return Err(format!("Unsupported argument: {arg}"));
        }
    }

    let email = email.ok_or_else(|| "Missing --email=<address>".to_string())?;
    validate_email(&email)?;

    let role = role.ok_or_else(|| "Missing --role=owner|admin|user".to_string())?;
    if !matches!(role.as_str(), "owner" | "admin" | "user") {
        return Err("Role must be one of: owner, admin, user".to_string());
    }

    if !matches!(env_name.as_str(), "dev" | "staging" | "prod") {
        return Err("Environment must be one of: dev, staging, prod".to_string());
    }

    Ok(CliArgs {
        email,
        role,
        env_name,
    })
}

fn validate_email(email: &str) -> Result<(), String> {
    if email.len() < 5 || email.len() > 254 {
        return Err("Invalid email length".to_string());
    }
    if email.contains('\'') || email.contains('"') || email.contains(';') || email.chars().any(char::is_whitespace) {
        return Err("Invalid characters in email".to_string());
    }
    let Some((local, domain)) = email.split_once('@') else {
        return Err("Email must contain '@'".to_string());
    };
    if local.is_empty() || domain.is_empty() || !domain.contains('.') {
        return Err("Invalid email domain".to_string());
    }
    Ok(())
}

fn read_secret_value(prompt: &str, env_key: Option<&str>, allow_dev_vars: bool) -> Result<String, String> {
    if let Some(key) = env_key {
        if let Ok(val) = env::var(key) {
            if !val.trim().is_empty() {
                return Ok(val);
            }
        }
        if allow_dev_vars {
            if let Ok(content) = fs::read_to_string(".dev.vars") {
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() || trimmed.starts_with('#') {
                        continue;
                    }
                    if let Some((k, v)) = trimmed.split_once('=') {
                        if k.trim() == key {
                            let cleaned = v.trim().trim_matches('"').trim_matches('\'');
                            if !cleaned.is_empty() {
                                return Ok(cleaned.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    if io::stdin().is_terminal() {
        rpassword::prompt_password(prompt).map_err(|e| format!("Failed to read input: {e}"))
    } else {
        let mut line = String::new();
        let stdin = io::stdin();
        stdin
            .lock()
            .read_line(&mut line)
            .map_err(|e| format!("Failed to read stdin: {e}"))?;
        Ok(line.trim_end_matches(&['\r', '\n'][..]).to_string())
    }
}

fn hash_password_peppered(password: &str, pepper: &str, salt: &[u8; SALT_LEN]) -> Result<String, String> {
    if password.len() < 12 {
        return Err("Password must be at least 12 characters".to_string());
    }
    if pepper.is_empty() {
        return Err("PASSWORD_PEPPER must not be empty".to_string());
    }

    let mut mac = HmacSha256::new_from_slice(pepper.as_bytes())
        .map_err(|_| "Invalid PASSWORD_PEPPER length".to_string())?;
    mac.update(password.as_bytes());
    let peppered = mac.finalize().into_bytes();

    let mut derived = [0u8; DERIVED_KEY_LEN];
    pbkdf2_hmac::<Sha256>(&peppered, salt, PBKDF2_ITERATIONS, &mut derived);

    let salt_b64 = STANDARD_NO_PAD.encode(salt);
    let dk_b64 = STANDARD_NO_PAD.encode(derived);
    Ok(format!(
        "$pbkdf2-sha256-peppered$i={PBKDF2_ITERATIONS}${salt_b64}${dk_b64}"
    ))
}

fn build_insert_sql(id: &str, email: &str, password_hash: &str, role: &str, now_secs: u64) -> String {
    format!(
        "INSERT INTO users (id, email, password_hash, role, is_active, created_at, updated_at) VALUES ('{id}', '{email}', '{password_hash}', '{role}', 1, {now_secs}, {now_secs});"
    )
}

fn run() -> Result<(), String> {
    let raw_args: Vec<String> = env::args().skip(1).collect();
    let parsed = parse_args(&raw_args)?;

    let pepper = read_secret_value(
        "Enter PASSWORD_PEPPER: ",
        Some("PASSWORD_PEPPER"),
        parsed.env_name == "dev",
    )?;
    let password = read_secret_value("Enter user password (min 12 chars): ", None, false)?;

    let mut salt = [0u8; SALT_LEN];
    getrandom::getrandom(&mut salt).map_err(|e| format!("CSPRNG failure: {e}"))?;

    let password_hash = hash_password_peppered(&password, &pepper, &salt)?;
    let user_id = Uuid::new_v4().to_string();
    let now_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("System clock error: {e}"))?
        .as_secs();

    let sql = build_insert_sql(&user_id, &parsed.email, &password_hash, &parsed.role, now_secs);

    let mut cmd = Command::new("npx");
    cmd.args(["wrangler", "d1", "execute", "DB"]);
    match parsed.env_name.as_str() {
        "dev" => {
            cmd.arg("--local");
        }
        "staging" => {
            cmd.args(["--env", "staging", "--remote"]);
        }
        "prod" => {
            cmd.args(["--env", "prod", "--remote"]);
        }
        _ => unreachable!(),
    }
    cmd.args(["--command", &sql]);

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to invoke wrangler: {e}"))?;
    if !status.success() {
        return Err(format!("wrangler d1 execute exited with status {status}"));
    }

    println!(
        "Provisioned user id={} email={} role={} env={}",
        user_id, parsed.email, parsed.role, parsed.env_name
    );
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("Error: {msg}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_password_flag_in_cli_args() {
        let args = vec![
            "--email=owner@example.com".to_string(),
            "--role=owner".to_string(),
            "--password=secret".to_string(),
        ];
        assert!(parse_args(&args).is_err());
    }

    #[test]
    fn hashes_with_versioned_format_deterministically() {
        let salt = [7u8; SALT_LEN];
        let hash = hash_password_peppered("correct-horse-battery-staple", "test-pepper", &salt)
            .expect("hash succeeds");
        assert!(hash.starts_with("$pbkdf2-sha256-peppered$i=100000$"));
        assert_eq!(hash.split('$').count(), 5);
    }

    #[test]
    fn rejects_sql_quote_in_email() {
        assert!(validate_email("bad'user@example.com").is_err());
    }
}
```

### File: `backend-admin/Makefile`
```makefile
.PHONY: check test build dev migrate-local migrate-staging migrate-prod create-user

check:
	cargo check --target wasm32-unknown-unknown

test:
	cargo test --workspace

build:
	worker-build --release

dev:
	npx wrangler dev

migrate-local:
	npx wrangler d1 migrations apply DB --local

migrate-staging:
	npx wrangler d1 migrations apply DB --env staging --remote

migrate-prod:
	npx wrangler d1 migrations apply DB --env prod --remote

# Usage: make create-user EMAIL=owner@maemorkratai.com ROLE=owner ENV=dev
create-user:
	cargo run --manifest-path tools/create-user/Cargo.toml -- --email=$(EMAIL) --role=$(ROLE) --env=$(or $(ENV),dev)
```

## 3. Verification Command
```bash
cd backend-admin && cargo test --manifest-path tools/create-user/Cargo.toml
```
