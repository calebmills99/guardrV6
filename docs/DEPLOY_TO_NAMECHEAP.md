# Deploying Guardr Website to Namecheap Hosting (guardr.app)

This document outlines what's needed to deploy the Guardr website to Namecheap hosting at guardr.app.

## Current Architecture

The Guardr platform consists of two parts:

1. **Frontend (Next.js)** - Static marketing site and demo interface
2. **Backend (Rust API)** - OSINT analysis, risk scoring, and data processing

**Key finding:** The website already uses `output: 'export'` in `next.config.ts`, meaning it generates a static HTML/CSS/JS bundle that can be deployed to any static hosting.

---

## Namecheap Hosting Options

### Option 1: Namecheap Shared Hosting (cPanel) 

**Best for:** Static website only, no API functionality

**What Namecheap Shared Hosting provides:**
- Static file hosting
- cPanel file manager
- FTP/SFTP access
- SSL certificates (AutoSSL)
- Domain-based hosting

**What you CAN deploy:**
- ✅ Static HTML/CSS/JS files (exported Next.js site)
- ✅ Marketing pages, pricing, how-it-works, safety-tips
- ❌ Cannot run Node.js, Rust, or Python backends
- ❌ The "Try Demo" feature will NOT work (requires API)

**Steps to deploy:**
```bash
# 1. Build the static export locally
cd website
npm install
npm run build

# 2. The static files are in: website/out/
# 3. Upload all contents of website/out/ to Namecheap via:
#    - cPanel File Manager → public_html/
#    - FTP client (FileZilla, etc.)
```

**Limitations:**
- No backend/API support
- Demo functionality won't work
- Form submissions need third-party services

---

### Option 2: Namecheap Static Hosting + External API

**Best for:** Full functionality with Namecheap domain

**Architecture:**
```
guardr.app (Namecheap hosting) → Static frontend
api.guardr.app (DigitalOcean)  → Rust backend API
```

**What you need:**

1. **Keep existing DigitalOcean backend** for API
2. **Deploy static frontend** to Namecheap
3. **Configure CORS** on the Rust API to allow guardr.app
4. **Update DNS** to point to both services

**Steps:**

1. **Configure backend URL in website**
   ```typescript
   // Update website/src/app/page.tsx
   const apiUrl = process.env.NEXT_PUBLIC_API_URL || 'https://api.guardr.app';
   ```

2. **Set environment variable during build**
   ```bash
   cd website
   NEXT_PUBLIC_API_URL=https://api.guardr.app npm run build
   ```

3. **Upload to Namecheap**
   - Upload `website/out/` contents to `public_html/`

4. **Configure DNS in Namecheap**
   - `guardr.app` → A Record pointing to Namecheap hosting
   - `api.guardr.app` → CNAME or A Record to DigitalOcean droplet

5. **Update Rust API CORS** (in `src/api/mod.rs`):
   ```rust
   let cors = CorsLayer::new()
       .allow_origin([
           "https://guardr.app".parse().unwrap(),
           "https://www.guardr.app".parse().unwrap(),
       ])
       .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
       .allow_headers([CONTENT_TYPE, AUTHORIZATION]);
   ```

---

### Option 3: GitHub Pages (Free) + Namecheap Domain

**Best for:** Free static hosting with custom domain

**How it works:**
- Deploy static site to GitHub Pages
- Point Namecheap DNS to GitHub Pages
- Keep backend on DigitalOcean

**You already have this workflow:** `.github/workflows/nextjs.yml`

**Steps:**

1. **Enable GitHub Pages** in repo settings
2. **Configure custom domain** in GitHub Pages settings
3. **Update Namecheap DNS:**
   - A Records pointing to GitHub Pages IPs:
     ```
     185.199.108.153
     185.199.109.153
     185.199.110.153
     185.199.111.153
     ```
   - Or CNAME for www: `yourusername.github.io`

4. **Create CNAME file** in `website/public/CNAME`:
   ```
   guardr.app
   ```

---

### Option 4: Keep Everything on DigitalOcean (Recommended)

**Best for:** Full functionality, easier maintenance

**Current setup already works!** The issue is just DNS pointing.

**What you need to do:**

1. **In Namecheap DNS settings:**
   - Remove old Vercel records
   - Add A/CNAME pointing to DigitalOcean app URL

2. **See existing guide:** `NAMECHEAP_DNS_SETUP.md`

---

## Comparison Table

| Feature | Namecheap Shared | Namecheap + DO API | GitHub Pages | Full DigitalOcean |
|---------|------------------|---------------------|--------------|-------------------|
| Cost | ~$50/year | ~$50/year + $10/mo | Free | ~$10/month |
| Static Site | ✅ | ✅ | ✅ | ✅ |
| API Backend | ❌ | ✅ (separate) | ✅ (separate) | ✅ |
| Demo Feature | ❌ | ✅ | ✅ | ✅ |
| SSL | ✅ AutoSSL | ✅ | ✅ | ✅ Let's Encrypt |
| Custom Domain | ✅ | ✅ | ✅ | ✅ |
| Auto-deploy | ❌ Manual | ❌ Manual frontend | ✅ GitHub Actions | ✅ GitHub Actions |
| Complexity | Low | Medium | Medium | Low |

