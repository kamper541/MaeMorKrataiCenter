---
id: RES-001
topic: "Backend caching & cross-Worker cache invalidation for tarot master data (D1)"
status: COMPLETED # DRAFT | IN_PROGRESS | COMPLETED
created_at: "2026-11-20"
investigator: "Researcher"
target_spec: "SPEC-002 (tarot master data in D1). Not yet written; expected at docs/client/specs/ and docs/admin/specs/"
---

# Research Spike: Backend caching & cross-Worker cache invalidation for tarot master data

## 1. Executive Summary & Recommendation for Spectre

**The problem.** The data is small (78 cards, ~400 KB of daily-reading content, a few taxonomy lists) and rarely changes. The **client** Worker reads it and the **admin** Worker writes it, both against the same D1 database (D02). The developer wants:
1. a backend cache in front of D1,
2. prompt invalidation of the client's cache when the admin writes, and
3. browser caching with ETag/304.

Two facts from the repo shape the options:
- Neither `client/wrangler.toml` nor `admin/wrangler.toml` declares `routes`, `custom_domain` or `workers_dev`. **Both Workers deploy to `*.workers.dev` today.** The `*.maemorkratai.com` hostnames only appear inside `vars`.
- The decisions run on the **Workers Free plan** (D03: 10 ms CPU per request), with 100k requests/day and 5M D1 rows read/day.

**Three Cloudflare cache layers, none of them a cross-Worker, globally consistent cache on its own:**
- **Cache API (`caches.default`).** Per data center. `cache.delete` purges only the local data center. It only works on custom domains.
- **Zone purge (by URL, tag or prefix).** Now on all plans, including Free (5 tag/prefix purge requests per minute). It needs a zone (custom domain) and an API token held as a secret.
- **Workers Cache** (launched July 2026, all plans, works on workers.dev). A tiered cache that sits *in front of* the Worker, with global purge by tag. However:
  - Purge can only be called by the **same Worker and the same entrypoint**. The admin Worker cannot purge it directly.
  - workers-rs 0.9.0 has **no binding** for it.
  - Cache hits skip the Worker entirely, so the D04 rate limiter, CORS and request-id middleware do not run on hits.

Workers KV is ruled out for invalidation: writes take "up to 60 seconds or more" to appear in other locations.

**Recommendation (developer decides): Option A, version-key + ETag/304, with the Cache API as an optional per-data-center layer.**
- **How it works.** A single-row `content_version` in D1 is bumped in the *same* `batch()` as every admin write. The client reads that one row by primary key on each request:
  - If the client's ETag matches, it returns `304`.
  - Otherwise it serves from `caches.default` under a key that includes the version (only once a custom domain is attached), or falls back to D1.
- **What it costs.** No new bindings, secrets, plans or unsupported Rust APIs.
- **Invalidation.** Immediate, because the next request sees the new version, and nothing has to be purged across Workers.
- **The trade-off.** It only **partly** meets requirement 1: identical requests still make one primary-key D1 read (1 row read) instead of the full query.
- **Upgrade path.** If traffic grows or Free-plan request/CPU budgets become tight, layer **Option B** (Workers Cache plus an admin→client purge over a service binding) on top. Its revalidation reuses the same ETag/version contract, so nothing built for A is thrown away.
- **Option C** (zone purge API with a token) is **not recommended**. It adds a long-lived API secret and a hard dependency on a custom domain to get a per-data-center cache that A's version key already makes correct without purging.

## 2. Research Intent & Live Web Search Queries
- **Investigation Objective**: How should the client Worker cache public GETs of D1 master data so that admin writes, made in a *different* Worker, invalidate the cache promptly? Also, which HTTP caching headers suit this API? The answer must be verified for the Free plan, `*.workers.dev`, and Rust workers-rs.
- **Live Web Search Queries** (exact):
  - `"Cloudflare Workers Cache API workers.dev no-op custom domain required cache.put"`
  - `"Cloudflare instant purge latency 150ms cache tags all plans free"`
  - `"workers-rs "Workers Cache" OR "cache.purge" OR "ctx.cache" rust worker crate 2026"`
