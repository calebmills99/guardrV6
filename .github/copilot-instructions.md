# Copilot Instructions for Guardr

## Project Overview
**Guardr** - AI-powered dating safety platform for the LGBTQ+ community. "2FA for your ❤️ heart"

This repository contains:
- **Rust Backend**: API server, CLI tools, OSINT analysis, risk scoring
- **Next.js Frontend**: Marketing site and demo interface

## Repository Structure

```
guardrV6/
├── src/                    # Rust backend
│   ├── api/               # API endpoints (auth, security, dating, reports, users)
│   ├── api_server.rs      # Main API server
│   ├── config.rs          # Configuration management
│   ├── database.rs        # Database operations (SQLite)
│   ├── errors.rs          # Error handling
│   ├── risk_score.rs      # Risk calculation algorithms
│   └── main.rs            # CLI entry point
├── website/               # Next.js 15 frontend
│   └── src/
│       ├── app/          # App Router pages
│       ├── components/   # React components
│       └── lib/          # Utilities
└── docs/                 # Documentation
```

---

## Rust Backend Guidelines

### Tech Stack
- **Language**: Rust 1.70+ (2021 edition)
- **Web Framework**: Axum 0.7 with Tokio async runtime
- **Database**: SQLite with SQLx ORM
- **Cache**: Redis (optional)
- **Authentication**: JWT with bcrypt/argon2
- **Logging**: tracing + tracing-subscriber

### Critical Rules

#### Configuration Management
```rust
// Settings loaded from multiple sources (priority order):
// 1. config/default.toml
// 2. config/{RUN_MODE}.toml (e.g., production.toml)
// 3. config/local.toml
// 4. Environment variables with GUARDR_ prefix

// Access config values:
let settings = Settings::new()?;
let api_key = env::var("HIBP_API_KEY")?;
```

#### Environment Variables
Required API keys (set in `.env` or `~/.apikeys.zsh`):
- `HIBP_API_KEY` - Have I Been Pwned
- `LEAK_LOOKUP_KEY` - Leak Lookup service
- `INTELX_API_KEY` - Intelligence X (dark web)
- `GEMINI_API_KEY` - Google Gemini AI

Optional configuration:
- `DATABASE_URL` - SQLite path (default: `sqlite://guardr.db`)
- `REDIS_URL` - Redis connection (default: `redis://127.0.0.1:6379`)
- `SERVER_HOST` - API host (default: `0.0.0.0`)
- `SERVER_PORT` - API port (default: `5000`)
- `JWT_SECRET` - JWT signing key
- `RUST_LOG` - Logging level (default: `info`)

**NEVER commit API keys or secrets to version control**

#### Database Patterns

```rust
// Use SQLx with compile-time checked queries
use sqlx::{SqlitePool, query_as};

// Transactions for multi-step operations
let mut tx = pool.begin().await?;
// ... operations
tx.commit().await?;

// Auto-initialization on first run
// Database created automatically if doesn't exist
```

#### Error Handling

```rust
// Use AppError for API responses
use crate::errors::AppError;

// Pattern: Convert errors to AppError
pub async fn handler() -> Result<Json<Response>, AppError> {
    let data = fetch_data().await
        .map_err(|e| AppError::ExternalServiceError(e.to_string()))?;
    Ok(Json(data))
}

// Structured error types in errors.rs
// - AuthenticationError
// - DatabaseError
// - ExternalServiceError
// - ValidationError
```

#### API Endpoint Structure

```rust
// Axum router pattern
use axum::{Router, routing::{get, post}, extract::State};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/api/check", post(check_person))
        .route("/api/health", get(health_check))
        .with_state(state)
        .layer(/* middleware */)
}

// Authenticated endpoints use AuthenticatedUser extractor
pub async fn protected_route(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<RequestData>,
) -> Result<Json<ResponseData>, AppError> {
    // Implementation
}
```

#### Risk Scoring Algorithm

