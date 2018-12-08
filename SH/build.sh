i686-gnu-as ../boot.s -o ../boot.o
i686-linux-gnu-gcc -c ../kernel.c -o ../kernel.o -std=gnu99 -ffreestanding -O2 -Wall -Wextra
i686-linux-gnu-gcc -T ../linker.ld -o ../Anix.bin -ffreestanding -O2 -nostdlib ../boot.o ../kernel.o -lgcc
