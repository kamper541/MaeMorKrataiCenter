# Task 03: Hybrid PBKDF2-100k Password Hashing, AES-256-GCM Envelope & ES256/RS256 JWT

- **Executor / Coder Rule**:
  1. ALWAYS read [00_OVERVIEW.md](./00_OVERVIEW.md) first before starting execution.
  2. Execute ONLY this single task. When done and verified, terminate so a fresh Executor / Coder agent handles the next task.
  3. Apply exact code below without assumption or improvisation.

## 1. Action Items
- [ ] Create `backend-admin/src/auth/mod.rs`
- [ ] Create `backend-admin/src/auth/password.rs`
- [ ] Create `backend-admin/src/auth/envelope.rs`
- [ ] Create `backend-admin/src/auth/jwt.rs`
- [ ] Update temporary `backend-admin/src/lib.rs` to export `pub mod auth;` (replaced in Task 05).
- [ ] Run host unit tests and `wasm32-unknown-unknown` check.

## 2. Exact Code / File Changes

### File: `backend-admin/src/auth/mod.rs`
```rust
pub mod envelope;
pub mod jwt;
pub mod password;
```

### File: `backend-admin/src/auth/password.rs`
```rust
use crate::error::AppError;
use base64::engine::general_purpose::STANDARD_NO_PAD;
use base64::Engine;
use hmac::{Hmac, Mac};
use rand_core::{OsRng, RngCore};
use sha2::Sha256;
use subtle::ConstantTimeEq;
use zeroize::Zeroizing;

type HmacSha256 = Hmac<Sha256>;

pub const PBKDF2_ITERATIONS: u32 = 100_000;
const SALT_LEN: usize = 16;
const DERIVED_KEY_LEN: usize = 32;
const PREFIX: &str = "pbkdf2-sha256-peppered";

fn compute_peppered_secret(password: &str, pepper: &str) -> Result<Zeroizing<Vec<u8>>, AppError> {
    if pepper.is_empty() {
        return Err(AppError::internal("PASSWORD_PEPPER is empty"));
    }
    let mut mac = HmacSha256::new_from_slice(pepper.as_bytes())
        .map_err(|_| AppError::internal("invalid PASSWORD_PEPPER"))?;
    mac.update(password.as_bytes());
    Ok(Zeroizing::new(mac.finalize().into_bytes().to_vec()))
}

#[cfg(not(target_arch = "wasm32"))]
async fn derive_pbkdf2_sha256(
    peppered: &[u8],
    salt: &[u8],
    iterations: u32,
) -> Result<[u8; DERIVED_KEY_LEN], AppError> {
    let mut derived = [0u8; DERIVED_KEY_LEN];
    pbkdf2::pbkdf2_hmac::<Sha256>(peppered, salt, iterations, &mut derived);
    Ok(derived)
}

#[cfg(target_arch = "wasm32")]
async fn derive_pbkdf2_sha256(
    peppered: &[u8],
    salt: &[u8],
    iterations: u32,
) -> Result<[u8; DERIVED_KEY_LEN], AppError> {
    use js_sys::{Array, Object, Reflect, Uint8Array};
    use wasm_bindgen::{JsCast, JsValue};
    use wasm_bindgen_futures::JsFuture;

    let global: web_sys::WorkerGlobalScope = js_sys::global()
        .dyn_into()
        .map_err(|_| AppError::internal("failed to acquire WorkerGlobalScope"))?;
    let subtle = global
        .crypto()
        .map_err(|_| AppError::internal("WebCrypto unavailable"))?
        .subtle();

    let key_bytes = Uint8Array::from(peppered);
    let usages = Array::of1(&JsValue::from_str("deriveBits"));
    let key_promise = subtle
        .import_key_with_str(
            "raw",
            key_bytes.as_ref(),
            "PBKDF2",
            false,
            usages.as_ref(),
        )
        .map_err(|_| AppError::internal("WebCrypto importKey failed"))?;
    let crypto_key: web_sys::CryptoKey = JsFuture::from(key_promise)
        .await
        .map_err(|_| AppError::internal("WebCrypto importKey promise rejected"))?
        .dyn_into()
        .map_err(|_| AppError::internal("WebCrypto returned invalid CryptoKey"))?;

    let salt_arr = Uint8Array::from(salt);
    let params = Object::new();
    Reflect::set(&params, &JsValue::from_str("name"), &JsValue::from_str("PBKDF2"))
        .map_err(|_| AppError::internal("failed to build Pbkdf2Params"))?;
    Reflect::set(&params, &JsValue::from_str("hash"), &JsValue::from_str("SHA-256"))
        .map_err(|_| AppError::internal("failed to set hash"))?;
    Reflect::set(&params, &JsValue::from_str("iterations"), &JsValue::from_f64(f64::from(iterations)))
        .map_err(|_| AppError::internal("failed to set iterations"))?;
    Reflect::set(&params, &JsValue::from_str("salt"), salt_arr.as_ref())
        .map_err(|_| AppError::internal("failed to set salt"))?;

    let bits_promise = subtle
        .derive_bits_with_object(&params, &crypto_key, (DERIVED_KEY_LEN * 8) as u32)
        .map_err(|_| AppError::internal("WebCrypto deriveBits failed"))?;
    let bits_buf = JsFuture::from(bits_promise)
        .await
        .map_err(|_| AppError::internal("WebCrypto deriveBits rejected"))?;

    let out_arr = Uint8Array::new(&bits_buf);
    if out_arr.length() as usize != DERIVED_KEY_LEN {
        return Err(AppError::internal("unexpected derived bit length"));
    }
    let mut out = [0u8; DERIVED_KEY_LEN];
    out_arr.copy_to(&mut out);
    Ok(out)
}

pub async fn hash_password(password: &str, pepper: &str) -> Result<String, AppError> {
    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);
    let peppered = compute_peppered_secret(password, pepper)?;
    let derived = derive_pbkdf2_sha256(&peppered, &salt, PBKDF2_ITERATIONS).await?;
    Ok(format!(
        "${PREFIX}$i={PBKDF2_ITERATIONS}${}${}",
        STANDARD_NO_PAD.encode(salt),
        STANDARD_NO_PAD.encode(derived)
    ))
}

pub async fn verify_password(
    password: &str,
    stored_hash: &str,
    pepper: &str,
) -> Result<bool, AppError> {
    let parts: Vec<&str> = stored_hash.split('$').collect();
    if parts.len() != 5 || !parts[0].is_empty() || parts[1] != PREFIX {
        return Err(AppError::internal("unsupported stored password hash format"));
    }

    let iter_str = parts[2]
        .strip_prefix("i=")
        .ok_or_else(|| AppError::internal("invalid iteration field in hash"))?;
    let iterations: u32 = iter_str
        .parse()
        .map_err(|_| AppError::internal("invalid iteration count"))?;
    if iterations != PBKDF2_ITERATIONS {
        return Err(AppError::internal("unsupported PBKDF2 iteration count"));
    }

    let salt = STANDARD_NO_PAD
        .decode(parts[3])
        .map_err(|_| AppError::internal("invalid base64 salt"))?;
    let expected = STANDARD_NO_PAD
        .decode(parts[4])
        .map_err(|_| AppError::internal("invalid base64 digest"))?;
    if expected.len() != DERIVED_KEY_LEN {
        return Err(AppError::internal("unexpected stored digest length"));
    }

    let peppered = compute_peppered_secret(password, pepper)?;
    let actual = derive_pbkdf2_sha256(&peppered, &salt, iterations).await?;
    Ok(bool::from(actual.ct_eq(expected.as_slice())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_and_cli_vector_match() {
        let rt = tokio_less_block_on(async {
            let pepper = "dev-pepper-secret";
            let pw = "correct-horse-battery-staple";
            let hashed = hash_password(pw, pepper).await.unwrap();
            assert!(verify_password(pw, &hashed, pepper).await.unwrap());
            assert!(!verify_password("wrong-password", &hashed, pepper).await.unwrap());
            assert!(!verify_password(pw, &hashed, "wrong-pepper").await.unwrap());
        });
        rt
    }

    fn tokio_less_block_on<F: std::future::Future>(fut: F) -> F::Output {
        use std::pin::pin;
        use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
        fn noop(_: *const ()) {}
        fn clone(ptr: *const ()) -> RawWaker {
            RawWaker::new(ptr, &VTABLE)
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
        let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
        let mut cx = Context::from_waker(&waker);
        let mut pinned = pin!(fut);
        match pinned.as_mut().poll(&mut cx) {
            Poll::Ready(val) => val,
            Poll::Pending => panic!("host pbkdf2 future must resolve synchronously"),
        }
    }
}
```

