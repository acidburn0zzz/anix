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
#ifndef INITRD_H
#define INITRD_H

#include "../vfs/vfs.h"
#include "../../types.h"

typedef struct
{
    u32 nfiles; // The number of files in the ramdisk.
} initrd_header_t;

typedef struct
{
    u8 magic;     // Magic number, for error checking.
    char name[64];  // Filename.
    char content[256]; // File content
    u32 offset;   // Offset in the initrd that the file starts.
    u32 length;   // Length of the file.
} initrd_file_header_t;

// Initialises the initial ramdisk. It gets passed the address of the multiboot module,
// and returns a completed filesystem node.
fs_node_t *initialise_initrd(u32 location);

#endif
