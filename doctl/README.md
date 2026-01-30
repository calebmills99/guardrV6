# DigitalOcean Deployment (doctl)

This folder contains configuration and reference for deploying Guardr to DigitalOcean.

## Files

| File | Description |
|------|-------------|
| `do-api-essentials.yaml` | Curated "Top 20" API endpoints for Guardr deployment |
| `.do.env` | Your DigitalOcean API token (git-ignored, create locally) |
| `README.md` | You're reading it, darling |

## Setup: Create Your `.do.env` File

**You must create this file locally** — it's git-ignored for security.

```bash
# Create the file in this folder:
echo 'DO_API_TOKEN=dop_v1_your_token_here' > doctl/.do.env
```

Or manually create `doctl/.do.env` with:

```env
DO_API_TOKEN=dop_v1_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

### Getting Your Token

1. Go to [DigitalOcean API Tokens](https://cloud.digitalocean.com/account/api/tokens)
2. Click **Generate New Token**
3. Name it (e.g., "guardr-deploy")
4. Select **Full Access** (Read + Write)
5. Copy the token immediately — you won't see it again!

## Quick Start with doctl

### 1. Install doctl

```bash
# Windows (winget)
winget install DigitalOcean.Doctl

# macOS
brew install doctl

# Linux
snap install doctl
```

### 2. Authenticate

```bash
# Option A: Interactive (prompts for token)
doctl auth init

# Option B: Use token from .do.env
$token = (Get-Content doctl/.do.env) -replace 'DO_API_TOKEN=', ''
doctl auth init -t $token
```

### 3. Create Guardr Droplet

```bash
# Get your SSH key ID first
doctl compute ssh-key list

# Create the droplet
doctl compute droplet create guardr-prod `
  --image ubuntu-22-04-x64 `
  --region nyc3 `
  --size s-2vcpu-4gb `
  --ssh-keys YOUR_SSH_KEY_ID `
  --tag-names guardr,production `
  --wait
```

### 4. Configure Firewall

```bash
# Get your droplet ID
doctl compute droplet list

# Create firewall
doctl compute firewall create `
  --name guardr-firewall `
  --droplet-ids YOUR_DROPLET_ID `
  --inbound-rules "protocol:tcp,ports:22,address:0.0.0.0/0 protocol:tcp,ports:80,address:0.0.0.0/0 protocol:tcp,ports:443,address:0.0.0.0/0" `
  --outbound-rules "protocol:tcp,ports:all,address:0.0.0.0/0 protocol:udp,ports:all,address:0.0.0.0/0"
```

## Common Commands

| Task | Command |
|------|---------|
| List droplets | `doctl compute droplet list` |
| SSH into droplet | `doctl compute ssh guardr-prod` |
| Get droplet IP | `doctl compute droplet get guardr-prod --format PublicIPv4` |
| Power off | `doctl compute droplet-action power-off DROPLET_ID` |
| Power on | `doctl compute droplet-action power-on DROPLET_ID` |
| Create snapshot | `doctl compute droplet-action snapshot DROPLET_ID --snapshot-name backup` |
| List regions | `doctl compute region list` |
| List sizes | `doctl compute size list` |

## Recommended Droplet Sizes for Guardr

| Environment | Size | Specs | ~Cost/mo |
|-------------|------|-------|----------|
| Dev/Test | `s-1vcpu-1gb` | 1 vCPU, 1GB RAM | $6 |
| Staging | `s-1vcpu-2gb` | 1 vCPU, 2GB RAM | $12 |
| **Production** | `s-2vcpu-4gb` | 2 vCPU, 4GB RAM | $24 |
| High Traffic | `s-4vcpu-8gb` | 4 vCPU, 8GB RAM | $48 |

## Security Notes

- `.do.env` is git-ignored — never commit your API token
- Use SSH keys, not passwords
- Enable automatic security updates on your droplet
- Configure firewall rules to limit access
- Consider enabling DigitalOcean's monitoring and alerts

## API Reference

See `do-api-essentials.yaml` for the curated API spec, or the full docs at:
https://docs.digitalocean.com/reference/api/

---

✨ *Deploy fierce, darling. Deploy safe.*
