# Guardr Debug Plan

This runbook gives a repeatable debug workflow for the Rust API + Next.js website and lists all secrets/env vars used by the project.

## 1) Debug success criteria

A debug session is successful when:

1. Redis, API, and website all start without config validation errors.
2. API health and smoke endpoints respond as expected.
3. At least one authenticated path is exercised (or known auth caveat is confirmed).
4. Any targeted OSINT/AI module is tested with its matching API key.

## 2) Fast start commands (local)

1. Start Redis:
   - `redis-server --daemonize yes`
2. Start API:
   - `DATABASE_URL="sqlite:./data/guardr.db?mode=rwc" cargo run --bin guardr-api`
3. Start website:
   - `cd website && NEXT_PUBLIC_API_URL=http://localhost:5000 npm run dev`

## 3) Required and optional secrets/config

### 3.1 Required for secure API startup

| Variable | Required | Why |
|---|---|---|
| `JWT_SECRET` | Yes | Required secret for JWT signing; must be at least 32 chars. |
| `ENCRYPTION_KEY` | Yes | Required secret for encryption; must be exactly 32 chars. |

### 3.2 Required runtime config (not secret, but critical)

| Variable | Required | Why |
|---|---|---|
| `DATABASE_URL` | Yes (recommended explicit set) | Required DB target; local dev should use `sqlite:./data/guardr.db?mode=rwc`. |
| `REDIS_URL` | Usually | API initializes Redis client at startup; defaults to `redis://127.0.0.1:6379` if unset. |
| `RUN_MODE` | No | Optional config layering selector (`development`, `production`, etc.). |
| `NEXT_PUBLIC_API_URL` | Website only | Points website to API; not a secret. |

### 3.3 Optional service/API secrets (module-specific)

These are optional globally, but required to fully test their corresponding modules:

- `HIBP_API_KEY`
- `XRAPID_API_KEY` (also feeds BreachDirectory key internally)
- `INTELX_API_KEY`
- `DEHASHED_API_KEY`
- `DEHASHED_EMAIL`
- `SERPER_API_KEY`
- `OPENAI_API_KEY`
- `ANTHROPIC_API_KEY`
- `GEMINI_API_KEY`
- `MISTRAL_API_KEY`
- `XAI_API_KEY`
- `GOOGLE_API_KEY`
- `FACECHECK_API_KEY`
- `REALITY_DEFENDER_API_KEY`
- `SHODAN_API_KEY`
- `EXA_API_KEY`
- `TAVILY_API_KEY`
- `SCRAPINGBEE_API_KEY`
- `FIRECRAWL_API_KEY`

### 3.4 Deployment/ops secrets (outside local runtime)

- `DO_TOKEN` (doctl auth in local/ops workflows)
- `DO_API_TOKEN` (doctl container workflow)
- `DIGITALOCEAN_ACCESS_TOKEN` (GitHub Actions secret)

## 4) Local secret bootstrap template

Use `.env.example` as the source of truth for all supported variables.

For local debugging, generate secure values before starting the API:

- `JWT_SECRET`: `openssl rand -hex 32` (64 chars, valid because it is >=32)
- `ENCRYPTION_KEY`: `openssl rand -hex 16` (exactly 32 chars)

## 5) Debug procedure

1. **Config validation first**
   - Start API and confirm no startup panic/errors about JWT or encryption key length.
2. **Health and smoke checks**
   - `curl -i http://localhost:5000/health`
   - `curl -i http://localhost:5000/v1/check`
3. **Auth path check**
   - Exercise register/login endpoints.
   - If 500 occurs with tier serialization symptoms, verify the known `UserSubscriptionTier` enum caveat.
4. **OSINT/AI module checks**
   - Test only modules whose keys are present.
   - Expect graceful degradation when keys are absent.
5. **Website integration**
   - Confirm website calls the API URL set in `NEXT_PUBLIC_API_URL`.
   - Validate at least one end-to-end user action in browser + API logs.
6. **Persistence checks**
   - Confirm DB file creation and expected rows after API calls.

## 6) Known caveats to account for during debugging

- Local DB path should be overridden with `DATABASE_URL="sqlite:./data/guardr.db?mode=rwc"`.
- Route mount is `/v1/...` in app code (production ingress strips `/api` prefix).
- `sqlx::migrate!("./migrations")` embeds migrations at compile time; touch `src/database.rs` then rebuild after migration changes.
- Existing lint warnings may be pre-existing and unrelated to your debug target.
