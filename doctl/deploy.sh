#!/bin/bash
# Quick deployment helper script for Docker
# Works on Windows (Git Bash), macOS, and Linux

set -e

cd "$(dirname "$0")"

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed"
    echo ""
    echo "Install Docker Desktop from:"
    echo "  Windows/macOS: https://www.docker.com/products/docker-desktop"
    echo "  Linux: https://docs.docker.com/engine/install/"
    exit 1
fi

# Check if docker-compose is available
if ! command -v docker-compose &> /dev/null && ! docker compose version &> /dev/null 2>&1; then
    echo "❌ Docker Compose is not installed"
    exit 1
fi

# Use docker compose (v2) or docker-compose (v1)
if docker compose version &> /dev/null 2>&1; then
    COMPOSE="docker compose"
else
    COMPOSE="docker-compose"
fi

# Check if .do.env exists
if [ ! -f .do.env ]; then
    echo "⚠️  Warning: .do.env not found"
    echo ""
    echo "Create .do.env with your DigitalOcean API token:"
    echo "  cp .do.env.example .do.env"
    echo "  # Edit .do.env and add your token"
    echo ""
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Parse command
COMMAND=${1:-help}

case $COMMAND in
    build)
        echo "🔨 Building deployment tools container..."
        $COMPOSE build
        echo "✅ Build complete!"
        ;;
    
    deploy)
        echo "🚀 Deploying to DigitalOcean App Platform..."
        $COMPOSE run --rm deploy deploy
        ;;
    
    secrets)
        echo "🔐 Generating secrets..."
        $COMPOSE run --rm deploy secrets
        ;;
    
    status|logs|restart|url)
        echo "📊 Managing app: $COMMAND"
        $COMPOSE run --rm deploy manage "$@"
        ;;
    
    droplet)
        echo "💧 Creating droplet..."
        $COMPOSE run --rm deploy droplet
        ;;
    
    shell|bash)
        echo "🐚 Starting interactive shell..."
        $COMPOSE run --rm deploy bash
        ;;
    
    clean)
        echo "🧹 Cleaning up..."
        $COMPOSE down -v
        docker rmi guardr-deploy-tools:latest 2>/dev/null || true
        echo "✅ Cleanup complete!"
        ;;
    
    help|*)
        echo "╔═══════════════════════════════════════════════════════╗"
        echo "║   Guardr - DigitalOcean Deployment (Docker)          ║"
        echo "╚═══════════════════════════════════════════════════════╝"
        echo ""
        echo "Usage: ./deploy.sh <command>"
        echo ""
        echo "Commands:"
        echo "  build     - Build the deployment tools container"
        echo "  deploy    - Deploy app to DigitalOcean"
        echo "  secrets   - Generate security secrets"
        echo "  status    - Show app status"
        echo "  logs      - View app logs"
        echo "  restart   - Restart the app"
        echo "  url       - Show app URL"
        echo "  droplet   - Create a droplet"
        echo "  shell     - Start interactive shell"
        echo "  clean     - Remove containers and images"
        echo "  help      - Show this help"
        echo ""
        echo "Examples:"
        echo "  ./deploy.sh build"
        echo "  ./deploy.sh deploy"
        echo "  ./deploy.sh status"
        echo "  ./deploy.sh logs api"
        echo ""
        echo "First time setup:"
        echo "  1. Copy .do.env.example to .do.env"
        echo "  2. Add your DigitalOcean API token to .do.env"
        echo "  3. Run: ./deploy.sh build"
        echo "  4. Run: ./deploy.sh deploy"
        ;;
esac
