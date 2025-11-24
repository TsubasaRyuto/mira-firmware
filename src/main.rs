#![no_std]
#![no_main]

#[unsafe(link_section = ".boot2")]
#[unsafe(no_mangle)]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

mod board;
mod config;
mod keycodes;
mod keymaps;
mod matrix;
mod usb;
mod debounce;

use cortex_m_rt::entry;
use panic_halt as _;

use matrix::Matrix;
use usb::KeyboardUsb;

#[entry]
fn main() -> ! {
    let (board, usb_bus_allocator) = board::init();
    let mut usb = KeyboardUsb::init(&usb_bus_allocator);
    let mut matrix = Matrix::new(board);

    loop {
        let _ = usb.usb_dev.poll(&mut [&mut usb.hid]);

        matrix.scan_matrix();
        let report = matrix.build_report();

        let _ = usb.hid.push_input(&report);
    }
}
