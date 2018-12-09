for S in $(ls ../S | grep .s) ; do
    i686-gnu-as ../S/$S -o ../O/${S%.s}.o
done

for C in $(ls ../C | grep .c) ; do
    i686-linux-gnu-gcc -c ../C/$C -o ../O/${C%.c}.o -std=gnu99 -ffreestanding -O2 -Wall -Wextra
done

i686-linux-gnu-gcc -T ../LD/linker.ld -o ../Others/Anix.bin -ffreestanding -O2 -nostdlib ../O/* -lgcc
