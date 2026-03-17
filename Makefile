.PHONY: help build run dev test bench clean docker-build docker-run \
        compose-up compose-down compose-logs download-models test-api \
        benchmark ollama-create ollama-run ollama-push lint fmt \
        release install pre-commit security-check

# Variables
CARGO = cargo
DOCKER = docker
COMPOSE = docker-compose
PROJECT_NAME = ethical-hacking-llm
BINARY_NAME = ethical-hacking-llm
VERSION = $(shell git describe --tags --always --dirty 2>/dev/null || echo "dev")

# Colors for output
GREEN = \033[0;32m
RED = \033[0;31m
YELLOW = \033[1;33m
NC = \033[0m

help: ## Show this help
	@awk 'BEGIN {FS = ":.*##"; printf "\n$(GREEN)Usage:$(NC) make $(YELLOW)<target>$(NC)\n\n"} \
		/^[a-zA-Z_-]+:.*?##/ { printf "  $(YELLOW)%-20s$(NC) %s\n", $$1, $$2 } \
		/^##@/ { printf "\n$(GREEN)%s$(NC)\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Build Commands

build: ## Build release binary
	@echo "$(GREEN)Building release binary...$(NC)"
	$(CARGO) build --release
	@echo "$(GREEN)Build complete!$(NC)"

run: ## Run locally
	@echo "$(GREEN)Running application...$(NC)"
	RUST_LOG=info $(CARGO) run --release

dev: ## Run in development mode with hot reload
	@echo "$(GREEN)Running in development mode...$(NC)"
	RUST_LOG=debug $(CARGO) run

##@ Test Commands

test: ## Run all tests
	@echo "$(GREEN)Running tests...$(NC)"
	$(CARGO) test -- --nocapture

test-unit: ## Run unit tests only
	@echo "$(GREEN)Running unit tests...$(NC)"
	$(CARGO) test --lib -- --nocapture

test-integration: ## Run integration tests
	@echo "$(GREEN)Running integration tests...$(NC)"
	$(CARGO) test --test '*' -- --nocapture

bench: ## Run benchmarks
	@echo "$(GREEN)Running benchmarks...$(NC)"
	$(CARGO) bench

##@ Docker Commands

docker-build: ## Build Docker image
	@echo "$(GREEN)Building Docker image...$(NC)"
	$(DOCKER) build -t $(PROJECT_NAME):$(VERSION) .
	$(DOCKER) tag $(PROJECT_NAME):$(VERSION) $(PROJECT_NAME):latest

docker-run: docker-build ## Run Docker container
	@echo "$(GREEN)Running Docker container...$(NC)"
	$(DOCKER) run -p 3000:3000 \
		-v $(PWD)/models:/app/models \
		-v $(PWD)/data:/app/data \
		$(PROJECT_NAME):latest

compose-up: ## Start all services with docker-compose
	@echo "$(GREEN)Starting all services...$(NC)"
	$(COMPOSE) up -d --build
	@echo "$(GREEN)Services started. Access at http://localhost:3000$(NC)"

compose-down: ## Stop all services
	@echo "$(GREEN)Stopping all services...$(NC)"
	$(COMPOSE) down -v

compose-logs: ## Show logs from all services
	@echo "$(GREEN)Showing logs...$(NC)"
	$(COMPOSE) logs -f

compose-restart: compose-down compose-up ## Restart all services

##@ Model Commands

download-models: ## Download required models
	@echo "$(GREEN)Downloading models...$(NC)"
	./scripts/download_models.sh

optimize-model: ## Optimize models for faster inference
	@echo "$(GREEN)Optimizing models...$(NC)"
	python3 scripts/optimize_model.py

##@ Ollama Commands

ollama-create: ## Create Ollama model
	@echo "$(GREEN)Creating Ollama model...$(NC)"
	./scripts/deploy_ollama.sh create

ollama-run: ## Run Ollama model
	@echo "$(GREEN)Running Ollama model...$(NC)"
	ollama run ethical-hacking-llm

ollama-push: ## Push Ollama model to registry
	@echo "$(GREEN)Pushing Ollama model...$(NC)"
	./ollama/ollama_push.sh

##@ Testing Commands

test-api: ## Test API endpoints
	@echo "$(GREEN)Testing API...$(NC)"
	./scripts/test_api.sh

benchmark: ## Run benchmarks
	@echo "$(GREEN)Running benchmarks...$(NC)"
	./scripts/benchmark.sh

load-test: ## Run load test with wrk
	@echo "$(GREEN)Running load test...$(NC)"
	wrk -t12 -c400 -d30s http://localhost:3000/query

##@ Code Quality

lint: ## Run linter
	@echo "$(GREEN)Running linter...$(NC)"
	$(CARGO) clippy -- -D warnings

fmt: ## Format code
	@echo "$(GREEN)Formatting code...$(NC)"
	$(CARGO) fmt

fmt-check: ## Check formatting
	@echo "$(GREEN)Checking formatting...$(NC)"
	$(CARGO) fmt -- --check

audit: ## Run security audit
	@echo "$(GREEN)Running security audit...$(NC)"
	$(CARGO) audit

##@ Release

release: test lint fmt ## Build for release with all checks
	@echo "$(GREEN)Building release version $(VERSION)...$(NC)"
	$(CARGO) build --release
	@echo "$(GREEN)Release build complete!$(NC)"

install: ## Install binary to /usr/local/bin
	@echo "$(GREEN)Installing binary...$(NC)"
	sudo cp target/release/$(BINARY_NAME) /usr/local/bin/

##@ Cleanup

clean: ## Clean build artifacts
	@echo "$(GREEN)Cleaning...$(NC)"
	$(CARGO) clean
	rm -rf target/
	rm -f *.log

clean-all: clean compose-down ## Clean everything including Docker volumes
	@echo "$(GREEN)Cleaning Docker volumes...$(NC)"
	$(DOCKER) system prune -f
	$(DOCKER) volume prune -f

##@ Development

pre-commit: fmt lint test ## Run pre-commit checks
	@echo "$(GREEN)Pre-commit checks passed!$(NC)"

security-check: ## Run security checks
	@echo "$(GREEN)Running security checks...$(NC)"
	$(CARGO) audit
	trivy fs --severity HIGH,CRITICAL .

##@ Documentation

doc: ## Generate documentation
	@echo "$(GREEN)Generating documentation...$(NC)"
	$(CARGO) doc --no-deps --open

doc-build: ## Build documentation
	@echo "$(GREEN)Building documentation...$(NC)"
	$(CARGO) doc --no-deps

##@ Utilities

version: ## Show version
	@echo "$(GREEN)Version: $(VERSION)$(NC)"

info: ## Show system information
	@echo "$(GREEN)System Information:$(NC)"
	@echo "  Rust version: $$(rustc --version)"
	@echo "  Cargo version: $$(cargo --version)"
	@echo "  Docker version: $$(docker --version 2>/dev/null || echo 'not installed')"
	@echo "  Project version: $(VERSION)"

upgrade: ## Upgrade dependencies
	@echo "$(GREEN)Upgrading dependencies...$(NC)"
	$(CARGO) update

.DEFAULT_GOAL := help
