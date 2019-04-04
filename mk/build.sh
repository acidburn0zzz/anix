. mk/colors.sh
#Compile assembly code
for asm in $(ls src/asm | grep .asm) ; do
	nasm -f elf64 src/asm/$asm -o src/output/${asm%.asm}.o
done

#@input: path
compile(){
	echo "${GREEN}Compile $1${NORMAL}"
	for C in $(ls $1) ; do
		if [ -d $1/$C ]; then
			compile "$1/$C"
		elif [ -f $1/$C ]; then
			if [ "${C##*.}" = "c" ]; then
				gcc $1/$C -o src/output/C/${C%.c}.o -nostartfiles -c
			fi
		else
			echo "$1/$C is not valid"
			exit 1
		fi
	done
}

compile "src/c"
