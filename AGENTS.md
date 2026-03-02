# Guardr Development Guide

## Agent interaction preferences

- Use a fierce, fabulous tone and Drag Race references where they fit naturally.
- Keep status updates concise and action-oriented.
- `sudo` may be used for required system tasks.
- Never store passwords, tokens, or other secrets in repository files.

## Cursor Cloud specific instructions

### Services overview

| Service | Port | How to start |
|---------|------|-------------|
| Rust API (Axum) | 5000 | `DATABASE_URL="sqlite:./data/guardr.db?mode=rwc" cargo run --bin guardr-api` |
| Next.js website | 3000 | `cd website && NEXT_PUBLIC_API_URL=http://localhost:5000 npm run dev` |
| Redis | 6379 | `redis-server --daemonize yes` |

Redis must be running before starting the API server (the server creates a Redis client in `AppState::new()`).

### Standard commands

See `README.md` for full details. Quick reference:

- **Build**: `cargo build`
- **Test**: `cargo test`
- **Lint (Rust)**: `cargo clippy`
- **Format check**: `cargo fmt --check`
- **Lint (website)**: `cd website && npx next lint`
- **CLI tool**: `cargo run --bin guardr -- <subcommand>`

### OSINT modules (`src/osint/`)

The API integrates 9 external services via the PERA investigation cycle (Plan-Execute-Review-Adjust, ported from Kallisto-OSINTer). Each module gracefully degrades when its API key is not set.

| Module | Service | Env var | Purpose |
|--------|---------|---------|---------|
| `hibp.rs` | Have I Been Pwned | `HIBP_API_KEY` | Email breach lookup |
| `breach_directory.rs` | BreachDirectory (RapidAPI) | `XRAPID_API_KEY` | Username/domain/IP breach search |
| `shodan.rs` | Shodan | `SHODAN_API_KEY` | IP/port/vulnerability scanning |
| `facecheck.rs` | FaceCheck.id | `FACECHECK_API_KEY` | Reverse image face search |
| `reality_defender.rs` | Reality Defender | `REALITY_DEFENDER_API_KEY` | Deepfake/AI-generated image detection |
| `username_search.rs` | Direct HTTP | (none) | Username presence across 11 platforms |
| `dns_lookup.rs` | Google DNS-over-HTTPS | (none) | DNS record enumeration |
| `ip_lookup.rs` | ipapi.co / RDAP | (none) | IP geolocation |

AI modules (`src/ai/`): `moderation.rs` (OpenAI free Moderation API via `OPENAI_API_KEY`), `risk_analyzer.rs` (multi-source scoring + LLM fallback via `OPENAI_API_KEY` / `ANTHROPIC_API_KEY`), `investigation.rs` (PERA cycle orchestrator).

### Deployment

- **Production**: DigitalOcean App Platform, app ID `ddace79c-4a08-49b3-b3da-e17623497a27`
- **URL**: `https://guardr-q7nh2.ondigitalocean.app` (custom domain `guardr.app` pending DNS propagation)
- **Deploy**: `doctl apps create-deployment ddace79c-4a08-49b3-b3da-e17623497a27` (deploys from branch configured in app spec)
- **Ingress**: `/api/*` routes to Rust API service (prefix stripped), `/*` routes to static website
- **Secrets**: API keys are set as encrypted env vars in the DO app spec via `doctl apps update` with `type: SECRET`
- **Auth with doctl**: `doctl auth init -t "$DO_TOKEN"`

### Non-obvious caveats

- **Database path**: The default config (`config/default.toml`) uses `sqlite:/app/data/guardr.db` which is a Docker container path. For local dev, override with `DATABASE_URL="sqlite:./data/guardr.db?mode=rwc"` env var.
- **Migration compile-time embedding**: `sqlx::migrate!("./migrations")` embeds migration SQL at compile time. After adding/modifying migration files, you must `touch src/database.rs` and rebuild for changes to take effect.
- **System dependency**: `libssl-dev` (OpenSSL headers) must be installed for the Rust build to succeed (`openssl-sys` crate).
- **API routes**: Routes are mounted at `/v1/...` (e.g., `/v1/auth/register`), not `/api/v1/...` as documented in the README. The `/api` prefix is stripped by the DO production ingress.
- **Auth enum bug**: The `UserSubscriptionTier` enum has a serialization mismatch — `Display` writes lowercase (`"free"`) but `sqlx::Type` derive expects capitalized (`"Free"`). This causes 500 errors on user registration/login. The `/check` endpoint works without auth.
- **Website API URL**: Set `NEXT_PUBLIC_API_URL` at build time. Local dev: `http://localhost:5000`. DO production: `/api` (relative, routed by ingress).
- **Username search platform quirks**: Instagram, Reddit, LinkedIn, Facebook use auth-wall detection (non-200 status codes like 403/999/400 treated as "profile exists"). TikTok was removed due to aggressive anti-scraping.
- **Risk scoring philosophy**: Breaches = proof of real digital identity (lower risk), not a negative signal. Zero breaches = suspicious (could be fabricated identity).
- **Existing lint warnings**: Both `cargo clippy` and `next lint` produce warnings/errors on existing code. These are pre-existing.
- **Config layering**: `config/default.toml` -> `config/{RUN_MODE}.toml` -> `config/local.toml` -> env vars prefixed with `GUARDR_`. Sensitive values overridden via `JWT_SECRET`, `DATABASE_URL`, `REDIS_URL`, `ENCRYPTION_KEY`, and `*_API_KEY` env vars.
