# Guardr Development Guide

## Cursor Cloud specific instructions

### Services overview

| Service | Port | How to start |
|---------|------|-------------|
| Rust API (Axum) | 5000 | `DATABASE_URL="sqlite:./data/guardr.db?mode=rwc" cargo run --bin guardr-api` |
| Next.js website | 3000 | `cd website && npm run dev` |
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

### Non-obvious caveats

- **Database path**: The default config (`config/default.toml`) uses `sqlite:/app/data/guardr.db` which is a Docker container path. For local dev, override with `DATABASE_URL="sqlite:./data/guardr.db?mode=rwc"` env var.
- **Migration compile-time embedding**: `sqlx::migrate!("./migrations")` embeds migration SQL at compile time. After adding/modifying migration files, you must `touch src/database.rs` and rebuild for changes to take effect.
- **System dependency**: `libssl-dev` (OpenSSL headers) must be installed for the Rust build to succeed (`openssl-sys` crate).
- **API routes**: Routes are mounted at `/v1/...` (e.g., `/v1/auth/register`), not `/api/v1/...` as documented in the README. The `/api` prefix is stripped by the production ingress.
- **Auth enum bug**: The `UserSubscriptionTier` enum has a serialization mismatch - `Display` writes lowercase (`"free"`) but `sqlx::Type` derive expects capitalized (`"Free"`). This causes 500 errors on user registration/login. The `/check` demo endpoint works without auth.
- **Existing lint warnings**: Both `cargo clippy` and `next lint` produce warnings/errors on existing code. These are pre-existing and not introduced by setup.
- **Config layering**: Config loads from `config/default.toml` -> `config/{RUN_MODE}.toml` -> `config/local.toml` -> env vars prefixed with `GUARDR_`. Sensitive values can be overridden via `JWT_SECRET`, `DATABASE_URL`, `REDIS_URL`, `ENCRYPTION_KEY`, and various `*_API_KEY` env vars.
