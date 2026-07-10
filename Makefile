.DEFAULT_GOAL := check

.PHONY: check fmt-check lint test doctest docs release package coverage miri

check: fmt-check lint test doctest docs release

fmt-check:
	cargo fmt --all -- --check

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	cargo test --workspace --all-targets --all-features

doctest:
	cargo test --workspace --doc --all-features

docs:
	RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
	mdbook test docs
	mdbook build docs

release:
	cargo build --workspace --release

package:
	cargo package -p specification-core --allow-dirty --locked

coverage:
	cargo llvm-cov --workspace --all-features --no-report

miri:
	cargo +nightly miri setup
	cargo +nightly miri test --workspace --all-targets --all-features
