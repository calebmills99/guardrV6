#!/bin/bash
# Manage Guardr app on DigitalOcean App Platform
# Quick commands for common operations

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Get app ID
APP_ID=$(doctl apps list --format ID,Spec.Name --no-header | grep -w "guardr" | awk '{print $1}' || true)

if [ -z "$APP_ID" ]; then
    echo -e "${RED}❌ Error: Guardr app not found${NC}"
    echo "Run ./doctl/scripts/01-deploy-app.sh first"
    exit 1
fi

# Parse command
COMMAND=${1:-help}

case $COMMAND in
    status|info)
        echo -e "${YELLOW}→ Getting app status...${NC}"
        echo ""
        doctl apps get "$APP_ID"
        ;;
    
    logs)
        echo -e "${YELLOW}→ Streaming app logs (Ctrl+C to stop)...${NC}"
        echo ""
        COMPONENT=${2:-api}
        doctl apps logs "$APP_ID" --type run --follow --component "$COMPONENT"
        ;;
    
    restart)
        echo -e "${YELLOW}→ Restarting app...${NC}"
        # Trigger a new deployment with the same spec
        doctl apps update "$APP_ID" --spec .do/app.yaml
        echo -e "${GREEN}✅ Restart triggered${NC}"
        ;;
    
    deployments)
        echo -e "${YELLOW}→ Listing recent deployments...${NC}"
        echo ""
        doctl apps list-deployments "$APP_ID"
        ;;
    
    url)
        echo -e "${YELLOW}→ App URL:${NC}"
        echo ""
        doctl apps get "$APP_ID" --format DefaultIngress --no-header
        ;;
    
    domain)
        DOMAIN=${2}
        if [ -z "$DOMAIN" ]; then
            echo -e "${RED}❌ Error: Domain name required${NC}"
            echo "Usage: $0 domain <your-domain.com>"
            exit 1
        fi
        echo -e "${YELLOW}→ Adding custom domain: $DOMAIN${NC}"
        doctl apps create-domain "$APP_ID" --domain "$DOMAIN"
        echo -e "${GREEN}✅ Domain added${NC}"
        echo ""
        echo "Add these DNS records at your domain registrar:"
        echo "  Type: CNAME"
        echo "  Name: @"
        echo "  Value: [shown in DO dashboard]"
        ;;
    
    list-domains)
        echo -e "${YELLOW}→ Listing domains...${NC}"
        echo ""
        doctl apps list-domains "$APP_ID"
        ;;
    
    delete)
        echo -e "${RED}⚠️  WARNING: This will delete the entire Guardr app!${NC}"
        read -p "Are you sure? Type 'yes' to confirm: " CONFIRM
        if [ "$CONFIRM" = "yes" ]; then
            echo -e "${YELLOW}→ Deleting app...${NC}"
            doctl apps delete "$APP_ID" --force
            echo -e "${GREEN}✅ App deleted${NC}"
        else
            echo "Cancelled"
        fi
        ;;
    
    help|*)
        echo -e "${GREEN}╔═══════════════════════════════════════════════════════╗${NC}"
        echo -e "${GREEN}║        Guardr App Management Commands                ║${NC}"
        echo -e "${GREEN}╚═══════════════════════════════════════════════════════╝${NC}"
        echo ""
        echo "Usage: $0 <command> [options]"
        echo ""
        echo "Commands:"
        echo "  ${BLUE}status${NC}                Show app status and details"
        echo "  ${BLUE}logs${NC} [component]     Stream logs (default: api)"
        echo "  ${BLUE}restart${NC}              Trigger new deployment"
        echo "  ${BLUE}deployments${NC}          List recent deployments"
        echo "  ${BLUE}url${NC}                  Show app URL"
        echo "  ${BLUE}domain${NC} <domain>      Add custom domain"
        echo "  ${BLUE}list-domains${NC}         List configured domains"
        echo "  ${BLUE}delete${NC}               Delete the app (dangerous!)"
        echo "  ${BLUE}help${NC}                 Show this help"
        echo ""
        echo "Examples:"
        echo "  $0 status"
        echo "  $0 logs api"
        echo "  $0 logs frontend"
        echo "  $0 domain guardr.app"
        echo ""
        echo "App ID: $APP_ID"
        ;;
esac
