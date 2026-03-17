# Models Directory

This directory contains the model files for the Ethical Hacking LLM.

## Directory Structure
models/
├── phi-2-q4/
│ ├── phi-2-q4.gguf # Quantized Phi-2 model (4-bit)
│ └── tokenizer.json # Tokenizer configuration
├── tinyllama-q4/
│ ├── tinyllama-q4.gguf # Quantized TinyLlama model (4-bit)
│ └── tokenizer.json # Tokenizer configuration
└── embeddings/
└── precomputed.json # Precomputed embeddings for common queries

## Model Download

Use the download script to get the models:

```bash
./scripts/download_models.sh
