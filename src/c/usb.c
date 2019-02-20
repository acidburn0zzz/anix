#include "controller.c"
#include "dev.c"

// ------------------------------------------------------------------------------------------------
void UsbPoll()
{
	//printk("UsbPoll");
    for (UsbController *c = g_usbControllerList; c; c = c->next)
    {
        if (c->poll){
			kattr = 0x80;
			//printk("c->poll");
            c->poll(c);
        }
    }

    for (UsbDevice *dev = g_usbDeviceList; dev; dev = dev->next)
    {
        if (dev->drvPoll)
        {
			kattr = 0x67;
			//printk("dev->drvPoll");
            dev->drvPoll(dev);
        }
    }
}
