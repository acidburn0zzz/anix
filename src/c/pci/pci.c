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
#include "pci.h"
#include "driver.c"
#include "registry.c"
#include "../lib.h"

static void PciVisit(uint bus, uint dev, uint func)
{
    uint id = PCI_MAKE_ID(bus, dev, func);

    PciDeviceInfo info;
    info.vendorId = PciRead16(id, PCI_CONFIG_VENDOR_ID);
    if (info.vendorId == 0xffff)
    {
        return;
    }

    info.deviceId = PciRead16(id, PCI_CONFIG_DEVICE_ID);
    info.progIntf = PciRead8(id, PCI_CONFIG_PROG_INTF);
    info.subclass = PciRead8(id, PCI_CONFIG_SUBCLASS);
    info.classCode = PciRead8(id, PCI_CONFIG_CLASS_CODE);
	
    printk("Found a %s with id: %d\n", PciClassName(info.classCode, info.subclass, info.progIntf), info.deviceId);
}

void PciInit()
{
    printk("PCI Initialization\n");
    for (uint bus = 0; bus < 256; ++bus)
    {
        for (uint dev = 0; dev < 32; ++dev)
        {
            uint baseId = PCI_MAKE_ID(bus, dev, 0);
            u8 headerType = PciRead8(baseId, PCI_CONFIG_HEADER_TYPE);
            uint funcCount = headerType & PCI_TYPE_MULTIFUNC ? 8 : 1;

            for (uint func = 0; func < funcCount; ++func)
            {
                PciVisit(bus, dev, func);
            }
        }
    }
}


