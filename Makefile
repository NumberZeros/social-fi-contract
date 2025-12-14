.PHONY: build test deploy lint help dev

help:
	@echo "Available commands:"
	@echo "  make build       - Build the smart contract"
	@echo "  make test        - Run all tests"
	@echo "  make deploy      - Deploy to configured cluster"
	@echo "  make lint        - Check code style"
	@echo "  make lint-fix    - Fix code style issues"
	@echo "  make dev         - Start local validator and deploy"
	@echo "  make clean       - Clean build artifacts"

build:
	anchor build

test:
	anchor test

deploy:
	anchor deploy

lint:
	pnpm lint

lint-fix:
	pnpm lint:fix

dev:
	solana-test-validator &
	sleep 5
	anchor build
	anchor deploy
	anchor test

clean:
	anchor clean
	rm -rf target node_modules .anchor

.DEFAULT_GOAL := help
