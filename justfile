setup:
	(cargo install cargo-near)

build:
	(cargo near build non-reproducible-wasm)