- **Pages fetched directly** (live, 2026-11-20): the Cache API runtime docs (HTML and `index.md`), How the cache works, Purge cache, the Purge API reference, Workers Cache (overview, purge, limitations, cache keys, configuration, examples), the Workers Cache `invalidate()` changelog, the Workers Cache launch blog, KV "How KV works", KV limits, D1 limits, D1 pricing, D1 read replication, Workers limits, Workers pricing, service bindings, the `worker` crate `Cache` docs, the workers-rs releases page, and MDN Cache-Control. See §6.

## 3. Current State of the Art & Modern Best Practices (2026)

### 3.1 Cache API (`caches.default` / `caches.open`) [1][2][3][4]
- **Per data center.** "The Cache API is available globally but the contents of the cache do not replicate outside of the originating data center." [1]
- **`cache.delete` purges only the local data center.** "The `cache.delete` method only purges content of the cache in the data center that the Worker was invoked." [1] The admin Worker therefore **cannot** invalidate the client's entries globally with `cache.delete`.
- **Custom domains only.** The current docs say: "Workers deployed to custom domains have access to functional `cache` operations." [2] Earlier doc revisions and Cloudflare community answers say operations on `*.workers.dev` have no effect, because the cache is zone-scoped [3]. **This is a no-op for our Workers until a custom domain is attached.**
- **Other restrictions.**
  - It is not compatible with Tiered Cache.
  - `stale-while-revalidate` is not supported.
  - Responses with `Set-Cookie` are never stored. [1]
- **Global purge of Cache API entries.** This needs a zone purge (§3.2). Cache-Tag is the recommended method, because a URL "that uses a custom cache key set by a Worker" cannot be purged by URL [4].
- **workers-rs support.** `worker` 0.9.0 exposes `Cache::default()`, `Cache::open(name)`, `put`, `get` and `delete` [12]. This is supported.

### 3.2 Zone cache purge (URL / tag / prefix / hostname / everything) [5][6][7]
- **All plans.** Since April 2025, "all purge methods [are] available to all customers", including Free [6]. The Free plan rate limit is **5 requests/minute** (bucket of 25) for tag/prefix/host/everything, and 800 URLs/s for single-file purges. Each request carries up to 100 operations [5].
- **Latency.** Cloudflare's "Instant Purge" reports a global P50 under **150 ms** [7]. That figure is a benchmark, not an SLA. A `200` from the API only means the purge was accepted [5].
- **API.** `POST /zones/{zone_id}/purge_cache` with `files`, `tags`, `prefixes`, `hosts` or `purge_everything`. The token needs the **Cache Purge** permission [8].
- **Can a Worker call it?** Yes, as an ordinary `fetch` to `api.cloudflare.com` with the token stored as a Wrangler Secret.
- **Security cost.** A long-lived account credential lives in the admin Worker. If it leaks, an attacker can purge the zone at will, which turns into a D1 load and cost amplification attack. It needs rotation and least-privilege scoping to that one zone.
- **Not useful today.** Zone purges do **not** affect Workers Cache content [10], and they need the client to be on a zone (custom domain).

### 3.3 Workers Cache (new, July 2026) [9][10][11][13][14][15][16]
- **What it is.** A tiered, read-through cache *in front of* the Worker. It is enabled with `[cache] enabled = true` (Wrangler ≥ 4.69.0) and driven by standard `Cache-Control`.
  - A hit does not run the Worker: no CPU and no D1. It still counts as a request.
  - It is "available today to every Worker on any plan", and "The cache works on `workers.dev`" [16].
  - It supports `stale-while-revalidate` and `Vary` [9][16].
- **Purge and invalidate.** `ctx.cache.purge({ tags | pathPrefixes | purgeEverything })` and `ctx.cache.invalidate()` [10][14].
  - Purges "propagate globally with the same guarantees as zone-level purges" [10].
  - They are always rate-limited at **Free-tier** purge limits [10][11].
  - `invalidate()` (2026-09-29) marks entries stale. The cache then sends the Worker a conditional request built from the stored `ETag`, and a `304` keeps the cached body [14].
