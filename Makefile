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

include mk/colors.sh

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
	echo	'Load Anix…'\\n\
	multiboot2 /boot/Anix.bin\\n\
	echo	'Load the memory disk…'\\n\
	module2 /boot/initrd.img\\n\
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
		ERROR = "There are no plugged disk on $(sdb) or $(sdc)"
	endif
endif

all: main

main:
	#Mount a key in your computer this script copy all files on it
	
	echo "${RED}IT WILL DESTROY ALL FILES IN YOUR USB DEVICE!${NORMAL}"
	echo "${RED}MAKE SURE THAT YOU DONT HAVE PLUGGED TWO DEVICES!${NORMAL}"
	sh mk/prompt.sh
	
	#Test if there are errors
ifeq ($(ERROR), "")
	
else
	$(error $(ERROR))
endif

	#Delete files
	rm -rf build
	mkdir -p build/root
	mkdir -p build/scripts
	
	rm -rf src/output
	mkdir -p src/output/C

	#Compile C and assembly
	sh mk/build.sh
	
	#Compile Rust code
	xargo rustc --target x86_64-unknown-linux-gnu -- -L src/output/main.o
	cp target/x86_64-unknown-linux-gnu/debug/libAnix.a src/output
	
	#Compile scripts
	gcc -o build/scripts/make_initrd src/scripts/make_initrd.c
	
	#Convert and copy images
	sh mk/images.sh
	
	#Link all files
	ld.lld -o build/bootimage-Anix.bin src/output/multiboot.o src/output/boot.o src/output/long_mode_init.o lib/relibc/*.a src/output/C/*.o src/output/libAnix.a -nostdlib --allow-multiple-definition -m elf_x86_64 -error-limit=0
	
	#Create grub config file
	rm src/grub/grub.cfg
	touch src/grub/grub.cfg
	@$(SHELL) -c "echo '$(GRUBCONFIG)'" >> src/grub/grub.cfg | sed -e 's/^ //'

	#Create initrd
	cd src/files ; ./../../build/scripts/make_initrd * ; cd ../..
	mv src/files/initrd.img build/initrd.img 
	
	#WARNING: If you are running this script for the first time
	#sudo parted /dev/sdb mklabel msdos
	#sudo mkfs.ext2 /dev/sdb1

	#Mount device
	sudo mount $(USBPORT)1 build/root

	#Copy files in device
	sudo mkdir -p build/root/boot/grub
	sudo cp -r src/files/* build/root/
	sudo cp -r src/grub/themes/* build/root/boot/grub/themes/
	sudo grub-install $(USBPORT) --target=i386-pc --boot-directory="build/root/boot" --force --allow-floppy --verbose > "grub_log.txt" 2>&1
	sudo cp src/grub/grub.cfg build/root/boot/grub/grub.cfg
	sudo cp build/initrd.img build/root/boot/initrd.img

	sudo cp build/bootimage-Anix.bin build/root/boot/Anix.bin
	
	#Unmount device
	sudo umount build/root

	sudo parted $(USBPORT) set 1 boot on

clean:
	#Clean Rust compiled files
	cargo clean
	xargo clean
	
	#Delete ouput directories
	rm -rf build
	mkdir -p build/root
	mkdir -p build/scripts
	
	rm -rf src/output
	mkdir -p src/output/C
	
	rm -ff lib/lib/build/*.rlib
doc:
	cargo doc

mem:
	xargo build --target x86_64-unknown-linux-gnu
	cp target/x86_64-unknown-linux-gnu/debug/libAnix.a target/debug/libAnix.rlib
	cargo size --lib libAnix -- -A
