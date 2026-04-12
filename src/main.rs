#![no_std]
#![no_main]

use defmt_rtt as _;

mod board;
mod config;
mod debounce;
mod keycodes;
mod keymaps;
mod layers;
mod macros;
mod matrix;
mod usb;

use embassy_executor::Spawner;
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

    let (board, usb_driver) = board::init();
    let (usb_dev, usb_hid) = init_usb(usb_driver);

    let matrix = Matrix::new(board);

    // タスクの起動
    spawner.spawn(usb_task(usb_dev).unwrap());
    spawner.spawn(usb_hid_task(usb_hid).unwrap());
    spawner.spawn(matrix_task(matrix).unwrap());

    defmt::info!("All tasks spawned.");
}
