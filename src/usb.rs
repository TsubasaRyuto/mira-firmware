use rp2040_hal::{usb::UsbBus};
use usb_device::descriptor::lang_id::LangID;
use usb_device::{class_prelude::*, device::StringDescriptors, prelude::*};
use usbd_hid::descriptor::{SerializedDescriptor};
use usbd_hid::hid_class::HIDClass;

use crate::config::{
    USB_POLLING_INTERVAL_MS,
    USB_MANUFACTURER,
    USB_PRODUCT,
    USB_VENDOR_ID,
    USB_PRODUCT_ID,
};

pub struct KeyboardUsb<'a, B: usb_device::bus::UsbBus> {
    pub usb_dev: UsbDevice<'a, B>,
    pub hid: HIDClass<'a, B>,
}

impl<'a> KeyboardUsb<'a, UsbBus> {
    pub fn init(bus_allocator: &'a UsbBusAllocator<UsbBus>) -> Self {
        let hid = HIDClass::new(
            &bus_allocator,
            usbd_hid::descriptor::KeyboardReport::desc(),
            USB_POLLING_INTERVAL_MS,
        );
    
        let usb_dev =
            UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(USB_VENDOR_ID, USB_PRODUCT_ID))
                .strings(&[StringDescriptors::new(LangID::EN)
                    .manufacturer(USB_MANUFACTURER)
                    .product(USB_PRODUCT)
                    .serial_number(USB_PRODUCT)])
                .unwrap()
                .device_class(0)
                .max_packet_size_0(64)
                .unwrap()
                .build();

        KeyboardUsb { usb_dev, hid }
    }
}
