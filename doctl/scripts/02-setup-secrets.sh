#!/bin/bash
# Setup secrets for Guardr on DigitalOcean App Platform
# This script helps configure sensitive environment variables

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${GREEN}╔═══════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║       Guardr - Secret Configuration Helper           ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════════════════════╝${NC}"
echo ""

# Check if doctl is installed
if ! command -v doctl &> /dev/null; then
    echo -e "${RED}❌ Error: doctl is not installed${NC}"
    exit 1
fi

# Get app ID
APP_ID=$(doctl apps list --format ID,Spec.Name --no-header | grep -w "guardr" | awk '{print $1}' || true)

if [ -z "$APP_ID" ]; then
    echo -e "${RED}❌ Error: Guardr app not found${NC}"
    echo "Run ./doctl/scripts/01-deploy-app.sh first"
    exit 1
fi

echo -e "${BLUE}Found Guardr app (ID: $APP_ID)${NC}"
echo ""

# Function to generate a random secret
generate_secret() {
    openssl rand -base64 32 | tr -d "=+/" | cut -c1-32
}

echo -e "${YELLOW}Generating secure secrets...${NC}"
JWT_SECRET=$(generate_secret)
SESSION_SECRET=$(generate_secret)

echo ""
echo -e "${GREEN}✅ Secrets generated!${NC}"
echo ""
echo -e "${YELLOW}╔═══════════════════════════════════════════════════════╗${NC}"
echo -e "${YELLOW}║  IMPORTANT: Save these secrets securely!             ║${NC}"
echo -e "${YELLOW}╚═══════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}JWT_SECRET:${NC}"
echo "$JWT_SECRET"
echo ""
echo -e "${BLUE}SESSION_SECRET:${NC}"
echo "$SESSION_SECRET"
echo ""

echo -e "${YELLOW}To set these secrets in your DigitalOcean app:${NC}"
echo ""
echo "1. Go to: https://cloud.digitalocean.com/apps/$APP_ID/settings"
echo "2. Navigate to 'Environment Variables' section"
echo "3. Add/Update the following secrets:"
echo ""
echo "   Key: GUARDR__SECURITY__JWT_SECRET"
echo "   Value: $JWT_SECRET"
echo "   Type: Secret (encrypted)"
echo ""
echo "   Key: GUARDR__SECURITY__SESSION_SECRET"
echo "   Value: $SESSION_SECRET"
echo "   Type: Secret (encrypted)"
echo ""
echo -e "${YELLOW}Or use the API (advanced):${NC}"
echo ""
echo "# Note: doctl doesn't support updating app env vars directly,"
echo "# so we need to update the full app spec with secrets"
echo ""
echo -e "${RED}⚠️  Never commit these secrets to git!${NC}"
echo ""
echo -e "${GREEN}✨ Keep it secret, keep it safe!${NC}"
