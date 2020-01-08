all:
	cargo rustc --target x86_64-unknown-linux-musl --release -- -C link-args="-Ttext 40000000"
	# strip target/x86_64-unknown-linux-musl/release/rust-test
	cp target/x86_64-unknown-linux-musl/release/rust-test ../../../build/bins/rust-test