Key risk factors (from `risk_score.rs` and `api/dating.rs`):
- **Data breaches**: HIBP matches increase risk
- **Dark web presence**: Intelligence X findings
- **Identity inconsistencies**: Mismatched profile data
- **Love bombing patterns**: Excessive compliments, intensity
- **Pressure indicators**: Time pressure, secrecy requests
- **Photo verification**: Reverse image search results

Risk levels:
- `LOW`: 80-100 score
- `MEDIUM`: 60-79 score
- `HIGH`: 40-59 score
- `CRITICAL`: 0-39 score

#### OSINT Integration Patterns

```rust
// Long-running operations (60-120 seconds)
// Use async/await with proper timeouts
use tokio::time::{timeout, Duration};

let result = timeout(
    Duration::from_secs(120),
    fetch_osint_data(name)
).await??;

// Rate limiting per API service
// Respect external API quotas
```

#### CORS Configuration

```rust
// Development: Allow localhost origins
cors_allowed_origins: vec![
    "http://localhost:3000".to_string(),
    "http://localhost:3001".to_string(),
]

// Production: Specific domains only
cors_allowed_origins: vec![
    "https://guardr.app".to_string(),
    "https://www.guardr.app".to_string(),
]
```

### Code Style

1. **Follow Rust conventions**: snake_case for functions/variables, CamelCase for types
2. **Use type annotations** for public APIs
3. **Document public functions** with `///` doc comments
4. **Prefer owned types** in public APIs, use references internally
5. **Use `?` operator** for error propagation
6. **Avoid unwrap()**: Use proper error handling
7. **Async by default**: All I/O operations should be async

### Testing

```bash
# Run tests
cargo test

# Run specific test module
cargo test api::dating::tests

# Build for development
cargo build

# Build optimized release
cargo build --release

# Run API server
cargo run --bin guardr-api

# Run CLI tool
cargo run --bin guardr -- check "John Doe"
```

---

## Next.js Frontend Guidelines

### Tech Stack
- **Framework**: Next.js 15.4.6 (App Router)
- **React**: 19.x
- **Styling**: Tailwind CSS (nightlife/neon theme)
- **Deployment**: DigitalOcean App Platform
- **Backend API**: Rust API on localhost:5000 (dev) / DO droplet (prod)

### Critical Rules

#### Environment Variables
- `NEXT_PUBLIC_API_URL` must be set in `website/.env.local`
  - Dev: `http://localhost:5000`
  - Prod: DO droplet URL
- **DO NOT** commit `.env.local` to version control

#### Design System

**Theme: Nightlife Palette**
```css
/* Dark backgrounds (CSS variables) */
--surface-100: #140b25
--surface-200: #1a1130
--surface-300: #20163b
--surface-400: #2a1f4c

/* Tailwind classes */
bg-primary-500    /* Purple/magenta */
bg-secondary-500  /* Pink */
bg-accent-500     /* Teal/cyan */

/* Neon glow effects */
shadow-glow-primary
shadow-glow-accent
```

**Typography Guidelines**:
- Headings: `text-white`, bold
- Body text: `text-white/85` (85% opacity)
- Small text: `text-white/80`

**Component Patterns**:
- Cards: `glass` effect with `backdrop-blur`
- Buttons: `btn-hover-lift` for 3D lift effect
- Borders: `border-white/10` or `border-white/5`

#### Form Handling Pattern

```typescript
// Demo form interface
interface FormData {
  name: string;        // REQUIRED - person to check
  location?: string;   // Optional - helps disambiguation
}

// API call with proper timeout
const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:5000';

const response = await fetch(`${API_URL}/api/check`, {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ name, location }),
  // IMPORTANT: OSINT calls take 60-120 seconds
  signal: AbortSignal.timeout(120000) // 2-minute timeout
});

// Expected response structure
interface CheckResponse {
  risk_level: "HIGH" | "MEDIUM" | "LOW";
  risk_score: number; // 0-100
  person_verification: object;
  recommendations: string[];
  safety_tips: Array<{
    category: string;
    message: string;
  }>;
}
```