### File: `backend-admin/src/auth/envelope.rs`
```rust
use crate::error::AppError;
use aes_gcm::aead::{Aead, KeyInit, Payload};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use rand_core::{OsRng, RngCore};
use zeroize::Zeroizing;

const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32;
const ENVELOPE_PREFIX: &str = "v1";

fn decode_kek(kek_b64: &str) -> Result<Zeroizing<[u8; KEY_LEN]>, AppError> {
    let raw = STANDARD
        .decode(kek_b64.trim())
        .map_err(|_| AppError::internal("invalid KEY_ENCRYPTION_KEY base64 encoding"))?;
    if raw.len() != KEY_LEN {
        return Err(AppError::internal("KEY_ENCRYPTION_KEY must be 32 bytes"));
    }
    let mut key = Zeroizing::new([0u8; KEY_LEN]);
    key.copy_from_slice(&raw);
    Ok(key)
}

fn build_aad(kid: &str, alg: &str) -> Vec<u8> {
    format!("{kid}|{alg}").into_bytes()
}

pub fn encrypt_private_key(
    kek_b64: &str,
    kid: &str,
    alg: &str,
    plaintext_pkcs8_der: &[u8],
) -> Result<String, AppError> {
    let key_bytes = decode_kek(kek_b64)?;
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key_bytes.as_ref()));

    let mut nonce_bytes = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let aad = build_aad(kid, alg);

    let ciphertext = cipher
        .encrypt(
            nonce,
            Payload {
                msg: plaintext_pkcs8_der,
                aad: &aad,
            },
        )
        .map_err(|_| AppError::internal("failed to encrypt signing key"))?;

    Ok(format!(
        "{ENVELOPE_PREFIX}${}${}",
        STANDARD.encode(nonce_bytes),
        STANDARD.encode(ciphertext)
    ))
}

pub fn decrypt_private_key(
    kek_b64: &str,
    kid: &str,
    alg: &str,
    envelope: &str,
) -> Result<Zeroizing<Vec<u8>>, AppError> {
    let parts: Vec<&str> = envelope.split('$').collect();
    if parts.len() != 3 || parts[0] != ENVELOPE_PREFIX {
        return Err(AppError::internal("unsupported encrypted key envelope format"));
    }

    let nonce_bytes = STANDARD
        .decode(parts[1])
        .map_err(|_| AppError::internal("invalid key envelope nonce"))?;
    if nonce_bytes.len() != NONCE_LEN {
        return Err(AppError::internal("invalid key envelope nonce length"));
    }

    let ciphertext = STANDARD
        .decode(parts[2])
        .map_err(|_| AppError::internal("invalid key envelope ciphertext"))?;

    let key_bytes = decode_kek(kek_b64)?;
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key_bytes.as_ref()));
    let aad = build_aad(kid, alg);

    let plaintext = cipher
        .decrypt(
            Nonce::from_slice(&nonce_bytes),
            Payload {
                msg: &ciphertext,
                aad: &aad,
            },
        )
        .map_err(|_| AppError::internal("KEY_ENCRYPTION_KEY decryption failed"))?;

    Ok(Zeroizing::new(plaintext))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_decrypt_roundtrip_and_aad_binding() {
        let kek = STANDARD.encode([42u8; 32]);
        let secret = b"pkcs8-private-key-material";
        let sealed = encrypt_private_key(&kek, "kid-1", "ES256", secret).unwrap();

        let opened = decrypt_private_key(&kek, "kid-1", "ES256", &sealed).unwrap();
        assert_eq!(opened.as_slice(), secret);

        // Wrong kid or alg fails AEAD tag check
        assert!(decrypt_private_key(&kek, "kid-2", "ES256", &sealed).is_err());
        assert!(decrypt_private_key(&kek, "kid-1", "RS256", &sealed).is_err());

        // Wrong KEK fails AEAD tag check
        let wrong_kek = STANDARD.encode([99u8; 32]);
        assert!(decrypt_private_key(&wrong_kek, "kid-1", "ES256", &sealed).is_err());
    }
}
```

