echo "---OS START LAUNCH---"

if grub-file --is-x86-multiboot ../myos.bin; then
  echo "
	---------------------
	|		            |
	|Multiboot confirmed|
	|		            |
	---------------------
	"
else
  echo "---The file is not multiboot---"
fi

echo "---WRITE GRUB---"

echo '
menuentry "Anix" {
	multiboot /boot/Anix.bin
}' >> ../grub.cfg

echo "---CREATE ISODIR---"

mkdir -p ../root/boot/grub
cp ../Anix.bin ../root/boot/Anix.bin
cp ../grub.cfg ../root/boot/grub/grub.cfg
grub-mkrescue -o ../Anix.iso ../root

echo "---OS LAUNCH IN QEMU ---"

qemu-system-i386 -cdrom ../Anix.iso

echo "SYSTEM LAUNCH SUCCESSFULLY !"

echo "
	-     |-    | ||  -   -
      |	 |    | -   |       -
     |	  |   |  -  | ||   - -
    |------|  |   - | ||  -   -
   |        | |    -| || -     -
"
