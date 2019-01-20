#Copyright (C) 2018 Nicolas Fouquet

#This program is free software: you can redistribute it and/or modify
#it under the terms of the GNU General Public License as published by
#the Free Software Foundation, either version 3 of the License, or
#(at your option) any later version.

#This program is distributed in the hope that it will be useful,
#but WITHOUT ANY WARRANTY; without even the implied warranty of
#MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#GNU General Public License for more details.

#You should have received a copy of the GNU General Public License
#along with this program.  If not, see https://www.gnu.org/licenses.
all:boot

boot:
	rm -rf build
	mkdir -p build/root/boot/grub

	cargo +nightly xbuild --verbose
	cp target/debug/libAnix.a src/output

	nasm -f elf64 src/asm/multiboot.asm -o src/output/multiboot.o
	nasm -f elf64 src/asm/boot.asm -o src/output/boot.o
	nasm -f elf64 src/asm/long_mode_init.asm -o src/output/long_mode_init.o

	ld -n -T src/asm/linker.ld -o build/bootimage-Anix.bin src/output/multiboot.o src/output/boot.o src/output/long_mode_init.o src/output/libAnix.a

	echo '\
	menuentry "Anix" {\
		multiboot2 /boot/Anix.bin\
		boot\
	}' >> build/root/boot/grub/grub.cfg
	
	cp -r src/files/* build/root/
	mv build/bootimage-Anix.bin build/root/boot/Anix.bin
	grub-mkrescue -o build/Anix.iso build/root
	sudo dd if=build/Anix.iso of=/dev/sdb
	#mkfs.ext4 build/Anix.iso
	
	#qemu-system-x86_64 -cdrom build/Anix.iso

