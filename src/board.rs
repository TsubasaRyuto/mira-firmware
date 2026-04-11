use embassy_rp::gpio::{Input, Output};
use embassy_rp::peripherals::USB;
use embassy_rp::usb::Driver;
use embassy_rp::{Peripherals};

use crate::config::{COL_PINS, MATRIX_COLS, MATRIX_ROWS, ROW_PINS};
use crate::macros::matrix_pins;

pub struct Board {
    pub rows: [Output<'static>; MATRIX_ROWS],
    pub cols: [Input<'static>; MATRIX_COLS],
}

pub fn init() -> (Board, Driver<'static, USB>) {
    let p = embassy_rp::init(Default::default());

    let (rows, cols) = matrix_pins!(p, ROW_PINS, COL_PINS);
    let usb_driver = Driver::new(p.USB, embassy_rp::interrupt::take!(USB));

    (Board { rows, cols }, usb_driver)
}
