/*Copyright (C) 2018-2019 Nicolas Fouquet 

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see https://www.gnu.org/licenses.
*/

#include "types.h"
#include "screen.h"
#include "kmalloc.h"

#include "pci/registry.h"
#include "pci/pci.c"

#include "fs/ext2/ext2.h"
#include "disk/disk.c"
#include "file/file.h"

#include "fs/initrd/initrd.c"
#include "fs/initrd/initrd.h"
#include "fs/vfs/vfs.h"

u32 initrd_start;

void ok(){
	int tmp_kattr = kattr;
	kattr = new_color(LightGreen, Black);
	
	printk(" [ OK ]");
	
	//Restore color
	kattr = tmp_kattr;
}

void test_fs(int row, int col, int color){
	kattr = color;
	kX = col;
	kY = row;
	
	printk("\nInitialize initrd");
	fs_node_t *fs_root = initialise_initrd(initrd_start);
	
	int i = 0;
	struct dirent *node = 0;
	
	while ((node = initrd_readdir(fs_root, i)) != 0){
		printk("\nFound file ");
		printk(node->name);
		fs_node_t *fsnode = initrd_finddir(fs_root, node->name);

		if ((fsnode->flags&0x7) == FS_DIRECTORY)
		{
			printk("(directory)");
		}
		else
		{
			char buf[256];
			read_fs(fsnode, 0, 0, buf);
			printk("\nContent: %s", buf);
		}
		i++;
	}
	
	/*
	//TODO: Ext2 file management
	struct partition *p1;
	struct disk *hd;
	struct file *root;
	
	p1 = (struct partition *) kmalloc(sizeof(struct partition));
	disk_read(0, 0x01BE, (char *) p1, 16);
	//TODO: Global variable -lba-
	//TODO: Instead of lba parameter put -partition-
	
	hd = create_disk(0, p1);
	
	root = init_root(hd, hd->part->s_lba);
	
	print_inode(root->inode);*/
}

void lspci(int row, int col, int color){
	kattr = color;
	kX = col;
	kY = row;
	PciInit();
}

void set_initrd_addr_start(u32 addr){
	initrd_start = addr;
}
