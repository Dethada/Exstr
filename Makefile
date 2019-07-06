build-release:
	cargo build --bin exstr --release
build-dev:
	cargo build --bin exstr
install:
	cp target/release/exstr /usr/bin/exstr