setup:
	(cargo install cargo-near)

build:
	(cargo near build non-reproducible-wasm)

deploy:
	(cargo near deploy build-non-reproducible-wasm)
