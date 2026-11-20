# Battleground Debate Record

- **Plan**: `SPEC-001-infra-scaffold`
- **Task**: `crypto-jwt-jwks-and-pbkdf2`
- **Date**: 2026-11-16
- **Status**: RESOLVED

## 1. The Conflict / Decision Point
How should `backend-admin` implement PBKDF2-HMAC-SHA256 (`100,000` iterations) + `PASSWORD_PEPPER`, AES-256-GCM key-wrapping, and `ES256`/`RS256` JWT/JWKS operations across `wasm32-unknown-unknown` (Cloudflare Workers Free tier 10 ms CPU cap) and host `cargo test` / `make create-user`?

## 2. Competing Options

### Option A: Pure-Rust `jsonwebtoken` (`rust_crypto`) and Pure-Rust `pbkdf2` Everywhere
- **Advocate**: Contender Agent A
- **Security (#1)**: Executing 100,000 iterations of pure-Rust `pbkdf2` inside `wasm32-unknown-unknown` consumes 30 to 100 ms of Wasm CPU time, breaching the Cloudflare Workers Free tier 10 ms CPU limit on `POST /v1/auth/login`. `jsonwebtoken` also leaves key generation, JWK serialization, and AES-256-GCM envelope encryption unaddressed.
- **Technical Debt (#2)**: Still requires `p256` and `aes-gcm` for `ES256` key rotation and private-key encryption, creating duplicate crypto dependencies. Default `jsonwebtoken` features pull `ring`, which fails `wasm32-unknown-unknown` builds.
- **Speed (#3)**: Saves minimal JWS string formatting code, but breaks at runtime on Free tier Workers.

### Option B: Hybrid `SubtleCrypto` PBKDF2-100k (`wasm32`) + Pure-Rust `pbkdf2` (Host) with `p256`, `aes-gcm`, `rsa` Verify-Only, and `sha2`
- **Advocate**: Contender Agent B
- **Security (#1)**: Under `#[cfg(target_arch = "wasm32")]`, `SubtleCrypto.deriveBits` executes PBKDF2-HMAC-SHA256 (`100,000` iterations) in native BoringSSL C++ under 2 ms, staying inside the 10 ms Free tier limit and respecting the `workerd` 100,000-iteration ceiling (`RES-001`). Under `#[cfg(not(target_arch = "wasm32"))]`, pure-Rust `pbkdf2` produces byte-identical `$pbkdf2-sha256-peppered$i=100000$...` hashes for `make create-user` and `cargo test`. Both paths apply HMAC-SHA256 `PASSWORD_PEPPER` and compare hashes via `subtle::ConstantTimeEq`. Pure-Rust `p256` and `aes-gcm` (`0.10.3`) provide deterministic `ES256` signing/keygen and AES-256-GCM envelope encryption bound to AAD `kid || "|" || alg`, with `zeroize` clearing plaintext key buffers. `rsa` (`0.9.10`) is used strictly for `RS256` public-key signature verification (no private-key operations, avoiding `RUSTSEC-2023-0071` decryption timing exposure). The JWT verifier rejects `alg: none`, rejects symmetric algorithms, binds `alg` to `kty`, and checks `iss`, `aud`, `exp`, `nbf`, and `kid`.
- **Technical Debt (#2)**: Consolidates all crypto into `src/auth/password.rs`, `src/auth/envelope.rs`, and `src/auth/jwt.rs` on aligned RustCrypto traits (`digest 0.10`, `signature 2.2`, `getrandom 0.2` `"js"`), compiling cleanly on both host and `wasm32-unknown-unknown`.
- **Speed (#3)**: Adds ~180 lines of `web-sys` `SubtleCrypto` and JWS serialization helpers while eliminating C-toolchain failures and Free tier CPU timeouts.

## 3. Outcome & Winning Rationale
- **Winner**: Option B
- **Justification**: Option B wins on Priority 1 Security and Priority 2 Technical Debt. It meets the Cloudflare Workers Free tier 10 ms CPU cap (`D03`), avoids `ring` and C dependencies on `wasm32-unknown-unknown`, enforces constant-time checks and AAD binding, and keeps host `cargo test` parity across all cryptographic flows.
