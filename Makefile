# Copyright (C) 2018 Nicolas Fouquet

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.

# You should have received a copy of the GNU General Public License
# along with this program.  If not, see https://www.gnu.org/licenses.

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

ARCH=x86_64-unknown-linux-gnu
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
		ERROR = "There are no plugged disk on $(sdb) or $(sdc)"
	endif
endif


# This task mount an usb device and copy all Anix files on it

# WARNING: If you are running this script for the first time on a real disk
# 	- Create a msdos label on your partition: run ´sudo parted /dev/sdb mklabel msdos´ or use Gparted (Partition->New->Msdos)
# 	- Format the partition in ext2: run ´sudo mkfs.ext2 /dev/sdb1´ or use Gparted (Click on the partition->Format in->Ext2)
# 	- If there is an error with #![feature(try_from)] and x86_64 crate it is normal! Add #![feature(try_from)] to the crate x86_64 0.6.0 (in ~/.cargo/registry/src/.../x86_64-0.6.0/src/lib.rs)

all: msg clear compile link test-errors mount copy umount set-bootable

user:
	@echo "${LIGHTPURPLE}Compile and copy userspace programs${NORMAL}" | tr -d "'"
	@cd userspace ; sh mk.sh
	@echo "${GREEN}Success!${NORMAL}" | tr -d "'"

msg:
	@echo "${RED}MAKE SURE THAT YOU DONT HAVE PLUGGED TWO DEVICES!${NORMAL}" | tr -d "'"
	@sh mk/prompt.sh

compile:
	@# Compile assembly
	@echo "${LIGHTPURPLE}Compile assembly${NORMAL}" | tr -d "'"
	@sh mk/build.sh $(ARCH)
	@echo "${LIGHTPURPLE}Compile rust code${NORMAL}" | tr -d "'"
	@cd kernel && RUST_TARGET_PATH=$(shell pwd) xargo rustc --target $(ARCH) --features $(ARCH) && cd ..
	@cp kernel/target/$(ARCH)/debug/libAnix.a kernel/src/output
	@echo "${GREEN}Success!${NORMAL}" | tr -d "'"

link:
	@# Link assembly and Rust files
	@echo "${LIGHTPURPLE}Link assembly and Rust files${NORMAL}" | tr -d "'"
	@ld.lld -o build/bootimage-Anix.bin kernel/src/output/* -nostdlib -m elf_x86_64 -error-limit=0 -T kernel/src/arch/$(ARCH)/linker.ld > /dev/null 2> /dev/null
	@echo "${GREEN}Success!${NORMAL}" | tr -d "'"

test-errors:
# Test if there are errors
ifeq ($(ERROR), "")

else
	@echo "${RED}$(ERROR)${NORMAL}" | tr -d "'"
	@killall make
endif

mount:
	@# Mount device
	@sudo mount $(USBPORT)1 build/root
	@# TODO: Prompt with the disk in /dev/disk/by-label, store the disk name in a variable and mount it on /media/$(USERNAME)/$(CHOOSED_NAME) with `udisksctl mount -b $(USBPORT)1`

copy:
	@# Copy files in device
	@echo "${LIGHTPURPLE}Copy files${NORMAL}" | tr -d "'"
	@sudo mkdir -p build/root/boot/grub/themes/breeze
	@sudo cp -r root/* build/root/
	@sudo grub-install $(USBPORT) --target=i386-pc --boot-directory="build/root/boot" --force --allow-floppy --verbose > "grub_log.txt" 2>&1
	@sudo cp build/bootimage-Anix.bin build/root/boot/Anix.bin
	@echo "${GREEN}Success!${NORMAL}" | tr -d "'"

umount:
	@# Unmount device
	@sudo umount build/root
	@# TODO: udisksctl unmount -b $(USBPORT)1

set-bootable:
	@sudo parted $(USBPORT) set 1 boot on > /dev/null 2> /dev/null
	@echo "${GREEN}Compile and copy success at $(shell date).${NORMAL}" | tr -d "'"

clear:
	@# Delete files
	@rm -rf build
	@rm -rf kernel/src/output

	@mkdir build
	@mkdir -p kernel/src/output

clean: clear
	# Clear Rust compiled files
	@cargo clean
	@xargo clean

doc:
	@cargo doc
	@cargo doc --open

qemu: ARCH=x86_64-qemu-Anix
qemu: prepare-qemu launch-qemu
prepare-qemu: clear compile user link
	@echo "${LIGHTPURPLE}Create the disk${NORMAL}" | tr -d "'"
	@mkdir -p build/root
	@# dd if=/dev/zero of=build/disk.iso count=2000000 > /dev/null 2> /dev/null
	@fallocate -l 1G build/disk.iso
	@echo -e "o\nn\np\n1\n\n\nw" | sudo fdisk -u -C2000000 -S63 -H16 build/disk.iso > /dev/null 2> /dev/null # Partition the disk

	@sudo losetup /dev/loop0 build/disk.iso
	@sudo losetup -o1048576 /dev/loop1 build/disk.iso
	@sudo mke2fs /dev/loop1 > /dev/null 2> /dev/null # Create an ext2 filesystem
	@sudo mount /dev/loop1 build/root

	@sudo cp -r root/* build/root/ # Copy files
	@sudo grub-install --root-directory=build/root --boot-directory=build/root/boot --no-floppy --modules="normal part_msdos ext2 multiboot biosdisk" /dev/loop0 > "grub_log.txt" 2>&1
	@sudo cp build/bootimage-Anix.bin build/root/boot/Anix.bin

	@sudo umount /dev/loop1
	@sudo losetup -d /dev/loop0
	@sudo losetup -d /dev/loop1

	@sudo chown $(USER):$(USER) build/disk.iso
	@echo "${GREEN}Success!${NORMAL}" | tr -d "'"

launch-qemu:
	@qemu-system-x86_64 -enable-kvm -m 3G -device ahci,id=ahci0\
		-drive if=none,file=build/disk.iso,format=raw,id=drive-sata0-0-0\
		-device ide-drive,bus=ahci0.0,drive=drive-sata0-0-0,id=sata0-0-0\
		-serial stdio -boot d
debug-qemu:
	@qemu-system-x86_64 -enable-kvm -m 3G -device ahci,id=ahci0\
		-drive if=none,file=build/disk.iso,format=raw,id=drive-sata0-0-0\
		-device ide-drive,bus=ahci0.0,drive=drive-sata0-0-0,id=sata0-0-0\
		-serial stdio -boot d -s