- **Scope (critical).**
  - "A Worker cannot reach into another Worker's cache" [10].
  - Within a Worker, purges are "scoped to the entrypoint that called purge()" [10].
  - The documented write-then-purge pattern runs inside the same Worker [15].
  - **For us:** the admin Worker must ask the client Worker to purge itself. The practical route is a service-binding `fetch` (POST, so it is never cached) to the client's default `fetch` entrypoint, which then calls `cache.purge`. Service-binding calls "do not incur additional request fees" [18][19].
- **Cache key** [13].
  - It includes the path, query string (order-sensitive), entrypoint and **Worker version** (on by default, so the hit rate resets on every deploy; `cross_version_cache = true` changes that).
  - It excludes the hostname and request headers unless they are listed in `Vary`. The client already emits `Vary: Origin` (`client/src/middleware/security.rs`), which keeps the cached `Access-Control-Allow-Origin` separate for `maemorkratai.com` and `www.maemorkratai.com`.
- **Limitations** [11].
  - Local tests don't exercise it ("Test on a Preview or deployed Worker").
  - There is no pre-warming.
  - Response size is temporarily capped at the Free-plan size.
  - Zone Cache Rules don't apply.
- **workers-rs support: none.** The workers-rs releases (latest v0.9.0, 11 Nov) mention no Workers Cache, `ctx.cache` or purge API [17]. Calling `cache.purge` from Rust would need a hand-written `wasm_bindgen` import of `cache` from `cloudflare:workers`. This is **plausible but unverified (needs a spike)**. Enabling the cache itself needs no Rust code (Wrangler config plus headers).

### 3.4 Workers KV [20][21]
- **Eventually consistent.** "Changes may take up to 60 seconds or more to be visible in other global network locations." [20] The minimum `cacheTtl` is 30 s [21].
- **Free plan.** 100,000 reads/day and 1,000 writes/day. Values can be up to 25 MiB, and one key accepts 1 write/s [21]. Both Workers can bind the same namespace.
- **Verdict.** It fails requirement 2 ("promptly"), whether used as a data cache or as a version store. It also adds a binding and duplicates D1. **Rejected.**

### 3.5 D1 as the version store [22][23][24]
- **Free plan.** 5M rows read/day, 100k rows written/day, 50 queries per invocation, 500 MB per database. When a daily limit is exceeded, queries error until 00:00 UTC [22][23].
- **Consistency.** D1 latency depends on distance to the primary. Reads from the primary are strongly consistent, so a version bump is visible to the next request.
- **If read replication is enabled later**, version reads must use the Sessions API or primary-first consistency, or they could return a stale version [24]. workers-rs 0.8.0+ has a typed D1 session API [17].
- **Budget check.** A 1-row primary-key read per request uses at most 100k rows/day at the Workers request cap. That is 2% of the D1 read budget. Without caching, a full-card list (78 or more rows) per request could exhaust 5M rows/day at about 64k requests.

### 3.6 HTTP caching best practice (browser / TanStack Query) [25]
- For data that must reflect admin edits quickly, use `Cache-Control: no-cache` plus a strong `ETag`. The browser revalidates each time and gets `304 Not Modified` with no body when nothing changed. "`no-cache` does not mean 'don't cache'." [25]
- Optionally add a small `max-age` (for example 60 s) with `stale-while-revalidate`, if a short staleness window is acceptable. TanStack Query's own `staleTime` covers in-app freshness, so the frontend does not need a long HTTP `max-age`.
- **Shared caches.** If Workers Cache is used (Option B), use `s-maxage` or `public` to control the edge cache independently of the browser `max-age`.
- **ETag correctness.**
  - Build it from `content_version` **plus a response-schema/code version** (for example `"v{schema}-{content_version}-{resource}"`). Otherwise a deploy that changes the JSON shape would return a `304` to clients holding the old shape.
  - Compare `If-None-Match` using the weak comparison for GET.
