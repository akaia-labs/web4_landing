setup:
	(cargo install cargo-near)

build:
	(cargo near build non-reproducible-wasm)

deploy:
	(cargo near deploy build-non-reproducible-wasm)

deploy-test:
	(cargo near deploy build-non-reproducible-wasm web4tester.testnet without-init-call network-config testnet sign-with-legacy-keychain send)

test: deploy-test
	(deno --allow-net ./test/web4_get.ts)
