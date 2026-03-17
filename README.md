# Ethical Hacking LLM

![Rust](https://img.shields.io/badge/rust-1.75%2B-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Docker](https://img.shields.io/badge/docker-ready-blue)

High-performance Language Model for Ethical Hacking with <100ms response time.

## 🚀 Features

- **Ultra-fast inference** (<100ms response time)
- **Multi-model support** (Local, Ollama, Quantized)
- **Ethical hacking knowledge base** (Commands, Techniques, CVEs)
- **Caching layer** (Redis & In-memory)
- **Security features** (Input validation, Rate limiting, Audit logs)
- **Multiple deployment options** (Docker, Kubernetes, Ollama)
- **Prometheus metrics** & Grafana dashboards
- **Comprehensive API** with streaming support

## 📋 Prerequisites

- Rust 1.75+
- Docker & Docker Compose (optional)
- 8GB+ RAM
- 10GB+ storage

## 🏗️ Architecture
┌─────────────┐ ┌──────────────┐ ┌─────────────┐
│ Client │────▶│ API Gateway │────▶│ Router │
└─────────────┘ └──────────────┘ └─────────────┘
│ │
▼ ▼
┌─────────────┐ ┌─────────────┐
│ Redis │ │ Model │
│ Cache │ │ Engine │
└─────────────┘ └─────────────┘
│
┌──────────┬──────────┼──────────┬──────────┐
▼ ▼ ▼ ▼ ▼
┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐
│ Local │ │Quantized│ │ Ollama │ │Hugging- │ │ Groq │
│ Model │ │ Model │ │ │ │ Face │ │ │
└─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘


## 🚀 Quick Start

### Local Development

```bash
# Clone repository
git clone https://github.com/yourusername/ethical-hacking-llm.git
cd ethical-hacking-llm

# Download models
make download-models

# Run in development mode
make dev

# Test API
make test-api
