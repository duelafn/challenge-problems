
DD ?= $(shell date +%d)

all:
	cargo build --release

run: all
	target/release/${DD}

test:
	cargo test
test-v:
	cargo test -- --nocapture

clean:
	cargo clean

start:
	perl -p -E 's/\bDD\b/${DD}/g' _tmpl.rs >> src/${DD}.rs
	echo '[[bin]]\nname="${DD}"\npath = "src/${DD}.rs"\n' >> Cargo.toml
	touch ${DD}.in
	emacsclient --no-wait src/${DD}.rs
	moz-remote 'https://adventofcode.com/'
	FvwmCommand 'GotoPage 1 1' 'WindowId root 1 WarpToWindow 50 50'
