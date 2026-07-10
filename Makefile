.DEFAULT_GOAL := check

.PHONY: check fmt-check lint test doctest docs docs-links release package coverage miri benchmark benchmark-build

check: fmt-check lint test doctest docs release

fmt-check:
	cargo fmt --all -- --check

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	cargo test --workspace --lib --tests --examples --all-features

doctest:
	cargo test --workspace --doc --all-features

docs:
	RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
	mdbook test docs
	mdbook build docs
	mkdir -p target/mdbook/api
	cp -R target/doc/. target/mdbook/api/
	cp docs/api/index.html target/mdbook/api/index.html

docs-links: docs
	lychee --offline --scheme file --root-dir target/mdbook --index-files index.html,. 'target/mdbook/**/*.html'

release:
	cargo build --workspace --release

package:
	cargo package --workspace --locked

coverage:
	cargo llvm-cov --workspace --all-features --fail-under-lines 90

miri:
	cargo +nightly miri setup
	cargo +nightly miri test --workspace --lib --tests --examples --all-features

benchmark:
	cargo bench -p specification-core --bench dispatch

benchmark-build:
	cargo bench -p specification-core --bench dispatch --no-run
