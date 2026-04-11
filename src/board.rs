use embassy_rp::gpio::{Input, Output};
use embassy_rp::peripherals::USB;
use embassy_rp::usb::Driver;
use embassy_rp::{bind_interrupts};

use crate::config::{COL_PINS, MATRIX_COLS, MATRIX_ROWS, ROW_PINS};
use crate::macros::matrix_pins;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
});

pub struct Board {
    pub rows: [Output<'static>; MATRIX_ROWS],
    pub cols: [Input<'static>; MATRIX_COLS],
}

pub fn init() -> (Board, Driver<'static, USB>) {
    let p = embassy_rp::init(Default::default());

    let (rows, cols) = matrix_pins!(p, ROW_PINS, COL_PINS);
    let usb_driver = Driver::new(p.USB, Irqs);

    (Board { rows, cols }, usb_driver)
}
