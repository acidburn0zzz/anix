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

echo "---OS START LAUNCH---"

if grub-file --is-x86-multiboot ../Others/Anix.bin; then
  echo "
	---------------------
	|                   |
	|Multiboot confirmed|
	|                   |
	---------------------
	"
else
  echo "---The file is not multiboot---"
fi

echo "---WRITE GRUB---"

echo '
menuentry "Anix" {
	multiboot /boot/Anix.bin
}' >> ../Others/grub.cfg

echo "---CREATE ROOT DIR---"

mkdir -p ../root/boot/grub
cp ../Others/Anix.bin ../root/boot/Anix.bin
cp ../Others/grub.cfg ../root/boot/grub/grub.cfg
grub-mkrescue -o ../Anix.iso ../root

echo "---OS LAUNCH IN QEMU ---"

qemu-system-i386 -cdrom ../Anix.iso

echo "SYSTEM LAUNCH SUCCESSFULLY !"

echo "
  ------------------------------
  |     -    |-    | ||  -   - |
  |    | |   | -   |       -   |
  |   |	  |  |  -  | ||   - -  |
  |  |-----| |   - | ||  -   - |
  | |       ||    -| || -     -|
  ------------------------------
"
