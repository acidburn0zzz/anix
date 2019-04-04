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

#ifndef KMALLOC
#define KMALLOC

#include "types.h"

#define KMALLOC_MINSIZE		16
#define KERN_HEAP		0x10000000
#define	PAGESIZE 	4096
#define KERN_HEAP_LIM		0x40000000

char *kern_heap;

struct kmalloc_header {
	unsigned long size:31;	/* taille totale de l'enregistrement */
	unsigned long used:1;
} __attribute__ ((packed));

void *ksbrk(int);
void *kmalloc(unsigned long);
void kfree(void *);
#endif
