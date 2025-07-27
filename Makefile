test:
	cargo test

build:
	cargo build

clean:
	cargo clean

clean-build:
	cargo clean
	cargo build

autoformat:
	rustfmt src/lib.rs --style-edition 2024
	rustfmt tests/integrations.rs --style-edition 2024

lint:
	rustfmt src/lib.rs --style-edition 2024 --check
	rustfmt tests/integrations.rs --style-edition 2024

clippy:
	cargo clippy

build-release:
	cargo clean
	cargo build --release

run-driver:
	./target/debug/alembic