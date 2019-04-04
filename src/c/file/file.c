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

#include "../lib.h"
#include "../list.h"
#include "../kmalloc.h"
#include "file.h"


/*
 * init_root(): initialise la structure de fichier decrivant la racine.
 */
struct file *init_root(struct disk *disk, u32 lba)
{
	struct file *fp;
	
	printk("\nInit root:");
	fp = (struct file *) kmalloc(sizeof(struct file));

	printk("\n   -Create name");
	fp->name =  kmalloc(sizeof("/"));
	fp->name = (char *) "/";
	ok();

	printk("\n   -Create disk");
	fp->disk = disk;
	ok();
	
	printk("\n   -Create inum");
	fp->inum = EXT2_INUM_ROOT;
	ok();
	
	printk("\n   -Create inode");
	fp->inode = read_inode(disk, fp->inum, lba);
	ok();
	
	printk("\n   -Create mmap");
	fp->mmap = 0;
	ok();
	
	printk("\n   -Create parent");
	fp->parent = fp;
	ok();
	
	//INIT_LIST_HEAD(&fp->leaf);
	//printk("\n   -Get root directories");
	//get_dir_entries(fp, lba);
	//ok();

	//INIT_LIST_HEAD(&fp->sibling);
	
	return fp;
}
