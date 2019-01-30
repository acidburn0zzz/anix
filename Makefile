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
\\n\
menuentry "Anix" {\\n\
	multiboot2 /boot/Anix.bin\\n\
	boot\\n\
}\\n

USBPORT = ""
ERROR = ""
sdb = /dev/sdb
sdc = /dev/sdc

ifeq ($(shell test -e $(sdb) && echo -n yes),yes)
	USBPORT=$(sdb)
else
	ifeq ($(shell test -e $(sdc) && echo -n yes),yes)
		USBPORT=$(sdc)
	else
		ERROR = "There are no plugged disk"
	endif
endif

all:boot

boot:
	#Mount a key in your computer this script copy all files on it
	
	#Test if there are errors
ifeq ($(ERROR), "")
	
else
	$(error $(ERROR))
endif
	
	#Delete files
	rm -rf build
	mkdir -p build/root

	#Compile Rust code
	cargo +nightly xbuild --verbose
		
	#Compile assembly code
	nasm -f elf64 src/asm/multiboot.asm -o src/output/multiboot.o
	nasm -f elf64 src/asm/boot.asm -o src/output/boot.o
	nasm -f elf64 src/asm/long_mode_init.asm -o src/output/long_mode_init.o

	#Link all files
	cp target/debug/libAnix.a src/output
	ld -n -T src/asm/linker.ld -o build/bootimage-Anix.bin src/output/multiboot.o src/output/boot.o src/output/long_mode_init.o src/output/libAnix.a

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
	sudo grub-install $(USBPORT) --target=i386-pc --boot-directory="build/root/boot" --force --allow-floppy --verbose
	sudo cp src/grub/grub.cfg build/root/boot/grub/grub.cfg

	sudo mv build/bootimage-Anix.bin build/root/boot/Anix.bin
		
	#For write in an usb key :
	sudo umount build/root

	sudo parted $(USBPORT) set 1 boot on

clean:
	rm -rf build
	mkdir -p build/root

