# File: Makefile

# Default target: show help
.DEFAULT_GOAL := help

# Build mode
MODE ?= debug

# Cargo commands
CARGO = cargo
TARGET = $(CARGO) run

# TLS paths (optional override)
CERT = examples/cert.pem
KEY  = examples/key.pem
CA   = examples/ca.pem

## Build all modes and run TCP example
tcp: ## Run TCP proxy example
	$(TARGET) --example tcp

udp: ## Run UDP proxy example
	$(TARGET) --example udp

unix: ## Run Unix domain socket example
	$(TARGET) --example unix

tls: ## Run TLS proxy example (requires --features tls)
	$(TARGET) --features tls --example tls -- \
		$(CERT) $(KEY)

mtls: ## Run mTLS proxy example (requires --features mtls)
	$(TARGET) --features mtls --example mtls -- \
		$(CERT) $(KEY) $(CA)

build: ## Build all examples
	$(CARGO) build --examples --all-features

clean: ## Clean build artifacts
	$(CARGO) clean

check: ## Check for formatting/linting issues
	cargo fmt --check && cargo clippy --all-targets --all-features

fmt: ## Auto-format the code
	cargo fmt

help: ## Show this help
	@echo "Usage: make <target>"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' Makefile | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-14s\033[0m %s\n", $$1, $$2}'

