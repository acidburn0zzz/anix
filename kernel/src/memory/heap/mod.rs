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

use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicUsize, Ordering};
use spin::Mutex;

use crate::memory::consts::KERNEL_HEAP_OFFSET;

pub const HEAP_START: usize = KERNEL_HEAP_OFFSET.start;
pub const HEAP_SIZE: usize = KERNEL_HEAP_OFFSET.size;


/// A simple allocator that allocates memory linearly and ignores freed memory.
#[derive(Debug)]
pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: AtomicUsize,
}

struct LockedBumpAllocator(Mutex<BumpAllocator>);

impl BumpAllocator {
    pub const fn new(heap_start: usize, heap_end: usize) -> Self {
        // NOTE: requires adding #![feature(const_atomic_usize_new)] to lib.rs
        Self { heap_start, heap_end, next: AtomicUsize::new(heap_start) }
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        loop {
            // load current state of the `next` field
            let current_next = self.next.load(Ordering::Relaxed);
            let alloc_start = align_up(current_next, layout.align());
            let alloc_end = alloc_start.saturating_add(layout.size());

            if alloc_end <= self.heap_end {
                // update the `next` pointer if it still has the value `current_next`
                let next_now = self.next.compare_and_swap(current_next, alloc_end,
                    Ordering::Relaxed);
                if next_now == current_next {
                    // next address was successfully updated, allocation succeeded
                    return alloc_start as *mut u8;
                }
            } else {
                //return AllocErr::Exhausted{ request: layout }
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // do nothing, leak memory
    }
}
/// Align downwards. Returns the greatest x with alignment `align`
/// so that x <= addr. The alignment must be a power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {
    if align.is_power_of_two() {
        addr & !(align - 1)
    } else if align == 0 {
        addr
    } else {
        panic!("`align` must be a power of 2");
    }
}

/// Align upwards. Returns the smallest x with alignment `align`
/// so that x >= addr. The alignment must be a power of 2.
pub fn align_up(addr: usize, align: usize) -> usize {
    align_down(addr + align - 1, align)
}

unsafe impl<'a> GlobalAlloc for &'a LockedBumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.lock().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.lock().dealloc(ptr, layout)
    }
}