---

## Recommended Approach

**If you want the demo to work:** Keep the backend on DigitalOcean and only use Namecheap for DNS management pointing to DigitalOcean (see `NAMECHEAP_DNS_SETUP.md`).

**If you want purely static marketing site:** Use Namecheap Shared Hosting, but disable/modify the demo section.

---

## Step-by-Step: Deploy Static Site to Namecheap Shared Hosting

### Prerequisites
- Namecheap shared hosting plan (EasyWP not needed for static sites)
- Node.js installed locally
- Access to cPanel or FTP credentials

### Step 1: Build the Static Export

```bash
# Navigate to the website directory from repository root
cd website

# Install dependencies
npm install

# Build static export (if you want demo to work with external API)
NEXT_PUBLIC_API_URL=https://api.guardr.app npm run build

# OR build without API (demo will show errors)
npm run build
```

The output will be in `website/out/` directory.

### Step 2: Prepare Files for Upload

The `out/` directory contains:
```
out/
├── index.html
├── pricing.html
├── how-it-works.html
├── safety-tips.html
├── _next/         # CSS, JS, and assets
├── favicon.ico
└── ... other assets
```

### Step 3: Upload via cPanel File Manager

1. Log into Namecheap → Hosting → cPanel
2. Open File Manager
3. Navigate to `public_html/`
4. Delete any existing files (backup first!)
5. Upload all contents from `out/` directory
6. Ensure `index.html` is in the root of `public_html/`

### Step 4: Upload via FTP (Alternative)

1. Get FTP credentials from cPanel → FTP Accounts
2. Connect with FTP client (FileZilla, Cyberduck)
3. Upload `out/` contents to `public_html/`

### Step 5: Configure SSL

1. In cPanel → SSL/TLS Status
2. Enable AutoSSL for guardr.app
3. Wait 5-15 minutes for certificate

### Step 6: Verify Deployment

Visit https://guardr.app to verify the site is live.

---

## CI/CD for Namecheap (Manual Alternative)

Since Namecheap shared hosting doesn't support GitHub Actions deployment, you have two options:

### Option A: Manual Deployment
Build locally and upload via FTP/cPanel after each change.

### Option B: GitHub Actions + FTP Deploy
Add a workflow to auto-deploy via FTP:

```yaml
# .github/workflows/deploy-namecheap.yml
name: Deploy to Namecheap

on:
  push:
    branches: [main]
    paths:
      - 'website/**'

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - uses: actions/setup-node@v4
        with:
          node-version: '20'
      
      - name: Build
        working-directory: website
        run: |
          npm ci
          npm run build
      
      - name: Deploy via FTP
        uses: SamKirkland/FTP-Deploy-Action@v4.3.5
        with:
          server: ${{ secrets.NAMECHEAP_FTP_SERVER }}
          username: ${{ secrets.NAMECHEAP_FTP_USERNAME }}
          password: ${{ secrets.NAMECHEAP_FTP_PASSWORD }}
          local-dir: ./website/out/
          server-dir: /public_html/
```

**Required Secrets:**
- `NAMECHEAP_FTP_SERVER`: Your Namecheap FTP hostname (e.g., `ftp.guardr.app`)
- `NAMECHEAP_FTP_USERNAME`: cPanel/FTP username
- `NAMECHEAP_FTP_PASSWORD`: cPanel/FTP password

---

## Environment Variables Summary

For static export with external API:

| Variable | Value | Where to Set |
|----------|-------|--------------|
| `NEXT_PUBLIC_API_URL` | `https://api.guardr.app` | Build time (in shell or CI) |

---

## DNS Configuration (if hosting frontend on Namecheap)

**To point guardr.app to Namecheap hosting:**

1. Go to Namecheap → Domain List → guardr.app → Manage
2. Advanced DNS tab
3. Set records:

| Type | Host | Value |
|------|------|-------|
| A Record | @ | [Namecheap hosting IP - from cPanel] |
| A Record | www | [Same IP] |
| CNAME | api | guardr-api-4c7ct.ondigitalocean.app (for API) |

---

## Summary

**What you need to deploy to Namecheap:**

1. ✅ **Already done:** Next.js configured for static export (`output: 'export'`)
2. ⚠️ **Decision needed:** Keep API on DigitalOcean or lose demo functionality
3. 📦 **Build step:** `npm run build` in `website/` directory
4. 📤 **Upload:** FTP/cPanel upload of `out/` contents to `public_html/`
5. 🔒 **SSL:** Enable AutoSSL in cPanel
6. 🌐 **DNS:** Configure A records if moving away from DigitalOcean

**Recommendation:** Use Namecheap only for DNS management and keep the actual hosting on DigitalOcean (or GitHub Pages for free). This gives you:
- Full demo functionality
- Automatic deployments
- Better performance
- Simpler architecture
