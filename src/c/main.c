#include "types.h"
#include "lib.c"
#include "io.h"
#include "screen.c"
#include "kmalloc.h"
#include "dev.h"
#include "controller.h"
#include "usb.c"
#include "registry.h"
#include "pci.c"

void test(int row, int col){
	kattr = 0x4E;
	kX = 0;
	kY = 17;
	print("In test");
	
	//int i = 0;
	//int j = 0;
	
	/*for(i = 0; i < 10; i++){
		for(j = 0; j < 10; j++){
 			u32 d = PciRead32(i, j);
			if((j + i) % 8 == 0){
				printk("\n");
			}
			printk("%d ", d);
		}
	}*/
	for(;;){
		UsbPoll();
	}
}

void lspci(){
	PciInit();
}
