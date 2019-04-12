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
#include "initrd.h"

initrd_header_t *initrd_header;     // The header.
initrd_file_header_t *file_headers; // The list of file headers.
fs_node_t *root_nodes;              // List of file nodes.
int nroot_nodes;                    // Number of file nodes.

struct dirent dirent;

static u32 initrd_read(fs_node_t *node, u32 offset, u32 size, char buffer[256]){
	int y;
	for(y = 0; y < node->length; y++){
		buffer[y] = node->content[y];
	}
	return 0;
}

static struct dirent *initrd_readdir(fs_node_t *node, u32 index)
{
    if (index >= nroot_nodes){
		return 0;
	}
	
    strcpy(dirent.name, root_nodes[index].name);
    dirent.name[strlen(root_nodes[index].name)] = 0;
    dirent.ino = root_nodes[index].inode;
    return &dirent;
}

static fs_node_t *initrd_finddir(fs_node_t *node, char *name)
{
    int i;
    for (i = 0; i < nroot_nodes; i++)
        if (!strcmp(name, root_nodes[i].name))
            return &root_nodes[i];
    return 0;
}

fs_node_t *initialise_initrd(u32 location)
{
    // Initialise the main and file header pointers and populate the root directory.
    initrd_header = (initrd_header_t *)location;
    file_headers = (initrd_file_header_t *) (location+sizeof(initrd_header_t));

	// Initialise the root directory.
    fs_node_t *initrd_root = (fs_node_t*)kmalloc(sizeof(fs_node_t));
    strcpy(initrd_root->name, "initrd");
    initrd_root->mask = initrd_root->uid = initrd_root->gid = initrd_root->inode = initrd_root->length = 0;
    initrd_root->flags = FS_DIRECTORY;
    initrd_root->read = 0;
    initrd_root->write = 0;
    initrd_root->open = 0;
    initrd_root->close = 0;
    initrd_root->readdir = &initrd_readdir;
    initrd_root->finddir = &initrd_finddir;
    initrd_root->ptr = 0;
    initrd_root->impl = 0;
	
    root_nodes = (fs_node_t*)kmalloc(sizeof(fs_node_t) * initrd_header->nfiles);
    nroot_nodes = initrd_header->nfiles;

    // For every file...
    int i;
    for (i = 0; i < initrd_header->nfiles; i++)
    {
        file_headers[i].offset += location;
        // Create a new file node.
        strcpy(root_nodes[i].name, &file_headers[i].name);
        root_nodes[i].mask = root_nodes[i].uid = root_nodes[i].gid = 0;
        root_nodes[i].length = file_headers[i].length;
        root_nodes[i].inode = i;
        root_nodes[i].flags = FS_FILE;
        root_nodes[i].read = &initrd_read;
        root_nodes[i].write = 0;
        root_nodes[i].readdir = 0;
        root_nodes[i].finddir = 0;
        root_nodes[i].open = 0;
        root_nodes[i].close = 0;
        root_nodes[i].impl = 0;
		
		int j;
		//For content, read the content field
		for(j = 0; j < file_headers[i].length; j++){
			root_nodes[i].content[j] = file_headers[i].content[j];
		}
    }
    
    return initrd_root;
}
