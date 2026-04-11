use embassy_futures::select::{select, Either};
use embassy_rp::peripherals::USB;
use embassy_rp::usb::Driver;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;
use embassy_usb::class::hid::{HidReaderWriter, State};
use embassy_usb::control::Config;
use embassy_usb::{Builder, Handler};
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};

use crate::config::{
    USB_MANUFACTURER, USB_PRODUCT, USB_PRODUCT_ID, USB_VENDOR_ID,
};

/// Channel for sending keyboard reports from the matrix task to the USB task.
pub static REPORT_CHANNEL: Channel<ThreadModeRawMutex, KeyboardReport, 8> = Channel::new();

pub struct MyDeviceHandler;

impl Handler for MyDeviceHandler {
    fn enabled(&mut self, enabled: bool) {
        if enabled {
            defmt::info!("USB enabled");
        } else {
            defmt::info!("USB disabled");
        }
    }

    fn reset(&mut self) {
        defmt::info!("USB reset");
    }

    fn addressed(&mut self, addr: u8) {
        defmt::info!("USB addressed to {}", addr);
    }

    fn configured(&mut self, configured: bool) {
        if configured {
            defmt::info!("USB configured");
        } else {
            defmt::info!("USB unconfigured");
        }
    }
}

#[embassy_executor::task]
pub async fn usb_task(mut device: embassy_usb::UsbDevice<'static, Driver<'static, USB>>) -> ! {
    device.run().await
}

#[embassy_executor::task]
pub async fn usb_hid_task(
    mut rw: HidReaderWriter<'static, Driver<'static, USB>, 1, 8>,
) -> ! {
    loop {
        let report = REPORT_CHANNEL.receive().await;
        match rw.write_serialize(&report).await {
            Ok(()) => {}
            Err(e) => defmt::warn!("Failed to send HID report: {:?}", e),
        }
    }
}

pub fn init_usb(
    driver: Driver<'static, USB>,
) -> (
    embassy_usb::UsbDevice<'static, Driver<'static, USB>>,
    HidReaderWriter<'static, Driver<'static, USB>, 1, 8>,
) {
    // Create embassy-usb Config
    let mut config = Config::new(USB_VENDOR_ID, USB_PRODUCT_ID);
    config.manufacturer = Some(USB_MANUFACTURER);
    config.product = Some(USB_PRODUCT);
    config.serial_number = Some("00000000"); // Standard serial number
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    // Create embassy-usb DeviceBuilder using the driver and config.
    // We need some static memory for the builder.
    static CONFIG_DESCRIPTOR: static_cell::StaticCell<[u8; 256]> = static_cell::StaticCell::new();
    static BOS_DESCRIPTOR: static_cell::StaticCell<[u8; 256]> = static_cell::StaticCell::new();
    static CONTROL_BUF: static_cell::StaticCell<[u8; 64]> = static_cell::StaticCell::new();

    let mut builder = Builder::new(
        driver,
        config,
        CONFIG_DESCRIPTOR.init([0; 256]),
        BOS_DESCRIPTOR.init([0; 256]),
        &mut [], // no msos
        CONTROL_BUF.init([0; 64]),
    );

    // Create HID class
    static STATE: static_cell::StaticCell<State> = static_cell::StaticCell::new();
    let config = embassy_usb::class::hid::Config {
        report_descriptor: KeyboardReport::desc(),
        request_handler: None,
        poll_ms: 1,
        max_packet_size: 8,
    };
    let rw = HidReaderWriter::new(&mut builder, STATE.init(State::new()), config);

    // Build the USB device
    let usb = builder.build();

    (usb, rw)
}