- **Repo constraints.**
  - D04 currently mandates `Cache-Control: no-store` on dynamic responses. `client/src/middleware/security.rs` forces `no-store` except where `preserve_cache` is set.
  - Errors and 404s **must stay `no-store`**, because heuristic freshness could otherwise cache a 404.
  - A new decision (D05) should formally carve out the cacheable master-data GETs.

### Deprecations / anti-patterns to avoid
- Using `cache.delete` from the admin Worker to "invalidate" the client: it is local to one data center and lives in another Worker's request context.
- Relying on `caches.default` on `*.workers.dev`: it does nothing.
- Using KV as the source of truth for "has the data changed?": up to 60 s or more of staleness.
- Weak `max-age` with no validator: clients stay stale until expiry, and there is no cheap 304 path.
- Caching error responses or responses carrying `Set-Cookie`.

## 4. Options & Technology Comparison Matrix

### Option A: Version key in D1 + ETag/304 (+ optional Cache API keyed by version) **(Recommended)**
1. Add a migration for `content_meta(id INTEGER PRIMARY KEY CHECK (id = 1), content_version INTEGER NOT NULL, updated_at ...)`.
2. Every admin mutation runs `DB.batch([mutation..., UPDATE content_meta SET content_version = content_version + 1 ...])` so the bump commits atomically with the data.
3. Each client GET:
   - Middleware (rate limit and CORS still run) reads `content_version` (1 row).
   - It computes the ETag. If it matches `If-None-Match`, it returns `304`.
   - Otherwise it looks up `caches.default` with the synthetic key `https://cache.internal/v{schema}/{content_version}{path}?{canonical_query}`. **This step is a no-op until a custom domain exists, so the code path degrades gracefully.**
   - On a miss it queries D1, serializes, `put`s the response (with no `Set-Cookie`) and returns `200` with `Cache-Control: no-cache` and the `ETag`.
4. Old-version entries are never purged. They age out by their own `max-age`.

Optional: a per-isolate in-memory memo of the serialized payload keyed by `content_version`. It is safe because the version check still runs each request, and it saves the D1 query and serialization CPU when an isolate is warm.

### Option B: Workers Cache on the client + admin-triggered purge over a service binding
1. Client `wrangler.toml`: `[cache] enabled = true`.
2. Cacheable GETs send `Cache-Control: public, max-age=0, s-maxage=<N>`, `Cache-Tag: tarot`, `Vary: Origin` and an `ETag`. The client's conditional-request path from A answers revalidations with a cheap `304`.
3. Admin `wrangler.toml` adds a `[[services]]` binding to the client Worker. After a successful D1 write, the admin sends `POST /internal/cache/purge {tags:["tarot"]}` over the binding.
4. The client handles that route on the **default entrypoint**, which is required because purge is entrypoint-scoped. It authorizes the call and then invokes `cache.purge({tags})` through a **custom wasm-bindgen import (spike needed)**.
   - **Authorization.** The client already ships `JwksVerifier` (SPEC-001; `JWKS_URL`, `JWT_ISSUER`, `JWT_AUDIENCE=maemorkratai-client`). The admin can mint a short-lived ES256 JWT with that audience, so **no new shared secret** is needed. The route is still publicly addressable, so it must fail closed.
   - **Fallback.** If the purge call fails, staleness is bounded by `s-maxage`. The admin should log it to `audit_events` and retry.

**Variant B-lite: Workers Cache with no purge at all.** Use a short `s-maxage` (for example 30–60 s) and rely on expiry and ETag revalidation. There is no cross-Worker coupling and no Rust purge binding. Invalidation latency equals `s-maxage`.

### Option C: Zone cache (Cache API, custom domain) + Cloudflare purge-by-tag REST API from the admin Worker
The client puts responses into `caches.default` (custom domain required) with `Cache-Tag: tarot`. The admin calls `POST /zones/{zone}/purge_cache {tags:["tarot"]}` using a `CF_PURGE_TOKEN` Wrangler Secret (Cache Purge permission, scoped to one zone). There is a Free-plan limit of 5 tag purges per minute.

