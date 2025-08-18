#!/bin/bash

# AMOS Railway Deployment Script
set -e

echo "🚀 AMOS Railway Deployment Script"
echo "================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to check prerequisites
check_prerequisites() {
    echo "Checking prerequisites..."
    
    # Check for Docker
    if ! command -v docker &> /dev/null; then
        echo -e "${RED}❌ Docker is not installed${NC}"
        exit 1
    fi
    
    # Check for Railway CLI (optional)
    if ! command -v railway &> /dev/null; then
        echo -e "${YELLOW}⚠️  Railway CLI not found. Install with: npm install -g @railway/cli${NC}"
    fi
    
    echo -e "${GREEN}✅ Prerequisites check passed${NC}"
}

# Function to build locally
build_local() {
    echo "Building AMOS locally..."
    
    # Build with Docker
    docker build -t amos-deploy:latest .
    
    echo -e "${GREEN}✅ Local build completed${NC}"
}

# Function to test locally
test_local() {
    echo "Testing AMOS locally..."
    
    # Stop any existing containers
    docker-compose down 2>/dev/null || true
    
    # Start services
    docker-compose up -d
    
    echo -e "${GREEN}✅ AMOS is running locally at http://localhost:8080${NC}"
    echo "Press Ctrl+C to stop..."
    
    # Follow logs
    docker-compose logs -f
}

# Function to deploy to Railway
deploy_railway() {
    echo "Deploying to Railway..."
    
    # Check if Railway CLI is installed
    if ! command -v railway &> /dev/null; then
        echo -e "${RED}❌ Railway CLI is required for deployment${NC}"
        echo "Install with: npm install -g @railway/cli"
        exit 1
    fi
    
    # Login to Railway
    railway login
    
    # Initialize project if needed
    if [ ! -f ".railway/config.json" ]; then
        echo "Initializing Railway project..."
        railway init
    fi
    
    # Deploy
    railway up
    
    echo -e "${GREEN}✅ Deployment completed!${NC}"
    railway open
}

# Function to show help
show_help() {
    echo "Usage: ./deploy.sh [command]"
    echo ""
    echo "Commands:"
    echo "  check    - Check prerequisites"
    echo "  build    - Build Docker image locally"
    echo "  test     - Test locally with docker-compose"
    echo "  deploy   - Deploy to Railway"
    echo "  help     - Show this help message"
    echo ""
    echo "Example:"
    echo "  ./deploy.sh test    # Test locally first"
    echo "  ./deploy.sh deploy  # Then deploy to Railway"
}

# Main script logic
case "$1" in
    check)
        check_prerequisites
        ;;
    build)
        check_prerequisites
        build_local
        ;;
    test)
        check_prerequisites
        build_local
        test_local
        ;;
    deploy)
        check_prerequisites
        deploy_railway
        ;;
    help|--help|-h)
        show_help
        ;;
    *)
        echo -e "${YELLOW}No command specified. Showing help...${NC}"
        echo ""
        show_help
        ;;
esac