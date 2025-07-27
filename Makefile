test:
	cargo test

build:
	cargo build

clean:
	cargo clean

clean-build:
	cargo clean
	cargo build

autocorrect:
	rustfmt src/lib.rs --style-edition 2024

lint:
	rustfmt src/lib.rs --style-edition 2024 --check

clippy:
	cargo clippy

build-release:
	cargo clean
	cargo build --release

run-driver:
	./target/debug/alembic