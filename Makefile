build-release:
	cargo build --release
build-dev:
	cargo build
install:
	cp target/release/exstr /usr/bin/exstr