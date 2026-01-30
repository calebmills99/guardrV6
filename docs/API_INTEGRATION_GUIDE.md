# 🚀 Guardr API Integration Guide

This guide explains how to utilize the new API keys added to the `gvv6` GitHub environment for OSINT and AI analysis.

## 🔑 Available Secret Keys

The following keys are now mapped from GitHub Actions to your production container:

| Secret Name | Service | Purpose |
|-------------|---------|---------|
| `HIBP_API_KEY` | Have I Been Pwned | Email breach checking |
| `INTELX_API_KEY` | Intelligence X | Dark web and paste monitoring |
| `GEMINI_API_KEY` | Google Gemini | AI-powered risk assessment and NLP |

## 🛠️ How to use them in Code (Rust)

The secrets are automatically loaded into the application state via `src/config.rs`. You can access them within your API handlers using the `AppState`.

### Example: Implementing HIBP Check

```rust
pub async fn check_breach(
    State(state): State<AppState>,
    Json(payload): Json<BreachCheckRequest>,
) -> Result<Json<BreachCheckResponse>, AppError> {
    let api_key = state.config.osint.hibp_api_key.as_ref()
        .ok_or_else(|| AppError::InternalError("HIBP API key not configured".to_string()))?;

    // Use reqwest to call HIBP
    let client = reqwest::Client::new();
    let response = client
        .get(format!("https://haveibeenpwned.com/api/v3/breachedaccount/{}", payload.email))
        .header("hibp-api-key", api_key)
        .header("user-agent", "Guardr-App")
        .send()
        .await
        .map_err(|e| AppError::ExternalApiError(e.to_string()))?;

    // Process response...
}
```

## 📋 Integration Status

- [x] **Environment Setup**: GitHub environment `gvv6` is linked to the deployment pipeline.
- [x] **Configuration**: `src/config.rs` updated to support `OsintConfig`.
- [ ] **Service Logic**: Implement the actual API calls in `src/api/security.rs` or new service modules.
- [ ] **Frontend**: Update the dashboard to display real breach and risk data.

## 💅 Lady Guardr's Tip
> "Darling, these keys are the jewels in your crown! Keep them safe in GitHub, and never let them slip into your commit history. Style is temporary, but a leaked API key is a forever-frown."