#### Component Structure

```tsx
// Use server components by default
export default function Page() {
  return <div>Server Component</div>;
}

// Mark with 'use client' ONLY when needed:
// - Forms with state
// - Event handlers
// - Browser APIs (useState, useEffect)
'use client';
export default function InteractiveForm() {
  const [state, setState] = useState();
  return <form>...</form>;
}

// File organization
// Shared components: website/src/components/ui/
// Layout components: website/src/components/layout/
// Page components: website/src/app/[route]/page.tsx
```

### Common Pitfalls

1. **API Timeouts**: OSINT analysis takes 60-120 seconds
   - Set fetch timeout to 120000ms minimum
   - Show loading state with "Analysis takes up to 2 minutes" messaging
   - Use progress indicators, not just spinners

2. **Mobile First**: Design mobile-first, enhance for desktop
   - Start with mobile viewport (375px)
   - Use responsive Tailwind: `sm:`, `md:`, `lg:`, `xl:`
   - Test on mobile DevTools before desktop

3. **Removed Content**: 
   - NO "About" page with placeholder content
   - Don't recreate removed pages without explicit approval

4. **API URL Configuration**:
   - Always use `process.env.NEXT_PUBLIC_API_URL`
   - Falls back to `http://localhost:5000` if not set
   - NEVER hardcode API URLs

5. **Styling Consistency**:
   - Use Tailwind utilities, NOT CSS modules
   - Use theme variables, NOT hardcoded hex colors
   - Neon effects via utility classes (`shadow-glow-*`), not inline styles

### Brand Voice & Messaging

- **Empowering, not alarmist**: "You are an adult. You decide."
- **LGBTQ+ focused** but universal safety principles
- **2-minute analysis is a FEATURE**: Captive audience for safety education
- **Mobile-first philosophy**: "It has to be a mobile app day one"
- **Pricing symbolism**: $6.99 = 69% LGBTQ+ harassment rate reminder

### Safety Tips Categories

Rotate through these 4 categories in the UI:
1. `smart_habits` - Practical safety advice
2. `friendly_reminders` - Gentle prompts
3. `did_you_know` - Statistics and education
4. `you_decide` - Empowering user choice

### Testing & Development

```bash
# Navigate to website directory
cd website/

# Install dependencies
npm install

# Development server (runs on port 3001 or 3002)
npm run dev

# Production build
npm run build

# Lint check
npm run lint

# Type check
npx tsc --noEmit
```

### Deployment

- **Platform**: DigitalOcean App Platform
- **Build Command**: `npm run build`
- **Output Directory**: `.next/`
- **Node Version**: 22.x
- **Auto-deploy**: Push to `main` branch triggers deployment
- **Domain**: guardr.app
- **SSL**: Automatic via Let's Encrypt
- **Backend Communication**: Internal networking via `NEXT_PUBLIC_API_URL=${guardr-api.PRIVATE_URL}`

#### Deployment Architecture
- **Frontend**: Web Service on port 3000, $5/month
- **Backend**: Python Service on port 8080, $5/month
- **Total Cost**: $10/month
- **Backend Command**: `gunicorn -w 2 -b 0.0.0.0:8080 --timeout 120 guardr_api:app`
- **Timeout**: 120 seconds (required for OSINT operations)

### Pre-commit Checklist

1. ✅ Test on localhost:3001 with API running on :5000
2. ✅ Verify mobile viewport (Chrome DevTools, 375px)
3. ✅ Check dark theme consistency across all pages
4. ✅ Ensure no hardcoded colors or API URLs
5. ✅ Run `npm run lint` without errors
6. ✅ Commit with clear, descriptive messages

---

## Cross-cutting Concerns

### Security Best Practices

1. **Never commit secrets**: Use `.env` files (gitignored)
2. **Validate all inputs**: Both frontend and backend
3. **Sanitize user data**: Prevent XSS, SQL injection
4. **Rate limiting**: Protect API endpoints from abuse
5. **HTTPS only** in production
6. **JWT expiration**: 24-hour tokens, 30-day refresh tokens

