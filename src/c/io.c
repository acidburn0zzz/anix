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

#include "types.h"

#ifndef IO
#define IO

//---Byte---

/* Read a byte on a port */
#define inb(port)({ \
	unsigned char _v;       \
	asm volatile ("inb %%dx, %%al" : "=a" (_v) : "d" (port)); \
        _v;     \
})

/* Write a byte on a port */
#define outb(port,value)({ \
	asm volatile ("outb %%al, %%dx" :: "d" (port), "a" (value)); \
})

//---16 bits---

/* Read a 16 bits word on a port */
#define inw(port)({ \
	u16 _v;			\
	asm volatile ("inw %%dx, %%ax" : "=a" (_v) : "d" (port)); \
        _v;			\
})

/* Write a 16 bits word on a port */
#define outw(port,value)({ \
	asm volatile ("outw %%ax, %%dx" :: "d" (port), "a" (value)); \
})

//---32 bits---

/* Write a 32 bits word on a port */
#define outl(port, data)({ \
    asm volatile("outl %0, %w1" : : "a" (data), "Nd" (port)); \
})

/* Read a 32 bits word on a port */
#define inl(port)({ \
    u32 data; \
    asm volatile("inl %w1, %0" : "=a" (data) : "Nd" (port)); \
    data; \
})

//---Array---

// Read array from port
#define insl(port, addr, cnt)({ \
  asm volatile("cld; rep insl"); \
})

// Write array to port
#define outsl(port, addr, cnt)({ \
  asm volatile("cld; rep outsl"); \
})

//---MSR---

// Get MSR
#define read_MSR(msr, lo, hi)({ \
  asm volatile("rdmsr"); \
})

//---Multiple sets of shorts---

//Write multiple sets of shorts
void outsm(unsigned short  port, unsigned char * data, unsigned long size) {
	asm volatile ("rep outsw" : "+S" (data), "+c" (size) : "d" (port));
}

//Read multiple sets of shorts
void insm(unsigned short  port, unsigned char * data, unsigned long size) {
	asm volatile ("rep insw" : "+D" (data), "+c" (size) : "d" (port) : "memory");
}

#endif
