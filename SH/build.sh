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

for S in $(ls ../S | grep .s) ; do
    i686-gnu-as ../S/$S -o ../O/${S%.s}.o
done

for C in $(ls ../C | grep .c) ; do
    i686-linux-gnu-gcc -c ../C/$C -o ../O/${C%.c}.o -std=gnu99 -ffreestanding -O2 -Wall -Wextra
done

i686-linux-gnu-gcc -T ../LD/linker.ld -o ../Others/Anix.bin -ffreestanding -O2 -nostdlib ../O/* -lgcc
