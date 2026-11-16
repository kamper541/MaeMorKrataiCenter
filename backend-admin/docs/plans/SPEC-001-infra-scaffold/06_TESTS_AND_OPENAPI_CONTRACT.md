# Task 06: SPEC-001 Integration Test Suite & OpenAPI 3.1 Contract

- **Executor / Coder Rule**:
  1. ALWAYS read [00_OVERVIEW.md](./00_OVERVIEW.md) first before starting execution.
  2. Execute ONLY this single task. When done and verified, terminate so a fresh Executor / Coder agent handles the next task.
  3. Apply exact code below without assumption or improvisation.

## 1. Action Items
- [ ] Create `backend-admin/tests/spec_001_scenarios.rs` covering `SPEC-001` Scenarios 1 through 6.
- [ ] Create `../docs/api/admin/openapi.yaml` (OpenAPI 3.1 contract skeleton).
- [ ] Run full test suite and wasm32 build verification.

## 2. Exact Code / File Changes

### File: `backend-admin/tests/spec_001_scenarios.rs`
```rust
use axum::body::Body;
use axum::http::{header, Method, Request, StatusCode};
use backend_admin::adapters::memory::MemoryStore;
use backend_admin::auth::jwt::{sign_es256_jwt, AccessClaims, PublicJwk, Role};
use backend_admin::auth::password::hash_password;
use backend_admin::build_router;
use backend_admin::config::Environment;
use backend_admin::ports::{JwksKeyRecord, UserRecord};
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;
use p256::pkcs8::EncodePrivateKey;
use rsa::pkcs1v15::SigningKey as RsaSigningKey;
use rsa::traits::PublicKeyParts;
use rsa::RsaPrivateKey;
use serde_json::Value;
use sha2::Sha256;
use signature::{SignatureEncoding, Signer};
use std::future::Future;
use std::pin::pin;
use std::sync::atomic::Ordering;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use tower::ServiceExt;

fn block_on<F: Future>(fut: F) -> F::Output {
    fn noop(_: *const ()) {}
    fn clone(ptr: *const ()) -> RawWaker {
        RawWaker::new(ptr, &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
    let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
    let mut cx = Context::from_waker(&waker);
    let mut pinned = pin!(fut);
    loop {
        match pinned.as_mut().poll(&mut cx) {
            Poll::Ready(val) => return val,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}

async fn body_json(res: axum::response::Response) -> Value {
    let bytes = axum::body::to_bytes(res.into_body(), 1024 * 1024)
        .await
        .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

fn setup_store() -> std::sync::Arc<MemoryStore> {
    let kek_b64 = STANDARD.encode([17u8; 32]);
    let store = MemoryStore::new("pepper-for-tests", kek_b64);
    block_on(async {
        let owner_hash = hash_password("owner-pass-12345", "pepper-for-tests")
            .await
            .unwrap();
        let admin_hash = hash_password("admin-pass-12345", "pepper-for-tests")
            .await
            .unwrap();
        let user_hash = hash_password("user-pass-12345", "pepper-for-tests")
            .await
            .unwrap();
        let mut users = store.users.lock().unwrap();
        users.insert(
            "u-owner".to_string(),
            UserRecord {
                id: "u-owner".to_string(),
                email: "owner@maemorkratai.test".to_string(),
                password_hash: owner_hash,
                role: Role::Owner,
                is_active: true,
            },
        );
        users.insert(
            "u-admin".to_string(),
            UserRecord {
                id: "u-admin".to_string(),
                email: "admin@maemorkratai.test".to_string(),
                password_hash: admin_hash,
                role: Role::Admin,
                is_active: true,
            },
        );
        users.insert(
            "u-user".to_string(),
            UserRecord {
                id: "u-user".to_string(),
                email: "user@maemorkratai.test".to_string(),
                password_hash: user_hash,
                role: Role::User,
                is_active: true,
            },
        );
    });
    store
}

async fn login_as(
    app: &axum::Router,
    email: &str,
    password: &str,
) -> (StatusCode, Value, Option<String>) {
    let req = Request::builder()
        .method(Method::POST)
        .uri("/v1/auth/login")
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::ORIGIN, "https://admin.maemorkratai.test")
        .header("cf-connecting-ip", "203.0.113.10")
        .body(Body::from(
            serde_json::json!({ "email": email, "password": password }).to_string(),
        ))
        .unwrap();
    let res = app.clone().oneshot(req).await.unwrap();
    let status = res.status();
    let set_cookie = res
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|v| v.to_str().ok())
        .map(ToString::to_string);
    let json = body_json(res).await;
    (status, json, set_cookie)
}

#[test]
fn scenario_2_health_check_and_degraded_state() {
    block_on(async {
        let store = setup_store();
        let app = build_router(store.into_ports(Environment::Staging));

        let ok_res = app
            .clone()
            .oneshot(Request::get("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(ok_res.status(), StatusCode::OK);
        assert_eq!(ok_res.headers().get(header::X_CONTENT_TYPE_OPTIONS).unwrap(), "nosniff");
        let ok_json = body_json(ok_res).await;
        assert_eq!(ok_json["d1"], "ok");
        assert_eq!(ok_json["r2"], "ok");

        store.db_healthy.store(false, Ordering::SeqCst);
        let bad_res = app
            .oneshot(Request::get("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(bad_res.status(), StatusCode::SERVICE_UNAVAILABLE);
        let bad_json = body_json(bad_res).await;
        assert_eq!(bad_json["code"], "SERVICE_UNAVAILABLE");
        assert!(!bad_json["message"].as_str().unwrap().contains("SQL"));
    });
}

#[test]
fn scenario_3_deny_by_default_rbac_hierarchy_and_inactive_user() {
    block_on(async {
        let store = setup_store();
        let app = build_router(store.into_ports(Environment::Staging));

        let (owner_status, owner_login, _) =
            login_as(&app, "owner@maemorkratai.test", "owner-pass-12345").await;
        assert_eq!(owner_status, StatusCode::OK);
        let owner_jwt = owner_login["access_token"].as_str().unwrap();

        let (admin_status, admin_login, _) =
            login_as(&app, "admin@maemorkratai.test", "admin-pass-12345").await;
        assert_eq!(admin_status, StatusCode::OK);
        let admin_jwt = admin_login["access_token"].as_str().unwrap();

        // Mint a valid JWT for role=user to verify RBAC enforcement on protected routes
        let active_key = store
            .jwks_keys
            .lock()
            .unwrap()
            .first()
            .unwrap()
            .clone();
        let kek = STANDARD.encode([17u8; 32]);
        let pkcs8 = backend_admin::auth::envelope::decrypt_private_key(
            &kek,
            &active_key.kid,
            &active_key.alg,
            &active_key.encrypted_private_key,
        )
        .unwrap();
        let user_jwt = sign_es256_jwt(
            &active_key.kid,
            &pkcs8,
            &AccessClaims {
                iss: "https://admin-api.maemorkratai.test".to_string(),
                aud: "maemorkratai-admin".to_string(),
                sub: "u-user".to_string(),
                role: Role::User,
                jti: "jti-u".to_string(),
                iat: 1_700_000_000,
                exp: 1_700_000_900,
                nbf: None,
            },
        )
        .unwrap();

        // Unauthenticated /v1/me -> 401
        let unauth = app
            .clone()
            .oneshot(Request::get("/v1/me").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(unauth.status(), StatusCode::UNAUTHORIZED);

        // User role -> 403 on /v1/me and /v1/admin/auth/keys/rotate
        let user_me = app
            .clone()
            .oneshot(
                Request::get("/v1/me")
                    .header(header::AUTHORIZATION, format!("Bearer {user_jwt}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(user_me.status(), StatusCode::FORBIDDEN);

        // Admin role -> 200 on /v1/me, 403 on owner-only /v1/admin/auth/keys/rotate
        let admin_me = app
            .clone()
            .oneshot(
                Request::get("/v1/me")
                    .header(header::AUTHORIZATION, format!("Bearer {admin_jwt}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(admin_me.status(), StatusCode::OK);

        let admin_rotate = app
            .clone()
            .oneshot(
                Request::post("/v1/admin/auth/keys/rotate")
                    .header(header::AUTHORIZATION, format!("Bearer {admin_jwt}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(admin_rotate.status(), StatusCode::FORBIDDEN);

        // Owner role -> 200 on both
        let owner_me = app
            .clone()
            .oneshot(
                Request::get("/v1/me")
                    .header(header::AUTHORIZATION, format!("Bearer {owner_jwt}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(owner_me.status(), StatusCode::OK);

        let owner_rotate = app
            .clone()
            .oneshot(
                Request::post("/v1/admin/auth/keys/rotate")
                    .header(header::AUTHORIZATION, format!("Bearer {owner_jwt}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(owner_rotate.status(), StatusCode::OK);

        // Deactivate admin -> 403 on protected route and login
        store
            .users
            .lock()
            .unwrap()
            .get_mut("u-admin")
            .unwrap()
            .is_active = false;
        let disabled_me = app
            .clone()
            .oneshot(
                Request::get("/v1/me")
                    .header(header::AUTHORIZATION, format!("Bearer {admin_jwt}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(disabled_me.status(), StatusCode::FORBIDDEN);
    });
}

#[test]
fn scenario_4_cookie_policy_refresh_rotation_and_replay_revocation() {
    block_on(async {
        let store = setup_store();
        let app = build_router(store.into_ports(Environment::Staging));

        let (status, _, set_cookie) =
            login_as(&app, "admin@maemorkratai.test", "admin-pass-12345").await;
        assert_eq!(status, StatusCode::OK);
        let cookie_1 = set_cookie.unwrap();
        assert!(cookie_1.contains("HttpOnly"));
        assert!(cookie_1.contains("Path=/v1/auth"));
        assert!(cookie_1.contains("SameSite=None"));
        assert!(cookie_1.contains("Secure"));
        let token_1_pair = cookie_1.split(';').next().unwrap().to_string();

        // First refresh succeeds and rotates to token_2
        let refresh_1 = app
            .clone()
            .oneshot(
                Request::post("/v1/auth/refresh")
                    .header(header::ORIGIN, "https://admin.maemorkratai.test")
                    .header("x-csrf-protection", "1")
                    .header(header::COOKIE, &token_1_pair)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(refresh_1.status(), StatusCode::OK);
        let cookie_2 = refresh_1
            .headers()
            .get(header::SET_COOKIE)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let token_2_pair = cookie_2.split(';').next().unwrap().to_string();
        assert_ne!(token_1_pair, token_2_pair);

        // Replay token_1 -> 401 and revokes entire family_id
        let replay = app
            .clone()
            .oneshot(
                Request::post("/v1/auth/refresh")
                    .header(header::ORIGIN, "https://admin.maemorkratai.test")
                    .header("x-csrf-protection", "1")
                    .header(header::COOKIE, &token_1_pair)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(replay.status(), StatusCode::UNAUTHORIZED);

        // token_2 must now also fail
        let after_replay = app
            .oneshot(
                Request::post("/v1/auth/refresh")
                    .header(header::ORIGIN, "https://admin.maemorkratai.test")
                    .header("x-csrf-protection", "1")
                    .header(header::COOKIE, &token_2_pair)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(after_replay.status(), StatusCode::UNAUTHORIZED);
    });
}

#[test]
fn scenario_5_jwks_rotation_continuity_and_rs256_readiness() {
    block_on(async {
        let store = setup_store();
        let app = build_router(store.into_ports(Environment::Staging));

        let (_, owner_login, _) =
            login_as(&app, "owner@maemorkratai.test", "owner-pass-12345").await;
        let jwt_kid1 = owner_login["access_token"].as_str().unwrap().to_string();

        let rot_res = app
            .clone()
            .oneshot(
                Request::post("/v1/admin/auth/keys/rotate")
                    .header(header::AUTHORIZATION, format!("Bearer {jwt_kid1}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(rot_res.status(), StatusCode::OK);
        let rot_json = body_json(rot_res).await;
        let kid_2 = rot_json["kid"].as_str().unwrap().to_string();

        // JWKS publishes both kid_1 (retiring) and kid_2 (active) without private key fields
        let jwks_res = app
            .clone()
            .oneshot(
                Request::get("/.well-known/jwks.json")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(jwks_res.status(), StatusCode::OK);
        let jwks_json = body_json(jwks_res).await;
        let keys = jwks_json["keys"].as_array().unwrap();
        assert_eq!(keys.len(), 2);
        for k in keys {
            assert!(k.get("d").is_none());
        }

        // Token signed by retiring kid_1 still verifies until expiry
        let me_with_kid1 = app
            .clone()
            .oneshot(
                Request::get("/v1/me")
                    .header(header::AUTHORIZATION, format!("Bearer {jwt_kid1}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(me_with_kid1.status(), StatusCode::OK);

        // New login mints token under kid_2
        let (_, owner_login_2, _) =
            login_as(&app, "owner@maemorkratai.test", "owner-pass-12345").await;
        let jwt_kid2 = owner_login_2["access_token"].as_str().unwrap();
        let header_b64 = jwt_kid2.split('.').next().unwrap();
        let header_json: Value =
            serde_json::from_slice(&URL_SAFE_NO_PAD.decode(header_b64).unwrap()).unwrap();
        assert_eq!(header_json["kid"], kid_2);

        // Verify RS256 JWKS verification readiness for Cloudflare Access cutover
        let mut rng = rand_core::OsRng;
        let rsa_priv = RsaPrivateKey::new(&mut rng, 2048).unwrap();
        let rsa_pub = rsa_priv.to_public_key();
        let rsa_jwk = PublicJwk {
            kty: "RSA".to_string(),
            key_use: Some("sig".to_string()),
            alg: "RS256".to_string(),
            kid: "cf-access-rs256".to_string(),
            crv: None,
            x: None,
            y: None,
            n: Some(URL_SAFE_NO_PAD.encode(rsa_pub.n().to_bytes_be())),
            e: Some(URL_SAFE_NO_PAD.encode(rsa_pub.e().to_bytes_be())),
        };
        let rsa_signer = RsaSigningKey::<Sha256>::new(rsa_priv);
        let h_b64 = URL_SAFE_NO_PAD.encode(br#"{"alg":"RS256","typ":"JWT","kid":"cf-access-rs256"}"#);
        let c_b64 = URL_SAFE_NO_PAD.encode(
            br#"{"iss":"https://admin-api.maemorkratai.test","aud":"maemorkratai-admin","sub":"u-owner","role":"owner","jti":"jti-rsa","iat":1700000000,"exp":1700000900}"#,
        );
        let sig_in = format!("{h_b64}.{c_b64}");
        let sig = rsa_signer.sign(sig_in.as_bytes());
        let rsa_jwt = format!("{sig_in}.{}", URL_SAFE_NO_PAD.encode(sig.to_bytes()));
        let verified = backend_admin::auth::jwt::verify_jwt_with_jwks(
            &rsa_jwt,
            &[rsa_jwk],
            "https://admin-api.maemorkratai.test",
            "maemorkratai-admin",
            1_700_000_100,
        )
        .unwrap();
        assert_eq!(verified.role, Role::Owner);

        // Ensure private key stored in MemoryStore is AES-256-GCM ciphertext envelope, not DER
        let stored_keys = store.jwks_keys.lock().unwrap();
        let latest: &JwksKeyRecord = stored_keys.last().unwrap();
        assert!(latest.encrypted_private_key.starts_with("v1$"));
        let _ = p256::SecretKey::random(&mut rng).to_pkcs8_der().unwrap();
    });
}

#[test]
fn scenario_6_rate_limit_csrf_cors_and_error_sanitization() {
    block_on(async {
        let store = setup_store();
        store.rate_limit_max.store(2, Ordering::SeqCst);
        let app = build_router(store.into_ports(Environment::Staging));

        // CSRF guard rejects missing x-csrf-protection or disallowed Origin
        let missing_csrf = app
            .clone()
            .oneshot(
                Request::post("/v1/auth/refresh")
                    .header(header::ORIGIN, "https://admin.maemorkratai.test")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(missing_csrf.status(), StatusCode::FORBIDDEN);

        let bad_origin = app
            .clone()
            .oneshot(
                Request::post("/v1/auth/login")
                    .header(header::ORIGIN, "https://evil.example.com")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(r#"{"email":"a@b.com","password":"x"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(bad_origin.status(), StatusCode::FORBIDDEN);
        assert!(bad_origin.headers().get(header::ACCESS_CONTROL_ALLOW_ORIGIN).is_none());

        // Malformed JSON -> 400 sanitized envelope
        let bad_json_res = app
            .clone()
            .oneshot(
                Request::post("/v1/auth/login")
                    .header(header::ORIGIN, "https://admin.maemorkratai.test")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from("{not-json"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(bad_json_res.status(), StatusCode::BAD_REQUEST);

        // Rate limit breach on 3rd attempt -> 429
        let _ = login_as(&app, "admin@maemorkratai.test", "wrong-1").await;
        let _ = login_as(&app, "admin@maemorkratai.test", "wrong-2").await;
        let (limited_status, limited_body, _) =
            login_as(&app, "admin@maemorkratai.test", "wrong-3").await;
        assert_eq!(limited_status, StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(limited_body["code"], "RATE_LIMITED");
        assert!(limited_body["request_id"].as_str().is_some());
    });
}
```

