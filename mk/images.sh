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

for img in $(ls assets/src | grep .png) ; do
	echo "    ${ORANGE}Convert image assets/src/${img} to assets/build/${img%.png}.bmp${NORMAL}"
	convert assets/src/${img} assets/build/${img%.png}.bmp
	
	echo "    ${ORANGE}Copy image assets/build/${img%.png}.bmp to src/files/${img%.png}.bmp${NORMAL}"
	cp assets/build/${img%.png}.bmp src/files/${img%.png}.bmp
done
