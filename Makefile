.PHONY: deploy
deploy:
	wrangler deploy --name goplayproxy

.PHONY: run
run:
	wrangler dev

.PHONY: lint
lint:
	cargo fmt --all --check
