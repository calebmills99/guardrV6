#!/bin/bash
# Deploy Guardr to DigitalOcean App Platform
# This script creates or updates the Guardr app on DO App Platform

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}╔═══════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║    Guardr - DigitalOcean App Platform Deployment     ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════════════════════╝${NC}"
echo ""

# Check if doctl is installed
if ! command -v doctl &> /dev/null; then
    echo -e "${RED}❌ Error: doctl is not installed${NC}"
    echo ""
    echo "Install doctl:"
    echo "  macOS:   brew install doctl"
    echo "  Linux:   snap install doctl"
    echo "  Windows: winget install DigitalOcean.Doctl"
    exit 1
fi

# Check if authenticated
if ! doctl auth list &> /dev/null; then
    echo -e "${RED}❌ Error: Not authenticated with DigitalOcean${NC}"
    echo ""
    echo "Run: doctl auth init"
    exit 1
fi

echo -e "${YELLOW}→ Checking for existing Guardr app...${NC}"

# Check if app already exists
APP_ID=$(doctl apps list --format ID,Spec.Name --no-header | grep -w "guardr" | awk '{print $1}' || true)

if [ -z "$APP_ID" ]; then
    echo -e "${YELLOW}→ No existing app found. Creating new app...${NC}"
    
    # Create new app
    doctl apps create --spec .do/app.yaml --wait
    
    echo -e "${GREEN}✅ App created successfully!${NC}"
    
    # Get the new app ID
    APP_ID=$(doctl apps list --format ID,Spec.Name --no-header | grep -w "guardr" | awk '{print $1}')
    
else
    echo -e "${YELLOW}→ Found existing app (ID: $APP_ID). Updating...${NC}"
    
    # Update existing app
    doctl apps update "$APP_ID" --spec .do/app.yaml --wait
    
    echo -e "${GREEN}✅ App updated successfully!${NC}"
fi

echo ""
echo -e "${GREEN}╔═══════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║                  Deployment Complete!                 ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════════════════════╝${NC}"
echo ""

# Get app details
echo -e "${YELLOW}App Details:${NC}"
doctl apps get "$APP_ID" --format ID,Spec.Name,DefaultIngress,ActiveDeployment.ID,Phase

echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo "1. Set environment secrets in DO dashboard:"
echo "   - GUARDR__SECURITY__JWT_SECRET"
echo "   - GUARDR__SECURITY__SESSION_SECRET"
echo ""
echo "2. Configure custom domain (optional):"
echo "   doctl apps create-domain $APP_ID --domain guardr.app"
echo ""
echo "3. View logs:"
echo "   doctl apps logs $APP_ID --type run"
echo ""
echo -e "${GREEN}✨ Deploy fierce, darling!${NC}"
