.DEFAULT_GOAL:=help
SHELL:=/bin/bash

.PHONY: help clean

help: ## Display this help
	$(info)
	@awk 'BEGIN {FS = ":.*##"} /^[a-zA-Z_-]+:.*?##/ { printf " \033[36m%-15s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

clean: ## Cleanup the project folders
	rm -rf build/ dist/ *.egg-info/

test: ## Run the tests
	python3 -m unittest discover tests/

setup-build: ## Setup the build packages
	python3 -m pip install --upgrade build

build: ## Build the package
	python3 -m build

setup-upload: ## Setup the packages for uploading to PyPi
	python3 -m pip install --user --upgrade twine

upload-test: ## Upload the dist/ packages to test PyPi
	python3 -m twine upload --repository testpypi dist/*

upload: ## Upload the dist/ packages to the real PyPi
	python3 -m twine upload dist/*