### Matrix (gb-sdlc priorities: 1 Security, 2 Low tech debt, 3 Speed)

| Option | Security posture (OWASP) | Maintenance & license | Tech debt & simplicity | Invalidation latency | Extra infra / bindings / secrets | Plan requirement | workers-rs support | Meets req. 1 / 2 / 3 | Status |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **A. D1 version key + ETag/304 (+ optional Cache API)** | **Strong.** Every request still passes D04 middleware (rate limit, CORS, request-id). No new secrets or endpoints. | First-party Cloudflare (D1, Cache API); workers-rs Apache-2.0/MIT | **Lowest.** 1 table, 1 batch rule, 1 header helper. Works identically in local, staging and prod (the Cache API layer is simply inert on workers.dev). | **Immediate** (next request; D1 primary is strongly consistent) | None (migration only) | Free | **Full** (`D1Database::batch`, `Cache` in `worker` 0.9.0) | 1: **partial** (1 PK row/request, full query only on miss) · 2: yes · 3: yes | **Recommended** |
| **B. Workers Cache + admin→client purge (service binding)** | **Moderate.** Hits bypass the Worker, so the D04 rate limiter, CORS logic and request-id don't run on hits (cached headers are replayed). A publicly addressable purge route must be JWT-guarded. A cache-poisoning surface exists if `Vary`/no-store-on-error is missed. | First-party, but launched July 2026 (young); JS/TS-only API surface | **Higher.** Hand-rolled wasm-bindgen purge import (unverified). Cross-Worker coupling. New D05 carve-out from D04. Can't be tested locally (Preview only). Hit rate resets per deploy unless `cross_version_cache`. | ~global purge (Cloudflare cites P50 <150 ms for zone purges; same guarantees). If the purge fails, bounded by `s-maxage`. | `[cache]` block (Wrangler ≥4.69), `[[services]]` binding admin→client, client purge route. No new secret if the existing JWKS verifier is reused. | Any plan (Free OK); purge always Free-tier rate-limited | **Partial.** Enabling: config only. Purge: **no binding, spike required**. | 1: **full** (hits skip Worker and D1, zero CPU) · 2: yes · 3: yes | Viable upgrade path (or B-lite) |
| **B-lite. Workers Cache, `s-maxage` + ETag revalidation, no purge** | Moderate (same hit-bypass caveat as B, but no purge route) | As B | Low to medium. Config plus headers; no Rust bindings needed | **= `s-maxage`** (e.g. 30–60 s) | `[cache]` block only | Any | **Full** (no purge call) | 1: full · 2: **not "prompt"** · 3: yes | Alternative if ≤60 s staleness is acceptable |
| **C. Cache API + zone purge-by-tag REST API** | **Weaker.** A long-lived account API token sits in the admin Worker; a leak enables arbitrary zone purges (D1 load amplification). | First-party REST API | Medium-high. Needs a custom domain/zone first. Token rotation. External API call in the write path. Per-data-center cache means low hit rate at low traffic. Rate limit 5/min on Free. | P50 <150 ms globally (benchmark, not SLA) | `CF_PURGE_TOKEN` secret, zone ID var, custom domain/route | Free OK (5 tag purges/min) | Full (`Cache` + `Fetch`) | 1: partial (per-DC) · 2: yes · 3: yes | **Rejected** (A gives the same correctness without a secret) |
| KV as cache / version store | Neutral | First-party | Adds a binding, duplicates D1 | **Up to 60 s or more** (eventual) | KV namespace bound to both Workers | Free (100k reads, 1k writes/day) | Full | 2: **no** | **Rejected** |
| Durable Object as coordinator | Neutral | First-party (SQLite DOs on Free) | Overkill for rarely-changing ~400 KB data; another class to deploy and test | Immediate | DO binding + migration | Free (SQLite-backed only) | Partial (DO support exists; adds complexity) | n/a | **Rejected** (not clearly relevant) |

