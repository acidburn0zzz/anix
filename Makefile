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

#This task mount an usb device and copy all Anix files on it

	#WARNING: If you are running this script for the first time
	#	-Create a msdos label on your partition: run ´sudo parted /dev/sdb mklabel msdos´ or use Gparted (Partition->New->Msdos)
	#	-Format the partition in ext2: run ´sudo mkfs.ext2 /dev/sdb1´ or use Gparted (Click on the partition->Format in->Ext2)
	#	-If there is an error with #![feature(try_from)] and x86_64 crate it is normal! Add #![feature(try_from)] to the crate x86_64 0.6.0 (in ~/.cargo/registry/src/.../x86_64-0.6.0/src/lib.rs)
	
all: msg clear compile convert link grub-config initrd test-errors mount copy umount set-bootable

msg:
	echo "${RED}IT WILL DESTROY ALL FILES IN YOUR USB DEVICE!${NORMAL}"
	echo "${RED}MAKE SURE THAT YOU DONT HAVE PLUGGED TWO DEVICES!${NORMAL}"
	sh mk/prompt.sh

compile:
	#Compile C and assembly
	sh mk/build.sh
	
	#Compile Rust code
	xargo rustc --target x86_64-unknown-linux-gnu -- -L src/output/main.o
	cp target/x86_64-unknown-linux-gnu/debug/libAnix.a src/output
	
	#Compile scripts
	gcc -o build/scripts/make_initrd src/scripts/make_initrd.c

convert:
	#Convert and copy images
	sh mk/images.sh

link:
	#Link all files
	ld.lld -o build/bootimage-Anix.bin src/output/multiboot.o src/output/boot.o src/output/long_mode_init.o src/output/task.o src/output/C/*.o src/output/libAnix.a -nostdlib --allow-multiple-definition -m elf_x86_64 -error-limit=0 -T src/arch/x86_64/linker.ld

grub-config:
	#Create grub config file
	@$(SHELL) -c "echo '$(GRUBCONFIG)'" >> src/grub/grub.cfg | sed -e 's/^ //'

initrd:
	#Create initrd
	cd src/files ; ./../../build/scripts/make_initrd * ; cd ../..
	mv src/files/initrd.img build/initrd.img 

test-errors:
	#Test if there are errors
ifeq ($(ERROR), "")
	
else
	echo "${RED}$(ERROR)${NORMAL}"
	killall make
endif

mount:
	#Mount device
	sudo mount $(USBPORT)1 build/root

copy:
	#Copy files in device
	sudo mkdir -p build/root/boot/grub/themes/breeze
	sudo cp -r src/files/* build/root/
	sudo grub-install $(USBPORT) --target=i386-pc --boot-directory="build/root/boot" --force --allow-floppy --verbose > "grub_log.txt" 2>&1
	sudo cp -r src/grub/themes/breeze/* build/root/boot/grub/themes/breeze
	sudo cp src/grub/grub.cfg build/root/boot/grub/grub.cfg
	sudo cp build/initrd.img build/root/boot/initrd.img

	sudo cp build/bootimage-Anix.bin build/root/boot/Anix.bin

umount:
	#Unmount device
	sudo umount build/root

set-bootable:
	sudo parted $(USBPORT) set 1 boot on
	
clear:
	#Delete files
	rm -rf build
	rm -rf assets/build
	rm -f src/grub/grub.cfg
	rm -rf src/output
	
	mkdir -p build/root
	mkdir -p build/scripts
	mkdir assets/build
	mkdir -p src/output/C
	touch src/grub/grub.cfg

clean: clear
	#Clean Rust compiled files
	cargo clean
	xargo clean
doc:
	cargo doc

mem:
	xargo build --target x86_64-unknown-linux-gnu
	cp target/x86_64-unknown-linux-gnu/debug/libAnix.a target/debug/libAnix.rlib
	cargo size --lib libAnix -- -A
