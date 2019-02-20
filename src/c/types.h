#define PACKED __attribute__((__packed__))
#define typeof __typeof__

#define bool _Bool
#define true 1
#define false 0

#define KB 1024
#define MB (1024 * 1024)

#ifndef _I386_TYPE_
#define _I386_TYPE_

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
typedef u64                 size_t;
typedef u64                 uintptr_t;
#endif
