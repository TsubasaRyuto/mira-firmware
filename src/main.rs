#![no_std]
#![no_main]

mod board;
mod config;
mod keycodes;
mod matrix;
mod usb;

use panic_halt as _;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    loop {}
}
