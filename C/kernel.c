/*Copyright (C) 2018 Nicolas Fouquet

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see https://www.gnu.org/licenses.
*/

#include "lib/strings.h"
#include "lib/screen.h"
#include "lib/input.h"

#include "lib/stdio.h"
#include "lib/stdlib.h"
#include "lib/stdbool.h"
#include "lib/stddef.h"
#include "lib/stdint.h"

//#include "lib/filesystem.h"

#define INT_DISABLE 0
#define INT_ENABLE  0x200
#define PIC1 0x20
#define PIC2 0xA0

#define ICW1 0x11
#define ICW4 0x01

void outb( unsigned short port, unsigned char val )
{
   asm volatile("outb %0, %1" : : "a"(val), "Nd"(port) );
}

static __inline unsigned char inb (unsigned short int port)
{
  unsigned char _v;

  __asm__ __volatile__ ("inb %w1,%0":"=a" (_v):"Nd" (port));
  return _v;
}

void init_pics(int pic1, int pic2)
{
   /* send ICW1 */
   outb(PIC1, ICW1);
   outb(PIC2, ICW1);

   /* send ICW2 */
   outb(PIC1 + 1, pic1);   
   outb(PIC2 + 1, pic2);   

   /* send ICW3 */
   outb(PIC1 + 1, 4);   
   outb(PIC2 + 1, 2);

   /* send ICW4 */
   outb(PIC1 + 1, ICW4);
   outb(PIC2 + 1, ICW4);

   /* disable all IRQs */
   outb(PIC1 + 1, 0xFF);
}

/*irrelevant code*/

#if defined(__cplusplus)
extern "C" /* Use C linkage for kernel_main. */
#endif

void kernel_main(void) 
{
	/* Initialize terminal interface */
	terminal_initialize();
    
    int c = 0;
    init_pics(0x20, 0x28);
    do
    {

    if(inb(0x60)!=c) //PORT FROM WHICH WE READ
    {
        c = inb(0x60);
        if(c>0)
            {
                scanCodeToChar(c);
            }
        }


    }
    while(c!=1); // 1= ESCAPE
}  
