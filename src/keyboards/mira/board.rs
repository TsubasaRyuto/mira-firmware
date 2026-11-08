use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Output};
use embassy_rp::peripherals::{PIO0, USB};
use embassy_rp::usb::Driver;
use embassy_rp::pio_programs::uart::{PioUartRx, PioUartTx};

use super::config::{COL_PINS, MATRIX_COLS, MATRIX_ROWS, ROW_PINS};
use crate::macros::matrix_pins;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
    PIO0_IRQ_0 => embassy_rp::pio::InterruptHandler<PIO0>;
});

pub struct Board {
    pub rows: [Output<'static>; MATRIX_ROWS],
    pub cols: [Input<'static>; MATRIX_COLS],
}

// --- 左手 (マスタ) の初期化 ---
#[cfg(feature = "left")]
pub fn init() -> (
    Board,
    Driver<'static, USB>,
    Option<PioUartTx<'static, PIO0, 0>>,
    Option<PioUartRx<'static, PIO0, 0>>,
) {
    let p = embassy_rp::init(Default::default());

    let (rows, cols) = matrix_pins!(p, ROW_PINS, COL_PINS);
    let usb_driver = Driver::new(p.USB, Irqs);

    // PIO0 の初期化
    let mut pio = embassy_rp::pio::Pio::new(p.PIO0, Irqs);
    // PIO UART 受信プログラムのロード
    let rx_program = embassy_rp::pio_programs::uart::PioUartRxProgram::new(&mut pio.common);
    // PIO UART 受信機の作成
    let uart_rx = PioUartRx::new(
        115_200,
        &mut pio.common,
        pio.sm0,
        p.PIN_11, // GP11 を RX として使用
        &rx_program,
    );

    (Board { rows, cols }, usb_driver, None, Some(uart_rx))
}

// --- 右手 (スレーブ) の初期化 ---
#[cfg(feature = "right")]
pub fn init() -> (
    Board,
    Driver<'static, USB>,
    Option<PioUartTx<'static, PIO0, 0>>,
    Option<PioUartRx<'static, PIO0, 0>>,
) {
    let p = embassy_rp::init(Default::default());

    let (rows, cols) = matrix_pins!(p, ROW_PINS, COL_PINS);
    let usb_driver = Driver::new(p.USB, Irqs);

    // PIO0 の初期化
    let mut pio = embassy_rp::pio::Pio::new(p.PIO0, Irqs);
    // PIO UART 送信プログラムのロード
    let tx_program = embassy_rp::pio_programs::uart::PioUartTxProgram::new(&mut pio.common);
    // PIO UART 送信機の作成
    let uart_tx = PioUartTx::new(
        115_200,
        &mut pio.common,
        pio.sm0,
        p.PIN_11, // GP11 を TX として使用
        &tx_program,
    );

    (Board { rows, cols }, usb_driver, Some(uart_tx), None)
}
