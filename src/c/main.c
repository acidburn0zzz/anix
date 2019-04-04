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

#include "fs/initrd/initrd.h"
#include "fs/vfs/vfs.h"

u32 initrd_start;

void print_file(fs_node_t *file){
	int tmp_kattr = kattr;
	kattr = new_color(Yellow, Black);
	
    printk("\nFILE:");
	printk("\n   -Name: %s", file->name);
    printk("\n   -Mask: %d", file->mask);
    printk("\n   -Uid: %d", file->uid);
    printk("\n   -Gid: %d", file->gid);
	
	//File or directory?
	char type[10];
	if(file->flags == FS_DIRECTORY){
		strcpy(type, "directory");
	}
	if(file->flags == FS_FILE){
		strcpy(type, "file");
	}
	else{
		strcpy(type, "unknown");
	}
	
    printk("\n   -Flags : %d -> %s", file->flags, type);
    printk("\n   -Inode: %d", file->inode);
    printk("\n   -Length: %d", file->length);
    printk("\n   -Impl: %d", file->impl);
	//Restore color
	kattr = tmp_kattr;
}

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
	fs_root = initialise_initrd(initrd_start);
	
	int i = 0;
	struct dirent *node = 0;
	while ( (node = readdir_fs(fs_root, i)) != 0){
		printk("\nFound file ");
		printk(node->name);
		fs_node_t *fsnode = finddir_fs(fs_root, node->name);

		if ((fsnode->flags&0x7) == FS_DIRECTORY)
		{
			printk("\n\t(directory)\n");
		}
		else
		{
			printk("\n\t contents: \"");
			char buf[256];
			u32 sz = read_fs(fsnode, 0, 256, buf);
			int j;
			for (j = 0; j < sz; j++){
				printk(buf[j]);
			}
			
			printk("\"\n");
		}
		i++;
	}
	
	/*int i = 0;
	struct dirent *node = 0;
	while ( (node = readdir_fs(fs_root, i)) != 0){
		printk("Found file ");
		printk(node->name);
		fs_node_t *fsnode = finddir_fs(fs_root, node->name);

		if ((fsnode->flags&0x7) == FS_DIRECTORY)
		{
			printk("\n\t(directory)\n");
		}
		else
		{
			printk("\n\t contents: \"");
			char buf[256];
			u32 sz = read_fs(fsnode, 0, 256, buf);
			int j;
			for (j = 0; j < sz; j++){
				printk(buf[j]);
			}
			
			printk("\"\n");
		}
		i++;
	}*/
    
	/*struct partition *p1;
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
