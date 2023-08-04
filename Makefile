# set your own spin PATH
SPIN = /home/alexys/fermyon/spin

prerequisites:
	rustup target add wasm32-unknown-unknown
	cargo install wasm-bindgen-cli

run: # local native run
	cd egui-endpoint && cargo run

build: # build
	${SPIN} build

up: build # local test using spin
	${SPIN} up

deploy: # deploy to fermyon cloud
	${SPIN} deploy