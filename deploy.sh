#!/bin/bash

# RPG Game VPS Deployment Script
# Usage: ./deploy.sh [command]

set -e

PROJECT_NAME="rpg-game"
DOMAIN="rpg.paymebro.xyz"

echo "🚀 Deploying RPG Game..."

# Function to deploy
deploy() {
    echo "🔧 Building and starting..."
    
    # Ensure proxy network exists
    docker network create proxy 2>/dev/null || true
    
    # Stop existing containers
    docker compose -f docker-compose.prod.yml down 2>/dev/null || true
    
    # Build and start with production config
    docker compose -f docker-compose.prod.yml up -d --build
    
    # Show logs
    echo "📋 Container logs:"
    docker compose -f docker-compose.prod.yml logs --tail=50
    
    echo "✅ Deployment complete!"
    echo "🌐 Your RPG game should be available at: https://${DOMAIN}"
}

# Function to check status
check_status() {
    echo "📊 Checking deployment status..."
    echo "Container status:"
    docker compose -f docker-compose.prod.yml ps
    
    echo -e "\nRecent logs:"
    docker compose -f docker-compose.prod.yml logs --tail=20
    
    echo -e "\nHealth check:"
    curl -s -o /dev/null -w "%{http_code}" http://localhost:5173 || echo "Service not responding"
}

# Function to show logs
show_logs() {
    echo "📋 Showing live logs..."
    docker compose -f docker-compose.prod.yml logs -f
}

# Function to stop deployment
stop_deployment() {
    echo "🛑 Stopping deployment..."
    docker compose -f docker-compose.prod.yml down
}

# Parse command line arguments
case "${1:-deploy}" in
    "deploy"|"--build")
        deploy
        ;;
    "status")
        check_status
        ;;
    "logs")
        show_logs
        ;;
    "stop")
        stop_deployment
        ;;
    "help"|"--help")
        echo "Usage: $0 [command]"
        echo "Commands:"
        echo "  deploy    Deploy (default)"
        echo "  status    Check deployment status"
        echo "  logs      Show live logs"
        echo "  stop      Stop the deployment"
        echo "  help      Show this help"
        ;;
    *)
        echo "Unknown command: $1"
        echo "Use '$0 help' for usage information"
        exit 1
        ;;
esac
