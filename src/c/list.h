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

#ifndef _LIST_H_
#define _LIST_H_

#define LIST_MAGIC  0xC0FFBABE

typedef struct list {

  uint32_t  magic;
  void*     data;

  struct list* next;
} list_t;

typedef uint8_t (*list_iterator_t)(list_t* l, void* udata);

list_t*
list_add(list_t* l, void* data);

list_t*
list_get(list_t* l, uint32_t idx);

list_t*
list_foreach(list_t* l, list_iterator_t it, void* udata);

#endif
