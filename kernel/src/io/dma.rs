/*
 * Copyright (C) 2016 Redox OS Developers
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

use core::{mem, ptr};
use core::ops::{Deref, DerefMut};
use crate::errors::Result;

#[allow(dead_code)]
struct PhysBox {
    address: usize,
    size: usize,
}

impl PhysBox {
    fn new(size: usize) -> Result<PhysBox> {
        let address = crate::physalloc()?;
        Ok(PhysBox {
            address: address,
            size: size,
        })
    }
}

impl Drop for PhysBox {
    fn drop(&mut self) {
        // let _ = unsafe { crate::physfree(self.address, self.size) };
    }
}

pub struct Dma<T> {
    phys: PhysBox,
    virt: *mut T
}

impl<T> Dma<T> {
    pub fn new(value: T) -> Result<Dma<T>> {
        let phys = PhysBox::new(mem::size_of::<T>())?;

        let virt = /*unsafe { crate::physmap(phys.address, phys.size, crate::PHYSMAP_WRITE)? } as *mut T*/phys.address as *mut T;

        unsafe { ptr::write(virt, value); }

        Ok(Dma {
            phys: phys,
            virt: virt,
        })
    }

    pub fn zeroed() -> Result<Dma<T>> {
        let phys = PhysBox::new(mem::size_of::<T>())?;
        let virt = /*unsafe { crate::physmap(phys.address, phys.size, crate::PHYSMAP_WRITE)? } as *mut T*/phys.address as *mut T;
        // unsafe { ptr::write_bytes(virt as *mut u8, 0, phys.size); }
        // TODO: Implement virt by convert phys addr to virt addr (see https://stackoverflow.com/questions/40292822/translate-virtual-address-to-physical-address) (now, Anix does not support mapping outside src/memory/mod.rs)
        Ok(Dma {
            phys: phys,
            virt: virt,
        })
    }

    pub fn physical(&self) -> usize {
        self.phys.address
    }
}

impl<T> Deref for Dma<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.virt }
    }
}

impl<T> DerefMut for Dma<T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.virt }
    }
}

impl<T> Drop for Dma<T> {
    fn drop(&mut self) {
        // unsafe { drop(ptr::read(self.virt)); }
        // let _ = unsafe { crate::physunmap(self.virt as usize) };
    }
}
