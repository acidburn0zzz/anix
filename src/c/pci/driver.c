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
#include "driver.h"
#include "../io.c"

// ------------------------------------------------------------------------------------------------
u8 PciRead8(uint id, uint reg)
{
    u32 addr = 0x80000000 | id | (reg & 0xfc);
    outl(PCI_CONFIG_ADDR, addr);
    return inb(PCI_CONFIG_DATA + (reg & 0x03));
}

// ------------------------------------------------------------------------------------------------
u16 PciRead16(uint id, uint reg)
{
    u32 addr = 0x80000000 | id | (reg & 0xfc);
    outl(PCI_CONFIG_ADDR, addr);
    return inw(PCI_CONFIG_DATA + (reg & 0x02));
}

// ------------------------------------------------------------------------------------------------
u32 PciRead32(uint id, uint reg)
{
    u32 addr = 0x80000000 | id | (reg & 0xfc);
    outl(PCI_CONFIG_ADDR, addr);
    return inl(PCI_CONFIG_DATA);
}

// ------------------------------------------------------------------------------------------------
void PciWrite8(uint id, uint reg, u8 data)
{
    u32 address = 0x80000000 | id | (reg & 0xfc);
    outl(PCI_CONFIG_ADDR, address);
    outb(PCI_CONFIG_DATA + (reg & 0x03), data);
}

// ------------------------------------------------------------------------------------------------
void PciWrite16(uint id, uint reg, u16 data)
{
    u32 address = 0x80000000 | id | (reg & 0xfc);
    outl(PCI_CONFIG_ADDR, address);
    outw(PCI_CONFIG_DATA + (reg & 0x02), data);
}

// ------------------------------------------------------------------------------------------------
void PciWrite32(uint id, uint reg, u32 data)
{
    u32 address = 0x80000000 | id | (reg & 0xfc);
    outl(PCI_CONFIG_ADDR, address);
    outl(PCI_CONFIG_DATA, data);
}

// ------------------------------------------------------------------------------------------------
static void PciReadBar(uint id, uint index, u32 *address, u32 *mask)
{
    uint reg = PCI_CONFIG_BAR0 + index * sizeof(u32);

    // Get address
    *address = PciRead32(id, reg);

    // Find out size of the bar
    PciWrite32(id, reg, 0xffffffff);
    *mask = PciRead32(id, reg);

    // Restore adddress
    PciWrite32(id, reg, *address);
}

// ------------------------------------------------------------------------------------------------
void PciGetBar(PciBar *bar, uint id, uint index)
{
    // Read pci bar register
    u32 addressLow;
    u32 maskLow;
    PciReadBar(id, index, &addressLow, &maskLow);

    if (addressLow & PCI_BAR_64)
    {
        // 64-bit mmio
        u32 addressHigh;
        u32 maskHigh;
        PciReadBar(id, index + 1, &addressHigh, &maskHigh);

        bar->u.address = (void *)(((uintptr_t)addressHigh << 32) | (addressLow & ~0xf));
        bar->size = ~(((u64)maskHigh << 32) | (maskLow & ~0xf)) + 1;
        bar->flags = addressLow & 0xf;
    }
    else if (addressLow & PCI_BAR_IO)
    {
        // i/o register
        bar->u.port = (u16)(addressLow & ~0x3);
        bar->size = (u16)(~(maskLow & ~0x3) + 1);
        bar->flags = addressLow & 0x3;
    }
    else
    {
        // 32-bit mmio
        bar->u.address = (void *)(uintptr_t)(addressLow & ~0xf);
        bar->size = ~(maskLow & ~0xf) + 1;
        bar->flags = addressLow & 0xf;
    }
}
