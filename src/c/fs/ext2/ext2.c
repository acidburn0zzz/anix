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

#include "ext2.h"
#include "../../disk/disk.h"
#include "../../kmalloc.h"
#include "../../lib.h"
#include "../../screen.h"

struct disk *create_disk(int device, struct partition *part)
{
	int i, j;
	struct disk *hd;

	printk("\nCreate disk:");
	hd = (struct disk *) kmalloc(sizeof(struct disk));
	ok();

	printk("\n   -Create name");
	hd->device = device;
	ok();
	
	printk("\n   -Create partition");
	hd->part = part;
	ok();
	
	printk("\n   -Create superblock");
	hd->sb = read_superblock(hd, part->s_lba * 512);
	ok();
	
	printk("\n   -Create hd->blocksize");
	hd->blocksize = 1024;
	printk("\n      -Blocksize: %d", hd->blocksize);
	ok();

	i = (hd->sb->s_blocks_count / hd->sb->s_blocks_per_group) +
	    ((hd->sb->s_blocks_count % hd->sb->s_blocks_per_group) ? 1 : 0);
	j = (hd->sb->s_inodes_count / hd->sb->s_inodes_per_group) +
	    ((hd->sb->s_inodes_count % hd->sb->s_inodes_per_group) ? 1 : 0);
		
	printk("\n   -Create groups");
	hd->groups = (i > j) ? i : j;
	ok();

	printk("\n   -Create group descriptor");
	hd->gd = read_group_descriptor(hd, part->s_lba * 512);
	ok();

	return hd;
}

struct superblock *read_superblock(struct disk *hd, int s_part)
{
	struct superblock *sb;

	sb = (struct superblock *) kmalloc(sizeof(struct superblock));
	disk_read(hd->device, s_part + 1024, (char *) sb, sizeof(struct superblock));

	return sb;
}

struct group_descriptor *read_group_descriptor(struct disk *hd, int s_part)
{
	struct group_descriptor *gd;
	int offset, gd_size;


	offset = (hd->blocksize == 1024) ? 2048 : hd->blocksize;


	gd_size = hd->groups * sizeof(struct group_descriptor);


	gd = (struct group_descriptor *) kmalloc(gd_size);

	disk_read(hd->device, s_part + offset, (char *) gd, gd_size);

	return gd;
}


struct inode *read_inode(struct disk *hd, int i_num, u32 lba){
	int gr_num, index, offset;
	struct inode *inode;
	
	hd->blocksize = 1024;
	hd->device = 0;
	hd->sb->s_inodes_per_group = 1712;
	
	inode = (struct inode *) kmalloc(sizeof(struct inode));

	/* groupe qui contient l'inode */
	gr_num = (i_num - 1) / hd->sb->s_inodes_per_group;
	
	/* index de l'inode dans le groupe */
	index = (i_num - 1) % hd->sb->s_inodes_per_group;
	
	/* offset de l'inode sur le disk */
	offset = hd->gd[gr_num].bg_inode_table * hd->blocksize + index * hd->sb->s_inode_size;
	
	printk("\nFULLY DEBUG (READ INODE):");
	printk("\n   -gr_num: %d", gr_num);
	printk("\n   -index: %d", index);
	printk("\n   -offset: %d", offset);
	printk("\n   -i_num: %d", i_num);
	printk("\n   -hd->sb->s_inodes_per_group: %d", hd->sb->s_inodes_per_group);
	printk("\n   -hd->gd[gr_num].bg_inode_table: %d", hd->gd[gr_num].bg_inode_table);
	printk("\n   -hd->blocksize: %d", hd->blocksize);
	printk("\n   -hd->sb->s_inode_size: %d", hd->sb->s_inode_size);
	printk("\n   -hd->device: %d", hd->device);
	printk("\n   -(lba * 512) + offset: %d", (lba * 512) + offset);
	
	disk_read(hd->device, (lba * 512) + offset, (char *) inode, hd->sb->s_inode_size);
	
	return inode;
}

char print_mode(int mode){
	switch(mode) {
		case EXT2_S_IFREG:
			printk("file");
			break;
			
		case EXT2_S_IFDIR:
			printk("directory");
			break;
			
		case EXT2_S_IFMT:
			printk("format mask");
			break;
		
		case EXT2_S_IFSOCK:
			printk("socket");
			break;
			
		case EXT2_S_IFLNK:
			printk("symbolic link");
			break;
		
		case EXT2_S_IFBLK:
			printk("block_device");
			break;
			
		case EXT2_S_IFCHR:
			printk("character device");
			break;
			
		case EXT2_S_IFIFO:
			printk("fifo");
			break;
		
		default :
			printk("strange mode %d", mode);
	}
}
