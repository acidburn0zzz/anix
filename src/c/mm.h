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

#ifndef MM
#define MM

#include "types.h"
#include "list.h"

#define	PAGESIZE 	4096
#define	RAM_MAXSIZE	0x100000000
#define	RAM_MAXPAGE	0x100000

#define IDTSIZE		0xFF	/* nombre max. de descripteurs dans la table */
#define GDTSIZE		0xFF	/* nombre max. de descripteurs dans la table */

#define IDTBASE		0x00000000	/* addr. physique ou doit resider la IDT */
#define GDTBASE		0x00000800	/* addr. physique ou doit resider la gdt */

#define	KERN_PDIR		0x00001000
#define	KERN_STACK		0x0009FFF0
#define	KERN_BASE		0x00100000
#define KERN_PG_HEAP		0x00800000
#define KERN_PG_HEAP_LIM	0x10000000
#define KERN_HEAP		0x10000000
#define KERN_HEAP_LIM		0x40000000

#define	USER_OFFSET 		0x40000000
#define	USER_STACK 		0xE0000000

#define	VADDR_PD_OFFSET(addr)	((addr) & 0xFFC00000) >> 22
#define	VADDR_PT_OFFSET(addr)	((addr) & 0x003FF000) >> 12
#define	VADDR_PG_OFFSET(addr)	(addr) & 0x00000FFF
#define PAGE(addr)		(addr) >> 12

#define	PAGING_FLAG 	0x80000000	/* CR0 - bit 31 */
#define PSE_FLAG	0x00000010	/* CR4 - bit 4  */

#define PG_PRESENT	0x00000001	/* page directory / table */
#define PG_WRITE	0x00000002
#define PG_USER		0x00000004
#define PG_4MB		0x00000080


#ifndef __MM_STRUCT__
#define __MM_STRUCT__

/* Structures generiques */
struct page {
	char *v_addr;
	char *p_addr;
	struct list_head list;
};

struct page_directory {
	struct page *base;
	struct list_head pt;
};

struct vm_area {
	char *vm_start;	
	char *vm_end;	/* exclude */
	struct list_head list;
};

#endif


/* Pointe sur le sommet du heap noyau */
char *kern_heap;

/* Pointe sur le debut de la liste des pages libres du noyau */
struct list_head kern_free_vm;


#ifdef __MM__
	u32 *pd0 = (u32 *) KERN_PDIR;	/* kernel page directory */
	char *pg0 = (char *) 0;		/* kernel page 0 (4MB) */
	char *pg1 = (char *) 0x400000;	/* kernel page 1 (4MB) */
	char *pg1_end = (char *) 0x800000;	/* limite de la page 1 */
	u8 mem_bitmap[RAM_MAXPAGE / 8];	/* bitmap allocation de pages (1 Go) */

	u32 kmalloc_used = 0;
#else
	u32 *pd0;
	extern u8 mem_bitmap[];

	u32 kmalloc_used;
#endif


/* Marque une page comme utilisee / libre dans le bitmap */
#define set_page_frame_used(page)	mem_bitmap[((u32) page)/8] |= (1 << (((u32) page)%8))
#define release_page_frame(p_addr)	mem_bitmap[((u32) p_addr/PAGESIZE)/8] &= ~(1 << (((u32) p_addr/PAGESIZE)%8))

/* Selectionne une page libre dans le bitmap */
char *get_page_frame(void);

/* Selectionne / libere une page libre dans le bitmap et l'associe a une page
 * virtuelle libre du heap */
struct page *get_page_from_heap(void);
int release_page_from_heap(char *);

/* Initialise les structures de donnees de gestion de la memoire */
void init_mm(u32);

/* Cree un repertoire de page pour un processus */
struct page_directory *pd_create(void);
int pd_destroy(struct page_directory *);

/* Ajoute une entree dans l'espace du noyau */
int pd0_add_page(char *, char *, int);

/* Ajoute / enleve une entree dans le repertoire de pages courant */
int pd_add_page(char *, char *, int, struct page_directory *);
int pd_remove_page(char *);

/* Retourne l'adresse physique associee a une adresse virtuelle */
char *get_p_addr(char *);
#endif
