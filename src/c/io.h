#include "types.h"

/* desactive les interruptions */
#define cli asm("cli"::)

/* reactive les interruptions */
#define sti asm("sti"::)

/* ecrit un octet sur un port */
#define outb(port,value) \
	asm volatile ("outb %%al, %%dx" :: "d" (port), "a" (value));

/* ecrit un octet sur un port et marque une temporisation  */
#define outbp(port,value) \
	asm volatile ("outb %%al, %%dx; jmp 1f; 1:" :: "d" (port), "a" (value));

/* lit un octet sur un port */
#define inb(port) ({    \
	unsigned char _v;       \
	asm volatile ("inb %%dx, %%al" : "=a" (_v) : "d" (port)); \
        _v;     \
})

/* ecrit un mot de 16 bits sur un port */
#define outw(port,value) \
	asm volatile ("outw %%ax, %%dx" :: "d" (port), "a" (value));

/* lit un mot de 16 bits sur un port */
#define inw(port) ({		\
	u16 _v;			\
	asm volatile ("inw %%dx, %%ax" : "=a" (_v) : "d" (port));	\
        _v;			\
})

#define outl(port, data) \
    asm volatile("outl %0, %w1" : : "a" (data), "Nd" (port));

#define inl(port)({ \
    u32 data; \
    asm volatile("inl %w1, %0" : "=a" (data) : "Nd" (port)); \
    data; \
})
