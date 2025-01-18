WRANGLER := npx wrangler

.PHONY: wrangler
wrangler:
	pnpm install wrangler

.PHONY: deploy
deploy:  wrangler
	$(WRANGLER) deploy --name goplayproxy

.PHONY: run
run:  wrangler
	$(WRANGLER) dev

.PHONY: lint
lint:
	cargo fmt --all --check

.PHONY: fmt
fmt:
	cargo fmt --all
