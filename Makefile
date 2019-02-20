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

GRUBCONFIG = set timeout=5\\n\
insmod part_msdos\\n\
insmod lvm\\n\
insmod ext2\\n\
insmod vbe\\n\
insmod vga\\n\
insmod gfxterm\\n\
terminal_output gfxterm\\n\
insmod png\\n\
set gfxmode=1024x768\\n\
loadfont /boot/grub/themes/breeze/Hack-18.pf2\\n\
set theme=/boot/grub/themes/breeze/theme.txt\\n\
export theme\\n\
\\n\
menuentry "Anix" --class anix {\\n\
	multiboot2 /boot/Anix.bin\\n\
	boot\\n\
}\\n

USBPORT = ""
ERROR = ""
sdb = /dev/sdb
sdc = /dev/sdc
HERE = $(shell pwd)

ifeq ($(shell test -e $(sdb) && echo -n yes),yes)
	USBPORT=$(sdb)
else
	ifeq ($(shell test -e $(sdc) && echo -n yes),yes)
		USBPORT=$(sdc)
	else
		ERROR = "There are no plugged disk"
	endif
endif

all: main

main:
	#Mount a key in your computer this script copy all files on it
	
	#Test if there are errors
ifeq ($(ERROR), "")
	
else
	$(error $(ERROR))
endif
	echo "IT WILL DESTROY ALL FILES IN YOUR USB DEVICE!"
	#Delete files
	rm -rf build
	mkdir -p build/root
	mkdir -p build/scripts
	
	rm -rf src/output
	mkdir -p src/output/C

	#Compile C and assembly
	sh mk/build.sh
	
	#Compile Rust code
	#rustc --crate-name libc ~/.cargo/registry/src/github.com-1ecc6299db9ec823/libc-0.2.48/src/lib.rs --crate-type lib --emit=dep-info,link -C debuginfo=2 --cfg 'feature="use_std"' -C metadata=0d3fedfca66a3bb2 -C extra-filename=-0d3fedfca66a3bb2 --out-dir $(HERE)/lib --target x86_64-unknown-linux-gnu -L dependency=$(HERE)/debug/deps --cap-lints allow
	xargo rustc --target x86_64-unknown-linux-gnu -- -L src/output/main.o
	cp target/x86_64-unknown-linux-gnu/debug/libAnix.a src/output
	
	#Link all files
	ld.lld -o build/bootimage-Anix.bin src/output/multiboot.o src/output/boot.o src/output/fs.o src/output/long_mode_init.o lib/relibc/*.a src/output/C/main.o src/output/libAnix.a -nostdlib
	
	#Create grub config file
	rm src/grub/grub.cfg
	touch src/grub/grub.cfg
	@$(SHELL) -c "echo '$(GRUBCONFIG)'" >> src/grub/grub.cfg | sed -e 's/^ //'

	#WARNING: If you are running this script for the first time
	#sudo parted /dev/sdb mklabel msdos
	#sudo mkfs.ext2 /dev/sdb1

	#Mount iso
	sudo mount $(USBPORT)1 build/root

	#Copy files in iso
	sudo mkdir -p build/root/boot/grub
	sudo cp -r src/files/* build/root/
	sudo cp -r src/grub/themes/* build/root/boot/grub/themes/
	sudo grub-install $(USBPORT) --target=i386-pc --boot-directory="build/root/boot" --force --allow-floppy --verbose
	sudo cp src/grub/grub.cfg build/root/boot/grub/grub.cfg

	sudo mv build/bootimage-Anix.bin build/root/boot/Anix.bin
	
	#For write in an usb key :
	sudo umount build/root

	sudo parted $(USBPORT) set 1 boot on

clean:
	#Clean Rust compiled files
	cargo clean
	xargo clean
	
	#Delete ouput directories
	rm -rf build
	mkdir -p build/root
	
	rm -rf src/output
	mkdir src/output/C
	
	rm -ff lib/lib/build/*.rlib
doc:
	cargo doc

mem:
	xargo build --target x86_64-unknown-linux-gnu
	cp target/x86_64-unknown-linux-gnu/debug/libAnix.a target/debug/libAnix.rlib
	cargo size --lib libAnix -- -A
