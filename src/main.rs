#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[unsafe(link_section = ".boot2")]
#[unsafe(no_mangle)]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

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
use matrix::Matrix;
use usb::{init_usb, usb_hid_task, usb_task};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    defmt::info!("Mira Firmware starting...");

    let (board, usb_driver) = board::init();
    let (usb_dev, usb_hid) = init_usb(usb_driver);

    let matrix = Matrix::new(board);

    // タスクの起動
    defmt::unwrap!(spawner.spawn(usb_task(usb_dev)));
    defmt::unwrap!(spawner.spawn(usb_hid_task(usb_hid)));
    defmt::unwrap!(spawner.spawn(Matrix::run(matrix)));

    defmt::info!("All tasks spawned.");
}
