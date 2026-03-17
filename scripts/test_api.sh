#!/bin/bash

GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

BASE_URL=${1:-http://localhost:3000}

echo "🧪 Testing Ethical Hacking LLM API at $BASE_URL"
echo "================================================"

# Function to test endpoint
test_endpoint() {
    local name=$1
    local method=$2
    local endpoint=$3
    local data=$4
    
    echo -n "Testing $name... "
    
    if [ "$method" = "GET" ]; then
        response=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL$endpoint")
    else
        response=$(curl -s -o /dev/null -w "%{http_code}" -X POST \
            -H "Content-Type: application/json" \
            -d "$data" \
            "$BASE_URL$endpoint")
    fi
    
    if [ "$response" = "200" ] || [ "$response" = "201" ]; then
        echo -e "${GREEN}✓ OK${NC} (HTTP $response)"
    else
        echo -e "${RED}✗ FAILED${NC} (HTTP $response)"
    fi
}

# Test health endpoints
test_endpoint "Health Check" "GET" "/health" ""
test_endpoint "Readiness" "GET" "/ready" ""
test_endpoint "Liveness" "GET" "/live" ""

# Test query endpoint
test_endpoint "Query" "POST" "/query" '{"query":"nmap port scanning","max_tokens":50}'

# Test models endpoint
test_endpoint "List Models" "GET" "/models" ""

# Test metrics
test_endpoint "Metrics" "GET" "/metrics" ""

echo "================================================"

# Test with actual response
echo -e "\n${YELLOW}Sample Query Response:${NC}"
curl -s -X POST "$BASE_URL/query" \
    -H "Content-Type: application/json" \
    -d '{"query":"What is nmap?","max_tokens":50}' | jq .

echo -e "\n${GREEN}✅ API tests complete!${NC}"