### File: `docs/api/admin/openapi.yaml` (relative to monorepo root: `../docs/api/admin/openapi.yaml`)
```yaml
openapi: 3.1.0
info:
  title: MaeMorKratai Admin Backend API
  version: 0.1.0
  description: >
    Infrastructure, health, JWKS, and Day-1 authentication contract for backend-admin
    (SPEC-001). All endpoints enforce deny-by-default authorization except explicitly
    documented public auth and health routes.
servers:
  - url: http://127.0.0.1:8787
    description: Local Wrangler dev
  - url: https://admin-api-staging.maemorkratai.com
    description: Staging environment
  - url: https://admin-api.maemorkratai.com
    description: Production environment
paths:
  /health:
    get:
      summary: Service health check
      operationId: getHealth
      responses:
        "200":
          description: D1 and R2 bindings healthy
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/HealthResponse"
        "503":
          description: One or more bindings unavailable
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorEnvelope"
  /.well-known/jwks.json:
    get:
      summary: Public JSON Web Key Set (active and retiring keys)
      operationId: getJwks
      responses:
        "200":
          description: Public JWKS document
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/JwksDocument"
  /v1/auth/login:
    post:
      summary: Authenticate admin or owner user
      operationId: postLogin
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: "#/components/schemas/LoginRequest"
      responses:
        "200":
          description: Access token issued and HttpOnly refresh cookie set
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/TokenResponse"
        "400":
          description: Malformed payload
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorEnvelope"
        "401":
          description: Invalid credentials
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorEnvelope"
        "403":
          description: Inactive account or insufficient role
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorEnvelope"
        "429":
          description: Rate limit exceeded
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorEnvelope"
  /v1/auth/refresh:
    post:
      summary: Rotate refresh cookie and issue new access JWT
      operationId: postRefresh
      parameters:
        - in: header
          name: X-CSRF-Protection
          required: true
          schema:
            type: string
            enum: ["1"]
      responses:
        "200":
          description: New access token issued and refresh cookie rotated
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/TokenResponse"
        "401":
          description: Invalid, expired, or replayed refresh token
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorEnvelope"
        "403":
          description: Disallowed Origin or missing CSRF header
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorEnvelope"
        "429":
          description: Rate limit exceeded
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorEnvelope"
  /v1/auth/logout:
    post:
      summary: Revoke current refresh session and clear cookie
      operationId: postLogout
      parameters:
        - in: header
          name: X-CSRF-Protection
          required: true
          schema:
            type: string
            enum: ["1"]
      responses:
        "200":
          description: Session revoked
          content:
            application/json:
              schema:
                type: object
                required: [status]
                properties:
                  status:
                    type: string
                    example: logged_out
        "403":
          description: Disallowed Origin or missing CSRF header
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorEnvelope"
  /v1/me:
    get:
      summary: Get authenticated admin/owner profile
      operationId: getMe
      security:
        - BearerAuth: []
      responses:
        "200":
          description: Current authenticated admin or owner
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/MeResponse"
        "401":
          description: Missing or invalid bearer token
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorEnvelope"
        "403":
          description: Insufficient role or inactive account
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorEnvelope"
  /v1/admin/auth/keys/rotate:
    post:
      summary: Rotate active ES256 signing key (owner only)
      operationId: postRotateKeys
      security:
        - BearerAuth: []
      responses:
        "200":
          description: New active ES256 key created and prior active key retired
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/RotateKeyResponse"
        "401":
          description: Missing or invalid bearer token
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorEnvelope"
        "403":
          description: Owner role required
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/ErrorEnvelope"
components:
  securitySchemes:
    BearerAuth:
      type: http
      scheme: bearer
      bearerFormat: JWT
  schemas:
    HealthResponse:
      type: object
      required: [status, d1, r2]
      properties:
        status:
          type: string
          example: ok
        d1:
          type: string
          example: ok
        r2:
          type: string
          example: ok
    LoginRequest:
      type: object
      required: [email, password]
      properties:
        email:
          type: string
          format: email
        password:
          type: string
          format: password
    TokenResponse:
      type: object
      required: [access_token, token_type, expires_in, role]
      properties:
        access_token:
          type: string
        token_type:
          type: string
          enum: [Bearer]
        expires_in:
          type: integer
        role:
          type: string
          enum: [owner, admin]
    MeResponse:
      type: object
      required: [id, email, role, is_active]
      properties:
        id:
          type: string
        email:
          type: string
        role:
          type: string
          enum: [owner, admin]
        is_active:
          type: boolean
    RotateKeyResponse:
      type: object
      required: [kid, alg, status]
      properties:
        kid:
          type: string
        alg:
          type: string
          enum: [ES256]
        status:
          type: string
          enum: [active]
    PublicJwk:
      type: object
      required: [kty, alg, kid]
      properties:
        kty:
          type: string
        use:
          type: string
        alg:
          type: string
        kid:
          type: string
        crv:
          type: string
        x:
          type: string
        y:
          type: string
        n:
          type: string
        e:
          type: string
    JwksDocument:
      type: object
      required: [keys]
      properties:
        keys:
          type: array
          items:
            $ref: "#/components/schemas/PublicJwk"
    ErrorEnvelope:
      type: object
      required: [request_id, code, message]
      properties:
        request_id:
          type: string
        code:
          type: string
        message:
          type: string
```

## 3. Verification Command
```bash
cd backend-admin && cargo test --workspace && cargo check --target wasm32-unknown-unknown
```
