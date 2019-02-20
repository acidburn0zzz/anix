#include "types.h"
#include "lib.h"
#include "kmalloc.h"

u32 kmalloc_used = 0;

void *kmalloc(unsigned long size)
{
	unsigned long realsize;	/* taille totale de l'enregistrement */
	struct kmalloc_header *chunk, *other;
	kern_heap = (char *) KERN_HEAP;

	if ((realsize = sizeof(struct kmalloc_header) + size) < KMALLOC_MINSIZE)
		realsize = KMALLOC_MINSIZE;

	/* 
	 * On recherche un bloc libre de 'size' octets en parcourant le HEAP
	 * kernel a partir du debut
	 */
	chunk = (struct kmalloc_header *) KERN_HEAP;
	/*while (chunk->used || chunk->size < realsize) {
		if (chunk->size == 0) {
			_print("PANIC: kmalloc(): corrupted chunk on %x with null size (heap %x) !\nSystem halted\n", chunk, kern_heap);
			asm("hlt");
		}

		chunk =
		    (struct kmalloc_header *) ((char *) chunk +
					       chunk->size);

		if (chunk == (struct kmalloc_header *) kern_heap) {
			if (ksbrk((realsize / PAGESIZE) + 1) < 0) {
				_print("PANIC: kmalloc(): no memory left for kernel !\nSystem halted\n");
				asm("hlt");
			}
		} else if (chunk > (struct kmalloc_header *) kern_heap) {
			print("PANIC: kmalloc(): chunk on %x while heap limit is on %x !\nSystem halted\n", chunk, kern_heap);
			asm("hlt");
		}
	}*/

	/* 
	 * On a trouve un bloc libre dont la taille est >= 'size'
	 * On fait de sorte que chaque bloc est une taille minimale
	 */
	if (chunk->size - realsize < KMALLOC_MINSIZE)
		chunk->used = 1;
	else {
		other =
		    (struct kmalloc_header *) ((char *) chunk + realsize);
		other->size = chunk->size - realsize;
		other->used = 0;

		chunk->size = realsize;
		chunk->used = 1;
	}

	kmalloc_used += realsize;

	/* retourne un pointeur sur la zone de donnees */
	return (char *) chunk + sizeof(struct kmalloc_header);
}

void kfree(void *v_addr)
{
	struct kmalloc_header *chunk, *other;
	kern_heap = (char *) KERN_HEAP;

	/* On libere le bloc alloue */
	chunk =
	    (struct kmalloc_header *) (v_addr -
				       sizeof(struct kmalloc_header));
	chunk->used = 0;

	kmalloc_used -= chunk->size;

	/* 
	 * On merge le bloc nouvellement libere avec le bloc suivant ci celui-ci
	 * est aussi libre
	 */
	while ((other =
		(struct kmalloc_header *) ((char *) chunk + chunk->size))
	       && other < (struct kmalloc_header *) kern_heap
	       && other->used == 0)
		chunk->size += other->size;
}
