# Copilot Instructions for Guardr

## Project Overview
**Guardr** - AI-powered dating safety platform for the LGBTQ+ community. "2FA for your ❤️ heart"

- **Rust Backend**: API server (`src/`), CLI tools, OSINT analysis, risk scoring
- **Next.js Frontend**: Marketing site and demo interface (`website/`)

---

## Build, Test, and Lint Commands

### Rust Backend (from repo root)
```bash
cargo build                           # Development build
cargo build --release                 # Production build
cargo test                            # Run all tests
cargo test api::dating::tests         # Run single test module
cargo test test_name                  # Run single test by name
cargo clippy -- -D warnings          # Lint (CI enforces warnings as errors)
cargo fmt                             # Format code
cargo run --bin guardr-api            # Run API server (port 5000)
cargo run --bin guardr -- check "Name" # Run CLI tool
RUST_LOG=debug cargo run --bin guardr-api  # Verbose logging
```

### Next.js Frontend (from `website/`)
```bash
npm install                           # Install dependencies
npm run dev                           # Dev server (port 3001)
npm run build                         # Production build
npm run lint                          # ESLint check
npx tsc --noEmit                      # Type check only
```

---

## Architecture Overview

### Rust Backend (`src/`)

**Binaries:**
- `guardr-api` (`api_server.rs`) - Axum web server on port 5000
- `guardr` (`main.rs`) - CLI tool for local analysis

**API Structure (`src/api/`):**
- Routes defined in `mod.rs` with Axum Router pattern
- Public endpoint: `POST /check` (demo, no auth)
- Authenticated endpoints: `/v1/auth/*`, `/v1/security/*`, `/v1/dating/*`, `/v1/reports/*`

**Key Modules:**
- `state.rs` - AppState with DB pool, Redis, config
- `errors.rs` - AppError enum with auto HTTP status mapping
- `risk_score.rs` - Risk calculation (0-100 scale, lower = higher risk)
- `config.rs` - Layered config from TOML + env vars

**Error Handling Pattern:**
```rust
use crate::errors::AppError;

pub async fn handler(State(state): State<AppState>) -> Result<Json<T>, AppError> {
    let data = fetch().await.map_err(|e| AppError::ExternalApiError(e.to_string()))?;
    Ok(Json(data))
}
```

### Next.js Frontend (`website/src/`)

**App Router Structure:**
- `app/page.tsx` - Landing page with demo form
- `app/pricing/`, `app/how-it-works/`, `app/safety-tips/` - Static pages
- `components/ui/` - Shared UI components
- `components/layout/` - Header, Footer

**API Integration:**
- Use `process.env.NEXT_PUBLIC_API_URL` (defaults to `http://localhost:5000`)
- OSINT calls take 60-120 seconds - set fetch timeout to 120000ms minimum

---

## Key Conventions

### Configuration Hierarchy
1. `config/default.toml` (base settings)
2. `config/{RUN_MODE}.toml` (environment-specific)
3. `config/local.toml` (local overrides, gitignored)
4. Environment variables with `GUARDR_` prefix

### Required Environment Variables
```bash
HIBP_API_KEY          # Have I Been Pwned
INTELX_API_KEY        # Intelligence X (dark web)
GEMINI_API_KEY        # Google Gemini AI
JWT_SECRET            # JWT signing key (production)
```

### Risk Scoring
- **Scale**: 0-100 (higher = safer)
- **Levels**: LOW (80-100), MEDIUM (60-79), HIGH (40-59), CRITICAL (0-39)
- **Factors**: Data breaches, dark web presence, identity inconsistencies, love bombing patterns

### Design System (Frontend)
**Theme**: Nightlife palette with neon effects
```
bg-[color:var(--surface-*)]   # Dark backgrounds (100-400)
bg-primary-500                # Purple/magenta
bg-secondary-500              # Pink
bg-accent-500                 # Teal/cyan
shadow-glow-primary           # Neon glow effects
text-white/85                 # Body text (85% opacity)
```

### API Response Pattern
```rust
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: DateTime<Utc>,
}
```

---

## CI/CD Pipeline

GitHub Actions workflow (`.github/workflows/deploy.yml`):
1. **Build & Test**: `cargo clippy -- -D warnings` and `cargo test`
2. **Docker Build**: Pushes to DigitalOcean Container Registry
3. **Deploy**: SSH to droplet, pulls latest image, restarts container

Push to `main` triggers automatic deployment.

---

## Commit Message Convention

```
<type>: <subject>

Types: feat, fix, style, refactor, perf, docs, chore
```

Examples:
- `feat: add location field to safety check form`
- `fix: increase API timeout to 120 seconds`
- `style: update neon glow effects`
