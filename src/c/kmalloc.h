#include "types.h"

#define KMALLOC_MINSIZE		16
#define KERN_HEAP		0x10000000
#define	PAGESIZE 	4096

char *kern_heap;

struct kmalloc_header {
	unsigned long size:31;	/* taille totale de l'enregistrement */
	unsigned long used:1;
} __attribute__ ((packed));

void *kmalloc(unsigned long);
void kfree(void *);
