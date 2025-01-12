.PHONY: deploy
deploy:
	wrangler deploy --name goplayproxy

.PHONY: run
run:
	wrangler dev
