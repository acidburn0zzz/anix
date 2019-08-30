/*
 * Copyright (C) 2018-2019 Nicolas Fouquet
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 2 of the License, or
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
use core::ffi::*;

/// Stop the computer
pub fn hlt_loop() -> ! {
    loop{
        x86_64::instructions::hlt();
    }
}

/// Read a slice ($src) on the number of bytes in the type ($ty)
#[macro_export]
macro_rules! read_num_bytes {
    ($ty:ty, $src:expr) => ({
        let size = core::mem::size_of::<$ty>();
        assert!(size <= $src.len());
        let mut data: $ty = 0;
        unsafe {
            copy_nonoverlapping(
                $src.as_ptr(),
                &mut data as *mut $ty as *mut u8,
                size);
        }
        data
    });
}

/// Core and Alloc crate use the bcmp function but it is not defined
#[no_mangle]
pub extern "C" fn bcmp(_s1: *const c_void, _s2: *const c_void, _n: usize) -> i32 {
    0
}
