.PHONY: help
help: ## Lists the available commands. Add a comment with '##' to describe a command.
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST)\
		| sort\
		| awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'


.PHONY: run
run: ## Build all services and run the application
	cargo build -p client
	cargo build -p server
	cargo run

.PHONY: publish
publish: ## Publish all the crates to crates.io
	cargo publish --manifest-path shared/Cargo.toml || true
	cargo publish --manifest-path tui/Cargo.toml || true
	cargo publish
