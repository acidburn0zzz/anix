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

#ifndef TYPES
#define TYPES

#define PACKED __attribute__((__packed__))
#define typeof __typeof__

#define bool _Bool
#define true 1
#define false 0

#define KB 1024
#define MB (1024 * 1024)

typedef unsigned char u8;
typedef unsigned short u16;
typedef unsigned int u32;
typedef unsigned long long u64;

typedef char                i8;
typedef short               i16;
typedef int                 i32;
typedef long long           i64;

typedef unsigned char uint8_t;
typedef unsigned short uint16_t;
typedef unsigned int uint32_t;
typedef unsigned int uint64_t;
typedef unsigned char uchar;

typedef float               f32;
typedef double              f64;

typedef unsigned int        uint;
typedef u64                 uintptr_t;
typedef unsigned int   size_t;

#ifndef NULL
# define NULL  (void*)0
#endif

#ifndef false
# define false 0
#endif

#ifndef FALSE
# define FALSE 0
#endif

#ifndef true
# define true  1
#endif

#ifndef TRUE
# define TRUE  1
#endif

typedef char __attribute__((__may_alias__))             int8_t;
typedef short __attribute__((__may_alias__))            int16_t;
typedef int __attribute__((__may_alias__))              int32_t;
typedef unsigned char __attribute__((__may_alias__))    uint8_t;
typedef unsigned short __attribute__((__may_alias__))   uint16_t;
typedef unsigned int __attribute__((__may_alias__))     uint32_t;

typedef struct {
  uint  year;
  uint  month;
  uint  day;
  uint  hour;
  uint  minute;
  uint  second;
 } time_t;
 
 typedef struct {
    uint32_t edi, esi, ebp, esp, ebx, edx, ecx, eax;
    uint32_t gs, fs, es, ds;
    uint32_t intn, err_code;
    uint32_t eip, cs, eflags, useresp, ss;
} int_regs_t;

typedef uint8_t BYTE;
typedef uint16_t WORD;
typedef uint32_t DWORD;
typedef uint64_t QWORD;
#endif
