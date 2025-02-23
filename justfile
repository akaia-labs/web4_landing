setup:
	(cargo install cargo-near)

build:
	(cargo near build non-reproducible-wasm)

deploy:
	(cargo near deploy build-non-reproducible-wasm)

test-deploy:
	(cargo near deploy build-non-reproducible-wasm web4tester.testnet without-init-call network-config testnet sign-with-legacy-keychain send)

test: test-deploy
	(deno --allow-net ./test/client.ts)

test-server:
	(deno serve --allow-net ./test/server.ts)
