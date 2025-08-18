#!/usr/bin/env bash

# AMOS Demonstration Runner
# Orchestrates demo execution using ruv-swarm

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DEMO_TYPE="${1:-basic}"
DEMO_NAME="${2:-all}"

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}🧠 AMOS Demonstration Runner${NC}"
echo -e "${YELLOW}Demo Type: ${DEMO_TYPE}${NC}"
echo -e "${YELLOW}Demo Name: ${DEMO_NAME}${NC}"
echo ""

# Function to run a specific demo
run_demo() {
    local category=$1
    local name=$2
    local readme="${SCRIPT_DIR}/amos-orchestration/${category}/README.md"
    
    echo -e "${GREEN}📋 Running ${category}/${name}${NC}"
    echo -e "${BLUE}Reading instructions from: ${readme}${NC}"
    
    # Use ruv-swarm to orchestrate the demo
    npx ruv-swarm orchestrate "Execute AMOS demo ${category}/${name} following instructions in ${readme}"
}

# Function to run all demos in a category
run_category() {
    local category=$1
    echo -e "${YELLOW}🔄 Running all ${category} demos${NC}"
    
    case $category in
        "basic")
            run_demo "basic" "hello-swarm"
            run_demo "basic" "agent-coordination"
            run_demo "basic" "neural-viz"
            ;;
        "advanced")
            run_demo "advanced" "emergent-consensus"
            run_demo "advanced" "adaptive-specialization"
            run_demo "advanced" "stress-response"
            run_demo "advanced" "complex-orchestration"
            ;;
        "integrations")
            run_demo "integrations" "unified-orchestration"
            run_demo "integrations" "neural-enhancement"
            run_demo "integrations" "fullstack-demo"
            ;;
        *)
            echo -e "${RED}Unknown category: ${category}${NC}"
            exit 1
            ;;
    esac
}

# Main execution
case $DEMO_TYPE in
    "all")
        echo -e "${YELLOW}Running ALL demos${NC}"
        run_category "basic"
        run_category "advanced"
        run_category "integrations"
        ;;
    "basic"|"advanced"|"integrations")
        if [ "$DEMO_NAME" = "all" ]; then
            run_category "$DEMO_TYPE"
        else
            run_demo "$DEMO_TYPE" "$DEMO_NAME"
        fi
        ;;
    *)
        echo -e "${RED}Usage: $0 [basic|advanced|integrations|all] [demo-name|all]${NC}"
        echo -e "${RED}Example: $0 basic hello-swarm${NC}"
        echo -e "${RED}Example: $0 advanced all${NC}"
        exit 1
        ;;
esac

echo -e "${GREEN}✅ Demo execution complete!${NC}"