## 5. Security & Risk Analysis
- **Vulnerability history.** None of these are third-party crates; they are all Cloudflare platform primitives. Workers Cache is new (GA July 2026, `invalidate()` added September 2026), so expect behavior changes. Re-verify before implementing B.
- **Authentication / authorization.**
  - Option A adds no new trust boundary.
  - Option B adds a mutation route on the *public* client Worker. The service binding does not make it private, because the default entrypoint also serves the internet. It must therefore verify an admin-issued, short-lived JWT (`aud=maemorkratai-client`, a dedicated claim or role such as `cache:purge`) via the existing `JwksVerifier`, and fail closed. A separate non-default entrypoint is **not** an option, because purges are entrypoint-scoped.
  - Option C stores an API token. If chosen, scope it to `Zone > Cache Purge` on the single zone and rotate it.
- **Data protection and cache poisoning.**
  - Cache only anonymous, client-safe master data.
  - Never cache responses carrying `Set-Cookie`, errors or 404s (keep `no-store`).
  - Canonicalize query strings before using them as cache keys, or reject unknown params, so attackers cannot fragment the cache or bypass it to load D1.
  - Keep `Vary: Origin` because the CORS allow-origin header is echoed per origin.
- **D04 conformance.**
  - A cacheable GET needs an explicit D05 decision that overrides the `no-store` default for those routes. `preserve_cache` already exists in `client/src/middleware/security.rs`.
  - Under Option B, cached hits are **not** rate-limited per IP by the Worker. That is acceptable because a hit costs no CPU or D1, but each hit still counts toward the 100k/day Free request limit, so edge abuse can still exhaust it. Consider a Cloudflare WAF rate-limiting rule once a custom domain exists.
- **CPU budget (all options).** Serializing ~400 KB of JSON in wasm within the Free plan's 10 ms CPU cap is the real cost on a miss. Measure it in staging. Consider splitting the daily-reading payload per card or per day, or pre-serializing JSON into D1 at admin-write time, so a miss is a single blob read.
- **Operational risk.**
  - Option A: if the version bump is not in the same `batch()` as the write, clients can get a `304` for changed data. Enforce this in one admin repository helper and test it.
  - Option B: a failed purge leaves data stale for up to `s-maxage`. Keep `s-maxage` modest (≤ 300 s) as a safety net.

