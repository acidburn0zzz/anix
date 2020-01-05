/*
 * Copyright (C) 2015 Philipp Oppermann
 * Copyright (C) 2018-2020 Nicolas Fouquet
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see https://www.gnu.org/licenses.
 */
use crate::memory::consts::KERNEL_HEAP_OFFSET;
use crate::HEAP_ALLOCATOR;

pub const HEAP_START: usize = KERNEL_HEAP_OFFSET.start;
pub const HEAP_SIZE: usize = KERNEL_HEAP_OFFSET.size;

pub fn init() {
    unsafe {
        HEAP_ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }
}
