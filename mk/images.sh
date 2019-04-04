. mk/colors.sh

for img in $(ls assets/src | grep .png) ; do
	echo "${CYAN}Convert image assets/src/${img} to assets/build/${img%.png}.bmp${NORMAL}"
	convert assets/src/${img} assets/build/${img%.png}.bmp
	
	echo "${CYAN}Copy image assets/build/${img%.png}.bmp to src/files/${img%.png}.bmp${NORMAL}"
	cp assets/build/${img%.png}.bmp src/files/${img%.png}.bmp
done
