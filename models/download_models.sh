
**models/download_models.sh:**
```bash
#!/bin/bash

set -e

echo "📥 Downloading models for Ethical Hacking LLM..."

# Create directories
mkdir -p models/phi-2-q4
mkdir -p models/tinyllama-q4
mkdir -p models/embeddings

# Download Phi-2 model (4-bit quantized)
echo "Downloading Phi-2 4-bit model..."
if [ ! -f models/phi-2-q4/phi-2-q4.gguf ]; then
    wget -O models/phi-2-q4/phi-2-q4.gguf \
        https://huggingface.co/TheBloke/phi-2-GGUF/resolve/main/phi-2.Q4_K_M.gguf
else
    echo "Phi-2 model already exists"
fi

# Download Phi-2 tokenizer
echo "Downloading Phi-2 tokenizer..."
if [ ! -f models/phi-2-q4/tokenizer.json ]; then
    wget -O models/phi-2-q4/tokenizer.json \
        https://huggingface.co/TheBloke/phi-2-GGUF/raw/main/tokenizer.json
else
    echo "Phi-2 tokenizer already exists"
fi

# Download TinyLlama model (4-bit quantized)
echo "Downloading TinyLlama 4-bit model..."
if [ ! -f models/tinyllama-q4/tinyllama-q4.gguf ]; then
    wget -O models/tinyllama-q4/tinyllama-q4.gguf \
        https://huggingface.co/TheBloke/TinyLlama-1.1B-GGUF/resolve/main/tinyllama-1.1b.Q4_K_M.gguf
else
    echo "TinyLlama model already exists"
fi

# Download TinyLlama tokenizer
echo "Downloading TinyLlama tokenizer..."
if [ ! -f models/tinyllama-q4/tokenizer.json ]; then
    wget -O models/tinyllama-q4/tokenizer.json \
        https://huggingface.co/TheBloke/TinyLlama-1.1B-GGUF/raw/main/tokenizer.json
else
    echo "TinyLlama tokenizer already exists"
fi

# Create sample embeddings
echo "Creating sample embeddings..."
cat > models/embeddings/precomputed.json << EOF
{
  "nmap_scan": [0.1, 0.2, 0.3, 0.4, 0.5],
  "sql_injection": [0.2, 0.3, 0.4, 0.5, 0.6],
  "metasploit": [0.3, 0.4, 0.5, 0.6, 0.7]
}
EOF

echo "✅ Model download complete!"
echo ""
echo "Model sizes:"
ls -lh models/phi-2-q4/ models/tinyllama-q4/
