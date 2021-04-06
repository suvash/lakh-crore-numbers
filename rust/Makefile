.DEFAULT_GOAL:=help
SHELL:=/usr/bin/env bash

.PHONY: help clean

help: ## Display this help
	$(info)
	@awk 'BEGIN {FS = ":.*##"} /^[a-zA-Z_-]+:.*?##/ { printf " \033[36m%-15s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

clean: ## Cleanup the project folders
	cargo clean

test: ## Run the tests
	cargo test

format: ## Format the crate
	cargo fmt