## 6. References & Verified URLs
- [1] [Cache API (Workers runtime APIs)](https://developers.cloudflare.com/workers/runtime-apis/cache/): per-data-center scope; `cache.delete` purges only the local data center; not compatible with tiered cache; no SWR; `Set-Cookie` never cached.
- [2] [Cache API, markdown source](https://developers.cloudflare.com/workers/runtime-apis/cache/index.md): "Workers deployed to custom domains have access to functional `cache` operations."
- [3] [Cloudflare Community: "CloudFlare worker cache does not work"](https://community.cloudflare.com/t/cloudflare-worker-cache-does-not-work/298125): Cache API is not enabled on workers.dev; deploy to a custom domain.
- [4] [How the cache works (Workers)](https://developers.cloudflare.com/workers/reference/how-the-cache-works/): global purge of Cache API entries via zone purge; use Cache-Tag; URLs with Worker-custom cache keys cannot be URL-purged.
- [5] [Purge cache (how-to)](https://developers.cloudflare.com/cache/how-to/purge-cache/): all purge methods on all plans; Free plan rate limit of 5 req/min for tag/prefix/host; a 200 only means accepted.
- [6] [Changelog 2025-04-01: purge for all](https://developers.cloudflare.com/changelog/2025-04-01-purge-for-all) and [blog: Instant Purge for all](https://blog.cloudflare.com/instant-purge-for-all/): every purge method on every plan.
- [7] [Instant Purge: invalidating cached content in under 150ms](https://blog.cloudflare.com/instant-purge/): P50 global purge latency below 150 ms (benchmark).
- [8] [API reference: Purge cached content](https://developers.cloudflare.com/api/resources/cache/methods/purge/): `POST /zones/{zone_id}/purge_cache`; body fields; "Cache Purge" token permission.
- [9] [Workers Cache overview](https://developers.cloudflare.com/workers/cache/): cache in front of the Worker; `[cache] enabled = true`; hits don't run the Worker; `Set-Cookie`/`Authorization`/`private` bypass; no separate pricing.
- [10] [Workers Cache: purge and invalidate](https://developers.cloudflare.com/workers/cache/purge/): "A Worker cannot reach into another Worker's cache"; entrypoint-scoped; zone purges don't affect it; Free-tier purge limits.
- [11] [Workers Cache: limitations](https://developers.cloudflare.com/workers/cache/limitations/): not exercised by local tests; Free-plan response size cap; no pre-warming; separate from the Cache API.
- [12] [docs.rs `worker::Cache` (worker 0.9.0)](https://docs.rs/worker/latest/worker/struct.Cache.html): `default`, `open`, `put`, `get`, `delete` available in Rust.
- [13] [Workers Cache: cache keys](https://developers.cloudflare.com/workers/cache/cache-keys/): path, query and version in the key; hostname excluded; `Vary` for headers; `cross_version_cache`.
- [14] [Changelog 2026-09-29: Workers Cache invalidate()](https://developers.cloudflare.com/changelog/post/2026-09-29-workers-cache-invalidate/): soft invalidation; conditional request with `If-None-Match`; `304` keeps the stored body.
- [15] [Workers Cache: examples](https://developers.cloudflare.com/workers/cache/examples/): write-then-purge only within the same Worker (Durable Object leaderboard); no cross-Worker example.
- [16] [Blog: Your Worker can now have its own cache in front of it (2026-07-06)](https://blog.cloudflare.com/workers-cache/): any plan; works on workers.dev; hits bill no CPU but count as requests; examples are JS/TS only.
- [17] [cloudflare/workers-rs releases](https://github.com/cloudflare/workers-rs/releases): v0.9.0 (11 Nov) latest; no Workers Cache/purge support; D1 typed session API since v0.8.0.
- [18] [Service bindings](https://developers.cloudflare.com/workers/runtime-apis/bindings/service-bindings/): Worker-to-Worker `fetch`; counts toward the subrequest limit.
- [19] [Workers pricing](https://developers.cloudflare.com/workers/platform/pricing/): service-binding requests incur no additional request fees; SQLite-backed Durable Objects on Free. *(Free-plan availability of service bindings is not stated explicitly; verify in the dashboard before choosing B.)*
- [20] [How KV works](https://developers.cloudflare.com/kv/concepts/how-kv-works/): "Changes may take up to 60 seconds or more to be visible in other global network locations."
- [21] [KV limits](https://developers.cloudflare.com/kv/platform/limits/): Free 100k reads/day, 1k writes/day, 25 MiB values, 1 write/s per key, minimum cacheTtl 30 s.
- [22] [D1 limits](https://developers.cloudflare.com/d1/platform/limits/): 50 queries/invocation (Free), 500 MB per database (Free).
- [23] [D1 pricing](https://developers.cloudflare.com/d1/platform/pricing/): Free 5M rows read/day, 100k rows written/day; queries fail when exceeded until 00:00 UTC.
- [24] [D1 read replication](https://developers.cloudflare.com/d1/best-practices/read-replication/): latency depends on distance to the primary; the Sessions API gives sequential consistency.
- [25] [MDN: Cache-Control](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Cache-Control): `no-cache` = store but revalidate; `s-maxage` for shared caches; `stale-while-revalidate`.
- [26] [Workers limits](https://developers.cloudflare.com/workers/platform/limits/): Free plan 100k requests/day, 10 ms CPU per request, 50 subrequests.
- [27] [Workers Cache: configuration](https://developers.cloudflare.com/workers/cache/configuration/): `[cache] enabled` (Wrangler ≥4.69.0); `cross_version_cache` and per-entrypoint `exports` (Wrangler ≥4.107.0); per-env overrides.
