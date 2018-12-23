all:boot

boot:
	rm -rf build
	mkdir build
	
	bootimage build
	cp target/x86_64-Anix/debug/bootimage-Anix.bin build
	qemu-system-x86_64 -drive format=raw,file=build/bootimage-Anix.bin

