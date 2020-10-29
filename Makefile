build-Subscribe:
	cargo build --bin subscribe --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/subscribe $(ARTIFACTS_DIR)/bootstrap

build-Like:
	cargo build --bin like --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/like $(ARTIFACTS_DIR)/bootstrap

build-Article:
	cargo build --bin article --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/article $(ARTIFACTS_DIR)/bootstrap
