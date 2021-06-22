release:
	cargo build --release

build:
	cargo build 

test:
	cargo test

install:
	cargo build --release
	sudo cp ./target/release/tmux-executor /usr/bin/tmcr

