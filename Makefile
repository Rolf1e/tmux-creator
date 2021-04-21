release:
	cargo build --release

build:
	cargo build 

test:
	cargo test

copy:
	cp ./target/release/tmux-executor /usr/bin/tmux-executor
	mv /usr/bin/tmux-executor /usr/bin/tmcr

install: release copy

vim-install: release