### File: `backend-admin/src/auth/jwt.rs`
```rust
use crate::error::AppError;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use p256::ecdsa::signature::{Signer, Verifier};
use p256::ecdsa::{Signature as P256Signature, SigningKey, VerifyingKey as P256VerifyingKey};
use p256::elliptic_curve::sec1::FromEncodedPoint;
use p256::pkcs8::{DecodePrivateKey, EncodePrivateKey};
use p256::{EncodedPoint, PublicKey as P256PublicKey};
use rand_core::OsRng;
use rsa::pkcs1v15::{Signature as RsaSignature, VerifyingKey as RsaVerifyingKey};
use rsa::{BigUint, RsaPublicKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use uuid::Uuid;
use zeroize::Zeroizing;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User = 1,
    Admin = 2,
    Owner = 3,
}

impl Role {
    pub fn parse(raw: &str) -> Result<Self, AppError> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "user" => Ok(Self::User),
            "admin" => Ok(Self::Admin),
            "owner" => Ok(Self::Owner),
            _ => Err(AppError::unauthorized("invalid role claim")),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Admin => "admin",
            Self::Owner => "owner",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtHeader {
    pub alg: String,
    pub typ: String,
    pub kid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessClaims {
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub role: Role,
    pub jti: String,
    pub iat: u64,
    pub exp: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nbf: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PublicJwk {
    pub kty: String,
    #[serde(rename = "use", skip_serializing_if = "Option::is_none")]
    pub key_use: Option<String>,
    pub alg: String,
    pub kid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwksDocument {
    pub keys: Vec<PublicJwk>,
}

pub struct GeneratedKeypair {
    pub kid: String,
    pub alg: String,
    pub public_jwk: PublicJwk,
    pub pkcs8_private_der: Zeroizing<Vec<u8>>,
}

pub fn generate_es256_keypair() -> Result<GeneratedKeypair, AppError> {
    let signing_key = SigningKey::random(&mut OsRng);
    let pkcs8 = signing_key
        .to_pkcs8_der()
        .map_err(|_| AppError::internal("failed to encode ES256 private key"))?;
    let verifying_key = signing_key.verifying_key();
    let point = verifying_key.to_encoded_point(false);
    let x = point
        .x()
        .ok_or_else(|| AppError::internal("missing EC x coordinate"))?;
    let y = point
        .y()
        .ok_or_else(|| AppError::internal("missing EC y coordinate"))?;

    let kid = format!("es256-{}", Uuid::new_v4());
    let public_jwk = PublicJwk {
        kty: "EC".to_string(),
        key_use: Some("sig".to_string()),
        alg: "ES256".to_string(),
        kid: kid.clone(),
        crv: Some("P-256".to_string()),
        x: Some(URL_SAFE_NO_PAD.encode(x)),
        y: Some(URL_SAFE_NO_PAD.encode(y)),
        n: None,
        e: None,
    };

    Ok(GeneratedKeypair {
        kid,
        alg: "ES256".to_string(),
        public_jwk,
        pkcs8_private_der: Zeroizing::new(pkcs8.as_bytes().to_vec()),
    })
}

pub fn sign_es256_jwt(
    kid: &str,
    pkcs8_private_der: &[u8],
    claims: &AccessClaims,
) -> Result<String, AppError> {
    let signing_key = SigningKey::from_pkcs8_der(pkcs8_private_der)
        .map_err(|_| AppError::internal("failed to parse ES256 private key"))?;

    let header = JwtHeader {
        alg: "ES256".to_string(),
        typ: "JWT".to_string(),
        kid: kid.to_string(),
    };
    let header_b64 = URL_SAFE_NO_PAD.encode(
        serde_json::to_vec(&header)
            .map_err(|_| AppError::internal("failed to encode JWT header"))?,
    );
    let claims_b64 = URL_SAFE_NO_PAD.encode(
        serde_json::to_vec(claims)
            .map_err(|_| AppError::internal("failed to encode JWT claims"))?,
    );
    let signing_input = format!("{header_b64}.{claims_b64}");

    let signature: P256Signature = signing_key.sign(signing_input.as_bytes());
    let sig_b64 = URL_SAFE_NO_PAD.encode(signature.to_bytes());
    Ok(format!("{signing_input}.{sig_b64}"))
}

pub fn verify_jwt_with_jwks(
    token: &str,
    jwks: &[PublicJwk],
    expected_iss: &str,
    expected_aud: &str,
    now_secs: u64,
) -> Result<AccessClaims, AppError> {
    let mut parts = token.split('.');
    let (Some(header_b64), Some(claims_b64), Some(sig_b64), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return Err(AppError::unauthorized("invalid token"));
    };

    let header_bytes = URL_SAFE_NO_PAD
        .decode(header_b64)
        .map_err(|_| AppError::unauthorized("invalid token"))?;
    let header: JwtHeader =
        serde_json::from_slice(&header_bytes).map_err(|_| AppError::unauthorized("invalid token"))?;

    if header.alg != "ES256" && header.alg != "RS256" {
        return Err(AppError::unauthorized("invalid token"));
    }

    let jwk = jwks
        .iter()
        .find(|k| k.kid == header.kid)
        .ok_or_else(|| AppError::unauthorized("invalid token"))?;
    if jwk.alg != header.alg {
        return Err(AppError::unauthorized("invalid token"));
    }

    let sig_bytes = URL_SAFE_NO_PAD
        .decode(sig_b64)
        .map_err(|_| AppError::unauthorized("invalid token"))?;
    let signing_input = format!("{header_b64}.{claims_b64}");

    match header.alg.as_str() {
        "ES256" => {
            if jwk.kty != "EC" || jwk.crv.as_deref() != Some("P-256") {
                return Err(AppError::unauthorized("invalid token"));
            }
            let x = URL_SAFE_NO_PAD
                .decode(jwk.x.as_deref().ok_or_else(|| AppError::unauthorized("invalid token"))?)
                .map_err(|_| AppError::unauthorized("invalid token"))?;
            let y = URL_SAFE_NO_PAD
                .decode(jwk.y.as_deref().ok_or_else(|| AppError::unauthorized("invalid token"))?)
                .map_err(|_| AppError::unauthorized("invalid token"))?;
            if x.len() != 32 || y.len() != 32 {
                return Err(AppError::unauthorized("invalid token"));
            }
            let mut sec1 = Vec::with_capacity(65);
            sec1.push(0x04);
            sec1.extend_from_slice(&x);
            sec1.extend_from_slice(&y);
            let point = EncodedPoint::from_bytes(&sec1)
                .map_err(|_| AppError::unauthorized("invalid token"))?;
            let pub_key = Option::<P256PublicKey>::from(P256PublicKey::from_encoded_point(&point))
                .ok_or_else(|| AppError::unauthorized("invalid token"))?;
            let vk = P256VerifyingKey::from(&pub_key);
            let sig = P256Signature::from_slice(&sig_bytes)
                .map_err(|_| AppError::unauthorized("invalid token"))?;
            vk.verify(signing_input.as_bytes(), &sig)
                .map_err(|_| AppError::unauthorized("invalid token"))?;
        }
        "RS256" => {
            if jwk.kty != "RSA" {
                return Err(AppError::unauthorized("invalid token"));
            }
            let n_bytes = URL_SAFE_NO_PAD
                .decode(jwk.n.as_deref().ok_or_else(|| AppError::unauthorized("invalid token"))?)
                .map_err(|_| AppError::unauthorized("invalid token"))?;
            let e_bytes = URL_SAFE_NO_PAD
                .decode(jwk.e.as_deref().ok_or_else(|| AppError::unauthorized("invalid token"))?)
                .map_err(|_| AppError::unauthorized("invalid token"))?;
            let rsa_pub = RsaPublicKey::new(
                BigUint::from_bytes_be(&n_bytes),
                BigUint::from_bytes_be(&e_bytes),
            )
            .map_err(|_| AppError::unauthorized("invalid token"))?;
            let vk = RsaVerifyingKey::<Sha256>::new(rsa_pub);
            let sig = RsaSignature::try_from(sig_bytes.as_slice())
                .map_err(|_| AppError::unauthorized("invalid token"))?;
            vk.verify(signing_input.as_bytes(), &sig)
                .map_err(|_| AppError::unauthorized("invalid token"))?;
        }
        _ => return Err(AppError::unauthorized("invalid token")),
    }

    let claims_bytes = URL_SAFE_NO_PAD
        .decode(claims_b64)
        .map_err(|_| AppError::unauthorized("invalid token"))?;
    let claims: AccessClaims =
        serde_json::from_slice(&claims_bytes).map_err(|_| AppError::unauthorized("invalid token"))?;

    if claims.iss != expected_iss || claims.aud != expected_aud {
        return Err(AppError::unauthorized("invalid token"));
    }
    if claims.exp <= now_secs {
        return Err(AppError::unauthorized("invalid token"));
    }
    if let Some(nbf) = claims.nbf {
        if nbf > now_secs {
            return Err(AppError::unauthorized("invalid token"));
        }
    }
    Ok(claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn es256_sign_and_verify_roundtrip() {
        let kp = generate_es256_keypair().unwrap();
        let claims = AccessClaims {
            iss: "https://issuer.test".to_string(),
            aud: "admin".to_string(),
            sub: "user-1".to_string(),
            role: Role::Admin,
            jti: "jti-1".to_string(),
            iat: 1000,
            exp: 1900,
            nbf: None,
        };
        let token = sign_es256_jwt(&kp.kid, &kp.pkcs8_private_der, &claims).unwrap();
        let verified = verify_jwt_with_jwks(
            &token,
            &[kp.public_jwk.clone()],
            "https://issuer.test",
            "admin",
            1500,
        )
        .unwrap();
        assert_eq!(verified.sub, "user-1");
        assert_eq!(verified.role, Role::Admin);

        // Expired token rejected
        assert!(verify_jwt_with_jwks(
            &token,
            &[kp.public_jwk],
            "https://issuer.test",
            "admin",
            1900,
        )
        .is_err());
    }

    #[test]
    fn rejects_alg_none_and_hs256_confusion() {
        let kp = generate_es256_keypair().unwrap();
        let header = URL_SAFE_NO_PAD.encode(
            format!(r#"{{"alg":"none","typ":"JWT","kid":"{}"}}"#, kp.kid).as_bytes(),
        );
        let claims = URL_SAFE_NO_PAD.encode(
            br#"{"iss":"https://issuer.test","aud":"admin","sub":"u1","role":"owner","jti":"j1","iat":1,"exp":9999999999}"#,
        );
        let forged = format!("{header}.{claims}.");
        assert!(verify_jwt_with_jwks(
            &forged,
            &[kp.public_jwk],
            "https://issuer.test",
            "admin",
            100,
        )
        .is_err());
    }
}
```

### File: `backend-admin/src/lib.rs` (Temporary Task 03 compile stub; replaced in Task 05)
```rust
pub mod auth;
pub mod config;
pub mod error;
```

## 3. Verification Command
```bash
cd backend-admin && cargo test --lib && cargo check --target wasm32-unknown-unknown
```
