# Deploy Guardr to DigitalOcean - Quick Guide

## Current Situation

- ✅ Rust backend working locally (port 5000)
- ✅ Next.js frontend working locally (port 3002)
- ❌ guardr.app pointing to deleted Vercel deployment (404)
- ❌ Not authenticated with DigitalOcean CLI

## Step 1: Authenticate with DigitalOcean

Get your API token from: https://cloud.digitalocean.com/account/api/tokens

Then run:
```bash
doctl auth init
# Paste your token when prompted
```

Verify:
```bash
doctl auth list
doctl account get
```

## Step 2: Check for Existing Apps

```bash
doctl apps list
```

If you have an existing app, get its ID:
```bash
doctl apps list --format ID,Spec.Name,DefaultIngress
```

## Step 3: Create App Spec (if needed)

If no app exists, we need to create one. DO App Platform needs an app spec that defines:
- Frontend (Next.js)
- Backend (Python/Rust - we need to decide)
- Environment variables
- Build commands

## Step 4: Deploy Options

### Option A: Python Backend (Quick Deploy)
The Python files (`guardr_ultimate_v2.py`) can be wrapped in a Flask/FastAPI app and deployed immediately.

**Pros:**
- Fast to deploy
- Already has OSINT logic
- Documented in your existing README

**Cons:**
- Not using the Rust backend you've been building

### Option B: Rust Backend (Better Long-term)
Deploy the Rust backend we've been working on.

**Pros:**
- Modern, type-safe, fast
- We just got it working locally
- Better architecture

**Cons:**
- DO doesn't natively support Rust apps
- Need Docker or compile to binary
- More setup required

### Option C: Hybrid Approach
- Deploy Python backend quickly to get site live
- Migrate to Rust backend later

## Step 5: Update DNS

Once app is deployed, update DNS:
1. Get your app's URL from DO (e.g., `guardr-xyz.ondigitalocean.app`)
2. Go to your domain registrar
3. Update A/CNAME records to point to DO
4. Or use DO's nameservers

## Recommended Next Steps

**For fastest deployment:**

1. **Wrap Python in Flask API** (15 minutes)
   ```python
   # Create app.py
   from flask import Flask, request, jsonify
   from guardr_ultimate_v2 import GuardrUltimate

   app = Flask(__name__)
   guardr = GuardrUltimate()

   @app.route('/api/check', methods=['POST'])
   def check():
       data = request.json
       result = guardr.analyze_person(data['name'], data.get('location'))
       return jsonify(result)
   ```

2. **Create requirements.txt**
   ```txt
   flask==3.0.0
   google-generativeai==0.3.0
   requests==2.31.0
   ```

3. **Push to GitHub**
   ```bash
   git add .
   git commit -m "Add Flask API wrapper for deployment"
   git push origin main
   ```

4. **Create DO App from GitHub**
   - Connect your repo
   - Set build command: `pip install -r requirements.txt`
   - Set run command: `gunicorn -w 2 -b 0.0.0.0:8080 --timeout 120 app:app`
   - Add environment variables (API keys)

## What Do You Want To Do?

**Quick & Dirty (Python):** Get it live in 30 minutes
**Proper Setup (Rust):** Take time to Dockerize and deploy correctly

Your call! 🚀
