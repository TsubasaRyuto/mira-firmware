use rp2040_hal::gpio::{DynPinId, FunctionSio, Pin, Pins, PullDown, PullUp, SioInput, SioOutput};
use rp2040_hal::{
    clocks::init_clocks_and_plls, pac, sio::Sio, timer::Timer, usb::UsbBus, watchdog::Watchdog,
};
use usb_device::class_prelude::*;

use crate::config::{MATRIX_COLS, MATRIX_ROWS};

const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

pub struct Board {
    pub rows: [Pin<DynPinId, FunctionSio<SioOutput>, PullDown>; MATRIX_ROWS],
    pub cols: [Pin<DynPinId, FunctionSio<SioInput>, PullUp>; MATRIX_COLS],
    pub timer: Timer,
}

pub fn init() -> (Board, UsbBusAllocator<UsbBus>) {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let row0 = pins.gpio10.into_push_pull_output();
    let row1 = pins.gpio9.into_push_pull_output();
    let col0 = pins.gpio0.into_pull_up_input();
    let col1 = pins.gpio1.into_pull_up_input();
    let col2 = pins.gpio2.into_pull_up_input();
    let col3 = pins.gpio3.into_pull_up_input();
    let col4 = pins.gpio4.into_pull_up_input();
    let col5 = pins.gpio15.into_pull_up_input();
    let col6 = pins.gpio13.into_pull_up_input();
    let col7 = pins.gpio27.into_pull_up_input();
    let col8 = pins.gpio28.into_pull_up_input();
    let col9 = pins.gpio29.into_pull_up_input();

    // Cast this Gpio pin into a dynamic pin type.
    let row0 = row0.into_dyn_pin();
    let row1 = row1.into_dyn_pin();
    let col0 = col0.into_dyn_pin();
    let col1 = col1.into_dyn_pin();
    let col2 = col2.into_dyn_pin();
    let col3 = col3.into_dyn_pin();
    let col4 = col4.into_dyn_pin();
    let col5 = col5.into_dyn_pin();
    let col6 = col6.into_dyn_pin();
    let col7 = col7.into_dyn_pin();
    let col8 = col8.into_dyn_pin();
    let col9 = col9.into_dyn_pin();

    let usb_bus = UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    );
    let usb_bus_allocator = UsbBusAllocator::new(usb_bus);

    (
        Board {
            rows: [row0, row1],
            cols: [col0, col1, col2, col3, col4, col5, col6, col7, col8, col9],
            timer,
        },
        usb_bus_allocator,
    )
}
