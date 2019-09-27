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

. mk/colors.sh
# Compile assembly code
for asm in $(ls "kernel/src/arch/$1/asm" | grep ".asm") ; do
	echo "    ${ORANGE}Compile kernel/src/arch/$1/asm/$asm${NORMAL}" | tr -d "'"
	nasm -f elf64 "kernel/src/arch/$1/asm/$asm" -o "kernel/src/output/${asm%.asm}.o"
done

echo "${GREEN}Success!${NORMAL}" | tr -d "'"
