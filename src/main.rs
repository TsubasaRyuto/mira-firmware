#![no_std]
#![no_main]

use defmt_rtt as _;

mod debounce;
mod keyboards;
mod keycodes;
mod layers;
mod macros;
mod matrix;
mod serial;
mod usb;

use embassy_executor::Spawner;
use keyboards::active_keyboard;
use matrix::{Matrix, matrix_task};
use usb::{init_usb, usb_hid_task, usb_task};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(export_name = "_defmt_panic")]
fn defmt_panic() -> ! {
    loop {}
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    defmt::info!("Mira Firmware starting...");

    let (board, usb_driver, uart_tx, uart_rx) = active_keyboard::board::init();
    let (usb_dev, usb_hid) = init_usb(usb_driver);

    let matrix = Matrix::new(board);

    let is_split = active_keyboard::config::IS_SPLIT;
    let is_left = active_keyboard::config::IS_LEFT;

    if !is_split {
        // Normal Keyboard: USB + HID + matix scan
        spawner.spawn(usb_task(usb_dev).unwrap());
        spawner.spawn(usb_hid_task(usb_hid).unwrap());
        spawner.spawn(matrix_task(matrix).unwrap());
    } else if is_left {
        // Split Keyboard - Left (master): USB + HID + merged matrix scan + communication
        spawner.spawn(usb_task(usb_dev).unwrap());
        spawner.spawn(usb_hid_task(usb_hid).unwrap());
        spawner.spawn(matrix_task(matrix).unwrap());
        spawner.spawn(serial::serial_rx_task(uart_rx.unwrap()).unwrap());
    } else {
        // Split Keyboard - Right (slave): matrix scan + communication
        spawner.spawn(matrix_task(matrix).unwrap());
        spawner.spawn(serial::serial_tx_task(uart_tx.unwrap()).unwrap());
    }

    defmt::info!("All tasks spawned.");
}
