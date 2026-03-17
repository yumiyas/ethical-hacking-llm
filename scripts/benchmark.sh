#!/bin/bash

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

BASE_URL=${1:-http://localhost:3000}
CONCURRENCY=${2:-10}
REQUESTS=${3:-100}

echo "📊 Benchmarking Ethical Hacking LLM"
echo "===================================="
echo "URL: $BASE_URL"
echo "Concurrency: $CONCURRENCY"
echo "Total Requests: $REQUESTS"
echo "===================================="

# Install dependencies if needed
if ! command -v wrk &> /dev/null; then
    echo "Installing wrk benchmark tool..."
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        sudo apt-get install -y wrk
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        brew install wrk
    fi
fi

if ! command -v jq &> /dev/null; then
    echo "Installing jq..."
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        sudo apt-get install -y jq
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        brew install jq
    fi
fi

# Create test data file
cat > /tmp/benchmark-payload.json << EOF
{
  "query": "How to use nmap for port scanning?",
  "max_tokens": 100,
  "temperature": 0.7
}
EOF

# Run wrk benchmark
echo -e "\n${YELLOW}Running wrk benchmark...${NC}"
wrk -t$CONCURRENCY -c$CONCURRENCY -d30s \
    -s /tmp/benchmark.lua \
    --timeout 10s \
    "$BASE_URL/query"

# Run custom benchmark for different query sizes
echo -e "\n${YELLOW}Testing different query sizes...${NC}"

for size in 10 50 100 200; do
    echo -n "Query size $size chars: "
    
    # Generate random string of specified length
    query=$(cat /dev/urandom | base64 | fold -w $size | head -n 1)
    
    total_time=0
    for i in {1..10}; do
        start=$(date +%s%N)
        curl -s -X POST "$BASE_URL/query" \
            -H "Content-Type: application/json" \
            -d "{\"query\":\"$query\",\"max_tokens\":50}" > /dev/null
        end=$(date +%s%N)
        elapsed=$(( ($end - $start) / 1000000 ))
        total_time=$((total_time + elapsed))
    done
    
    avg=$((total_time / 10))
    echo "${avg}ms avg"
done

# Test concurrency scaling
echo -e "\n${YELLOW}Testing concurrency scaling...${NC}"

for conc in 1 5 10 20 50; do
    echo -n "Concurrency $conc: "
    
    total_time=0
    for i in $(seq 1 $conc); do
        curl -s -X POST "$BASE_URL/query" \
            -H "Content-Type: application/json" \
            -d '{"query":"test","max_tokens":10}' > /dev/null &
    done
    wait
    
    echo "completed"
done

echo -e "\n${GREEN}✅ Benchmark complete!${NC}"
