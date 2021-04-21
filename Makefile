release:
	cargo build --release

build:
	cargo build 

test:
	cargo test

install:
	cp ./target/release/tmux-executor /usr/bin/tmux-executor
	mv /usr/bin/tmux-executor /usr/bin/tmcr

vim-install: release


