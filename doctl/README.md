# DigitalOcean Deployment Guide

Complete guide for deploying Guardr to DigitalOcean using App Platform or Droplets.

## 📋 Table of Contents

- [Quick Start](#quick-start)
- [Files Overview](#files-overview)
- [Setup](#setup)
- [Deployment Methods](#deployment-methods)
  - [App Platform (Recommended)](#app-platform-recommended)
  - [Droplet (Advanced)](#droplet-advanced)
- [Management Scripts](#management-scripts)
- [GitHub Actions CI/CD](#github-actions-cicd)
- [Troubleshooting](#troubleshooting)

## 🚀 Quick Start

### Prerequisites

1. **DigitalOcean Account**: [Sign up here](https://cloud.digitalocean.com/registrations/new)
2. **doctl CLI**: Install the DigitalOcean CLI tool
   ```bash
   # macOS
   brew install doctl
   
   # Linux
   snap install doctl
   
   # Windows
   winget install DigitalOcean.Doctl
   ```

3. **API Token**: Get from [DigitalOcean API Tokens](https://cloud.digitalocean.com/account/api/tokens)

### Authenticate

```bash
doctl auth init
# Paste your API token when prompted
```

Verify authentication:
```bash
doctl account get
```

## 📁 Files Overview

| File | Description |
|------|-------------|
| `.do/app.yaml` | App Platform deployment specification |
| `do-api-essentials.yaml` | Curated API endpoints reference |
| `.do.env` | Your API token (create locally, git-ignored) |
| `scripts/01-deploy-app.sh` | Automated deployment to App Platform |
| `scripts/02-setup-secrets.sh` | Generate and configure secrets |
| `scripts/03-manage-app.sh` | App management commands |
| `scripts/04-create-droplet.sh` | Create VM-based deployment |
| `README.md` | This file |

## ⚙️ Setup

### 1. Create Local Environment File (Optional)

```bash
# Create .do.env in the doctl/ directory
echo 'DO_API_TOKEN=dop_v1_your_token_here' > doctl/.do.env
```

This file is **git-ignored** for security. Never commit API tokens!

### 2. Configure GitHub Secrets (For CI/CD)

Add these secrets to your GitHub repository:

1. Go to: `Settings` → `Secrets and variables` → `Actions`
2. Add secret:
   - **Name**: `DIGITALOCEAN_ACCESS_TOKEN`
   - **Value**: Your DigitalOcean API token

## 🎯 Deployment Methods

### App Platform (Recommended)

**Best for:** Quick deployment, automatic SSL, managed infrastructure

#### Automated Deployment

Use our deployment script:

```bash
./doctl/scripts/01-deploy-app.sh
```

This script will:
- ✅ Check for existing Guardr app
- ✅ Create new app or update existing
- ✅ Wait for deployment to complete
- ✅ Show app URL and next steps

#### Manual Deployment

```bash
# Create new app
doctl apps create --spec .do/app.yaml --wait

# Or update existing app
APP_ID=$(doctl apps list --format ID,Spec.Name --no-header | grep -w "guardr" | awk '{print $1}')
doctl apps update $APP_ID --spec .do/app.yaml --wait
```

#### Configure Secrets

**Important**: Set these environment variables in the DigitalOcean dashboard or use our helper script:

```bash
./doctl/scripts/02-setup-secrets.sh
```

Required secrets:
- `GUARDR__SECURITY__JWT_SECRET` - JWT signing key
- `GUARDR__SECURITY__SESSION_SECRET` - Session encryption key

**Set in DO Dashboard:**
1. Go to your app settings
2. Navigate to "Environment Variables"
3. Add secrets (type: encrypted)

#### App Platform Pricing

| Environment | Size | Specs | Cost/month |
|-------------|------|-------|------------|
| Dev/Test | basic-xxs | 512MB RAM | $5 |
| Production | basic-xs | 512MB RAM | $5 |
| Production+ | basic-s | 1GB RAM | $12 |

### Droplet (Advanced)

**Best for:** Full control, custom configuration, multiple services

#### Create Droplet

Use our automated script:

```bash
./doctl/scripts/04-create-droplet.sh
```

This will:
1. List your SSH keys
2. Create droplet with chosen configuration
3. Configure firewall rules
4. Provide next steps

#### Manual Droplet Setup

```bash
# Get SSH key ID
doctl compute ssh-key list

# Create droplet
doctl compute droplet create guardr-prod \
  --image ubuntu-22-04-x64 \
  --region nyc3 \
  --size s-2vcpu-4gb \
  --ssh-keys YOUR_SSH_KEY_ID \
  --tag-names guardr,production \
  --wait

# Create firewall
DROPLET_ID=$(doctl compute droplet list --format ID,Name --no-header | grep "guardr-prod" | awk '{print $1}')

doctl compute firewall create \
  --name guardr-firewall \
  --droplet-ids $DROPLET_ID \
  --inbound-rules "protocol:tcp,ports:22,address:0.0.0.0/0 protocol:tcp,ports:80,address:0.0.0.0/0 protocol:tcp,ports:443,address:0.0.0.0/0" \
  --outbound-rules "protocol:tcp,ports:all,address:0.0.0.0/0 protocol:udp,ports:all,address:0.0.0.0/0"
```

#### Deploy to Droplet

```bash
# SSH into droplet
doctl compute ssh guardr-prod

# Update system
apt update && apt upgrade -y

# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh

# Clone repository
git clone https://github.com/calebmills99/guardrV6.git
cd guardrV6

# Build and run
docker build -t guardr .
docker run -d -p 5000:5000 --name guardr-api \
  -e GUARDR__SECURITY__JWT_SECRET="your-secret" \
  -e GUARDR__SECURITY__SESSION_SECRET="your-secret" \
  guardr
```

#### Droplet Pricing

| Environment | Size | Specs | Cost/month |
|-------------|------|-------|------------|
| Dev/Test | s-1vcpu-1gb | 1 vCPU, 1GB RAM | $6 |
| Staging | s-1vcpu-2gb | 1 vCPU, 2GB RAM | $12 |
| **Production** | s-2vcpu-4gb | 2 vCPU, 4GB RAM | $24 |
| High Traffic | s-4vcpu-8gb | 4 vCPU, 8GB RAM | $48 |

## 🛠️ Management Scripts

### App Management (`03-manage-app.sh`)

Quick commands for managing your deployed app:

```bash
# Show app status
./doctl/scripts/03-manage-app.sh status

# Stream logs
./doctl/scripts/03-manage-app.sh logs api
./doctl/scripts/03-manage-app.sh logs frontend

# Restart app
./doctl/scripts/03-manage-app.sh restart

# List deployments
./doctl/scripts/03-manage-app.sh deployments

# Get app URL
./doctl/scripts/03-manage-app.sh url

# Add custom domain
./doctl/scripts/03-manage-app.sh domain guardr.app

# List domains
./doctl/scripts/03-manage-app.sh list-domains

# Delete app (dangerous!)
./doctl/scripts/03-manage-app.sh delete
```

### Common doctl Commands

| Task | Command |
|------|---------|
| List apps | `doctl apps list` |
| Get app details | `doctl apps get <app-id>` |
| View logs | `doctl apps logs <app-id> --type run --follow` |
| List droplets | `doctl compute droplet list` |
| SSH into droplet | `doctl compute ssh <droplet-name>` |
| Get droplet IP | `doctl compute droplet get <id> --format PublicIPv4` |
| List regions | `doctl compute region list` |
| List sizes | `doctl compute size list` |

## 🔄 GitHub Actions CI/CD

A reference GitHub Actions workflow is provided in [`github-actions-workflow.yml`](./github-actions-workflow.yml).

**To enable CI/CD:**
1. Copy `doctl/github-actions-workflow.yml` to `.github/workflows/deploy-digitalocean.yml`
2. Commit and push the workflow file (requires workflows permission)

### What It Does

1. **Test**: Runs on every push
   - Rust tests
   - Format checking
   - Clippy linting

2. **Deploy**: Runs on push to `main`
   - Deploys to DigitalOcean App Platform
   - Updates existing app or creates new one
   - Comments deployment URL on PRs

### Setup

1. Add `DIGITALOCEAN_ACCESS_TOKEN` secret to GitHub (see [Setup](#setup))
2. Push to `main` branch
3. Monitor deployment in Actions tab

### Manual Trigger

You can manually trigger deployment:

1. Go to GitHub Actions tab
2. Select "Deploy to DigitalOcean" workflow
3. Click "Run workflow"

## 🔧 Troubleshooting

### App Won't Start

```bash
# Check logs
APP_ID=$(doctl apps list --format ID,Spec.Name --no-header | grep -w "guardr" | awk '{print $1}')
doctl apps logs $APP_ID --type run --follow
```

Common issues:
- Missing environment secrets
- Build failures (check build logs)
- Port conflicts

### Can't Connect to Droplet

```bash
# Verify droplet is running
doctl compute droplet list

# Check firewall rules
doctl compute firewall list

# Test SSH
doctl compute ssh <droplet-name>
```

### Deployment Fails in CI

1. Check GitHub Actions logs
2. Verify `DIGITALOCEAN_ACCESS_TOKEN` secret is set
3. Ensure `.do/app.yaml` is valid
4. Check DigitalOcean service status

### App Spec Validation

```bash
# Validate app spec before deploying
doctl apps spec validate .do/app.yaml
```

## 🌐 Custom Domain Setup

### Option 1: Use DigitalOcean DNS

```bash
# Add domain to your app
APP_ID=$(doctl apps list --format ID,Spec.Name --no-header | grep -w "guardr" | awk '{print $1}')
doctl apps create-domain $APP_ID --domain guardr.app

# Configure DNS at your registrar
# Point to DigitalOcean nameservers:
# ns1.digitalocean.com
# ns2.digitalocean.com
# ns3.digitalocean.com
```

### Option 2: Use External DNS

```bash
# Get your app URL
APP_URL=$(doctl apps get $APP_ID --format DefaultIngress --no-header)

# Add CNAME record at your DNS provider:
# Type: CNAME
# Name: @ (or www)
# Value: <your-app>.ondigitalocean.app
```

## 🔐 Security Best Practices

- ✅ Never commit `.do.env` or secrets to git
- ✅ Use SSH keys, not passwords for droplets
- ✅ Enable automatic security updates
- ✅ Configure firewall rules restrictively
- ✅ Use encrypted environment variables in App Platform
- ✅ Enable DigitalOcean monitoring and alerts
- ✅ Regularly update dependencies
- ✅ Use SSL/TLS (automatic with App Platform)

## 📊 Monitoring

### App Platform

Built-in monitoring available in DO dashboard:
- CPU usage
- Memory usage
- Request count
- Response times

### Alerts

Configure in DigitalOcean dashboard:
1. Go to app settings
2. Navigate to "Alerts"
3. Set up notifications for:
   - Deployment failures
   - High CPU/memory
   - Domain issues

## 💡 Tips & Best Practices

1. **Start with App Platform**: Easier to set up and manage
2. **Use environment variables**: Never hardcode secrets
3. **Monitor costs**: Check usage regularly in DO dashboard
4. **Enable backups**: For droplets, enable automatic backups
5. **Use staging environment**: Test changes before production
6. **Review logs regularly**: Catch issues early
7. **Keep dependencies updated**: Security and performance

## 📚 Additional Resources

- [DigitalOcean Documentation](https://docs.digitalocean.com/)
- [doctl Reference](https://docs.digitalocean.com/reference/doctl/)
- [App Platform Docs](https://docs.digitalocean.com/products/app-platform/)
- [DigitalOcean API](https://docs.digitalocean.com/reference/api/)
- [API Essentials](./do-api-essentials.yaml)

## 🆘 Support

- **DigitalOcean Support**: [cloud.digitalocean.com/support](https://cloud.digitalocean.com/support)
- **Community**: [DigitalOcean Community](https://www.digitalocean.com/community)
- **GitHub Issues**: [Report bugs](https://github.com/calebmills99/guardrV6/issues)

---

✨ **Deploy fierce, darling. Deploy safe.**
