all:
	@xargo build --target x86_64-unknown-linux-gnu
	@ld.lld -Ttext=40000000 -entry main -m elf_x86_64 target/x86_64-unknown-linux-gnu/debug/librust_test.rlib -o rust-test
	@cp rust-test ../../build/rust-test
