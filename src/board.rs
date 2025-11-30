use rp2040_hal::gpio::{DynPinId, FunctionSio, Pin, Pins, PullDown, PullUp, SioInput, SioOutput};
use rp2040_hal::{
    clocks::init_clocks_and_plls, pac, sio::Sio, timer::Timer, usb::UsbBus, watchdog::Watchdog,
};
use usb_device::class_prelude::*;

use crate::config::{COL_PINS, MATRIX_COLS, MATRIX_ROWS, ROW_PINS};
use crate::macros::matrix_pins;

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

    let usb_bus = UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    );
    let usb_bus_allocator = UsbBusAllocator::new(usb_bus);

    let (rows, cols) = matrix_pins!(pins, ROW_PINS, COL_PINS);

    (Board { rows, cols, timer }, usb_bus_allocator)
}
