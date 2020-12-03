
DD ?= $(shell date +%d)

all:
	cargo build --release

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
	FvwmCommand 'GotoPage 1 0' 'WindowId root 1 WarpToWindow 50 50'
