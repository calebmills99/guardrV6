#!/bin/bash
# Docker entrypoint for DigitalOcean deployment tools

set -e

echo "╔═══════════════════════════════════════════════════════╗"
echo "║  DigitalOcean Deployment Tools Container             ║"
echo "╚═══════════════════════════════════════════════════════╝"
echo ""

# Check if doctl is authenticated
if [ -n "$DO_API_TOKEN" ]; then
    echo "→ Authenticating with DigitalOcean..."
    echo "$DO_API_TOKEN" | doctl auth init --access-token -
    echo "✅ Authenticated successfully"
    echo ""
else
    echo "⚠️  Warning: DO_API_TOKEN not set"
    echo "   Set it with: docker run -e DO_API_TOKEN=your_token ..."
    echo ""
fi

# Show available commands
if [ "$1" = "help" ] || [ "$1" = "--help" ]; then
    echo "Available commands:"
    echo ""
    echo "  deploy        - Deploy app to DigitalOcean App Platform"
    echo "  secrets       - Generate and display security secrets"
    echo "  manage        - Manage deployed app"
    echo "  droplet       - Create a droplet"
    echo "  bash          - Start interactive bash shell"
    echo ""
    echo "Examples:"
    echo "  docker-compose run deploy deploy"
    echo "  docker-compose run deploy secrets"
    echo "  docker-compose run deploy manage status"
    echo "  docker-compose run deploy bash"
    exit 0
fi

# Execute command based on argument
case "$1" in
    deploy)
        exec /workspace/scripts/01-deploy-app.sh
        ;;
    secrets)
        exec /workspace/scripts/02-setup-secrets.sh
        ;;
    manage)
        shift
        exec /workspace/scripts/03-manage-app.sh "$@"
        ;;
    droplet)
        exec /workspace/scripts/04-create-droplet.sh
        ;;
    bash|shell)
        exec /bin/bash
        ;;
    *)
        # If no recognized command, execute as-is
        exec "$@"
        ;;
esac
