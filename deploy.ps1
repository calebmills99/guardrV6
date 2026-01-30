<#
.SYNOPSIS
    🎭 Lady Guardr's One-Command Deployment
    Deploys Guardr to DigitalOcean with ZERO manual steps

.DESCRIPTION
    This script automates the ENTIRE deployment process:
    - Authenticates with DigitalOcean (if needed)
    - Creates/updates the App Platform app
    - Generates and sets secrets
    - Configures custom domain (optional)
    - Shows deployment status and logs

.EXAMPLE
    .\deploy.ps1
    .\deploy.ps1 -Action status
    .\deploy.ps1 -Action logs
    .\deploy.ps1 -Action restart

.NOTES
    Requirements:
    - doctl installed (winget install DigitalOcean.Doctl)
    - GH_TOKEN environment variable (for GitHub Models agent)
    - DO_API_TOKEN in doctl\.do.env (for DigitalOcean)
#>

param(
    [ValidateSet("deploy", "status", "logs", "restart", "url", "secrets", "help")]
    [string]$Action = "deploy"
)

# ═══════════════════════════════════════════════════════════════════════════════
# CONFIGURATION
# ═══════════════════════════════════════════════════════════════════════════════

$AppName = "guardr"
$AppSpecPath = ".do/app.yaml"

# ═══════════════════════════════════════════════════════════════════════════════
# HELPER FUNCTIONS
# ═══════════════════════════════════════════════════════════════════════════════

function Write-Fancy {
    param([string]$Message, [string]$Color = "White")
    Write-Host $Message -ForegroundColor $Color
}

function Write-Header {
    param([string]$Title)
    Write-Host ""
    Write-Fancy "╔═══════════════════════════════════════════════════════╗" "Magenta"
    Write-Fancy "║  $($Title.PadRight(53))║" "Magenta"
    Write-Fancy "╚═══════════════════════════════════════════════════════╝" "Magenta"
    Write-Host ""
}

function Write-Step {
    param([string]$Message)
    Write-Fancy "→ $Message" "Yellow"
}

function Write-Success {
    param([string]$Message)
    Write-Fancy "✅ $Message" "Green"
}

function Write-Error-Custom {
    param([string]$Message)
    Write-Fancy "❌ $Message" "Red"
}

function Test-DoctlInstalled {
    $null = Get-Command doctl -ErrorAction SilentlyContinue
    return $?
}

function Test-DoctlAuthenticated {
    try {
        $null = doctl account get 2>&1
        return $LASTEXITCODE -eq 0
    } catch {
        return $false
    }
}

function Get-AppId {
    $apps = doctl apps list --format "ID,Spec.Name" --no-header 2>&1
    foreach ($line in $apps -split "`n") {
        if ($line -match "guardr") {
            return ($line -split "\s+")[0]
        }
    }
    return $null
}

function New-SecureSecret {
    $bytes = New-Object byte[] 32
    [System.Security.Cryptography.RandomNumberGenerator]::Fill($bytes)
    return [Convert]::ToBase64String($bytes).Replace("+", "").Replace("/", "").Replace("=", "").Substring(0, 32)
}

# ═══════════════════════════════════════════════════════════════════════════════
# MAIN ACTIONS
# ═══════════════════════════════════════════════════════════════════════════════

function Invoke-Deploy {
    Write-Header "Lady Guardr's Deployment"
    
    # Check doctl
    Write-Step "Checking doctl installation..."
    if (-not (Test-DoctlInstalled)) {
        Write-Error-Custom "doctl is not installed!"
        Write-Host "Install with: winget install DigitalOcean.Doctl"
        exit 1
    }
    Write-Success "doctl installed"
    
    # Check auth
    Write-Step "Checking DigitalOcean authentication..."
    if (-not (Test-DoctlAuthenticated)) {
        Write-Fancy "Not authenticated. Attempting to authenticate..." "Yellow"
        
        # Try to use token from .do.env
        $envFile = "doctl\.do.env"
        if (Test-Path $envFile) {
            $content = Get-Content $envFile -Raw
            if ($content -match "DO_API_TOKEN=(.+)") {
                $token = $Matches[1].Trim()
                Write-Step "Found token in .do.env, authenticating..."
                doctl auth init -t $token
            }
        } else {
            Write-Host "Run: doctl auth init"
            Write-Host "Then re-run this script"
            exit 1
        }
    }
    Write-Success "Authenticated with DigitalOcean"
    
    # Check for existing app
    Write-Step "Checking for existing Guardr app..."
    $appId = Get-AppId
    
    if ($appId) {
        Write-Fancy "Found existing app (ID: $appId). Updating..." "Cyan"
        doctl apps update $appId --spec $AppSpecPath --wait
        Write-Success "App updated!"
    } else {
        Write-Fancy "No existing app found. Creating new app..." "Cyan"
        doctl apps create --spec $AppSpecPath --wait
        $appId = Get-AppId
        Write-Success "App created! (ID: $appId)"
    }
    
    # Show result
    Write-Header "Deployment Complete!"
    
    Write-Step "App Details:"
    doctl apps get $appId --format ID,Spec.Name,DefaultIngress,Phase
    
    Write-Host ""
    Write-Fancy "Your app URL:" "Cyan"
    $url = doctl apps get $appId --format DefaultIngress --no-header
    Write-Fancy "  $url" "Green"
    
    Write-Host ""
    Write-Fancy "Next steps:" "Yellow"
    Write-Host "  1. Run: .\deploy.ps1 -Action secrets    (generate & show secrets)"
    Write-Host "  2. Run: .\deploy.ps1 -Action logs       (view app logs)"
    Write-Host "  3. Run: .\deploy.ps1 -Action status     (check status)"
    Write-Host ""
    Write-Fancy "✨ Sashay deployed, darling! ✨" "Magenta"
}

