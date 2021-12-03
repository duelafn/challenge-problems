
DD ?= $(shell date +%d)

all:
	cargo build --release

run:
	cargo run --bin ${DD}

test:
	cargo test
test-v:
	cargo test -- --nocapture

clean:
	cargo clean

start:
	perl -p -E 's/\bDD\b/${DD}/g' _tmpl.rs >> src/bin/${DD}.rs
	touch ${DD}.in
	emacsclient --no-wait src/bin/${DD}.rs
# 	moz-remote 'https://adventofcode.com/'
	FvwmCommand 'GotoPage 1 0' 'WindowId root 1 WarpToWindow 50 50'
