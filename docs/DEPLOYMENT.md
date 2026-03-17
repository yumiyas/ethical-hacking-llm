
**docs/DEPLOYMENT.md:**
```markdown
# Deployment Guide

## Local Deployment

### Prerequisites
- Rust 1.75+
- 8GB+ RAM
- 10GB+ storage

### Steps

```bash
# Clone repository
git clone https://github.com/yumiyas/ethical-hacking-llm.git
cd ethical-hacking-llm

# Download models
make download-models

# Build
make build

# Run
make run
