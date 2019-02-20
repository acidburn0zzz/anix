#Compile assembly code
for asm in $(ls src/asm | grep .asm) ; do
	nasm -f elf64 src/asm/$asm -o src/output/${asm%.asm}.o
done

#Compile C code
for C in $(ls src/c) ; do
	if [ "${C##*.}" = "c" ]; then
		gcc src/c/$C -o src/output/C/${C%.c}.o -nostartfiles -c
	fi
done