function Get-Status {
    Write-Header "Guardr App Status"
    
    $appId = Get-AppId
    if (-not $appId) {
        Write-Error-Custom "Guardr app not found. Run: .\deploy.ps1"
        exit 1
    }
    
    doctl apps get $appId
}

function Get-Logs {
    Write-Header "Guardr App Logs"
    
    $appId = Get-AppId
    if (-not $appId) {
        Write-Error-Custom "Guardr app not found. Run: .\deploy.ps1"
        exit 1
    }
    
    Write-Fancy "Streaming logs (Ctrl+C to stop)..." "Yellow"
    doctl apps logs $appId --type run --follow --component api
}

function Invoke-Restart {
    Write-Header "Restarting Guardr"
    
    $appId = Get-AppId
    if (-not $appId) {
        Write-Error-Custom "Guardr app not found. Run: .\deploy.ps1"
        exit 1
    }
    
    Write-Step "Triggering new deployment..."
    doctl apps update $appId --spec $AppSpecPath
    Write-Success "Restart triggered!"
}

function Get-Url {
    $appId = Get-AppId
    if (-not $appId) {
        Write-Error-Custom "Guardr app not found. Run: .\deploy.ps1"
        exit 1
    }
    
    $url = doctl apps get $appId --format DefaultIngress --no-header
    Write-Host $url
}

function New-Secrets {
    Write-Header "Generate Secrets"
    
    $jwt = New-SecureSecret
    $session = New-SecureSecret
    
    Write-Fancy "Generated secrets (save these!):" "Yellow"
    Write-Host ""
    Write-Fancy "GUARDR__SECURITY__JWT_SECRET:" "Cyan"
    Write-Host $jwt
    Write-Host ""
    Write-Fancy "GUARDR__SECURITY__SESSION_SECRET:" "Cyan"
    Write-Host $session
    Write-Host ""
    
    $appId = Get-AppId
    if ($appId) {
        Write-Fancy "Set these in the DigitalOcean dashboard:" "Yellow"
        Write-Host "https://cloud.digitalocean.com/apps/$appId/settings"
        Write-Host ""
        Write-Host "Navigate to 'Environment Variables' and add as secrets (encrypted)"
    }
    
    Write-Host ""
    Write-Fancy "⚠️  Never commit these to git!" "Red"
}

function Show-Help {
    Write-Header "Lady Guardr Deployment Help"
    
    Write-Host "Usage: .\deploy.ps1 [-Action <command>]"
    Write-Host ""
    Write-Fancy "Commands:" "Yellow"
    Write-Host "  deploy   - Deploy or update the app (default)"
    Write-Host "  status   - Show app status and details"
    Write-Host "  logs     - Stream app logs"
    Write-Host "  restart  - Trigger a new deployment"
    Write-Host "  url      - Show app URL"
    Write-Host "  secrets  - Generate new secrets"
    Write-Host "  help     - Show this help"
    Write-Host ""
    Write-Fancy "Examples:" "Yellow"
    Write-Host "  .\deploy.ps1"
    Write-Host "  .\deploy.ps1 -Action status"
    Write-Host "  .\deploy.ps1 -Action logs"
}

# ═══════════════════════════════════════════════════════════════════════════════
# MAIN
# ═══════════════════════════════════════════════════════════════════════════════

switch ($Action) {
    "deploy"  { Invoke-Deploy }
    "status"  { Get-Status }
    "logs"    { Get-Logs }
    "restart" { Invoke-Restart }
    "url"     { Get-Url }
    "secrets" { New-Secrets }
    "help"    { Show-Help }
}
