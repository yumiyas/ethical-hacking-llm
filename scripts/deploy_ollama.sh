#!/bin/bash

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

MODEL_NAME=${1:-ethical-hacking-llm}
OLLAMA_HOST=${OLLAMA_HOST:-http://localhost:11434}

echo "🦙 Deploying to Ollama: $MODEL_NAME"
echo "===================================="

# Check if ollama is installed
if ! command -v ollama &> /dev/null; then
    echo "Installing Ollama..."
    curl -fsSL https://ollama.com/install.sh | sh
fi

# Check if ollama is running
if ! curl -s "$OLLAMA_HOST/api/tags" > /dev/null; then
    echo "Starting Ollama server..."
    ollama serve &
    sleep 5
fi

# Create Modelfile
echo -e "\n${YELLOW}Creating Modelfile...${NC}"
cat > Modelfile << EOF
# Base model
FROM tinyllama:latest

# Set parameters
PARAMETER temperature 0.7
PARAMETER top_p 0.9
PARAMETER stop "</s>"
PARAMETER num_ctx 2048

# System prompt for ethical hacking
SYSTEM """
You are an ethical hacking assistant. Provide safe, legal, and educational information about cybersecurity.
Always emphasize responsible disclosure and legal compliance.
Never provide instructions for illegal activities.
Focus on defensive security, penetration testing methodologies, and security best practices.
"""

# Template for response
TEMPLATE """
### Instruction:
{{ .Prompt }}

### Response:
"""
EOF

# Create the model
echo -e "\n${YELLOW}Creating Ollama model...${NC}"
ollama create $MODEL_NAME -f Modelfile

# Test the model
echo -e "\n${YELLOW}Testing model...${NC}"
ollama run $MODEL_NAME "How to scan for open ports?" --verbose

# Push to registry (optional)
if [ "$PUSH" = "true" ]; then
    echo -e "\n${YELLOW}Pushing to registry...${NC}"
    ollama push $MODEL_NAME
fi

echo -e "\n${GREEN}✅ Deployment complete!${NC}"
echo "Model: $MODEL_NAME"
echo "Run with: ollama run $MODEL_NAME"
