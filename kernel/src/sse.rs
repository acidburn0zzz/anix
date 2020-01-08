/*
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

/// Enables Streaming SIMD Extensions (SSE) support for loaded kernels.
pub fn enable_sse() {
    use bit_field::BitField;
    use x86_64::registers::control::Cr0;
    let mut flags = Cr0::read_raw();
    flags.set_bit(2, false);
    flags.set_bit(1, true);
    flags.set_bit(9, true);
    flags.set_bit(10, true);
    unsafe {
        Cr0::write_raw(flags);
    }
    // For now, we must use inline ASM here
    let mut cr4: u64;
    unsafe {
        asm!("mov %cr4, $0" : "=r" (cr4));
    }
    cr4.set_bit(9, true);
    cr4.set_bit(10, true);
    unsafe {
        asm!("mov $0, %cr4" :: "r" (cr4) : "memory");
    }
}
