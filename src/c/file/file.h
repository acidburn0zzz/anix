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

#include "../fs/ext2/ext2.h"
#include "../list.h"

struct file {
	struct disk *disk;
	u32 inum;		/* inode number */
	char *name;		/* file name */
	struct inode *inode;
	char *mmap;		/* buffer (if opened) */
	int opened;		/* number of process that have opened the file */

	struct file *parent;	/* parent directory */
	struct list_head leaf;	/* child files linked list */
	struct list_head sibling;	/* siblings */
};

struct open_file {
	struct file *file;	/* descripteur de fichier */
	u32 ptr;		/* pointeur de lecture dans le fichier */
	struct open_file *next;
};

struct file *root;


struct file *init_root(struct disk *, u32);
int is_directory(struct file *, u32);
struct file *is_cached_leaf(struct file *, char *);
int get_dir_entries(struct file *, u32);
struct file *path_to_file(char *, u32);
