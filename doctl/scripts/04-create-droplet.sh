#!/bin/bash
# Create a DigitalOcean Droplet for Guardr
# Alternative to App Platform for those who prefer VM-based deployment

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${GREEN}╔═══════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║       Guardr - Droplet Creation & Setup              ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════════════════════╝${NC}"
echo ""

# Check if doctl is installed
if ! command -v doctl &> /dev/null; then
    echo -e "${RED}❌ Error: doctl is not installed${NC}"
    exit 1
fi

# Configuration
DROPLET_NAME="guardr-prod"
REGION="nyc3"
SIZE="s-2vcpu-4gb"
IMAGE="ubuntu-22-04-x64"

echo -e "${YELLOW}→ Checking for SSH keys...${NC}"
echo ""

# List SSH keys
doctl compute ssh-key list

echo ""
read -p "Enter SSH key ID (from list above): " SSH_KEY_ID

if [ -z "$SSH_KEY_ID" ]; then
    echo -e "${RED}❌ Error: SSH key ID required${NC}"
    exit 1
fi

echo ""
echo -e "${BLUE}Droplet Configuration:${NC}"
echo "  Name:   $DROPLET_NAME"
echo "  Region: $REGION"
echo "  Size:   $SIZE (2 vCPU, 4GB RAM)"
echo "  Image:  $IMAGE"
echo "  SSH:    $SSH_KEY_ID"
echo ""

read -p "Create droplet? (yes/no): " CONFIRM
if [ "$CONFIRM" != "yes" ]; then
    echo "Cancelled"
    exit 0
fi

echo ""
echo -e "${YELLOW}→ Creating droplet...${NC}"

# Create droplet
doctl compute droplet create "$DROPLET_NAME" \
    --image "$IMAGE" \
    --region "$REGION" \
    --size "$SIZE" \
    --ssh-keys "$SSH_KEY_ID" \
    --tag-names guardr,production \
    --wait

echo -e "${GREEN}✅ Droplet created!${NC}"
echo ""

# Get droplet details
DROPLET_ID=$(doctl compute droplet list --format ID,Name --no-header | grep "$DROPLET_NAME" | awk '{print $1}')
DROPLET_IP=$(doctl compute droplet get "$DROPLET_ID" --format PublicIPv4 --no-header)

echo -e "${BLUE}Droplet Details:${NC}"
echo "  ID:  $DROPLET_ID"
echo "  IP:  $DROPLET_IP"
echo ""

echo -e "${YELLOW}→ Creating firewall...${NC}"

# Create firewall
doctl compute firewall create \
    --name guardr-firewall \
    --droplet-ids "$DROPLET_ID" \
    --inbound-rules "protocol:tcp,ports:22,address:0.0.0.0/0 protocol:tcp,ports:80,address:0.0.0.0/0 protocol:tcp,ports:443,address:0.0.0.0/0" \
    --outbound-rules "protocol:tcp,ports:all,address:0.0.0.0/0 protocol:udp,ports:all,address:0.0.0.0/0"

echo -e "${GREEN}✅ Firewall configured!${NC}"
echo ""

echo -e "${GREEN}╔═══════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║                Setup Complete!                        ║${NC}"
echo -e "${GREEN}╚═══════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${YELLOW}Next Steps:${NC}"
echo ""
echo "1. SSH into your droplet:"
echo "   ${BLUE}doctl compute ssh $DROPLET_NAME${NC}"
echo "   or"
echo "   ${BLUE}ssh root@$DROPLET_IP${NC}"
echo ""
echo "2. Once connected, run these commands:"
echo "   ${BLUE}# Update system${NC}"
echo "   apt update && apt upgrade -y"
echo ""
echo "   ${BLUE}# Install Docker${NC}"
echo "   curl -fsSL https://get.docker.com -o get-docker.sh"
echo "   sh get-docker.sh"
echo ""
echo "   ${BLUE}# Clone repository${NC}"
echo "   git clone https://github.com/calebmills99/guardrV6.git"
echo "   cd guardrV6"
echo ""
echo "   ${BLUE}# Build and run with Docker${NC}"
echo "   docker build -t guardr ."
echo "   docker run -d -p 5000:5000 --name guardr-api guardr"
echo ""
echo "3. Set up reverse proxy with Nginx (optional but recommended)"
echo ""
echo "4. Configure SSL with Let's Encrypt:"
echo "   ${BLUE}snap install --classic certbot${NC}"
echo "   ${BLUE}certbot --nginx -d guardr.app${NC}"
echo ""
echo -e "${GREEN}✨ Your droplet is ready!${NC}"