### Logging & Monitoring

```rust
// Backend logging
use tracing::{info, warn, error, debug};

info!("User {} performed action", user_id);
error!("Database operation failed: {}", error);

// Structured logging with fields
info!(
    user_id = %user.id,
    action = "check_person",
    "Risk check initiated"
);
```

```typescript
// Frontend logging (development only)
if (process.env.NODE_ENV === 'development') {
  console.log('API response:', data);
}
```

### Performance Optimization

**Backend**:
- Use connection pooling (SQLite, Redis)
- Cache frequently accessed data
- Implement rate limiting per user tier
- Use async/await for concurrent operations

**Frontend**:
- Use Next.js Image component for optimized images
- Implement code splitting with dynamic imports
- Minimize JavaScript bundle size
- Use React Server Components where possible

### Accessibility (a11y)

- Use semantic HTML elements
- Include ARIA labels for interactive elements
- Ensure keyboard navigation works
- Maintain color contrast ratios (WCAG AA)
- Test with screen readers

### Git Workflow

```bash
# Feature branches from main
git checkout -b feature/description

# Descriptive commit messages
git commit -m "feat: Add identity verification endpoint"
git commit -m "fix: Handle API timeout in demo form"
git commit -m "docs: Update Copilot instructions"

# Push triggers auto-deploy to DigitalOcean
git push origin main
```

---

## External Dependencies & APIs

### Data Sources

1. **Have I Been Pwned (HIBP)**
   - Breach data lookup
   - API key required
   - Rate limit: 10 requests/minute (free tier)

2. **Leak Lookup**
   - Additional breach monitoring
   - API key required

3. **Intelligence X**
   - Dark web monitoring
   - OSINT data aggregation
   - API key required
   - Slowest API (60-90 seconds)

4. **Google Gemini AI**
   - Conversation analysis
   - Risk pattern detection
   - API key required

### API Rate Limits

Respect external service limits:
- Implement exponential backoff for retries
- Cache results when appropriate
- Queue requests to avoid bursts
- Monitor quota usage

---

## Troubleshooting

### Common Issues

**Backend**:
- **Database locked**: Check for long-running transactions
- **API timeout**: Increase timeout for OSINT operations
- **Redis connection failed**: Redis is optional, gracefully degrade
- **CORS errors**: Verify `cors_allowed_origins` configuration

**Frontend**:
- **API URL undefined**: Set `NEXT_PUBLIC_API_URL` in `.env.local`
- **Hydration errors**: Check server/client component usage
- **Build fails**: Clear `.next/` and rebuild
- **Styles not applying**: Check Tailwind config, rebuild CSS

### Debug Mode

```bash
# Backend verbose logging
RUST_LOG=debug cargo run --bin guardr-api

# Frontend with source maps
npm run dev

# Check environment variables
# Backend
printenv | grep GUARDR

# Frontend
printenv | grep NEXT_PUBLIC
```

---

## Resources & Support

### Documentation
- **API Docs**: https://api.guardr.app/docs (when deployed)
- **Next.js Docs**: https://nextjs.org/docs
- **Rust Book**: https://doc.rust-lang.org/book/
- **Axum Guide**: https://docs.rs/axum/latest/axum/

### Community
- **GitHub Issues**: Bug reports and feature requests
- **Discord**: Community support (link in README)
- **Email**: support@guardr.app

### LGBTQ+ Resources
- **LGBT National Hotline**: 1-888-843-4564 (24/7)
- **The Trevor Project**: 1-866-488-7386 (Crisis support)
- **GLAAD**: glaad.org (Media advocacy)

---

## Remember

> **"You are an adult. You decide."**
> 
> Our mission is to **empower**, not control. We provide information, you make the choices.
> Safety is a spectrum, not a binary. Trust your instincts. 🌈
